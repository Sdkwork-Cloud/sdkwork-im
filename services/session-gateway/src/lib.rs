use std::sync::Arc;

use axum::extract::{DefaultBodyLimit, Extension, Query, State};
use axum::http::HeaderMap;
use axum::http::Request;
use axum::http::header::CONTENT_TYPE;
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
use im_app_context::{
    AppContext, AppContextError, AppContextSignatureConfig, require_app_context_signature,
    resolve_app_context, resolve_app_context_for_request,
};
use im_domain_core::realtime::{
    RealtimeAckState, RealtimeEventWindow, RealtimeSubscriptionSnapshot,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;

mod assembly;
mod client_route_registration;
mod client_route_state;
mod cluster;
mod presence;
mod presence_routes;
mod principal_scope;
mod realtime;
mod websocket;
mod websocket_route;
mod websocket_upgrade;

pub use assembly::RealtimePlaneAssembly;
use client_route_registration::ClientRouteRegistration;
use client_route_state::ClientRouteState;
pub use cluster::{
    RealtimeClientRoute, RealtimeClusterBridge, RealtimeClusterError, RealtimeNodeLifecycleView,
    RealtimeRouteDeliveryResult, RealtimeRouteMigrationResult,
};
pub use presence::{PresenceRuntime, PresenceRuntimeError};
pub use realtime::{
    AckRealtimeEventsRequest, ListRealtimeEventsQuery, RealtimeClientRouteStateSnapshot,
    RealtimeDeliveryRuntime, RealtimeInboxDiagnosticsSnapshot, RealtimePostgresAdapterPlan,
    RealtimePostgresBindingError, RealtimePostgresBindingValue, RealtimePostgresBoundParameter,
    RealtimePostgresBoundStatement, RealtimePostgresBoundTransaction,
    RealtimePostgresCheckpointMutation, RealtimePostgresClientRouteEventMutation,
    RealtimePostgresMethodAtomicity, RealtimePostgresMethodPlan, RealtimePostgresMethodStep,
    RealtimePostgresParameterBinding, RealtimePostgresRowColumn, RealtimePostgresRowMapping,
    RealtimePostgresSqlContract, RealtimeRuntimeError, RealtimeScopeAccessPolicy,
    RealtimeSubscriptionItemInput, StandaloneRealtimeScopeAccessPolicy,
    SyncRealtimeSubscriptionsRequest, realtime_postgres_adapter_plan,
    realtime_postgres_bind_ack_transaction, realtime_postgres_bind_checkpoint_upsert,
    realtime_postgres_bind_client_route_event_upsert, realtime_postgres_bind_publish_transaction,
    realtime_postgres_bind_save_subscription_transaction,
    realtime_postgres_bind_subscription_scope_clear,
    realtime_postgres_bind_subscription_scope_replacements,
    realtime_postgres_bind_subscription_upsert, realtime_postgres_bind_trim_client_route_events,
    realtime_postgres_sql_contract_specs, realtime_postgres_sql_contracts,
    realtime_postgres_transaction_plans,
};
pub use websocket::{
    CCP_WEBSOCKET_SUBPROTOCOL, REALTIME_OVERLOAD_CLOSE_CODE, REALTIME_OVERLOAD_CLOSE_REASON,
    RealtimeRouteOwner, RealtimeRouteOwnerError, RealtimeWebsocketMode,
    SESSION_DISCONNECT_CLOSE_CODE, SESSION_DISCONNECT_CLOSE_REASON, serve_realtime_websocket,
};

const SESSION_GATEWAY_MAX_DEVICE_ID_BYTES: usize = 256;
const REALTIME_MAX_WEBSOCKET_CONNECTIONS_ENV: &str = "CRAW_CHAT_REALTIME_MAX_WEBSOCKET_CONNECTIONS";
const REALTIME_MAX_WEBSOCKET_CONNECTIONS_DEFAULT: usize = 10_000;
const REALTIME_MAX_WEBSOCKET_CONNECTIONS_MAX: usize = 100_000;
const SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS_ENV: &str =
    "CRAW_CHAT_SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS";
const SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 2_000;
const SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 50_000;
const SESSION_GATEWAY_MAX_REQUEST_BODY_BYTES_ENV: &str =
    "CRAW_CHAT_SESSION_GATEWAY_MAX_REQUEST_BODY_BYTES";
const SESSION_GATEWAY_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
const SESSION_GATEWAY_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;
const SESSION_GATEWAY_REQUIRE_DUAL_TOKEN_HEADERS_ENV: &str =
    "CRAW_CHAT_SESSION_GATEWAY_REQUIRE_DUAL_TOKEN_HEADERS";

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Clone)]
struct AppState {
    presence_runtime: Arc<PresenceRuntime>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    client_route_state: ClientRouteState,
    client_route_registration: ClientRouteRegistration,
    websocket_connection_semaphore: Arc<Semaphore>,
}

#[derive(Clone)]
struct PublicAppGuardrails {
    request_gate: Arc<Semaphore>,
    require_dual_token_headers: bool,
    app_context_signature_config: AppContextSignatureConfig,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct PresenceHeartbeatRequest {
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

impl From<AppContextError> for ApiError {
    fn from(value: AppContextError) -> Self {
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
            "conversation_archived" | "conversation_blocked" | "realtime_scope_access_denied" => {
                axum::http::StatusCode::FORBIDDEN
            }
            "checkpoint_store_unavailable"
            | "subscription_store_unavailable"
            | "event_window_store_unavailable" => axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "checkpoint_store_conflict"
            | "subscription_store_conflict"
            | "event_window_store_conflict" => axum::http::StatusCode::CONFLICT,
            "checkpoint_store_unsupported"
            | "subscription_store_unsupported"
            | "event_window_store_unsupported" => axum::http::StatusCode::NOT_IMPLEMENTED,
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

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status;
        let detail = self.message;
        let message = detail.clone();
        let title = status.canonical_reason().unwrap_or("Unknown Error");
        (
            status,
            [(CONTENT_TYPE, "application/problem+json; charset=utf-8")],
            Json(serde_json::json!({
                "type": "about:blank",
                "title": title,
                "status": status.as_u16(),
                "detail": detail,
                "code": self.code,
                "message": message
            })),
        )
            .into_response()
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
    presence_runtime: Arc<PresenceRuntime>,
) -> Router {
    build_app_with_state(AppState::with_cluster_and_runtime_and_presence(
        realtime_cluster,
        realtime_runtime,
        presence_runtime,
    ))
}

pub fn build_public_app() -> Router {
    let guardrails = PublicAppGuardrails {
        request_gate: Arc::new(Semaphore::new(resolve_max_in_flight_requests())),
        require_dual_token_headers: resolve_require_dual_token_headers(),
        app_context_signature_config: AppContextSignatureConfig::from_env(),
    };
    build_app()
        .layer(DefaultBodyLimit::max(resolve_max_http_request_body_bytes()))
        .layer(middleware::from_fn_with_state(
            guardrails,
            require_app_context,
        ))
}

fn build_app_with_state(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs))
        .route(
            "/im/v3/api/presence/heartbeat",
            post(presence_routes::heartbeat_presence),
        )
        .route(
            "/im/v3/api/presence/me",
            get(presence_routes::get_presence_me),
        )
        .route(
            "/im/v3/api/realtime/subscriptions/sync",
            post(sync_realtime_subscriptions),
        )
        .route(
            "/im/v3/api/realtime/ws",
            get(websocket_upgrade::realtime_websocket),
        )
        .route("/im/v3/api/realtime/events/ack", post(ack_realtime_events))
        .route("/im/v3/api/realtime/events", get(list_realtime_events))
        .with_state(state)
}

async fn require_app_context(
    State(guardrails): State<PublicAppGuardrails>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    match request.uri().path() {
        "/healthz" | "/openapi.json" | "/docs" => next.run(request).await,
        _ => {
            let permit = match guardrails.request_gate.clone().try_acquire_owned() {
                Ok(permit) => permit,
                Err(_) => {
                    return ApiError {
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
            if let Err(error) = require_app_context_signature(
                request.headers(),
                &guardrails.app_context_signature_config,
            ) {
                return ApiError::from(error).into_response();
            }
            let resolved = match resolve_app_context_for_request(
                request.headers(),
                request.uri().path(),
                request.method().as_str(),
            ) {
                Ok(resolved) => resolved,
                Err(error) => return ApiError::from(error).into_response(),
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
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<SyncRealtimeSubscriptionsRequest>,
) -> Result<Json<RealtimeSubscriptionSnapshot>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    state
        .realtime_runtime
        .validate_subscriptions_for_principal_kind(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            &request.items,
        )?;
    state.prepare_active_client_route(&auth, device_id.as_str(), "http", false)?;
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
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<RealtimeEventWindow>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    let device_id = resolve_requested_device_id(&auth, None)?;
    let limit = query.limit.unwrap_or(100);
    realtime::validate_realtime_event_limit(limit)?;
    state.prepare_active_client_route(&auth, device_id.as_str(), "http_poll", false)?;
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
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AckRealtimeEventsRequest>,
) -> Result<Json<RealtimeAckState>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    let previous_route = state.current_active_client_route(&auth, device_id.as_str());
    state.prepare_active_client_route(&auth, device_id.as_str(), "http", false)?;
    let bound_route = state.current_active_client_route(&auth, device_id.as_str());
    let ack = state.realtime_runtime.ack_events_for_principal_kind(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        auth.actor_kind.as_str(),
        device_id.as_str(),
        request.acked_seq,
    );
    match ack {
        Ok(ack) => Ok(Json(ack)),
        Err(error) => {
            match (previous_route, bound_route) {
                (Some(previous_route), Some(bound_route)) => {
                    state.restore_active_client_route_if_current(&bound_route, previous_route);
                }
                (None, _) => {
                    state.release_active_client_route_if_current_session(&auth, device_id.as_str());
                }
                _ => {}
            }
            Err(error.into())
        }
    }
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
            Arc::new(RealtimeDeliveryRuntime::standalone_gateway()),
        )
    }

    fn with_cluster_and_runtime(
        realtime_cluster: Arc<RealtimeClusterBridge>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    ) -> Self {
        Self::with_cluster_and_runtime_and_presence(
            realtime_cluster,
            realtime_runtime,
            Arc::new(PresenceRuntime::default()),
        )
    }

    fn with_cluster_and_runtime_and_presence(
        realtime_cluster: Arc<RealtimeClusterBridge>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
        presence_runtime: Arc<PresenceRuntime>,
    ) -> Self {
        let node_id = "session_gateway_local_1".to_owned();
        realtime_cluster.bind_node_runtime(node_id.as_str(), realtime_runtime.clone());
        let client_route_state = ClientRouteState::default();
        let max_connections = resolve_max_websocket_connections();
        Self {
            client_route_registration: ClientRouteRegistration::new(
                node_id.clone(),
                realtime_cluster.clone(),
                presence_runtime.clone(),
                realtime_runtime.clone(),
                client_route_state.clone(),
            ),
            presence_runtime,
            realtime_runtime,
            client_route_state,
            websocket_connection_semaphore: Arc::new(Semaphore::new(max_connections)),
        }
    }

    fn prepare_active_client_route(
        &self,
        auth: &AppContext,
        device_id: &str,
        connection_kind: &str,
        allow_session_takeover: bool,
    ) -> Result<(), ApiError> {
        self.client_route_registration.prepare_active_client_route(
            auth,
            device_id,
            connection_kind,
            allow_session_takeover,
        )
    }

    fn current_active_client_route(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Option<RealtimeClientRoute> {
        self.client_route_registration
            .current_active_client_route(auth, device_id)
    }

    fn restore_active_client_route_if_current(
        &self,
        expected_current: &RealtimeClientRoute,
        restore_to: RealtimeClientRoute,
    ) -> Option<RealtimeClientRoute> {
        self.client_route_registration
            .restore_active_client_route_if_current(expected_current, restore_to)
    }

    fn release_active_client_route_if_current_session(&self, auth: &AppContext, device_id: &str) {
        self.client_route_registration
            .release_active_client_route_if_current_session(auth, device_id);
    }

    fn client_route_state_snapshot(
        &self,
        auth: &AppContext,
        requested_device_id: Option<&str>,
    ) -> Result<client_route_state::ClientRouteStateSnapshot, ApiError> {
        self.client_route_state
            .client_route_state_snapshot(auth, requested_device_id)
    }
}

fn resolve_requested_device_id(
    auth: &AppContext,
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

pub(crate) fn resolve_request_app_context(
    auth: Option<Extension<AppContext>>,
    headers: &HeaderMap,
) -> Result<AppContext, ApiError> {
    match auth {
        Some(Extension(auth)) => Ok(auth),
        None => resolve_app_context(headers).map_err(ApiError::from),
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

fn require_dual_token_headers(headers: &HeaderMap) -> Result<(), ApiError> {
    if !has_bearer_auth_token(headers) {
        return Err(ApiError {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: "auth_token_missing",
            message: "authorization header must provide a bearer token".to_owned(),
        });
    }
    if !has_access_token_header(headers) {
        return Err(ApiError {
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

fn build_session_gateway_openapi_document() -> Result<serde_json::Value, String> {
    let routes = extract_routes_from_function(
        include_str!("lib.rs"),
        "build_app_with_state",
        &[WebsocketRouteMetadata {
            path: "/im/v3/api/realtime/ws".to_owned(),
            subprotocols: vec![CCP_WEBSOCKET_SUBPROTOCOL.to_owned()],
        }],
        &["/openapi.json", "/docs"],
    )?;

    Ok(build_openapi_document(
        &session_gateway_openapi_spec(),
        &routes,
        session_gateway_tag,
        session_gateway_requires_app_context,
        session_gateway_summary,
    ))
}

fn session_gateway_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Craw Chat Realtime Gateway API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the session-gateway router for presence, realtime polling, and websocket upgrade flows.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn session_gateway_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" => "system".to_owned(),
        path if path.starts_with("/im/v3/api/presence/") => "presence".to_owned(),
        path if path.starts_with("/im/v3/api/realtime/") => "realtime".to_owned(),
        _ => "misc".to_owned(),
    }
}

fn session_gateway_requires_app_context(path: &str, _method: HttpMethod) -> bool {
    path != "/healthz"
}

fn session_gateway_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check session gateway health".to_owned(),
        ("/im/v3/api/presence/heartbeat", HttpMethod::Post) => {
            "Refresh device presence heartbeat".to_owned()
        }
        ("/im/v3/api/presence/me", HttpMethod::Get) => {
            "Get current device presence snapshot".to_owned()
        }
        ("/im/v3/api/realtime/subscriptions/sync", HttpMethod::Post) => {
            "Sync realtime subscriptions".to_owned()
        }
        ("/im/v3/api/realtime/ws", HttpMethod::Get) => {
            "Open realtime websocket client route".to_owned()
        }
        ("/im/v3/api/realtime/events/ack", HttpMethod::Post) => {
            "Acknowledge realtime events".to_owned()
        }
        ("/im/v3/api/realtime/events", HttpMethod::Get) => "Pull realtime event window".to_owned(),
        _ => format!("{:?} {}", method, path),
    }
}

fn resolve_max_websocket_connections() -> usize {
    std::env::var(REALTIME_MAX_WEBSOCKET_CONNECTIONS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(REALTIME_MAX_WEBSOCKET_CONNECTIONS_DEFAULT)
        .min(REALTIME_MAX_WEBSOCKET_CONNECTIONS_MAX)
}

fn resolve_max_in_flight_requests() -> usize {
    std::env::var(SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS_MAX)
}

fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(SESSION_GATEWAY_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(SESSION_GATEWAY_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(SESSION_GATEWAY_MAX_REQUEST_BODY_BYTES_MAX)
}

fn resolve_require_dual_token_headers() -> bool {
    std::env::var(SESSION_GATEWAY_REQUIRE_DUAL_TOKEN_HEADERS_ENV)
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

#[cfg(test)]
mod tests {
    use axum::http::{HeaderMap, HeaderValue};

    use super::{has_access_token_header, has_bearer_auth_token, parse_truthy_env_flag};

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
}
