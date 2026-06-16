use std::collections::HashMap;
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::{Arc, Mutex, MutexGuard};

use std::collections::BTreeMap;

use axum::extract::{DefaultBodyLimit, Extension, Path, Query, State};
use axum::http::HeaderMap;
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use im_app_context::{
    AppContext, AppContextError, resolve_app_context, resolve_app_context_for_request,
};
use im_domain_core::message::Sender;
use im_domain_core::stream::{
    StreamDurabilityClass, StreamFrame, StreamSession, StreamSessionState,
};
use im_time::utc_now_rfc3339_millis;
use sdkwork_im_api_registry::HttpMethod;
use sdkwork_im_contract_core::ContractError;
use sdkwork_im_contract_stream::{StreamStateRecord, StreamStateStore};
use sdkwork_im_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;

const STREAM_SESSION_DELIVERY_PROOF_VERSION: &str = "stream.session.delivery-proof.v1";
const STREAM_FRAME_DELIVERY_PROOF_VERSION: &str = "stream.frame.delivery-proof.v1";
const STREAM_MAX_STREAM_ID_BYTES: usize = 256;
const STREAM_MAX_STREAM_TYPE_BYTES: usize = 128;
const STREAM_MAX_SCOPE_KIND_BYTES: usize = 64;
const STREAM_MAX_SCOPE_ID_BYTES: usize = 512;
const STREAM_MAX_DURABILITY_CLASS_BYTES: usize = 64;
const STREAM_MAX_SCHEMA_REF_BYTES: usize = 256;
const STREAM_MAX_FRAME_TYPE_BYTES: usize = 64;
const STREAM_MAX_FRAME_ENCODING_BYTES: usize = 32;
const STREAM_MAX_FRAME_PAYLOAD_BYTES: usize = 256 * 1024;
const STREAM_MAX_FRAME_ATTRIBUTES_BYTES: usize = 64 * 1024;
const STREAM_MAX_RESULT_MESSAGE_ID_BYTES: usize = 256;
const STREAM_MAX_ABORT_REASON_BYTES: usize = 8 * 1024;
const STREAM_FRAME_LIST_MAX_LIMIT: usize = 1000;
const STREAMING_MAX_IN_FLIGHT_REQUESTS_ENV: &str = "SDKWORK_IM_STREAMING_MAX_IN_FLIGHT_REQUESTS";
const STREAMING_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 1_000;
const STREAMING_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 20_000;
const STREAMING_MAX_REQUEST_BODY_BYTES_ENV: &str = "SDKWORK_IM_STREAMING_MAX_REQUEST_BODY_BYTES";
const STREAMING_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
const STREAMING_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;
const STREAMING_REQUIRE_DUAL_TOKEN_HEADERS_ENV: &str =
    "SDKWORK_IM_STREAMING_REQUIRE_DUAL_TOKEN_HEADERS";

#[derive(Clone)]
struct AppState {
    runtime: Arc<StreamingRuntime>,
}

#[derive(Clone)]
struct PublicAppGuardrails {
    request_gate: Arc<Semaphore>,
    require_dual_token_headers: bool,
}

pub struct StreamingRuntime {
    sessions: Mutex<HashMap<String, StreamSession>>,
    frames: Mutex<HashMap<String, BTreeMap<u64, StreamFrame>>>,
    state_store: Arc<dyn StreamStateStore>,
}

#[derive(Clone, Debug)]
pub struct AppendStreamFrameOutcome {
    pub frame: StreamFrame,
    pub applied: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamFrameDeliveryStatus {
    Applied,
    Replayed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamFrameMutationResponse {
    #[serde(flatten)]
    pub frame: StreamFrame,
    pub request_key: String,
    pub delivery_status: StreamFrameDeliveryStatus,
    pub proof_version: String,
}

impl StreamFrameMutationResponse {
    pub fn from_outcome(outcome: AppendStreamFrameOutcome, request_key: String) -> Self {
        Self {
            frame: outcome.frame,
            request_key,
            delivery_status: if outcome.applied {
                StreamFrameDeliveryStatus::Applied
            } else {
                StreamFrameDeliveryStatus::Replayed
            },
            proof_version: STREAM_FRAME_DELIVERY_PROOF_VERSION.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StreamSessionMutationOutcome {
    pub session: StreamSession,
    pub applied: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamSessionDeliveryStatus {
    Applied,
    Replayed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamSessionMutationResponse {
    #[serde(flatten)]
    pub session: StreamSession,
    pub request_key: String,
    pub delivery_status: StreamSessionDeliveryStatus,
    pub proof_version: String,
}

impl StreamSessionMutationResponse {
    pub fn from_outcome(outcome: StreamSessionMutationOutcome, request_key: String) -> Self {
        Self {
            session: outcome.session,
            request_key,
            delivery_status: if outcome.applied {
                StreamSessionDeliveryStatus::Applied
            } else {
                StreamSessionDeliveryStatus::Replayed
            },
            proof_version: STREAM_SESSION_DELIVERY_PROOF_VERSION.into(),
        }
    }
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
    fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code,
            message: message.into(),
        }
    }

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

    fn payload_too_large(field: &'static str, max_bytes: usize, actual_bytes: usize) -> Self {
        Self {
            status: axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
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
        validate_stream_id(stream_id)?;
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
            lock_stream_mutex(&self.frames, "stream runtime")
                .insert(scope_key, stream_frame_index(record.frames));
        }

        Ok(())
    }

    pub fn session(
        &self,
        auth: &AppContext,
        stream_id: &str,
    ) -> Result<StreamSession, StreamingError> {
        self.ensure_stream_state(auth.tenant_id.as_str(), stream_id)?;
        let session = lock_stream_mutex(&self.sessions, "stream runtime")
            .get(stream_scope_key(auth.tenant_id.as_str(), stream_id).as_str())
            .cloned()
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "stream_not_found",
                message: format!("stream not found: {stream_id}"),
            })?;
        ensure_stream_session_actor_access(&session, auth, stream_id)?;
        Ok(session)
    }

    pub fn open_stream(
        &self,
        auth: &AppContext,
        request: OpenStreamRequest,
    ) -> Result<StreamSession, StreamingError> {
        Ok(self.open_stream_with_outcome(auth, request)?.session)
    }

    pub fn open_stream_with_outcome(
        &self,
        auth: &AppContext,
        request: OpenStreamRequest,
    ) -> Result<StreamSessionMutationOutcome, StreamingError> {
        validate_open_stream_request_payload_size(&request)?;
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
            if stream_session_matches_open_request(&existing, auth, &request, &durability_class) {
                drop(sessions);
                lock_stream_mutex(&self.frames, "stream runtime")
                    .entry(scope_key)
                    .or_default();
                return Ok(StreamSessionMutationOutcome {
                    session: existing,
                    applied: false,
                });
            }

            return Err(StreamingError::conflict(request.stream_id.as_str()));
        }

        let opened_at = utc_now_rfc3339_millis();
        let session = StreamSession {
            tenant_id: auth.tenant_id.clone(),
            stream_id: request.stream_id.clone(),
            owner_principal_id: auth.actor_id.clone(),
            owner_principal_kind: auth.actor_kind.clone(),
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
            complete_frame_seq: None,
            abort_frame_seq: None,
            abort_reason: None,
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

        Ok(StreamSessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub fn append_frame(
        &self,
        auth: &AppContext,
        stream_id: &str,
        request: AppendStreamFrameRequest,
    ) -> Result<StreamFrame, StreamingError> {
        Ok(self
            .append_frame_with_outcome(auth, stream_id, request)?
            .frame)
    }

    pub fn append_frame_with_outcome(
        &self,
        auth: &AppContext,
        stream_id: &str,
        request: AppendStreamFrameRequest,
    ) -> Result<AppendStreamFrameOutcome, StreamingError> {
        validate_append_frame_request_payload_size(&request)?;
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
        ensure_stream_session_actor_access(session, auth, stream_id)?;

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

        let sender = resolve_stream_frame_sender(auth);

        let mut frames = lock_stream_mutex(&self.frames, "stream runtime");
        let stream_frames = frames.entry(scope_key).or_default();

        if let Some(existing) = stream_frames.get(&request.frame_seq) {
            let is_same_retry = existing.frame_type == request.frame_type
                && existing.schema_ref == request.schema_ref
                && existing.encoding == request.encoding
                && existing.payload == request.payload
                && existing.sender == sender
                && existing.attributes == request.attributes;
            if is_same_retry {
                return Ok(AppendStreamFrameOutcome {
                    frame: existing.clone(),
                    applied: false,
                });
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

        stream_frames.insert(frame.frame_seq, frame.clone());
        session.last_frame_seq = frame.frame_seq;
        session.state = StreamSessionState::Active;
        drop(frames);
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), stream_id)?;

        Ok(AppendStreamFrameOutcome {
            frame,
            applied: true,
        })
    }

    pub fn checkpoint_stream(
        &self,
        auth: &AppContext,
        stream_id: &str,
        request: CheckpointStreamRequest,
    ) -> Result<StreamSession, StreamingError> {
        Ok(self
            .checkpoint_stream_with_outcome(auth, stream_id, request)?
            .session)
    }

    pub fn checkpoint_stream_with_outcome(
        &self,
        auth: &AppContext,
        stream_id: &str,
        request: CheckpointStreamRequest,
    ) -> Result<StreamSessionMutationOutcome, StreamingError> {
        self.ensure_stream_state(auth.tenant_id.as_str(), stream_id)?;
        let mut sessions = lock_stream_mutex(&self.sessions, "stream runtime");
        let session = sessions
            .get_mut(stream_scope_key(auth.tenant_id.as_str(), stream_id).as_str())
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "stream_not_found",
                message: format!("stream not found: {stream_id}"),
            })?;
        ensure_stream_session_actor_access(session, auth, stream_id)?;

        if session.last_checkpoint_seq == Some(request.frame_seq) {
            if stream_checkpoint_matches_request(session, auth, &request) {
                return Ok(StreamSessionMutationOutcome {
                    session: session.clone(),
                    applied: false,
                });
            }
            return Err(StreamingError::conflict(stream_id));
        }

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

        if session
            .last_checkpoint_seq
            .is_some_and(|last_checkpoint_seq| request.frame_seq < last_checkpoint_seq)
        {
            return Err(StreamingError::conflict(stream_id));
        }

        session.last_frame_seq = session.last_frame_seq.max(request.frame_seq);
        session.last_checkpoint_seq = Some(request.frame_seq);
        session.state = StreamSessionState::Checkpointed;
        let session = session.clone();
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), stream_id)?;

        Ok(StreamSessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub fn complete_stream(
        &self,
        auth: &AppContext,
        stream_id: &str,
        request: CompleteStreamRequest,
    ) -> Result<StreamSession, StreamingError> {
        Ok(self
            .complete_stream_with_outcome(auth, stream_id, request)?
            .session)
    }

    pub fn complete_stream_with_outcome(
        &self,
        auth: &AppContext,
        stream_id: &str,
        request: CompleteStreamRequest,
    ) -> Result<StreamSessionMutationOutcome, StreamingError> {
        validate_complete_stream_request_payload_size(&request)?;
        self.ensure_stream_state(auth.tenant_id.as_str(), stream_id)?;
        let mut sessions = lock_stream_mutex(&self.sessions, "stream runtime");
        let session = sessions
            .get_mut(stream_scope_key(auth.tenant_id.as_str(), stream_id).as_str())
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "stream_not_found",
                message: format!("stream not found: {stream_id}"),
            })?;
        ensure_stream_session_actor_access(session, auth, stream_id)?;

        if session.state == StreamSessionState::Completed {
            if stream_completion_matches_request(session, auth, &request) {
                return Ok(StreamSessionMutationOutcome {
                    session: session.clone(),
                    applied: false,
                });
            }
            return Err(StreamingError::conflict(stream_id));
        }

        if session.state == StreamSessionState::Aborted {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "stream_state_invalid",
                message: format!("stream is already closed: {stream_id}"),
            });
        }

        session.last_frame_seq = session.last_frame_seq.max(request.frame_seq);
        session.result_message_id = request.result_message_id;
        session.complete_frame_seq = Some(request.frame_seq);
        session.state = StreamSessionState::Completed;
        session.closed_at = Some(utc_now_rfc3339_millis());
        let session = session.clone();
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), stream_id)?;

        Ok(StreamSessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub fn abort_stream(
        &self,
        auth: &AppContext,
        stream_id: &str,
        request: AbortStreamRequest,
    ) -> Result<StreamSession, StreamingError> {
        Ok(self
            .abort_stream_with_outcome(auth, stream_id, request)?
            .session)
    }

    pub fn abort_stream_with_outcome(
        &self,
        auth: &AppContext,
        stream_id: &str,
        request: AbortStreamRequest,
    ) -> Result<StreamSessionMutationOutcome, StreamingError> {
        validate_abort_stream_request_payload_size(&request)?;
        self.ensure_stream_state(auth.tenant_id.as_str(), stream_id)?;
        let mut sessions = lock_stream_mutex(&self.sessions, "stream runtime");
        let session = sessions
            .get_mut(stream_scope_key(auth.tenant_id.as_str(), stream_id).as_str())
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "stream_not_found",
                message: format!("stream not found: {stream_id}"),
            })?;
        ensure_stream_session_actor_access(session, auth, stream_id)?;

        if session.state == StreamSessionState::Aborted {
            if stream_abort_matches_request(session, auth, &request) {
                return Ok(StreamSessionMutationOutcome {
                    session: session.clone(),
                    applied: false,
                });
            }
            return Err(StreamingError::conflict(stream_id));
        }

        if session.state == StreamSessionState::Completed {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "stream_state_invalid",
                message: format!("stream is already closed: {stream_id}"),
            });
        }

        if let Some(frame_seq) = request.frame_seq {
            session.last_frame_seq = session.last_frame_seq.max(frame_seq);
        }
        session.state = StreamSessionState::Aborted;
        session.abort_frame_seq = request.frame_seq;
        session.abort_reason = request.reason;
        session.closed_at = Some(utc_now_rfc3339_millis());
        let session = session.clone();
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), stream_id)?;

        Ok(StreamSessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub fn list_frames(
        &self,
        auth: &AppContext,
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
        if limit > STREAM_FRAME_LIST_MAX_LIMIT {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "invalid_limit",
                message: format!(
                    "limit must be less than or equal to {STREAM_FRAME_LIST_MAX_LIMIT}"
                ),
            });
        }

        let scope_key = stream_scope_key(auth.tenant_id.as_str(), stream_id);
        let sessions = lock_stream_mutex(&self.sessions, "stream runtime");
        let session = sessions
            .get(scope_key.as_str())
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "stream_not_found",
                message: format!("stream not found: {stream_id}"),
            })?;
        ensure_stream_session_actor_access(session, auth, stream_id)?;
        drop(sessions);

        let frames = lock_stream_mutex(&self.frames, "stream runtime");
        let mut has_more = false;
        let mut items = Vec::new();
        if let Some(stream_frames) = frames.get(scope_key.as_str()) {
            for (_, frame) in stream_frames.range((Excluded(after_frame_seq), Unbounded)) {
                if items.len() == limit {
                    has_more = true;
                    break;
                }
                items.push(frame.clone());
            }
        }
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
            .unwrap_or_default()
            .into_values()
            .collect();

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
        let key = stream_scope_key(record.tenant_id.as_str(), record.stream_id.as_str());
        let mut states = lock_stream_mutex(&self.states, "stream state store");
        let next = states
            .remove(key.as_str())
            .map(|previous| previous.merge_monotonic(record.clone()))
            .unwrap_or(record);
        states.insert(key, next);
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

impl From<AppContextError> for StreamingError {
    fn from(value: AppContextError) -> Self {
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
    let guardrails = PublicAppGuardrails {
        request_gate: Arc::new(Semaphore::new(resolve_max_in_flight_requests())),
        require_dual_token_headers: resolve_require_dual_token_headers(),
    };
    build_default_app()
        .layer(DefaultBodyLimit::max(resolve_max_http_request_body_bytes()))
        .layer(middleware::from_fn_with_state(
            guardrails,
            require_app_context,
        ))
}

pub fn build_app(runtime: Arc<StreamingRuntime>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs))
        .route("/im/v3/api/streams", post(open_stream))
        .route(
            "/im/v3/api/streams/{stream_id}/frames",
            post(append_stream_frame).get(list_stream_frames),
        )
        .route(
            "/im/v3/api/streams/{stream_id}/checkpoint",
            post(checkpoint_stream),
        )
        .route(
            "/im/v3/api/streams/{stream_id}/complete",
            post(complete_stream),
        )
        .route("/im/v3/api/streams/{stream_id}/abort", post(abort_stream))
        .with_state(AppState { runtime })
}

async fn require_app_context(
    State(guardrails): State<PublicAppGuardrails>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    match request.uri().path() {
        "/healthz" | "/readyz" | "/openapi.json" | "/docs" => next.run(request).await,
        _ => {
            let permit = match guardrails.request_gate.clone().try_acquire_owned() {
                Ok(permit) => permit,
                Err(_) => {
                    return StreamingError {
                        status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                        code: "http_overloaded",
                        message:
                            "server is at maximum in-flight request capacity, please retry later"
                                .to_owned(),
                    }
                    .into_response();
                }
            };
            if guardrails.require_dual_token_headers
                && let Err(error) = require_dual_token_headers(request.headers())
            {
                return error.into_response();
            }
            let resolved = match resolve_app_context_for_request(
                request.headers(),
                request.uri().path(),
                request.method().as_str(),
            ) {
                Ok(resolved) => resolved,
                Err(error) => return StreamingError::from(error).into_response(),
            };
            request
                .extensions_mut()
                .insert(resolved.app_request_context);
            request.extensions_mut().insert(resolved.app_context);
            let response = next.run(request).await;
            drop(permit);
            response
        }
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

async fn openapi_json() -> Result<Json<serde_json::Value>, StreamingError> {
    Ok(Json(build_streaming_service_openapi_document().map_err(
        |message| StreamingError::internal("openapi_export_failed", message),
    )?))
}

async fn docs() -> Html<String> {
    Html(render_docs_html(&streaming_service_openapi_spec()))
}

fn build_streaming_service_openapi_document() -> Result<serde_json::Value, String> {
    let routes = extract_routes_from_function(
        include_str!("lib.rs"),
        "build_app",
        &[],
        &["/openapi.json", "/docs"],
    )?;

    Ok(build_openapi_document(
        &streaming_service_openapi_spec(),
        &routes,
        streaming_service_tag,
        streaming_service_requires_app_context,
        streaming_service_summary,
    ))
}

fn streaming_service_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Sdkwork IM Streaming Service API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the streaming-service router for stream session lifecycle and frame append/query flows.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn streaming_service_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" | "/readyz" => "system".to_owned(),
        _ => "streams".to_owned(),
    }
}

fn streaming_service_requires_app_context(path: &str, _method: HttpMethod) -> bool {
    !matches!(path, "/healthz" | "/readyz")
}

fn streaming_service_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check streaming service health".to_owned(),
        ("/readyz", HttpMethod::Get) => "Check streaming service readiness".to_owned(),
        _ => format!(
            "{} {}",
            streaming_service_method_display(method),
            path.trim_matches('/').replace('/', " ")
        ),
    }
}

fn streaming_service_method_display(method: HttpMethod) -> &'static str {
    match method {
        HttpMethod::Delete => "Delete",
        HttpMethod::Get => "Get",
        HttpMethod::Head => "Head",
        HttpMethod::Options => "Options",
        HttpMethod::Patch => "Patch",
        HttpMethod::Post => "Post",
        HttpMethod::Put => "Put",
    }
}

async fn open_stream(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<OpenStreamRequest>,
) -> Result<Json<StreamSessionMutationResponse>, StreamingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_stream_open_allowed(&request)?;
    let request_key = stream_open_request_key(&auth, request.stream_id.as_str());
    Ok(Json(StreamSessionMutationResponse::from_outcome(
        state.runtime.open_stream_with_outcome(&auth, request)?,
        request_key,
    )))
}

async fn checkpoint_stream(
    Path(stream_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CheckpointStreamRequest>,
) -> Result<Json<StreamSessionMutationResponse>, StreamingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
    let request_key = stream_checkpoint_request_key(&auth, stream_id.as_str(), request.frame_seq);
    Ok(Json(StreamSessionMutationResponse::from_outcome(
        state
            .runtime
            .checkpoint_stream_with_outcome(&auth, stream_id.as_str(), request)?,
        request_key,
    )))
}

async fn append_stream_frame(
    Path(stream_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AppendStreamFrameRequest>,
) -> Result<Json<StreamFrameMutationResponse>, StreamingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
    let request_key = stream_append_request_key(&auth, stream_id.as_str(), request.frame_seq);
    Ok(Json(StreamFrameMutationResponse::from_outcome(
        state
            .runtime
            .append_frame_with_outcome(&auth, stream_id.as_str(), request)?,
        request_key,
    )))
}

async fn list_stream_frames(
    Path(stream_id): Path<String>,
    Query(query): Query<ListStreamFramesQuery>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<StreamFrameWindow>, StreamingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
    Ok(Json(state.runtime.list_frames(
        &auth,
        stream_id.as_str(),
        query,
    )?))
}

async fn complete_stream(
    Path(stream_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CompleteStreamRequest>,
) -> Result<Json<StreamSessionMutationResponse>, StreamingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
    let request_key = stream_complete_request_key(&auth, stream_id.as_str());
    Ok(Json(StreamSessionMutationResponse::from_outcome(
        state
            .runtime
            .complete_stream_with_outcome(&auth, stream_id.as_str(), request)?,
        request_key,
    )))
}

async fn abort_stream(
    Path(stream_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AbortStreamRequest>,
) -> Result<Json<StreamSessionMutationResponse>, StreamingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
    let request_key = stream_abort_request_key(&auth, stream_id.as_str());
    Ok(Json(StreamSessionMutationResponse::from_outcome(
        state
            .runtime
            .abort_stream_with_outcome(&auth, stream_id.as_str(), request)?,
        request_key,
    )))
}

fn validate_payload_size(
    field: &'static str,
    payload: &str,
    max_bytes: usize,
) -> Result<(), StreamingError> {
    let payload_len = payload.len();
    if payload_len > max_bytes {
        return Err(StreamingError::payload_too_large(
            field,
            max_bytes,
            payload_len,
        ));
    }
    Ok(())
}

fn resolve_request_app_context(
    auth: Option<Extension<AppContext>>,
    headers: &HeaderMap,
) -> Result<AppContext, StreamingError> {
    match auth {
        Some(Extension(auth)) => Ok(auth),
        None => resolve_app_context(headers).map_err(StreamingError::from),
    }
}

fn require_dual_token_headers(headers: &HeaderMap) -> Result<(), StreamingError> {
    if !has_bearer_auth_token(headers) {
        return Err(StreamingError {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: "auth_token_missing",
            message: "authorization header must provide a bearer token".to_owned(),
        });
    }
    if !has_access_token_header(headers) {
        return Err(StreamingError {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: "access_token_missing",
            message: "access-token header is required".to_owned(),
        });
    }
    Ok(())
}

fn has_bearer_auth_token(headers: &HeaderMap) -> bool {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .and_then(|value| {
            let (scheme, token) = value.split_once(' ')?;
            if scheme.eq_ignore_ascii_case("bearer") && !token.trim().is_empty() {
                return Some(());
            }
            None
        })
        .is_some()
}

fn has_access_token_header(headers: &HeaderMap) -> bool {
    headers
        .get("access-token")
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .is_some_and(|value| !value.is_empty())
}

fn resolve_max_in_flight_requests() -> usize {
    std::env::var(STREAMING_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(STREAMING_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(STREAMING_MAX_IN_FLIGHT_REQUESTS_MAX)
}

fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(STREAMING_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(STREAMING_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(STREAMING_MAX_REQUEST_BODY_BYTES_MAX)
}

fn resolve_require_dual_token_headers() -> bool {
    std::env::var(STREAMING_REQUIRE_DUAL_TOKEN_HEADERS_ENV)
        .ok()
        .map(|value| parse_truthy_env_flag(Some(value)))
        .unwrap_or(true)
}

fn parse_truthy_env_flag(raw: Option<String>) -> bool {
    raw.as_deref().map(str::trim).is_some_and(|value| {
        matches!(
            value.to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    })
}

fn validate_stream_id(stream_id: &str) -> Result<(), StreamingError> {
    validate_payload_size("streamId", stream_id, STREAM_MAX_STREAM_ID_BYTES)
}

fn validate_open_stream_request_payload_size(
    request: &OpenStreamRequest,
) -> Result<(), StreamingError> {
    validate_stream_id(request.stream_id.as_str())?;
    validate_payload_size(
        "streamType",
        request.stream_type.as_str(),
        STREAM_MAX_STREAM_TYPE_BYTES,
    )?;
    validate_payload_size(
        "scopeKind",
        request.scope_kind.as_str(),
        STREAM_MAX_SCOPE_KIND_BYTES,
    )?;
    validate_payload_size(
        "scopeId",
        request.scope_id.as_str(),
        STREAM_MAX_SCOPE_ID_BYTES,
    )?;
    validate_payload_size(
        "durabilityClass",
        request.durability_class.as_str(),
        STREAM_MAX_DURABILITY_CLASS_BYTES,
    )?;
    if let Some(schema_ref) = request.schema_ref.as_deref() {
        validate_payload_size("schemaRef", schema_ref, STREAM_MAX_SCHEMA_REF_BYTES)?;
    }
    Ok(())
}

fn validate_append_frame_request_payload_size(
    request: &AppendStreamFrameRequest,
) -> Result<(), StreamingError> {
    validate_payload_size(
        "frameType",
        request.frame_type.as_str(),
        STREAM_MAX_FRAME_TYPE_BYTES,
    )?;
    validate_payload_size(
        "encoding",
        request.encoding.as_str(),
        STREAM_MAX_FRAME_ENCODING_BYTES,
    )?;
    validate_payload_size(
        "payload",
        request.payload.as_str(),
        STREAM_MAX_FRAME_PAYLOAD_BYTES,
    )?;
    if let Some(schema_ref) = request.schema_ref.as_deref() {
        validate_payload_size("schemaRef", schema_ref, STREAM_MAX_SCHEMA_REF_BYTES)?;
    }
    let attributes_bytes = request
        .attributes
        .iter()
        .map(|(key, value)| key.len() + value.len())
        .sum::<usize>();
    if attributes_bytes > STREAM_MAX_FRAME_ATTRIBUTES_BYTES {
        return Err(StreamingError::payload_too_large(
            "attributes",
            STREAM_MAX_FRAME_ATTRIBUTES_BYTES,
            attributes_bytes,
        ));
    }
    Ok(())
}

fn validate_complete_stream_request_payload_size(
    request: &CompleteStreamRequest,
) -> Result<(), StreamingError> {
    if let Some(result_message_id) = request.result_message_id.as_deref() {
        validate_payload_size(
            "resultMessageId",
            result_message_id,
            STREAM_MAX_RESULT_MESSAGE_ID_BYTES,
        )?;
    }
    Ok(())
}

fn validate_abort_stream_request_payload_size(
    request: &AbortStreamRequest,
) -> Result<(), StreamingError> {
    if let Some(reason) = request.reason.as_deref() {
        validate_payload_size("reason", reason, STREAM_MAX_ABORT_REASON_BYTES)?;
    }
    Ok(())
}

fn lock_stream_mutex<'a, T>(mutex: &'a Mutex<T>, lock_name: &'static str) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovered poisoned stream mutex lock={lock_name}");
            poisoned.into_inner()
        }
    }
}

fn stream_scope_key(tenant_id: &str, stream_id: &str) -> String {
    encode_stream_key_segments([tenant_id, stream_id])
}

fn encode_stream_key_segments<'a>(segments: impl IntoIterator<Item = &'a str>) -> String {
    let mut encoded = String::new();
    for segment in segments {
        encoded.push_str(segment.len().to_string().as_str());
        encoded.push('#');
        encoded.push_str(segment);
    }
    encoded
}

fn stream_frame_index(frames: Vec<StreamFrame>) -> BTreeMap<u64, StreamFrame> {
    frames.into_iter().filter(|frame| frame.frame_seq > 0).fold(
        BTreeMap::new(),
        |mut index, frame| {
            index.insert(frame.frame_seq, frame);
            index
        },
    )
}

pub fn stream_open_request_key(auth: &AppContext, stream_id: &str) -> String {
    encode_stream_key_segments([
        auth.tenant_id.as_str(),
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
        "open",
        stream_id,
    ])
}

pub fn stream_complete_request_key(auth: &AppContext, stream_id: &str) -> String {
    encode_stream_key_segments([
        auth.tenant_id.as_str(),
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
        "complete",
        stream_id,
    ])
}

pub fn stream_checkpoint_request_key(auth: &AppContext, stream_id: &str, frame_seq: u64) -> String {
    let frame_seq = frame_seq.to_string();
    encode_stream_key_segments([
        auth.tenant_id.as_str(),
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
        "checkpoint",
        stream_id,
        frame_seq.as_str(),
    ])
}

pub fn stream_abort_request_key(auth: &AppContext, stream_id: &str) -> String {
    encode_stream_key_segments([
        auth.tenant_id.as_str(),
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
        "abort",
        stream_id,
    ])
}

pub fn stream_append_request_key(auth: &AppContext, stream_id: &str, frame_seq: u64) -> String {
    let frame_seq = frame_seq.to_string();
    encode_stream_key_segments([
        auth.tenant_id.as_str(),
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
        "append",
        stream_id,
        frame_seq.as_str(),
    ])
}

fn ensure_standalone_stream_open_allowed(
    request: &OpenStreamRequest,
) -> Result<(), StreamingError> {
    if request.scope_kind != "conversation" {
        return Ok(());
    }

    Err(conversation_gateway_required(
        "conversation-bound streams must be opened through an authorizing IM gateway",
    ))
}

fn ensure_standalone_stream_session_allowed(
    runtime: &StreamingRuntime,
    auth: &AppContext,
    stream_id: &str,
) -> Result<(), StreamingError> {
    let session = runtime.session(auth, stream_id)?;
    if session.scope_kind != "conversation" {
        return Ok(());
    }

    Err(conversation_gateway_required(
        "conversation-bound streams must be accessed through an authorizing IM gateway",
    ))
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
    auth: &AppContext,
    request: &OpenStreamRequest,
    durability_class: &StreamDurabilityClass,
) -> bool {
    stream_session_matches_owner_principal(session, auth)
        && session.stream_id == request.stream_id.as_str()
        && session.stream_type == request.stream_type.as_str()
        && session.scope_kind == request.scope_kind.as_str()
        && session.scope_id == request.scope_id.as_str()
        && session.durability_class == *durability_class
        && session.schema_ref.as_ref() == request.schema_ref.as_ref()
}

fn stream_checkpoint_matches_request(
    session: &StreamSession,
    auth: &AppContext,
    request: &CheckpointStreamRequest,
) -> bool {
    stream_session_matches_owner_principal(session, auth)
        && session.last_checkpoint_seq == Some(request.frame_seq)
}

fn stream_completion_matches_request(
    session: &StreamSession,
    auth: &AppContext,
    request: &CompleteStreamRequest,
) -> bool {
    stream_session_matches_owner_principal(session, auth)
        && session.state == StreamSessionState::Completed
        && session.complete_frame_seq == Some(request.frame_seq)
        && session.result_message_id == request.result_message_id
}

fn stream_abort_matches_request(
    session: &StreamSession,
    auth: &AppContext,
    request: &AbortStreamRequest,
) -> bool {
    stream_session_matches_owner_principal(session, auth)
        && session.state == StreamSessionState::Aborted
        && session.abort_frame_seq == request.frame_seq
        && session.abort_reason == request.reason
}

fn stream_session_matches_owner_principal(session: &StreamSession, auth: &AppContext) -> bool {
    session.owner_principal_id == auth.actor_id && session.owner_principal_kind == auth.actor_kind
}

fn ensure_stream_session_actor_access(
    session: &StreamSession,
    auth: &AppContext,
    stream_id: &str,
) -> Result<(), StreamingError> {
    if session.scope_kind != "conversation"
        && !stream_session_matches_owner_principal(session, auth)
    {
        return Err(StreamingError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "stream_not_found",
            message: format!("stream not found: {stream_id}"),
        });
    }

    Ok(())
}

fn resolve_stream_frame_sender(auth: &AppContext) -> Sender {
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
    use axum::http::{HeaderMap, HeaderValue};

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

    #[test]
    fn test_runtime_memory_state_store_rejects_stale_cursor_and_frame_regression() {
        let store = RuntimeMemoryStreamStateStore::default();
        store
            .save_state(test_stream_state_record(
                StreamSessionState::Completed,
                3,
                Some(2),
                Some(3),
                vec![1, 2, 3],
                "2026-05-06T00:00:03.000Z",
            ))
            .expect("current stream state save should succeed");
        store
            .save_state(test_stream_state_record(
                StreamSessionState::Active,
                1,
                None,
                None,
                vec![1],
                "2026-05-06T00:00:01.000Z",
            ))
            .expect("stale stream state save should not fail the caller");

        let state = store
            .load_state("t_demo", "st_demo")
            .expect("stream state load should succeed")
            .expect("stream state should be present");
        assert_eq!(state.session.state, StreamSessionState::Completed);
        assert_eq!(state.session.last_frame_seq, 3);
        assert_eq!(state.session.last_checkpoint_seq, Some(2));
        assert_eq!(state.session.complete_frame_seq, Some(3));
        assert_eq!(
            state
                .frames
                .iter()
                .map(|frame| frame.frame_seq)
                .collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
        assert_eq!(state.updated_at, "2026-05-06T00:00:03.000Z");
    }

    #[test]
    fn test_stream_state_store_scope_key_is_segment_safe() {
        let store = RuntimeMemoryStreamStateStore::default();
        store
            .save_state(test_stream_state_record_with_identity(
                "tenant:a",
                "b",
                "2026-05-06T00:00:01.000Z",
            ))
            .expect("first stream state should save");
        store
            .save_state(test_stream_state_record_with_identity(
                "tenant",
                "a:b",
                "2026-05-06T00:00:02.000Z",
            ))
            .expect("second stream state should save");

        assert_eq!(
            store
                .load_state("tenant:a", "b")
                .expect("first stream load should succeed")
                .expect("first stream should not be overwritten")
                .stream_id,
            "b"
        );
        assert_eq!(
            store
                .load_state("tenant", "a:b")
                .expect("second stream load should succeed")
                .expect("second stream should be retrievable")
                .stream_id,
            "a:b"
        );
    }

    #[test]
    fn test_stream_request_keys_are_segment_safe() {
        let first = AppContext {
            tenant_id: "tenant:a".into(),
            organization_id: "default".to_owned(),
            user_id: "b".into(),
            session_id: None,
            app_id: None,
            environment: None,
            deployment_mode: None,
            auth_level: None,
            data_scope: Default::default(),
            permission_scope: Default::default(),
            actor_id: "b".into(),
            actor_kind: "user".into(),
            device_id: None,
        };
        let second = AppContext {
            tenant_id: "tenant".into(),
            organization_id: "default".to_owned(),
            user_id: "b".into(),
            session_id: None,
            app_id: None,
            environment: None,
            deployment_mode: None,
            auth_level: None,
            data_scope: Default::default(),
            permission_scope: Default::default(),
            actor_id: "b".into(),
            actor_kind: "a:user".into(),
            device_id: None,
        };

        assert_ne!(
            stream_open_request_key(&first, "stream"),
            stream_open_request_key(&second, "stream")
        );
        assert_ne!(
            stream_append_request_key(&first, "stream", 7),
            stream_append_request_key(&second, "stream", 7)
        );
    }

    #[test]
    fn parse_truthy_env_flag_accepts_common_truthy_values() {
        for value in ["1", "true", "TRUE", " yes ", "On"] {
            assert!(parse_truthy_env_flag(Some(value.to_owned())));
        }
        for value in ["0", "false", "off", "no", "", "  "] {
            assert!(!parse_truthy_env_flag(Some(value.to_owned())));
        }
        assert!(!parse_truthy_env_flag(None));
    }

    #[test]
    fn dual_token_header_helpers_validate_auth_and_access_headers() {
        let mut headers = HeaderMap::new();
        assert!(!has_bearer_auth_token(&headers));
        assert!(!has_access_token_header(&headers));

        headers.insert(
            axum::http::header::AUTHORIZATION,
            HeaderValue::from_static("Bearer auth_token"),
        );
        headers.insert("access-token", HeaderValue::from_static("access_token"));
        assert!(has_bearer_auth_token(&headers));
        assert!(has_access_token_header(&headers));
    }

    fn test_stream_state_record(
        state: StreamSessionState,
        last_frame_seq: u64,
        last_checkpoint_seq: Option<u64>,
        complete_frame_seq: Option<u64>,
        frame_seqs: Vec<u64>,
        updated_at: &str,
    ) -> StreamStateRecord {
        StreamStateRecord {
            tenant_id: "t_demo".into(),
            stream_id: "st_demo".into(),
            session: StreamSession {
                tenant_id: "t_demo".into(),
                stream_id: "st_demo".into(),
                owner_principal_id: "u_demo".into(),
                owner_principal_kind: "user".into(),
                stream_type: "custom.delta.text".into(),
                scope_kind: "request".into(),
                scope_id: "req_demo".into(),
                durability_class: StreamDurabilityClass::DurableSession,
                ordering_scope: "stream".into(),
                schema_ref: Some("custom.delta.text.v1".into()),
                state,
                last_frame_seq,
                last_checkpoint_seq,
                result_message_id: complete_frame_seq.map(|_| "msg_done".into()),
                complete_frame_seq,
                abort_frame_seq: None,
                abort_reason: None,
                opened_at: "2026-05-06T00:00:00.000Z".into(),
                closed_at: complete_frame_seq.map(|_| "2026-05-06T00:00:03.000Z".into()),
                expires_at: None,
            },
            frames: frame_seqs.into_iter().map(test_stream_frame).collect(),
            updated_at: updated_at.into(),
        }
    }

    fn test_stream_state_record_with_identity(
        tenant_id: &str,
        stream_id: &str,
        updated_at: &str,
    ) -> StreamStateRecord {
        let mut record = test_stream_state_record(
            StreamSessionState::Active,
            1,
            None,
            None,
            vec![1],
            updated_at,
        );
        record.tenant_id = tenant_id.into();
        record.stream_id = stream_id.into();
        record.session.tenant_id = tenant_id.into();
        record.session.stream_id = stream_id.into();
        for frame in &mut record.frames {
            frame.tenant_id = tenant_id.into();
            frame.stream_id = stream_id.into();
        }
        record
    }

    fn test_stream_frame(frame_seq: u64) -> StreamFrame {
        StreamFrame {
            tenant_id: "t_demo".into(),
            stream_id: "st_demo".into(),
            stream_type: "custom.delta.text".into(),
            scope_kind: "request".into(),
            scope_id: "req_demo".into(),
            frame_seq,
            frame_type: "delta".into(),
            schema_ref: Some("custom.delta.text.v1".into()),
            encoding: "json".into(),
            payload: format!("{{\"seq\":{frame_seq}}}"),
            sender: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: BTreeMap::new(),
            },
            attributes: BTreeMap::new(),
            occurred_at: format!("2026-05-06T00:00:0{frame_seq}.000Z"),
        }
    }
}
