use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use std::collections::BTreeMap;

use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use craw_chat_contract_core::ContractError;
use craw_chat_contract_stream::{StreamStateRecord, StreamStateStore};
use im_auth_context::{
    AuthContext, AuthContextError, resolve_auth_context, resolve_public_bearer_auth_context,
};
use im_domain_core::message::Sender;
use im_domain_core::stream::{
    StreamDurabilityClass, StreamFrame, StreamSession, StreamSessionState,
};
use im_platform_contracts::DeviceSubject;
use im_time::utc_now_rfc3339_millis;
use serde::{Deserialize, Serialize};

const DEVICE_SCOPE_KIND: &str = "device";
const DEVICE_TELEMETRY_STREAM_TYPE: &str = "device.telemetry";

#[derive(Clone)]
struct AppState {
    runtime: Arc<StreamingRuntime>,
}

pub struct StreamingRuntime {
    sessions: Mutex<HashMap<String, StreamSession>>,
    frames: Mutex<HashMap<String, Vec<StreamFrame>>>,
    state_store: Arc<dyn StreamStateStore>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenStreamRequest {
    pub stream_id: String,
    pub stream_type: String,
    pub scope_kind: String,
    pub scope_id: String,
    pub durability_class: String,
    pub schema_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckpointStreamRequest {
    pub frame_seq: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteStreamRequest {
    pub frame_seq: u64,
    pub result_message_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbortStreamRequest {
    pub frame_seq: Option<u64>,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppendStreamFrameRequest {
    pub frame_seq: u64,
    pub frame_type: String,
    pub schema_ref: Option<String>,
    pub encoding: String,
    pub payload: String,
    #[serde(default)]
    pub attributes: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListStreamFramesQuery {
    pub after_frame_seq: Option<u64>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamFrameWindow {
    pub items: Vec<StreamFrame>,
    pub next_after_frame_seq: Option<u64>,
    pub has_more: bool,
}

#[derive(Debug)]
pub struct StreamingError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl StreamingError {
    pub fn status(&self) -> axum::http::StatusCode {
        self.status
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    fn conflict(stream_id: &str) -> Self {
        Self {
            status: axum::http::StatusCode::CONFLICT,
            code: "stream_conflict",
            message: format!(
                "stream open request conflicts with existing stream idempotency key: {stream_id}"
            ),
        }
    }

    fn stream_store(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "stream_store_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                code: "stream_store_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                status: axum::http::StatusCode::NOT_IMPLEMENTED,
                code: "stream_store_unsupported",
                message,
            },
        }
    }
}

impl axum::response::IntoResponse for StreamingError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status,
            Json(serde_json::json!({
                "code": self.code,
                "message": self.message
            })),
        )
            .into_response()
    }
}

impl StreamingRuntime {
    pub fn with_store(state_store: Arc<dyn StreamStateStore>) -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            frames: Mutex::new(HashMap::new()),
            state_store,
        }
    }

    fn ensure_stream_state(&self, tenant_id: &str, stream_id: &str) -> Result<(), StreamingError> {
        let scope_key = stream_scope_key(tenant_id, stream_id);
        let needs_restore =
            !lock_stream_mutex(&self.sessions, "stream runtime").contains_key(scope_key.as_str());
        if !needs_restore {
            lock_stream_mutex(&self.frames, "stream runtime")
                .entry(scope_key)
                .or_default();
            return Ok(());
        }

        let restored = self
            .state_store
            .load_state(tenant_id, stream_id)
            .map_err(StreamingError::stream_store)?;
        if let Some(record) = restored {
            lock_stream_mutex(&self.sessions, "stream runtime")
                .insert(scope_key.clone(), record.session);
            lock_stream_mutex(&self.frames, "stream runtime").insert(scope_key, record.frames);
        }

        Ok(())
    }

    pub fn session(
        &self,
        auth: &AuthContext,
        stream_id: &str,
    ) -> Result<StreamSession, StreamingError> {
        self.ensure_stream_state(auth.tenant_id.as_str(), stream_id)?;
        lock_stream_mutex(&self.sessions, "stream runtime")
            .get(stream_scope_key(auth.tenant_id.as_str(), stream_id).as_str())
            .cloned()
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "stream_not_found",
                message: format!("stream not found: {stream_id}"),
            })
    }

    pub fn open_stream(
        &self,
        auth: &AuthContext,
        request: OpenStreamRequest,
    ) -> Result<StreamSession, StreamingError> {
        let durability_class = match request.durability_class.as_str() {
            "transient" => StreamDurabilityClass::Transient,
            "durableSession" => StreamDurabilityClass::DurableSession,
            "eventLog" => StreamDurabilityClass::EventLog,
            other => {
                return Err(StreamingError {
                    status: axum::http::StatusCode::BAD_REQUEST,
                    code: "invalid_durability_class",
                    message: format!("unsupported durability class: {other}"),
                });
            }
        };

        let scope_key = stream_scope_key(auth.tenant_id.as_str(), request.stream_id.as_str());
        self.ensure_stream_state(auth.tenant_id.as_str(), request.stream_id.as_str())?;
        let mut sessions = lock_stream_mutex(&self.sessions, "stream runtime");
        if let Some(existing) = sessions.get(scope_key.as_str()).cloned() {
            if stream_session_matches_open_request(&existing, &request, &durability_class) {
                drop(sessions);
                lock_stream_mutex(&self.frames, "stream runtime")
                    .entry(scope_key)
                    .or_default();
                return Ok(existing);
            }

            return Err(StreamingError::conflict(request.stream_id.as_str()));
        }

        let opened_at = utc_now_rfc3339_millis();
        let session = StreamSession {
            tenant_id: auth.tenant_id.clone(),
            stream_id: request.stream_id.clone(),
            stream_type: request.stream_type,
            scope_kind: request.scope_kind,
            scope_id: request.scope_id,
            durability_class,
            ordering_scope: "stream".into(),
            schema_ref: request.schema_ref,
            state: StreamSessionState::Opened,
            last_frame_seq: 0,
            last_checkpoint_seq: None,
            result_message_id: None,
            opened_at,
            closed_at: None,
            expires_at: None,
        };

        sessions.insert(scope_key.clone(), session.clone());
        drop(sessions);
        lock_stream_mutex(&self.frames, "stream runtime")
            .entry(scope_key)
            .or_default();
        self.persist_state(auth.tenant_id.as_str(), request.stream_id.as_str())?;

        Ok(session)
    }

    pub fn append_frame(
        &self,
        auth: &AuthContext,
        stream_id: &str,
        request: AppendStreamFrameRequest,
    ) -> Result<StreamFrame, StreamingError> {
        if request.frame_seq == 0 {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "invalid_frame_seq",
                message: "frameSeq must start from 1".into(),
            });
        }

        let scope_key = stream_scope_key(auth.tenant_id.as_str(), stream_id);
        self.ensure_stream_state(auth.tenant_id.as_str(), stream_id)?;
        let mut sessions = lock_stream_mutex(&self.sessions, "stream runtime");
        let session = sessions
            .get_mut(scope_key.as_str())
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "stream_not_found",
                message: format!("stream not found: {stream_id}"),
            })?;

        if matches!(
            session.state,
            StreamSessionState::Completed | StreamSessionState::Aborted
        ) {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "stream_state_invalid",
                message: format!("stream is already closed: {stream_id}"),
            });
        }

        let sender = resolve_stream_frame_sender(auth, session);

        let mut frames = lock_stream_mutex(&self.frames, "stream runtime");
        let stream_frames = frames.entry(scope_key).or_default();

        if let Some(existing) = stream_frames
            .iter()
            .find(|frame| frame.frame_seq == request.frame_seq)
        {
            let is_same_retry = existing.frame_type == request.frame_type
                && existing.schema_ref == request.schema_ref
                && existing.encoding == request.encoding
                && existing.payload == request.payload
                && existing.sender == sender
                && existing.attributes == request.attributes;
            if is_same_retry {
                return Ok(existing.clone());
            }
            return Err(StreamingError {
                status: axum::http::StatusCode::CONFLICT,
                code: "stream_frame_conflict",
                message: format!("frame seq conflict: {}", request.frame_seq),
            });
        }

        if request.frame_seq != session.last_frame_seq + 1 {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "stream_frame_out_of_order",
                message: format!(
                    "expected next frame seq {}, got {}",
                    session.last_frame_seq + 1,
                    request.frame_seq
                ),
            });
        }

        let occurred_at = utc_now_rfc3339_millis();
        let frame = StreamFrame {
            tenant_id: auth.tenant_id.clone(),
            stream_id: session.stream_id.clone(),
            stream_type: session.stream_type.clone(),
            scope_kind: session.scope_kind.clone(),
            scope_id: session.scope_id.clone(),
            frame_seq: request.frame_seq,
            frame_type: request.frame_type,
            schema_ref: request.schema_ref,
            encoding: request.encoding,
            payload: request.payload,
            sender,
            attributes: request.attributes,
            occurred_at,
        };

        stream_frames.push(frame.clone());
        session.last_frame_seq = frame.frame_seq;
        session.state = StreamSessionState::Active;
        drop(frames);
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), stream_id)?;

        Ok(frame)
    }

    pub fn checkpoint_stream(
        &self,
        auth: &AuthContext,
        stream_id: &str,
        request: CheckpointStreamRequest,
    ) -> Result<StreamSession, StreamingError> {
        self.ensure_stream_state(auth.tenant_id.as_str(), stream_id)?;
        let mut sessions = lock_stream_mutex(&self.sessions, "stream runtime");
        let session = sessions
            .get_mut(stream_scope_key(auth.tenant_id.as_str(), stream_id).as_str())
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "stream_not_found",
                message: format!("stream not found: {stream_id}"),
            })?;

        if matches!(
            session.state,
            StreamSessionState::Completed | StreamSessionState::Aborted
        ) {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "stream_state_invalid",
                message: format!("stream is already closed: {stream_id}"),
            });
        }

        session.last_frame_seq = session.last_frame_seq.max(request.frame_seq);
        session.last_checkpoint_seq = Some(request.frame_seq);
        session.state = StreamSessionState::Checkpointed;
        let session = session.clone();
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), stream_id)?;

        Ok(session)
    }

    pub fn complete_stream(
        &self,
        auth: &AuthContext,
        stream_id: &str,
        request: CompleteStreamRequest,
    ) -> Result<StreamSession, StreamingError> {
        self.ensure_stream_state(auth.tenant_id.as_str(), stream_id)?;
        let mut sessions = lock_stream_mutex(&self.sessions, "stream runtime");
        let session = sessions
            .get_mut(stream_scope_key(auth.tenant_id.as_str(), stream_id).as_str())
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "stream_not_found",
                message: format!("stream not found: {stream_id}"),
            })?;

        if matches!(
            session.state,
            StreamSessionState::Completed | StreamSessionState::Aborted
        ) {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "stream_state_invalid",
                message: format!("stream is already closed: {stream_id}"),
            });
        }

        session.last_frame_seq = session.last_frame_seq.max(request.frame_seq);
        session.last_checkpoint_seq = Some(request.frame_seq);
        session.result_message_id = request.result_message_id;
        session.state = StreamSessionState::Completed;
        session.closed_at = Some(utc_now_rfc3339_millis());
        let session = session.clone();
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), stream_id)?;

        Ok(session)
    }

    pub fn abort_stream(
        &self,
        auth: &AuthContext,
        stream_id: &str,
        request: AbortStreamRequest,
    ) -> Result<StreamSession, StreamingError> {
        self.ensure_stream_state(auth.tenant_id.as_str(), stream_id)?;
        let mut sessions = lock_stream_mutex(&self.sessions, "stream runtime");
        let session = sessions
            .get_mut(stream_scope_key(auth.tenant_id.as_str(), stream_id).as_str())
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "stream_not_found",
                message: format!("stream not found: {stream_id}"),
            })?;

        if matches!(
            session.state,
            StreamSessionState::Completed | StreamSessionState::Aborted
        ) {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "stream_state_invalid",
                message: format!("stream is already closed: {stream_id}"),
            });
        }

        if let Some(frame_seq) = request.frame_seq {
            session.last_frame_seq = session.last_frame_seq.max(frame_seq);
            session.last_checkpoint_seq = Some(frame_seq);
        }
        let _reason = request.reason;
        session.state = StreamSessionState::Aborted;
        session.closed_at = Some(utc_now_rfc3339_millis());
        let session = session.clone();
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), stream_id)?;

        Ok(session)
    }

    pub fn list_frames(
        &self,
        auth: &AuthContext,
        stream_id: &str,
        query: ListStreamFramesQuery,
    ) -> Result<StreamFrameWindow, StreamingError> {
        self.ensure_stream_state(auth.tenant_id.as_str(), stream_id)?;
        let after_frame_seq = query.after_frame_seq.unwrap_or(0);
        let limit = query.limit.unwrap_or(100);
        if limit == 0 {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "invalid_limit",
                message: "limit must be greater than 0".into(),
            });
        }

        let scope_key = stream_scope_key(auth.tenant_id.as_str(), stream_id);
        if !lock_stream_mutex(&self.sessions, "stream runtime").contains_key(scope_key.as_str()) {
            return Err(StreamingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "stream_not_found",
                message: format!("stream not found: {stream_id}"),
            });
        }

        let frames = lock_stream_mutex(&self.frames, "stream runtime");
        let items: Vec<StreamFrame> = frames
            .get(scope_key.as_str())
            .into_iter()
            .flat_map(|stream_frames| stream_frames.iter())
            .filter(|frame| frame.frame_seq > after_frame_seq)
            .take(limit)
            .cloned()
            .collect();

        let total_after = frames
            .get(scope_key.as_str())
            .map(|stream_frames| {
                stream_frames
                    .iter()
                    .filter(|frame| frame.frame_seq > after_frame_seq)
                    .count()
            })
            .unwrap_or(0);
        let has_more = total_after > items.len();
        let next_after_frame_seq = items.last().map(|frame| frame.frame_seq);

        Ok(StreamFrameWindow {
            items,
            next_after_frame_seq,
            has_more,
        })
    }

    fn persist_state(&self, tenant_id: &str, stream_id: &str) -> Result<(), StreamingError> {
        self.state_store
            .save_state(self.state_record(tenant_id, stream_id)?)
            .map_err(StreamingError::stream_store)
    }

    fn state_record(
        &self,
        tenant_id: &str,
        stream_id: &str,
    ) -> Result<StreamStateRecord, StreamingError> {
        let scope_key = stream_scope_key(tenant_id, stream_id);
        let session = lock_stream_mutex(&self.sessions, "stream runtime")
            .get(scope_key.as_str())
            .cloned()
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::CONFLICT,
                code: "stream_state_inconsistent",
                message: format!(
                    "stream session missing while persisting state for stream id: {stream_id}"
                ),
            })?;
        let frames = lock_stream_mutex(&self.frames, "stream runtime")
            .get(scope_key.as_str())
            .cloned()
            .unwrap_or_default();

        Ok(StreamStateRecord {
            tenant_id: tenant_id.into(),
            stream_id: stream_id.into(),
            session,
            frames,
            updated_at: utc_now_rfc3339_millis(),
        })
    }
}

#[derive(Clone, Default)]
struct RuntimeMemoryStreamStateStore {
    states: Arc<Mutex<HashMap<String, StreamStateRecord>>>,
}

impl StreamStateStore for RuntimeMemoryStreamStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        stream_id: &str,
    ) -> Result<Option<StreamStateRecord>, ContractError> {
        Ok(lock_stream_mutex(&self.states, "stream state store")
            .get(stream_scope_key(tenant_id, stream_id).as_str())
            .cloned())
    }

    fn save_state(&self, record: StreamStateRecord) -> Result<(), ContractError> {
        lock_stream_mutex(&self.states, "stream state store").insert(
            stream_scope_key(record.tenant_id.as_str(), record.stream_id.as_str()),
            record,
        );
        Ok(())
    }

    fn clear_state(&self, tenant_id: &str, stream_id: &str) -> Result<bool, ContractError> {
        Ok(lock_stream_mutex(&self.states, "stream state store")
            .remove(stream_scope_key(tenant_id, stream_id).as_str())
            .is_some())
    }
}

impl Default for StreamingRuntime {
    fn default() -> Self {
        Self::with_store(Arc::new(RuntimeMemoryStreamStateStore::default()))
    }
}

impl From<AuthContextError> for StreamingError {
    fn from(value: AuthContextError) -> Self {
        Self {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

pub fn build_default_app() -> Router {
    build_app(Arc::new(StreamingRuntime::default()))
}

pub fn build_public_app() -> Router {
    build_default_app().layer(middleware::from_fn(require_public_bearer_auth))
}

pub fn build_app(runtime: Arc<StreamingRuntime>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/api/v1/streams", post(open_stream))
        .route(
            "/api/v1/streams/{stream_id}/frames",
            post(append_stream_frame).get(list_stream_frames),
        )
        .route(
            "/api/v1/streams/{stream_id}/checkpoint",
            post(checkpoint_stream),
        )
        .route(
            "/api/v1/streams/{stream_id}/complete",
            post(complete_stream),
        )
        .route("/api/v1/streams/{stream_id}/abort", post(abort_stream))
        .with_state(AppState { runtime })
}

async fn require_public_bearer_auth(request: Request<axum::body::Body>, next: Next) -> Response {
    match request.uri().path() {
        "/healthz" | "/readyz" => next.run(request).await,
        _ => match resolve_public_bearer_auth_context(request.headers()) {
            Ok(_) => next.run(request).await,
            Err(error) => StreamingError::from(error).into_response(),
        },
    }
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "streaming-service",
    })
}

async fn readyz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "streaming-service",
    })
}

async fn open_stream(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<OpenStreamRequest>,
) -> Result<Json<StreamSession>, StreamingError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_standalone_stream_open_allowed(&request)?;
    Ok(Json(state.runtime.open_stream(&auth, request)?))
}

async fn checkpoint_stream(
    Path(stream_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CheckpointStreamRequest>,
) -> Result<Json<StreamSession>, StreamingError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
    Ok(Json(state.runtime.checkpoint_stream(
        &auth,
        stream_id.as_str(),
        request,
    )?))
}

async fn append_stream_frame(
    Path(stream_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AppendStreamFrameRequest>,
) -> Result<Json<StreamFrame>, StreamingError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
    Ok(Json(state.runtime.append_frame(
        &auth,
        stream_id.as_str(),
        request,
    )?))
}

async fn list_stream_frames(
    Path(stream_id): Path<String>,
    Query(query): Query<ListStreamFramesQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<StreamFrameWindow>, StreamingError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
    Ok(Json(state.runtime.list_frames(
        &auth,
        stream_id.as_str(),
        query,
    )?))
}

async fn complete_stream(
    Path(stream_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CompleteStreamRequest>,
) -> Result<Json<StreamSession>, StreamingError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
    Ok(Json(state.runtime.complete_stream(
        &auth,
        stream_id.as_str(),
        request,
    )?))
}

async fn abort_stream(
    Path(stream_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AbortStreamRequest>,
) -> Result<Json<StreamSession>, StreamingError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
    Ok(Json(state.runtime.abort_stream(
        &auth,
        stream_id.as_str(),
        request,
    )?))
}

fn lock_stream_mutex<'a, T>(mutex: &'a Mutex<T>, lock_name: &'static str) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("warn: recovered poisoned stream mutex lock={lock_name}");
            poisoned.into_inner()
        }
    }
}

fn stream_scope_key(tenant_id: &str, stream_id: &str) -> String {
    format!("{tenant_id}:{stream_id}")
}

fn ensure_standalone_stream_open_allowed(
    request: &OpenStreamRequest,
) -> Result<(), StreamingError> {
    if request.scope_kind != "conversation" && request.scope_kind != DEVICE_SCOPE_KIND {
        return Ok(());
    }

    let message = if request.scope_kind == DEVICE_SCOPE_KIND {
        "device-bound streams must be opened through an authorizing IM gateway"
    } else {
        "conversation-bound streams must be opened through an authorizing IM gateway"
    };
    Err(conversation_gateway_required(message))
}

fn ensure_standalone_stream_session_allowed(
    runtime: &StreamingRuntime,
    auth: &AuthContext,
    stream_id: &str,
) -> Result<(), StreamingError> {
    let session = runtime.session(auth, stream_id)?;
    if session.scope_kind != "conversation" && session.scope_kind != DEVICE_SCOPE_KIND {
        return Ok(());
    }

    let message = if session.scope_kind == DEVICE_SCOPE_KIND {
        "device-bound streams must be accessed through an authorizing IM gateway"
    } else {
        "conversation-bound streams must be accessed through an authorizing IM gateway"
    };
    Err(conversation_gateway_required(message))
}

fn conversation_gateway_required(message: &str) -> StreamingError {
    StreamingError {
        status: axum::http::StatusCode::FORBIDDEN,
        code: "conversation_gateway_required",
        message: message.into(),
    }
}

fn stream_session_matches_open_request(
    session: &StreamSession,
    request: &OpenStreamRequest,
    durability_class: &StreamDurabilityClass,
) -> bool {
    session.stream_id == request.stream_id.as_str()
        && session.stream_type == request.stream_type.as_str()
        && session.scope_kind == request.scope_kind.as_str()
        && session.scope_id == request.scope_id.as_str()
        && session.durability_class == *durability_class
        && session.schema_ref.as_ref() == request.schema_ref.as_ref()
}

fn resolve_stream_frame_sender(auth: &AuthContext, session: &StreamSession) -> Sender {
    if session.scope_kind == DEVICE_SCOPE_KIND
        && session.stream_type == DEVICE_TELEMETRY_STREAM_TYPE
        && auth.actor_kind == "device"
    {
        let device_id = auth
            .device_id
            .clone()
            .unwrap_or_else(|| session.scope_id.clone());
        return DeviceSubject {
            device_id,
            owner_principal_id: Some(auth.actor_id.clone()),
            session_id: auth.session_id.clone(),
            metadata: BTreeMap::new(),
        }
        .sender(None);
    }

    Sender {
        id: auth.actor_id.clone(),
        kind: auth.actor_kind.clone(),
        member_id: None,
        device_id: auth.device_id.clone(),
        session_id: auth.session_id.clone(),
        metadata: BTreeMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_stream_state_recovers_from_poisoned_sessions_lock() {
        let runtime = StreamingRuntime::default();
        let _ = std::panic::catch_unwind(|| {
            let _guard = runtime.sessions.lock().expect("stream runtime should lock");
            panic!("poison stream runtime sessions lock");
        });

        runtime
            .ensure_stream_state("t_demo", "st_poison")
            .expect("poisoned sessions lock should be recovered");
    }

    #[test]
    fn test_runtime_memory_state_store_load_recovers_from_poisoned_lock() {
        let store = RuntimeMemoryStreamStateStore::default();
        let _ = std::panic::catch_unwind(|| {
            let _guard = store.states.lock().expect("stream state store should lock");
            panic!("poison stream state store lock");
        });

        let restored = store
            .load_state("t_demo", "st_poison")
            .expect("poisoned state store lock should be recovered");
        assert!(restored.is_none());
    }
}
