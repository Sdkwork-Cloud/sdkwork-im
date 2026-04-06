use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::{HeaderMap, Request, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use im_auth_context::{
    AuthContext, AuthContextError, resolve_auth_context, resolve_public_bearer_auth_context,
};
use serde::{Deserialize, Serialize};
use session_gateway::{
    RealtimeClusterBridge, RealtimeClusterError, RealtimeNodeLifecycleView,
    RealtimeRouteMigrationResult,
};

#[derive(Clone)]
struct AppState {
    realtime_cluster: Arc<RealtimeClusterBridge>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MigrateRoutesRequest {
    target_node_id: String,
}

#[derive(Debug)]
struct ControlPlaneError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

impl From<RealtimeClusterError> for ControlPlaneError {
    fn from(value: RealtimeClusterError) -> Self {
        let status = match value.code {
            "node_not_found" | "target_node_not_found" | "node_runtime_missing" => {
                StatusCode::NOT_FOUND
            }
            "same_node_migration"
            | "node_not_draining"
            | "target_node_unavailable"
            | "node_draining" => StatusCode::CONFLICT,
            _ => StatusCode::BAD_REQUEST,
        };
        Self {
            status,
            code: value.code,
            message: value.message,
        }
    }
}

impl From<AuthContextError> for ControlPlaneError {
    fn from(value: AuthContextError) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl ControlPlaneError {
    fn forbidden(required_permission: &'static str) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code: "permission_denied",
            message: format!("missing required permission: {required_permission}"),
        }
    }
}

impl axum::response::IntoResponse for ControlPlaneError {
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
    build_app_with_cluster(Arc::new(RealtimeClusterBridge::default()))
}

pub fn build_public_app() -> Router {
    build_app().layer(middleware::from_fn(require_public_bearer_auth))
}

pub fn build_app_with_cluster(realtime_cluster: Arc<RealtimeClusterBridge>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/api/v1/control/nodes/{node_id}/drain", post(drain_node))
        .route(
            "/api/v1/control/nodes/{node_id}/activate",
            post(activate_node),
        )
        .route(
            "/api/v1/control/nodes/{node_id}/routes/migrate",
            post(migrate_node_routes),
        )
        .with_state(AppState { realtime_cluster })
}

async fn require_public_bearer_auth(request: Request<axum::body::Body>, next: Next) -> Response {
    match request.uri().path() {
        "/healthz" => next.run(request).await,
        _ => match resolve_public_bearer_auth_context(request.headers()) {
            Ok(_) => next.run(request).await,
            Err(error) => ControlPlaneError::from(error).into_response(),
        },
    }
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "control-plane-api",
    })
}

async fn drain_node(
    Path(node_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<RealtimeNodeLifecycleView>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;
    Ok(Json(
        state
            .realtime_cluster
            .mark_node_draining(node_id.as_str())?,
    ))
}

async fn activate_node(
    Path(node_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<RealtimeNodeLifecycleView>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;
    Ok(Json(
        state.realtime_cluster.activate_node(node_id.as_str())?,
    ))
}

async fn migrate_node_routes(
    Path(node_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<MigrateRoutesRequest>,
) -> Result<Json<RealtimeRouteMigrationResult>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;
    Ok(Json(state.realtime_cluster.migrate_node_routes(
        node_id.as_str(),
        request.target_node_id.as_str(),
    )?))
}

fn ensure_control_write_access(auth: &AuthContext) -> Result<(), ControlPlaneError> {
    if auth.has_permission("control.write") {
        return Ok(());
    }

    Err(ControlPlaneError::forbidden("control.write"))
}
