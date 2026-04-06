use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::{Arc, Mutex};

use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::IntoResponse;
use axum::response::Response;
use axum::{
    Json, Router,
    routing::{get, post},
};
use im_auth_context::{
    AuthContext, AuthContextError, resolve_auth_context, resolve_public_bearer_auth_context,
};
use im_domain_core::realtime::{
    RealtimeAckState, RealtimeEventWindow, RealtimeSubscriptionSnapshot,
};
use im_domain_core::session::{
    DevicePresenceStatus, DevicePresenceView, PresenceSnapshotView, SessionResumeView,
};
use im_platform_contracts::{ContractError, PresenceStateRecord, PresenceStateStore};
use im_time::utc_now_rfc3339_millis;
use serde::{Deserialize, Serialize};

mod cluster;
mod realtime;
mod websocket;

pub use cluster::{
    RealtimeClusterBridge, RealtimeClusterError, RealtimeDeviceRoute, RealtimeNodeLifecycleView,
    RealtimeRouteDeliveryResult, RealtimeRouteMigrationResult,
};
pub use realtime::{
    AckRealtimeEventsRequest, ListRealtimeEventsQuery, RealtimeDeliveryRuntime,
    RealtimeDeviceStateSnapshot, RealtimeRuntimeError, RealtimeSubscriptionItemInput,
    SyncRealtimeSubscriptionsRequest,
};
pub use websocket::{
    SESSION_DISCONNECT_CLOSE_CODE, SESSION_DISCONNECT_CLOSE_REASON, serve_realtime_websocket,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Clone)]
struct AppState {
    node_id: String,
    realtime_cluster: Arc<RealtimeClusterBridge>,
    presence_runtime: Arc<SessionPresenceRuntime>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    registered_devices: Arc<Mutex<HashMap<String, BTreeSet<String>>>>,
    latest_sync_sequences: Arc<Mutex<HashMap<String, u64>>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PresenceRuntimeEntry {
    view: DevicePresenceView,
    resume_required: bool,
}

#[derive(Clone)]
pub struct SessionPresenceRuntime {
    entries: Arc<Mutex<HashMap<String, HashMap<String, PresenceRuntimeEntry>>>>,
    restored_principals: Arc<Mutex<HashSet<String>>>,
    state_store: Arc<dyn PresenceStateStore>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResumeSessionRequest {
    device_id: Option<String>,
    last_seen_sync_seq: Option<u64>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct PresenceDeviceRequest {
    device_id: Option<String>,
}

#[derive(Debug)]
struct ApiError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl ApiError {
    fn bad_request(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::BAD_REQUEST,
            code,
            message: message.into(),
        }
    }
}

#[derive(Clone, Default)]
struct RuntimeMemoryPresenceStateStore {
    states: Arc<Mutex<HashMap<String, PresenceStateRecord>>>,
}

impl PresenceStateStore for RuntimeMemoryPresenceStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError> {
        Ok(self
            .states
            .lock()
            .expect("presence state store should lock")
            .get(device_scope_key(tenant_id, principal_id, device_id).as_str())
            .cloned())
    }

    fn save_state(&self, record: PresenceStateRecord) -> Result<(), ContractError> {
        self.states
            .lock()
            .expect("presence state store should lock")
            .insert(
                device_scope_key(
                    record.tenant_id.as_str(),
                    record.principal_id.as_str(),
                    record.device_id.as_str(),
                ),
                record,
            );
        Ok(())
    }

    fn list_states_for_principal(
        &self,
        tenant_id: &str,
        principal_id: &str,
    ) -> Result<Vec<PresenceStateRecord>, ContractError> {
        Ok(self
            .states
            .lock()
            .expect("presence state store should lock")
            .values()
            .filter(|record| record.tenant_id == tenant_id && record.principal_id == principal_id)
            .cloned()
            .collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresenceRuntimeError {
    code: &'static str,
    message: String,
}

impl PresenceRuntimeError {
    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    fn reconnect_required(device_id: &str) -> Self {
        Self {
            code: "reconnect_required",
            message: format!("device must resume a fresh session before continuing: {device_id}"),
        }
    }

    fn presence_store(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                code: "presence_store_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                code: "presence_store_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                code: "presence_store_unsupported",
                message,
            },
        }
    }
}

impl From<AuthContextError> for ApiError {
    fn from(value: AuthContextError) -> Self {
        Self {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl From<RealtimeClusterError> for ApiError {
    fn from(value: RealtimeClusterError) -> Self {
        Self {
            status: if value.code == "disconnect_fence_store_unavailable"
                || value.code == "checkpoint_store_unavailable"
                || value.code == "subscription_store_unavailable"
            {
                axum::http::StatusCode::SERVICE_UNAVAILABLE
            } else {
                axum::http::StatusCode::CONFLICT
            },
            code: value.code,
            message: value.message,
        }
    }
}

impl From<RealtimeRuntimeError> for ApiError {
    fn from(value: RealtimeRuntimeError) -> Self {
        let status = match value.code {
            "checkpoint_store_unavailable" | "subscription_store_unavailable" => {
                axum::http::StatusCode::SERVICE_UNAVAILABLE
            }
            "checkpoint_store_conflict" | "subscription_store_conflict" => {
                axum::http::StatusCode::CONFLICT
            }
            "checkpoint_store_unsupported" | "subscription_store_unsupported" => {
                axum::http::StatusCode::NOT_IMPLEMENTED
            }
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self {
            status,
            code: value.code,
            message: value.message,
        }
    }
}

impl From<PresenceRuntimeError> for ApiError {
    fn from(value: PresenceRuntimeError) -> Self {
        let status = match value.code {
            "presence_store_unavailable" => axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "presence_store_conflict" | "reconnect_required" => axum::http::StatusCode::CONFLICT,
            "presence_store_unsupported" => axum::http::StatusCode::NOT_IMPLEMENTED,
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self {
            status,
            code: value.code,
            message: value.message,
        }
    }
}

impl IntoResponse for ApiError {
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

impl SessionPresenceRuntime {
    pub fn with_store<S>(state_store: Arc<S>) -> Self
    where
        S: PresenceStateStore + 'static,
    {
        Self {
            entries: Arc::new(Mutex::new(HashMap::new())),
            restored_principals: Arc::new(Mutex::new(HashSet::new())),
            state_store,
        }
    }

    pub fn register_device(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<(), PresenceRuntimeError> {
        self.ensure_device_entry(tenant_id, principal_id, device_id)
            .map(|_| ())
    }

    pub fn ensure_device_resume_not_required(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<(), PresenceRuntimeError> {
        let entry = self.ensure_device_entry(tenant_id, principal_id, device_id)?;
        if entry.resume_required {
            return Err(PresenceRuntimeError::reconnect_required(device_id));
        }
        Ok(())
    }

    pub fn resume(
        &self,
        auth: &AuthContext,
        device_id: String,
        last_seen_sync_seq: u64,
        latest_sync_seq: u64,
        registered_devices: Vec<String>,
    ) -> Result<SessionResumeView, PresenceRuntimeError> {
        self.ensure_device_entry(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            device_id.as_str(),
        )?;
        let resumed_at = session_timestamp();
        let updated_entry = {
            let scope = principal_scope_key(auth.tenant_id.as_str(), auth.actor_id.as_str());
            let mut entries = self.entries.lock().expect("presence store should lock");
            let scope_entries = entries.entry(scope).or_default();
            let entry =
                scope_entries
                    .entry(device_id.clone())
                    .or_insert_with(|| PresenceRuntimeEntry {
                        view: empty_presence_view(auth, device_id.as_str()),
                        resume_required: false,
                    });
            entry.view.session_id = auth.session_id.clone();
            entry.view.status = DevicePresenceStatus::Online;
            entry.view.last_sync_seq = latest_sync_seq;
            entry.view.last_resume_at = Some(resumed_at.clone());
            entry.view.last_seen_at = Some(resumed_at.clone());
            entry.resume_required = false;
            entry.clone()
        };
        self.persist_entry(updated_entry, resumed_at.clone())?;

        let presence = self.presence_snapshot(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            Some(device_id.clone()),
            registered_devices,
        )?;
        let resume_required = latest_sync_seq > last_seen_sync_seq;
        let resume_from_sync_seq = if latest_sync_seq == 0 {
            0
        } else if resume_required {
            last_seen_sync_seq.saturating_add(1)
        } else {
            latest_sync_seq
        };

        Ok(SessionResumeView {
            tenant_id: auth.tenant_id.clone(),
            actor_id: auth.actor_id.clone(),
            actor_kind: auth.actor_kind.clone(),
            session_id: auth.session_id.clone(),
            device_id,
            resume_required,
            resume_from_sync_seq,
            latest_sync_seq,
            resumed_at,
            presence,
        })
    }

    pub fn presence_snapshot(
        &self,
        tenant_id: &str,
        principal_id: &str,
        current_device_id: Option<String>,
        registered_devices: Vec<String>,
    ) -> Result<PresenceSnapshotView, PresenceRuntimeError> {
        self.ensure_principal_state(tenant_id, principal_id)?;
        let scope = principal_scope_key(tenant_id, principal_id);
        let stored_devices = self
            .entries
            .lock()
            .expect("presence store should lock")
            .get(scope.as_str())
            .cloned()
            .unwrap_or_default();

        let mut device_ids = BTreeSet::new();
        for device_id in registered_devices {
            device_ids.insert(device_id);
        }
        if let Some(device_id) = current_device_id.clone() {
            device_ids.insert(device_id);
        }
        for device_id in stored_devices.keys() {
            device_ids.insert(device_id.clone());
        }

        let mut devices = device_ids
            .into_iter()
            .map(|device_id| {
                stored_devices
                    .get(device_id.as_str())
                    .map(|entry| entry.view.clone())
                    .unwrap_or_else(|| {
                        empty_presence_view_for_scope(tenant_id, principal_id, &device_id)
                    })
            })
            .collect::<Vec<_>>();

        devices.sort_by(|left, right| {
            let left_current = current_device_id
                .as_deref()
                .map(|value| value == left.device_id.as_str())
                .unwrap_or(false);
            let right_current = current_device_id
                .as_deref()
                .map(|value| value == right.device_id.as_str())
                .unwrap_or(false);
            right_current
                .cmp(&left_current)
                .then_with(|| left.device_id.cmp(&right.device_id))
        });

        Ok(PresenceSnapshotView {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            current_device_id,
            devices,
        })
    }

    pub fn heartbeat(
        &self,
        auth: &AuthContext,
        device_id: String,
        latest_sync_seq: u64,
        registered_devices: Vec<String>,
    ) -> Result<PresenceSnapshotView, PresenceRuntimeError> {
        self.ensure_device_resume_not_required(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            device_id.as_str(),
        )?;
        let observed_at = session_timestamp().to_owned();
        self.update_presence_entry(
            auth,
            device_id.clone(),
            latest_sync_seq,
            Some(auth.session_id.clone()),
            DevicePresenceStatus::Online,
            observed_at,
            false,
            false,
        )?;
        self.presence_snapshot(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            Some(device_id),
            registered_devices,
        )
    }

    pub fn disconnect(
        &self,
        auth: &AuthContext,
        device_id: String,
        registered_devices: Vec<String>,
    ) -> Result<PresenceSnapshotView, PresenceRuntimeError> {
        self.ensure_device_entry(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            device_id.as_str(),
        )?;
        let latest_sync_seq = self
            .entries
            .lock()
            .expect("presence store should lock")
            .get(principal_scope_key(auth.tenant_id.as_str(), auth.actor_id.as_str()).as_str())
            .and_then(|scope_entries| scope_entries.get(device_id.as_str()))
            .map(|entry| entry.view.last_sync_seq)
            .unwrap_or_default();
        self.update_presence_entry(
            auth,
            device_id.clone(),
            latest_sync_seq,
            Some(None),
            DevicePresenceStatus::Offline,
            session_timestamp(),
            false,
            true,
        )?;
        self.presence_snapshot(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            Some(device_id),
            registered_devices,
        )
    }

    fn ensure_principal_state(
        &self,
        tenant_id: &str,
        principal_id: &str,
    ) -> Result<(), PresenceRuntimeError> {
        let scope_key = principal_scope_key(tenant_id, principal_id);
        if self
            .restored_principals
            .lock()
            .expect("presence runtime should lock")
            .contains(scope_key.as_str())
        {
            return Ok(());
        }

        let restored = self
            .state_store
            .list_states_for_principal(tenant_id, principal_id)
            .map_err(PresenceRuntimeError::presence_store)?;
        let mut normalized_records = Vec::new();
        let mut runtime_entries = Vec::new();
        for record in restored {
            let (entry, normalized_record) = normalize_presence_record(record);
            if let Some(normalized_record) = normalized_record {
                normalized_records.push(normalized_record);
            }
            runtime_entries.push((entry.view.device_id.clone(), entry));
        }

        for normalized_record in normalized_records {
            self.state_store
                .save_state(normalized_record)
                .map_err(PresenceRuntimeError::presence_store)?;
        }

        let mut entries = self.entries.lock().expect("presence runtime should lock");
        let scope_entries = entries.entry(scope_key.clone()).or_default();
        for (device_id, entry) in runtime_entries {
            scope_entries.entry(device_id).or_insert(entry);
        }
        drop(entries);
        self.restored_principals
            .lock()
            .expect("presence runtime should lock")
            .insert(scope_key);

        Ok(())
    }

    fn ensure_device_entry(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<PresenceRuntimeEntry, PresenceRuntimeError> {
        self.ensure_principal_state(tenant_id, principal_id)?;

        if let Some(entry) = self
            .entries
            .lock()
            .expect("presence store should lock")
            .get(principal_scope_key(tenant_id, principal_id).as_str())
            .and_then(|scope_entries| scope_entries.get(device_id))
            .cloned()
        {
            return Ok(entry);
        }

        let entry = PresenceRuntimeEntry {
            view: empty_presence_view_for_scope(tenant_id, principal_id, device_id),
            resume_required: false,
        };
        let scope = principal_scope_key(tenant_id, principal_id);
        let mut entries = self.entries.lock().expect("presence store should lock");
        entries
            .entry(scope)
            .or_default()
            .insert(device_id.to_owned(), entry.clone());
        drop(entries);
        self.persist_entry(entry.clone(), session_timestamp())?;

        Ok(entry)
    }

    fn update_presence_entry(
        &self,
        auth: &AuthContext,
        device_id: String,
        latest_sync_seq: u64,
        session_id: Option<Option<String>>,
        status: DevicePresenceStatus,
        observed_at: String,
        refresh_resume_at: bool,
        resume_required: bool,
    ) -> Result<(), PresenceRuntimeError> {
        self.ensure_device_entry(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            device_id.as_str(),
        )?;
        let scope = principal_scope_key(auth.tenant_id.as_str(), auth.actor_id.as_str());
        let mut entries = self.entries.lock().expect("presence store should lock");
        let scope_entries = entries.entry(scope).or_default();
        let entry =
            scope_entries
                .entry(device_id.clone())
                .or_insert_with(|| PresenceRuntimeEntry {
                    view: empty_presence_view(auth, device_id.as_str()),
                    resume_required: false,
                });
        if let Some(session_id) = session_id {
            entry.view.session_id = session_id;
        }
        entry.view.status = status;
        entry.view.last_sync_seq = latest_sync_seq;
        if refresh_resume_at || entry.view.last_resume_at.is_none() {
            entry.view.last_resume_at = Some(observed_at.clone());
        }
        entry.view.last_seen_at = Some(observed_at.clone());
        entry.resume_required = resume_required;
        let updated = entry.clone();
        drop(entries);
        self.persist_entry(updated, observed_at)
    }

    fn persist_entry(
        &self,
        entry: PresenceRuntimeEntry,
        updated_at: String,
    ) -> Result<(), PresenceRuntimeError> {
        self.state_store
            .save_state(PresenceStateRecord {
                tenant_id: entry.view.tenant_id.clone(),
                principal_id: entry.view.principal_id.clone(),
                device_id: entry.view.device_id.clone(),
                presence: entry.view,
                resume_required: entry.resume_required,
                updated_at,
            })
            .map_err(PresenceRuntimeError::presence_store)
    }
}

impl Default for SessionPresenceRuntime {
    fn default() -> Self {
        Self::with_store(Arc::new(RuntimeMemoryPresenceStateStore::default()))
    }
}

pub fn build_app() -> Router {
    build_app_with_state(AppState::default())
}

pub fn build_app_with_cluster(realtime_cluster: Arc<RealtimeClusterBridge>) -> Router {
    build_app_with_state(AppState::with_cluster(realtime_cluster))
}

pub fn build_app_with_cluster_and_runtime(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
) -> Router {
    build_app_with_state(AppState::with_cluster_and_runtime(
        realtime_cluster,
        realtime_runtime,
    ))
}

pub fn build_app_with_cluster_runtime_and_presence(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    presence_runtime: Arc<SessionPresenceRuntime>,
) -> Router {
    build_app_with_state(AppState::with_cluster_and_runtime_and_presence(
        realtime_cluster,
        realtime_runtime,
        presence_runtime,
    ))
}

pub fn build_public_app() -> Router {
    build_app().layer(middleware::from_fn(require_public_bearer_auth))
}

fn build_app_with_state(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/api/v1/sessions/resume", post(resume_session))
        .route("/api/v1/sessions/disconnect", post(disconnect_session))
        .route("/api/v1/presence/heartbeat", post(heartbeat_presence))
        .route("/api/v1/presence/me", get(get_presence_me))
        .route(
            "/api/v1/realtime/subscriptions/sync",
            post(sync_realtime_subscriptions),
        )
        .route("/api/v1/realtime/ws", get(realtime_websocket))
        .route("/api/v1/realtime/events/ack", post(ack_realtime_events))
        .route("/api/v1/realtime/events", get(list_realtime_events))
        .with_state(state)
}

async fn require_public_bearer_auth(request: Request<axum::body::Body>, next: Next) -> Response {
    match request.uri().path() {
        "/healthz" => next.run(request).await,
        _ => match resolve_public_bearer_auth_context(request.headers()) {
            Ok(_) => next.run(request).await,
            Err(error) => ApiError::from(error).into_response(),
        },
    }
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "session-gateway",
    })
}

async fn resume_session(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<ResumeSessionRequest>,
) -> Result<Json<SessionResumeView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    state.register_device(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
        "http",
        true,
    )?;
    let view = state.presence_runtime.resume(
        &auth,
        device_id.clone(),
        request.last_seen_sync_seq.unwrap_or_default(),
        state.latest_device_sync_seq(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            device_id.as_str(),
        ),
        state.registered_devices(auth.tenant_id.as_str(), auth.actor_id.as_str()),
    )?;
    Ok(Json(view))
}

async fn get_presence_me(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<PresenceSnapshotView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.presence_runtime.presence_snapshot(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        auth.device_id.clone(),
        state.registered_devices(auth.tenant_id.as_str(), auth.actor_id.as_str()),
    )?))
}

async fn heartbeat_presence(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PresenceDeviceRequest>,
) -> Result<Json<PresenceSnapshotView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    state.realtime_cluster.ensure_route_session_current(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
    )?;
    state.register_device(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
        "http",
        false,
    )?;
    Ok(Json(state.presence_runtime.heartbeat(
        &auth,
        device_id.clone(),
        state.latest_device_sync_seq(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            device_id.as_str(),
        ),
        state.registered_devices(auth.tenant_id.as_str(), auth.actor_id.as_str()),
    )?))
}

async fn disconnect_session(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PresenceDeviceRequest>,
) -> Result<Json<PresenceSnapshotView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    if state.realtime_cluster.disconnect_fence_matches_session(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
    )? {
        state.realtime_runtime.signal_device_disconnect(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            device_id.as_str(),
        )?;
        return Ok(Json(state.presence_runtime.presence_snapshot(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            Some(device_id),
            state.registered_devices(auth.tenant_id.as_str(), auth.actor_id.as_str()),
        )?));
    }
    state.realtime_cluster.ensure_route_session_current(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
    )?;
    state.register_device(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
        "http",
        false,
    )?;
    state.realtime_runtime.clear_device_subscriptions(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
    )?;
    let _ = state.realtime_cluster.release_device_route(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        state.node_id.as_str(),
    );
    state.realtime_cluster.mark_device_disconnected(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
        state.node_id.as_str(),
    )?;
    state.realtime_runtime.signal_device_disconnect(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
    )?;
    Ok(Json(state.presence_runtime.disconnect(
        &auth,
        device_id,
        state.registered_devices(auth.tenant_id.as_str(), auth.actor_id.as_str()),
    )?))
}

async fn sync_realtime_subscriptions(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<SyncRealtimeSubscriptionsRequest>,
) -> Result<Json<RealtimeSubscriptionSnapshot>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    state.realtime_cluster.ensure_route_session_current(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
    )?;
    state.register_device(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
        "http",
        false,
    )?;
    Ok(Json(state.realtime_runtime.sync_subscriptions(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        request.items,
    )?))
}

async fn list_realtime_events(
    Query(query): Query<ListRealtimeEventsQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<RealtimeEventWindow>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, None)?;
    state.realtime_cluster.ensure_route_session_current(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
    )?;
    state.register_device(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
        "http_poll",
        false,
    )?;
    let limit = query.limit.unwrap_or(100);
    if limit == 0 {
        return Err(ApiError::bad_request(
            "limit_invalid",
            "limit must be greater than 0",
        ));
    }
    Ok(Json(state.realtime_runtime.list_events(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        query.after_seq.unwrap_or_default(),
        limit,
    )?))
}

async fn ack_realtime_events(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AckRealtimeEventsRequest>,
) -> Result<Json<RealtimeAckState>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    state.realtime_cluster.ensure_route_session_current(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
    )?;
    state.register_device(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
        "http",
        false,
    )?;
    Ok(Json(state.realtime_runtime.ack_events(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        request.acked_seq,
    )?))
}

async fn realtime_websocket(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<axum::response::Response, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, None)?;
    state.realtime_cluster.ensure_route_session_current(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
    )?;
    state.register_device(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
        "websocket",
        false,
    )?;
    let runtime = state.realtime_runtime.clone();
    Ok(ws
        .on_upgrade(move |socket| serve_realtime_websocket(socket, auth, device_id, runtime))
        .into_response())
}

impl Default for AppState {
    fn default() -> Self {
        Self::with_cluster(Arc::new(RealtimeClusterBridge::default()))
    }
}

impl AppState {
    fn with_cluster(realtime_cluster: Arc<RealtimeClusterBridge>) -> Self {
        Self::with_cluster_and_runtime(
            realtime_cluster,
            Arc::new(RealtimeDeliveryRuntime::default()),
        )
    }

    fn with_cluster_and_runtime(
        realtime_cluster: Arc<RealtimeClusterBridge>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    ) -> Self {
        Self::with_cluster_and_runtime_and_presence(
            realtime_cluster,
            realtime_runtime,
            Arc::new(SessionPresenceRuntime::default()),
        )
    }

    fn with_cluster_and_runtime_and_presence(
        realtime_cluster: Arc<RealtimeClusterBridge>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
        presence_runtime: Arc<SessionPresenceRuntime>,
    ) -> Self {
        let node_id = "session_gateway_local_1".to_owned();
        realtime_cluster.bind_node_runtime(node_id.as_str(), realtime_runtime.clone());
        Self {
            node_id,
            realtime_cluster,
            presence_runtime,
            realtime_runtime,
            registered_devices: Arc::new(Mutex::new(HashMap::new())),
            latest_sync_sequences: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    fn register_device(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        session_id: Option<&str>,
        connection_kind: &str,
        allow_session_takeover: bool,
    ) -> Result<(), ApiError> {
        if !allow_session_takeover {
            self.realtime_cluster.ensure_device_resume_not_required(
                tenant_id,
                principal_id,
                device_id,
            )?;
            self.presence_runtime.ensure_device_resume_not_required(
                tenant_id,
                principal_id,
                device_id,
            )?;
        }
        self.presence_runtime
            .register_device(tenant_id, principal_id, device_id)?;
        self.realtime_runtime
            .ensure_device_state(tenant_id, principal_id, device_id)?;
        self.registered_devices
            .lock()
            .expect("registered device store should lock")
            .entry(principal_scope_key(tenant_id, principal_id))
            .or_default()
            .insert(device_id.into());
        self.latest_sync_sequences
            .lock()
            .expect("latest sync sequence store should lock")
            .entry(device_scope_key(tenant_id, principal_id, device_id))
            .or_insert(0);
        self.realtime_cluster.bind_device_route(
            tenant_id,
            principal_id,
            device_id,
            self.node_id.as_str(),
            session_id,
            connection_kind,
        )?;
        if allow_session_takeover {
            self.realtime_cluster.clear_device_disconnect_fence(
                tenant_id,
                principal_id,
                device_id,
            )?;
        }
        Ok(())
    }

    fn registered_devices(&self, tenant_id: &str, principal_id: &str) -> Vec<String> {
        self.registered_devices
            .lock()
            .expect("registered device store should lock")
            .get(principal_scope_key(tenant_id, principal_id).as_str())
            .map(|items| items.iter().cloned().collect())
            .unwrap_or_default()
    }

    fn latest_device_sync_seq(&self, tenant_id: &str, principal_id: &str, device_id: &str) -> u64 {
        self.latest_sync_sequences
            .lock()
            .expect("latest sync sequence store should lock")
            .get(device_scope_key(tenant_id, principal_id, device_id).as_str())
            .copied()
            .unwrap_or_default()
    }
}

fn resolve_requested_device_id(
    auth: &AuthContext,
    requested_device_id: Option<String>,
) -> Result<String, ApiError> {
    match (requested_device_id, auth.device_id.clone()) {
        (Some(requested), Some(bound)) => {
            if requested != bound {
                return Err(ApiError::bad_request(
                    "device_id_mismatch",
                    format!("device id does not match auth context: {requested}"),
                ));
            }
            Ok(requested)
        }
        (Some(requested), None) => Ok(requested),
        (None, Some(bound)) => Ok(bound),
        (None, None) => Err(ApiError::bad_request(
            "device_id_missing",
            "device id must be provided by auth context or request body",
        )),
    }
}

fn empty_presence_view(auth: &AuthContext, device_id: &str) -> DevicePresenceView {
    empty_presence_view_for_scope(auth.tenant_id.as_str(), auth.actor_id.as_str(), device_id)
}

fn empty_presence_view_for_scope(
    tenant_id: &str,
    principal_id: &str,
    device_id: &str,
) -> DevicePresenceView {
    DevicePresenceView {
        tenant_id: tenant_id.into(),
        principal_id: principal_id.into(),
        device_id: device_id.into(),
        platform: None,
        session_id: None,
        status: DevicePresenceStatus::Offline,
        last_sync_seq: 0,
        last_resume_at: None,
        last_seen_at: None,
    }
}

fn normalize_presence_record(
    record: PresenceStateRecord,
) -> (PresenceRuntimeEntry, Option<PresenceStateRecord>) {
    let mut presence = record.presence.clone();
    let mut resume_required = record.resume_required;
    let mut normalized = false;

    if matches!(presence.status, DevicePresenceStatus::Online) {
        presence.status = DevicePresenceStatus::Offline;
        presence.session_id = None;
        resume_required = true;
        normalized = true;
    } else if presence.session_id.is_some() {
        presence.session_id = None;
        normalized = true;
    }

    let entry = PresenceRuntimeEntry {
        view: presence.clone(),
        resume_required,
    };
    let normalized_record = if normalized || resume_required != record.resume_required {
        Some(PresenceStateRecord {
            tenant_id: record.tenant_id,
            principal_id: record.principal_id,
            device_id: record.device_id,
            presence,
            resume_required,
            updated_at: session_timestamp(),
        })
    } else {
        None
    };

    (entry, normalized_record)
}

fn principal_scope_key(tenant_id: &str, principal_id: &str) -> String {
    format!("{tenant_id}:{principal_id}")
}

fn device_scope_key(tenant_id: &str, principal_id: &str, device_id: &str) -> String {
    format!("{tenant_id}:{principal_id}:{device_id}")
}

fn session_timestamp() -> String {
    utc_now_rfc3339_millis()
}
