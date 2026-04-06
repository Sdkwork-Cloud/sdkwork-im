use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use im_auth_context::{
    AuthContext, AuthContextError, resolve_auth_context, resolve_public_bearer_auth_context,
};
use im_domain_core::message::Sender;
use im_domain_core::rtc::{RtcSession, RtcSessionState, RtcSignalEvent};
use im_platform_contracts::{ContractError, RtcStateRecord, RtcStateStore};
use im_time::utc_now_rfc3339_millis;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct AppState {
    runtime: Arc<RtcRuntime>,
}

pub struct RtcRuntime {
    sessions: Mutex<HashMap<String, RtcSession>>,
    signals: Mutex<HashMap<String, Vec<RtcSignalEvent>>>,
    state_store: Arc<dyn RtcStateStore>,
}

#[derive(Clone, Debug)]
pub struct RtcSessionMutationOutcome {
    pub session: RtcSession,
    pub applied: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRtcSessionRequest {
    pub rtc_session_id: String,
    pub conversation_id: Option<String>,
    pub rtc_mode: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteRtcSessionRequest {
    pub signaling_stream_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRtcSessionRequest {
    pub artifact_message_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRtcSignalRequest {
    pub signal_type: String,
    pub schema_ref: Option<String>,
    pub payload: String,
    pub signaling_stream_id: Option<String>,
}

impl RtcRuntime {
    pub fn with_store(state_store: Arc<dyn RtcStateStore>) -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            signals: Mutex::new(HashMap::new()),
            state_store,
        }
    }

    fn ensure_session_state(&self, tenant_id: &str, rtc_session_id: &str) -> Result<(), RtcError> {
        let scope_key = rtc_scope_key(tenant_id, rtc_session_id);
        let needs_restore = !self
            .sessions
            .lock()
            .expect("rtc runtime should lock")
            .contains_key(scope_key.as_str());
        if !needs_restore {
            self.signals
                .lock()
                .expect("rtc runtime should lock")
                .entry(scope_key)
                .or_default();
            return Ok(());
        }

        let restored = self
            .state_store
            .load_state(tenant_id, rtc_session_id)
            .map_err(RtcError::rtc_store)?;
        if let Some(record) = restored {
            self.sessions
                .lock()
                .expect("rtc runtime should lock")
                .insert(scope_key.clone(), record.session);
            self.signals
                .lock()
                .expect("rtc runtime should lock")
                .insert(scope_key, record.signals);
        }

        Ok(())
    }

    pub fn session(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
    ) -> Result<RtcSession, RtcError> {
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        self.sessions
            .lock()
            .expect("rtc runtime should lock")
            .get(rtc_scope_key(auth.tenant_id.as_str(), rtc_session_id).as_str())
            .cloned()
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })
    }

    pub fn create_session(
        &self,
        auth: &AuthContext,
        request: CreateRtcSessionRequest,
    ) -> Result<RtcSession, RtcError> {
        let scope_key = rtc_scope_key(auth.tenant_id.as_str(), request.rtc_session_id.as_str());
        self.ensure_session_state(auth.tenant_id.as_str(), request.rtc_session_id.as_str())?;
        let mut sessions = self.sessions.lock().expect("rtc runtime should lock");
        if let Some(existing) = sessions.get(scope_key.as_str()).cloned() {
            if rtc_session_matches_create_request(&existing, auth, &request) {
                return Ok(existing);
            }

            return Err(RtcError::conflict(request.rtc_session_id.as_str()));
        }

        let started_at = utc_now_rfc3339_millis();
        let session = RtcSession {
            tenant_id: auth.tenant_id.clone(),
            rtc_session_id: request.rtc_session_id.clone(),
            conversation_id: request.conversation_id,
            rtc_mode: request.rtc_mode,
            initiator_id: auth.actor_id.clone(),
            state: RtcSessionState::Started,
            signaling_stream_id: None,
            artifact_message_id: None,
            started_at,
            ended_at: None,
        };

        sessions.insert(scope_key, session.clone());
        drop(sessions);
        self.signals
            .lock()
            .expect("rtc runtime should lock")
            .entry(rtc_scope_key(
                auth.tenant_id.as_str(),
                request.rtc_session_id.as_str(),
            ))
            .or_default();
        self.persist_state(auth.tenant_id.as_str(), request.rtc_session_id.as_str())?;

        Ok(session)
    }

    pub fn invite_session(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
        request: InviteRtcSessionRequest,
    ) -> Result<RtcSession, RtcError> {
        Ok(self
            .invite_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn invite_session_with_outcome(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
        request: InviteRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, RtcError> {
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = self.sessions.lock().expect("rtc runtime should lock");
        let session = sessions
            .get_mut(rtc_scope_key(auth.tenant_id.as_str(), rtc_session_id).as_str())
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })?;

        if matches!(
            session.state,
            RtcSessionState::Rejected | RtcSessionState::Ended
        ) {
            return Err(RtcError::state_conflict(
                rtc_session_id,
                "invite",
                &session.state,
            ));
        }

        if rtc_session_matches_invite_request(session, &request) {
            return Ok(RtcSessionMutationOutcome {
                session: session.clone(),
                applied: false,
            });
        }

        if matches!(session.state, RtcSessionState::Accepted) {
            return Err(RtcError::state_conflict(
                rtc_session_id,
                "invite",
                &session.state,
            ));
        }

        if let Some(signaling_stream_id) = request.signaling_stream_id {
            session.signaling_stream_id = Some(signaling_stream_id);
            let outcome = RtcSessionMutationOutcome {
                session: session.clone(),
                applied: true,
            };
            drop(sessions);
            self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;
            return Ok(outcome);
        }

        Err(RtcError::state_conflict(
            rtc_session_id,
            "invite",
            &session.state,
        ))
    }

    pub fn accept_session(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSession, RtcError> {
        Ok(self
            .accept_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn accept_session_with_outcome(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, RtcError> {
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = self.sessions.lock().expect("rtc runtime should lock");
        let session = sessions
            .get_mut(rtc_scope_key(auth.tenant_id.as_str(), rtc_session_id).as_str())
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })?;

        match session.state {
            RtcSessionState::Started => {
                session.state = RtcSessionState::Accepted;
                session.artifact_message_id = request.artifact_message_id;
                let outcome = RtcSessionMutationOutcome {
                    session: session.clone(),
                    applied: true,
                };
                drop(sessions);
                self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;
                Ok(outcome)
            }
            RtcSessionState::Accepted if rtc_session_matches_update_request(session, &request) => {
                Ok(RtcSessionMutationOutcome {
                    session: session.clone(),
                    applied: false,
                })
            }
            _ => Err(RtcError::state_conflict(
                rtc_session_id,
                "accept",
                &session.state,
            )),
        }
    }

    pub fn reject_session(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSession, RtcError> {
        Ok(self
            .reject_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn reject_session_with_outcome(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, RtcError> {
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = self.sessions.lock().expect("rtc runtime should lock");
        let session = sessions
            .get_mut(rtc_scope_key(auth.tenant_id.as_str(), rtc_session_id).as_str())
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })?;

        match session.state {
            RtcSessionState::Started => {
                session.state = RtcSessionState::Rejected;
                session.artifact_message_id = request.artifact_message_id;
                session.ended_at = Some(utc_now_rfc3339_millis());
                let outcome = RtcSessionMutationOutcome {
                    session: session.clone(),
                    applied: true,
                };
                drop(sessions);
                self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;
                Ok(outcome)
            }
            RtcSessionState::Rejected if rtc_session_matches_update_request(session, &request) => {
                Ok(RtcSessionMutationOutcome {
                    session: session.clone(),
                    applied: false,
                })
            }
            _ => Err(RtcError::state_conflict(
                rtc_session_id,
                "reject",
                &session.state,
            )),
        }
    }

    pub fn end_session(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSession, RtcError> {
        Ok(self
            .end_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn end_session_with_outcome(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, RtcError> {
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = self.sessions.lock().expect("rtc runtime should lock");
        let session = sessions
            .get_mut(rtc_scope_key(auth.tenant_id.as_str(), rtc_session_id).as_str())
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })?;

        match session.state {
            RtcSessionState::Started | RtcSessionState::Accepted => {
                session.state = RtcSessionState::Ended;
                session.artifact_message_id = request.artifact_message_id;
                session.ended_at = Some(utc_now_rfc3339_millis());
                let outcome = RtcSessionMutationOutcome {
                    session: session.clone(),
                    applied: true,
                };
                drop(sessions);
                self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;
                Ok(outcome)
            }
            RtcSessionState::Ended if rtc_session_matches_update_request(session, &request) => {
                Ok(RtcSessionMutationOutcome {
                    session: session.clone(),
                    applied: false,
                })
            }
            _ => Err(RtcError::state_conflict(
                rtc_session_id,
                "end",
                &session.state,
            )),
        }
    }

    pub fn post_signal(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
        request: PostRtcSignalRequest,
    ) -> Result<RtcSignalEvent, RtcError> {
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = self.sessions.lock().expect("rtc runtime should lock");
        let session = sessions
            .get_mut(rtc_scope_key(auth.tenant_id.as_str(), rtc_session_id).as_str())
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })?;

        if matches!(
            session.state,
            RtcSessionState::Rejected | RtcSessionState::Ended
        ) {
            return Err(RtcError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "rtc_session_closed",
                message: format!("rtc session is closed: {rtc_session_id}"),
            });
        }

        if let Some(signaling_stream_id) = request.signaling_stream_id {
            session.signaling_stream_id = Some(signaling_stream_id);
        }

        let occurred_at = utc_now_rfc3339_millis();
        let signal = RtcSignalEvent {
            tenant_id: auth.tenant_id.clone(),
            rtc_session_id: session.rtc_session_id.clone(),
            conversation_id: session.conversation_id.clone(),
            rtc_mode: session.rtc_mode.clone(),
            signal_type: request.signal_type,
            schema_ref: request.schema_ref,
            payload: request.payload,
            sender: Sender {
                id: auth.actor_id.clone(),
                kind: auth.actor_kind.clone(),
                member_id: None,
                device_id: auth.device_id.clone(),
                session_id: auth.session_id.clone(),
                metadata: BTreeMap::new(),
            },
            signaling_stream_id: session.signaling_stream_id.clone(),
            occurred_at,
        };

        drop(sessions);

        self.signals
            .lock()
            .expect("rtc signal store should lock")
            .entry(rtc_scope_key(auth.tenant_id.as_str(), rtc_session_id))
            .or_default()
            .push(signal.clone());
        self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;

        Ok(signal)
    }

    fn persist_state(&self, tenant_id: &str, rtc_session_id: &str) -> Result<(), RtcError> {
        self.state_store
            .save_state(self.state_record(tenant_id, rtc_session_id))
            .map_err(RtcError::rtc_store)
    }

    fn state_record(&self, tenant_id: &str, rtc_session_id: &str) -> RtcStateRecord {
        let scope_key = rtc_scope_key(tenant_id, rtc_session_id);
        let session = self
            .sessions
            .lock()
            .expect("rtc runtime should lock")
            .get(scope_key.as_str())
            .cloned()
            .expect("rtc session should exist before persistence");
        let signals = self
            .signals
            .lock()
            .expect("rtc runtime should lock")
            .get(scope_key.as_str())
            .cloned()
            .unwrap_or_default();

        RtcStateRecord {
            tenant_id: tenant_id.into(),
            rtc_session_id: rtc_session_id.into(),
            session,
            signals,
            updated_at: utc_now_rfc3339_millis(),
        }
    }
}

#[derive(Debug)]
pub struct RtcError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl RtcError {
    fn rtc_store(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "rtc_store_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                code: "rtc_store_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                status: axum::http::StatusCode::NOT_IMPLEMENTED,
                code: "rtc_store_unsupported",
                message,
            },
        }
    }

    fn conflict(rtc_session_id: &str) -> Self {
        Self {
            status: axum::http::StatusCode::CONFLICT,
            code: "rtc_session_conflict",
            message: format!(
                "rtc session create request conflicts with existing rtc session idempotency key: {rtc_session_id}"
            ),
        }
    }

    fn state_conflict(
        rtc_session_id: &str,
        transition: &'static str,
        current_state: &RtcSessionState,
    ) -> Self {
        Self {
            status: axum::http::StatusCode::CONFLICT,
            code: "rtc_session_state_conflict",
            message: format!(
                "rtc session transition {transition} conflicts with current state {}: {rtc_session_id}",
                current_state.as_wire_value()
            ),
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
}

#[derive(Clone, Default)]
struct RuntimeMemoryRtcStateStore {
    states: Arc<Mutex<HashMap<String, RtcStateRecord>>>,
}

impl RtcStateStore for RuntimeMemoryRtcStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<Option<RtcStateRecord>, ContractError> {
        Ok(self
            .states
            .lock()
            .expect("rtc state store should lock")
            .get(rtc_scope_key(tenant_id, rtc_session_id).as_str())
            .cloned())
    }

    fn save_state(&self, record: RtcStateRecord) -> Result<(), ContractError> {
        self.states
            .lock()
            .expect("rtc state store should lock")
            .insert(
                rtc_scope_key(record.tenant_id.as_str(), record.rtc_session_id.as_str()),
                record,
            );
        Ok(())
    }

    fn clear_state(&self, tenant_id: &str, rtc_session_id: &str) -> Result<bool, ContractError> {
        Ok(self
            .states
            .lock()
            .expect("rtc state store should lock")
            .remove(rtc_scope_key(tenant_id, rtc_session_id).as_str())
            .is_some())
    }
}

impl Default for RtcRuntime {
    fn default() -> Self {
        Self::with_store(Arc::new(RuntimeMemoryRtcStateStore::default()))
    }
}

impl From<AuthContextError> for RtcError {
    fn from(value: AuthContextError) -> Self {
        Self {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl axum::response::IntoResponse for RtcError {
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

pub fn build_default_app() -> Router {
    build_app(Arc::new(RtcRuntime::default()))
}

pub fn build_public_app() -> Router {
    build_default_app().layer(middleware::from_fn(require_public_bearer_auth))
}

pub fn build_app(runtime: Arc<RtcRuntime>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/api/v1/rtc/sessions", post(create_session))
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/invite",
            post(invite_session),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/accept",
            post(accept_session),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/reject",
            post(reject_session),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/end",
            post(end_session),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/signals",
            post(post_signal),
        )
        .with_state(AppState { runtime })
}

async fn require_public_bearer_auth(request: Request<axum::body::Body>, next: Next) -> Response {
    match request.uri().path() {
        "/healthz" | "/readyz" => next.run(request).await,
        _ => match resolve_public_bearer_auth_context(request.headers()) {
            Ok(_) => next.run(request).await,
            Err(error) => RtcError::from(error).into_response(),
        },
    }
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "rtc-signaling-service",
    })
}

async fn readyz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "rtc-signaling-service",
    })
}

async fn create_session(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateRtcSessionRequest>,
) -> Result<Json<RtcSession>, RtcError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_standalone_rtc_create_allowed(&request)?;
    Ok(Json(state.runtime.create_session(&auth, request)?))
}

async fn invite_session(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<InviteRtcSessionRequest>,
) -> Result<Json<RtcSession>, RtcError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_standalone_rtc_session_allowed(&state.runtime, &auth, rtc_session_id.as_str())?;
    Ok(Json(state.runtime.invite_session(
        &auth,
        rtc_session_id.as_str(),
        request,
    )?))
}

async fn accept_session(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<RtcSession>, RtcError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_standalone_rtc_session_allowed(&state.runtime, &auth, rtc_session_id.as_str())?;
    Ok(Json(state.runtime.accept_session(
        &auth,
        rtc_session_id.as_str(),
        request,
    )?))
}

async fn reject_session(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<RtcSession>, RtcError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_standalone_rtc_session_allowed(&state.runtime, &auth, rtc_session_id.as_str())?;
    Ok(Json(state.runtime.reject_session(
        &auth,
        rtc_session_id.as_str(),
        request,
    )?))
}

async fn end_session(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<RtcSession>, RtcError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_standalone_rtc_session_allowed(&state.runtime, &auth, rtc_session_id.as_str())?;
    Ok(Json(state.runtime.end_session(
        &auth,
        rtc_session_id.as_str(),
        request,
    )?))
}

async fn post_signal(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PostRtcSignalRequest>,
) -> Result<Json<RtcSignalEvent>, RtcError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_standalone_rtc_session_allowed(&state.runtime, &auth, rtc_session_id.as_str())?;
    Ok(Json(state.runtime.post_signal(
        &auth,
        rtc_session_id.as_str(),
        request,
    )?))
}

fn rtc_scope_key(tenant_id: &str, rtc_session_id: &str) -> String {
    format!("{tenant_id}:{rtc_session_id}")
}

fn ensure_standalone_rtc_create_allowed(request: &CreateRtcSessionRequest) -> Result<(), RtcError> {
    if request.conversation_id.is_none() {
        return Ok(());
    }

    Err(conversation_gateway_required(
        "conversation-bound RTC sessions must be created through an authorizing IM gateway",
    ))
}

fn ensure_standalone_rtc_session_allowed(
    runtime: &RtcRuntime,
    auth: &AuthContext,
    rtc_session_id: &str,
) -> Result<(), RtcError> {
    let session = runtime.session(auth, rtc_session_id)?;
    if session.conversation_id.is_none() {
        return Ok(());
    }

    Err(conversation_gateway_required(
        "conversation-bound RTC sessions must be accessed through an authorizing IM gateway",
    ))
}

fn conversation_gateway_required(message: &str) -> RtcError {
    RtcError {
        status: axum::http::StatusCode::FORBIDDEN,
        code: "conversation_gateway_required",
        message: message.into(),
    }
}

fn rtc_session_matches_create_request(
    session: &RtcSession,
    auth: &AuthContext,
    request: &CreateRtcSessionRequest,
) -> bool {
    session.rtc_session_id == request.rtc_session_id.as_str()
        && session.initiator_id == auth.actor_id.as_str()
        && session.conversation_id.as_ref() == request.conversation_id.as_ref()
        && session.rtc_mode == request.rtc_mode.as_str()
}

fn rtc_session_matches_invite_request(
    session: &RtcSession,
    request: &InviteRtcSessionRequest,
) -> bool {
    session.signaling_stream_id.as_ref() == request.signaling_stream_id.as_ref()
}

fn rtc_session_matches_update_request(
    session: &RtcSession,
    request: &UpdateRtcSessionRequest,
) -> bool {
    session.artifact_message_id.as_ref() == request.artifact_message_id.as_ref()
}
