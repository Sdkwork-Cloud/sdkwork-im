use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::{
    Json, Router,
    routing::{get, post},
};
use craw_chat_api_registry::HttpMethod;
use craw_chat_openapi::{
    OpenApiServiceSpec, WebsocketRouteMetadata, build_openapi_document,
    extract_routes_from_function, render_docs_html,
};
use im_adapter_iot_access_local::LocalDeviceAccessProvider;
use im_auth_context::{
    AuthContext, AuthContextError, resolve_auth_context, resolve_public_bearer_auth_context,
};
use im_domain_core::realtime::{
    RealtimeAckState, RealtimeEventWindow, RealtimeSubscriptionSnapshot,
};
use im_platform_contracts::{ContractError, DeviceAccessProvider};
use serde::{Deserialize, Serialize};

mod assembly;
mod cluster;
mod device_registration;
mod presence;
mod principal_scope;
mod realtime;
mod session;
mod session_state;
mod websocket;
mod websocket_route;
mod websocket_upgrade;

pub use assembly::RealtimePlaneAssembly;
pub use cluster::{
    RealtimeClusterBridge, RealtimeClusterError, RealtimeDeviceRoute, RealtimeNodeLifecycleView,
    RealtimeRouteDeliveryResult, RealtimeRouteMigrationResult,
};
use device_registration::{DisconnectActiveDeviceRouteOutcome, SessionDeviceRegistration};
pub use presence::{PresenceRuntimeError, SessionPresenceRuntime};
pub use realtime::{
    AckRealtimeEventsRequest, ListRealtimeEventsQuery, RealtimeDeliveryRuntime,
    RealtimeDeviceStateSnapshot, RealtimeRuntimeError, RealtimeScopeAccessPolicy,
    RealtimeSubscriptionItemInput, SyncRealtimeSubscriptionsRequest,
};
use session_state::SessionSyncState;
pub use websocket::{
    CCP_WEBSOCKET_SUBPROTOCOL, REALTIME_OVERLOAD_CLOSE_CODE, REALTIME_OVERLOAD_CLOSE_REASON,
    RealtimeRouteOwner, RealtimeRouteOwnerError, RealtimeWebsocketMode,
    SESSION_DISCONNECT_CLOSE_CODE, SESSION_DISCONNECT_CLOSE_REASON, serve_realtime_websocket,
};

const SESSION_GATEWAY_MAX_DEVICE_ID_BYTES: usize = 256;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Clone)]
struct AppState {
    presence_runtime: Arc<SessionPresenceRuntime>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    session_state: SessionSyncState,
    device_registration: SessionDeviceRegistration,
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

    fn conflict(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::CONFLICT,
            code,
            message: message.into(),
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

    fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code,
            message: message.into(),
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
            "payload_too_large" => axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            "limit_invalid" => axum::http::StatusCode::BAD_REQUEST,
            "conversation_archived" | "conversation_blocked" => axum::http::StatusCode::FORBIDDEN,
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
        let status = match value.code() {
            "presence_store_unavailable" => axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "presence_store_conflict" | "reconnect_required" => axum::http::StatusCode::CONFLICT,
            "presence_store_unsupported" => axum::http::StatusCode::NOT_IMPLEMENTED,
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self {
            status,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl From<ContractError> for ApiError {
    fn from(value: ContractError) -> Self {
        match value {
            ContractError::UnsupportedCapability(message) => Self {
                status: axum::http::StatusCode::NOT_IMPLEMENTED,
                code: "provider_capability_unsupported",
                message,
            },
            ContractError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                code: "provider_conflict",
                message,
            },
            ContractError::Unavailable(message) => Self {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "provider_unavailable",
                message,
            },
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

pub fn build_app() -> Router {
    build_app_with_state(AppState::default())
}

pub fn build_app_with_device_access_provider(
    device_access_provider: Arc<dyn DeviceAccessProvider>,
) -> Router {
    build_app_with_state(AppState::with_device_access_provider(
        device_access_provider,
    ))
}

pub fn build_app_with_cluster(realtime_cluster: Arc<RealtimeClusterBridge>) -> Router {
    build_app_with_state(AppState::with_cluster(realtime_cluster))
}

pub fn build_app_with_cluster_and_device_access_provider(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    device_access_provider: Arc<dyn DeviceAccessProvider>,
) -> Router {
    build_app_with_state(AppState::with_cluster_and_device_access_provider(
        realtime_cluster,
        device_access_provider,
    ))
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

fn build_default_device_access_provider() -> Arc<dyn DeviceAccessProvider> {
    Arc::new(LocalDeviceAccessProvider::default())
}

fn build_app_with_state(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs))
        .route("/api/v1/sessions/resume", post(session::resume_session))
        .route(
            "/api/v1/sessions/disconnect",
            post(session::disconnect_session),
        )
        .route(
            "/api/v1/presence/heartbeat",
            post(session::heartbeat_presence),
        )
        .route("/api/v1/presence/me", get(session::get_presence_me))
        .route(
            "/api/v1/realtime/subscriptions/sync",
            post(sync_realtime_subscriptions),
        )
        .route(
            "/api/v1/realtime/ws",
            get(websocket_upgrade::realtime_websocket),
        )
        .route("/api/v1/realtime/events/ack", post(ack_realtime_events))
        .route("/api/v1/realtime/events", get(list_realtime_events))
        .with_state(state)
}

async fn require_public_bearer_auth(request: Request<axum::body::Body>, next: Next) -> Response {
    match request.uri().path() {
        "/healthz" | "/openapi.json" | "/docs" => next.run(request).await,
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

async fn openapi_json() -> Result<Json<serde_json::Value>, ApiError> {
    Ok(Json(build_session_gateway_openapi_document().map_err(
        |message| ApiError::internal("openapi_export_failed", message),
    )?))
}

async fn docs() -> Html<String> {
    Html(render_docs_html(&session_gateway_openapi_spec()))
}

async fn sync_realtime_subscriptions(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<SyncRealtimeSubscriptionsRequest>,
) -> Result<Json<RealtimeSubscriptionSnapshot>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    state.prepare_active_device_route(&auth, device_id.as_str(), "http", false)?;
    Ok(Json(
        state
            .realtime_runtime
            .sync_subscriptions_for_principal_kind(
                auth.tenant_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                device_id.as_str(),
                request.items,
            )?,
    ))
}

async fn list_realtime_events(
    Query(query): Query<ListRealtimeEventsQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<RealtimeEventWindow>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, None)?;
    state.prepare_active_device_route(&auth, device_id.as_str(), "http_poll", false)?;
    let limit = query.limit.unwrap_or(100);
    if limit == 0 {
        return Err(ApiError::bad_request(
            "limit_invalid",
            "limit must be greater than 0",
        ));
    }
    realtime::validate_realtime_event_limit(limit)?;
    Ok(Json(
        state.realtime_runtime.list_events_for_principal_kind(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            device_id.as_str(),
            query.after_seq.unwrap_or_default(),
            limit,
        )?,
    ))
}

async fn ack_realtime_events(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AckRealtimeEventsRequest>,
) -> Result<Json<RealtimeAckState>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    state.prepare_active_device_route(&auth, device_id.as_str(), "http", false)?;
    Ok(Json(state.realtime_runtime.ack_events_for_principal_kind(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        auth.actor_kind.as_str(),
        device_id.as_str(),
        request.acked_seq,
    )?))
}

impl Default for AppState {
    fn default() -> Self {
        Self::with_device_access_provider(build_default_device_access_provider())
    }
}

impl AppState {
    fn with_device_access_provider(device_access_provider: Arc<dyn DeviceAccessProvider>) -> Self {
        Self::with_cluster_and_device_access_provider(
            Arc::new(RealtimeClusterBridge::default()),
            device_access_provider,
        )
    }

    fn with_cluster(realtime_cluster: Arc<RealtimeClusterBridge>) -> Self {
        Self::with_cluster_and_device_access_provider(
            realtime_cluster,
            build_default_device_access_provider(),
        )
    }

    fn with_cluster_and_device_access_provider(
        realtime_cluster: Arc<RealtimeClusterBridge>,
        device_access_provider: Arc<dyn DeviceAccessProvider>,
    ) -> Self {
        Self::with_cluster_and_runtime_and_device_access_provider(
            realtime_cluster,
            Arc::new(RealtimeDeliveryRuntime::default()),
            device_access_provider,
        )
    }

    fn with_cluster_and_runtime(
        realtime_cluster: Arc<RealtimeClusterBridge>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    ) -> Self {
        Self::with_cluster_and_runtime_and_device_access_provider(
            realtime_cluster,
            realtime_runtime,
            build_default_device_access_provider(),
        )
    }

    fn with_cluster_and_runtime_and_device_access_provider(
        realtime_cluster: Arc<RealtimeClusterBridge>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
        device_access_provider: Arc<dyn DeviceAccessProvider>,
    ) -> Self {
        Self::with_cluster_and_runtime_and_presence_and_device_access_provider(
            realtime_cluster,
            realtime_runtime,
            Arc::new(SessionPresenceRuntime::default()),
            device_access_provider,
        )
    }

    fn with_cluster_and_runtime_and_presence(
        realtime_cluster: Arc<RealtimeClusterBridge>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
        presence_runtime: Arc<SessionPresenceRuntime>,
    ) -> Self {
        Self::with_cluster_and_runtime_and_presence_and_device_access_provider(
            realtime_cluster,
            realtime_runtime,
            presence_runtime,
            build_default_device_access_provider(),
        )
    }

    fn with_cluster_and_runtime_and_presence_and_device_access_provider(
        realtime_cluster: Arc<RealtimeClusterBridge>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
        presence_runtime: Arc<SessionPresenceRuntime>,
        device_access_provider: Arc<dyn DeviceAccessProvider>,
    ) -> Self {
        let node_id = "session_gateway_local_1".to_owned();
        realtime_cluster.bind_node_runtime(node_id.as_str(), realtime_runtime.clone());
        let session_state = SessionSyncState::default();
        Self {
            device_registration: SessionDeviceRegistration::new(
                node_id.clone(),
                realtime_cluster.clone(),
                presence_runtime.clone(),
                realtime_runtime.clone(),
                session_state.clone(),
                device_access_provider,
            ),
            presence_runtime,
            realtime_runtime,
            session_state,
        }
    }

    fn register_device(
        &self,
        auth: &AuthContext,
        device_id: &str,
        connection_kind: &str,
        allow_session_takeover: bool,
    ) -> Result<(), ApiError> {
        self.device_registration.register_device(
            auth,
            device_id,
            connection_kind,
            allow_session_takeover,
        )
    }

    fn prepare_active_device_route(
        &self,
        auth: &AuthContext,
        device_id: &str,
        connection_kind: &str,
        allow_session_takeover: bool,
    ) -> Result<(), ApiError> {
        self.device_registration.prepare_active_device_route(
            auth,
            device_id,
            connection_kind,
            allow_session_takeover,
        )
    }

    #[rustfmt::skip]
    fn disconnect_active_device_route(
        &self,
        auth: &AuthContext,
        device_id: &str,
        connection_kind: &str,
    ) -> Result<DisconnectActiveDeviceRouteOutcome, ApiError> {
        self.device_registration.disconnect_active_device_route(auth, device_id, connection_kind)
    }

    fn device_sync_session_state(
        &self,
        auth: &AuthContext,
        requested_device_id: Option<&str>,
    ) -> Result<session_state::DeviceSyncSessionState, ApiError> {
        self.session_state
            .device_sync_session_state(auth, requested_device_id)
    }
}

fn resolve_requested_device_id(
    auth: &AuthContext,
    requested_device_id: Option<String>,
) -> Result<String, ApiError> {
    match (requested_device_id, auth.device_id.clone()) {
        (Some(requested), Some(bound)) => {
            validate_device_id(requested.as_str())?;
            validate_device_id(bound.as_str())?;
            if requested != bound {
                return Err(ApiError::bad_request(
                    "device_id_mismatch",
                    format!("device id does not match auth context: {requested}"),
                ));
            }
            Ok(requested)
        }
        (Some(requested), None) => {
            validate_device_id(requested.as_str())?;
            Ok(requested)
        }
        (None, Some(bound)) => {
            validate_device_id(bound.as_str())?;
            Ok(bound)
        }
        (None, None) => Err(ApiError::bad_request(
            "device_id_missing",
            "device id must be provided by auth context or request body",
        )),
    }
}

fn validate_device_id(device_id: &str) -> Result<(), ApiError> {
    let actual_bytes = device_id.len();
    if actual_bytes > SESSION_GATEWAY_MAX_DEVICE_ID_BYTES {
        return Err(ApiError::payload_too_large(
            "deviceId",
            SESSION_GATEWAY_MAX_DEVICE_ID_BYTES,
            actual_bytes,
        ));
    }
    Ok(())
}

fn build_session_gateway_openapi_document() -> Result<serde_json::Value, String> {
    let routes = extract_routes_from_function(
        include_str!("lib.rs"),
        "build_app_with_state",
        &[WebsocketRouteMetadata {
            path: "/api/v1/realtime/ws".to_owned(),
            subprotocols: vec![CCP_WEBSOCKET_SUBPROTOCOL.to_owned()],
        }],
        &["/openapi.json", "/docs"],
    )?;

    Ok(build_openapi_document(
        &session_gateway_openapi_spec(),
        &routes,
        session_gateway_tag,
        session_gateway_requires_bearer,
        session_gateway_summary,
    ))
}

fn session_gateway_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Craw Chat Session Gateway API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the session-gateway router for session lifecycle, presence, realtime polling, and websocket upgrade flows.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn session_gateway_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" => "system".to_owned(),
        path if path.starts_with("/api/v1/sessions/") => "sessions".to_owned(),
        path if path.starts_with("/api/v1/presence/") => "presence".to_owned(),
        path if path.starts_with("/api/v1/realtime/") => "realtime".to_owned(),
        _ => "misc".to_owned(),
    }
}

fn session_gateway_requires_bearer(path: &str, _method: HttpMethod) -> bool {
    path != "/healthz"
}

fn session_gateway_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check session gateway health".to_owned(),
        ("/api/v1/sessions/resume", HttpMethod::Post) => {
            "Resume session and return device presence snapshot".to_owned()
        }
        ("/api/v1/sessions/disconnect", HttpMethod::Post) => {
            "Disconnect current session device".to_owned()
        }
        ("/api/v1/presence/heartbeat", HttpMethod::Post) => {
            "Refresh device presence heartbeat".to_owned()
        }
        ("/api/v1/presence/me", HttpMethod::Get) => {
            "Get current device presence snapshot".to_owned()
        }
        ("/api/v1/realtime/subscriptions/sync", HttpMethod::Post) => {
            "Sync realtime subscriptions".to_owned()
        }
        ("/api/v1/realtime/ws", HttpMethod::Get) => "Open realtime websocket session".to_owned(),
        ("/api/v1/realtime/events/ack", HttpMethod::Post) => {
            "Acknowledge realtime events".to_owned()
        }
        ("/api/v1/realtime/events", HttpMethod::Get) => "Pull realtime event window".to_owned(),
        _ => format!("{:?} {}", method, path),
    }
}

#[cfg(test)]
mod tests {
    // websocket upgrade seam tests live in `src/websocket_upgrade.rs`
}
