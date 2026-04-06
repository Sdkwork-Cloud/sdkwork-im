use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::http::{HeaderMap, Request};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::{Json, Router, routing::get};
use im_auth_context::{
    AuthContext, AuthContextError, resolve_auth_context, resolve_public_bearer_auth_context,
};
use im_time::utc_now_rfc3339_millis;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct AppState {
    runtime: Arc<OpsRuntime>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceHealthView {
    pub service: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpsHealthResponse {
    pub items: Vec<ServiceHealthView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterNodeView {
    pub node_id: String,
    pub profile: String,
    pub bind_addr: String,
    pub drain_status: String,
    pub rebalance_state: String,
    pub device_route_count: usize,
    pub owned_scopes: Vec<String>,
    pub services: Vec<ServiceHealthView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterView {
    pub nodes: Vec<ClusterNodeView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LagItem {
    pub component: String,
    pub scope_id: String,
    pub current_offset: u64,
    pub committed_offset: u64,
    pub lag: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LagView {
    pub items: Vec<LagItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirInspectionItem {
    pub file_name: String,
    pub path: String,
    pub required: bool,
    pub exists: bool,
    pub parseable: bool,
    pub status: String,
    pub size_bytes: Option<u64>,
    pub parse_error: Option<String>,
    pub recommended_action: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirInspectionView {
    pub status: String,
    pub runtime_dir: Option<String>,
    pub state_dir: Option<String>,
    pub healthy_file_count: usize,
    pub missing_file_count: usize,
    pub corrupt_file_count: usize,
    pub files: Vec<RuntimeDirInspectionItem>,
}

impl RuntimeDirInspectionView {
    pub fn unmanaged() -> Self {
        Self {
            status: "unmanaged".into(),
            runtime_dir: None,
            state_dir: None,
            healthy_file_count: 0,
            missing_file_count: 0,
            corrupt_file_count: 0,
            files: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticBundle {
    pub generated_at: String,
    pub profile: String,
    pub node_id: String,
    pub bind_addr: String,
    pub drain_status: String,
    pub rebalance_state: String,
    pub owned_scopes: Vec<String>,
    pub services: Vec<ServiceHealthView>,
    pub lag: Vec<LagItem>,
    pub device_routes: Vec<RouteOwnershipView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteOwnershipView {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
    pub owner_node_id: String,
    pub connection_kind: String,
    pub bound_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

pub struct OpsRuntime {
    node_id: String,
    profile: String,
    bind_addr: String,
    services: Vec<ServiceHealthView>,
    owned_scopes: Vec<String>,
    lag_items: Vec<LagItem>,
    drain_status: Mutex<String>,
    rebalance_state: Mutex<String>,
    device_routes: Mutex<Vec<RouteOwnershipView>>,
    runtime_dir_inspection: Mutex<RuntimeDirInspectionView>,
}

impl Default for OpsRuntime {
    fn default() -> Self {
        Self::new(
            "ops_node_1",
            "standalone",
            "127.0.0.1:18091",
            vec!["ops-service".into()],
            vec!["node:ops_node_1".into()],
        )
    }
}

impl OpsRuntime {
    pub fn new(
        node_id: impl Into<String>,
        profile: impl Into<String>,
        bind_addr: impl Into<String>,
        service_names: Vec<String>,
        owned_scopes: Vec<String>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            profile: profile.into(),
            bind_addr: bind_addr.into(),
            services: service_names
                .into_iter()
                .map(|service| ServiceHealthView {
                    service,
                    status: "ok".into(),
                })
                .collect(),
            owned_scopes,
            lag_items: vec![LagItem {
                component: "commit_journal".into(),
                scope_id: "local-minimal".into(),
                current_offset: 0,
                committed_offset: 0,
                lag: 0,
            }],
            drain_status: Mutex::new("active".into()),
            rebalance_state: Mutex::new("stable".into()),
            device_routes: Mutex::new(Vec::new()),
            runtime_dir_inspection: Mutex::new(RuntimeDirInspectionView::unmanaged()),
        }
    }

    pub fn set_node_lifecycle(&self, drain_status: &str, rebalance_state: &str) {
        *self
            .drain_status
            .lock()
            .expect("ops drain status should lock") = drain_status.into();
        *self
            .rebalance_state
            .lock()
            .expect("ops rebalance state should lock") = rebalance_state.into();
    }

    pub fn update_route_ownership(&self, mut device_routes: Vec<RouteOwnershipView>) {
        device_routes.sort_by(|left, right| {
            left.tenant_id
                .cmp(&right.tenant_id)
                .then_with(|| left.principal_id.cmp(&right.principal_id))
                .then_with(|| left.device_id.cmp(&right.device_id))
        });
        *self
            .device_routes
            .lock()
            .expect("ops device routes should lock") = device_routes;
    }

    pub fn update_runtime_dir_inspection(&self, inspection: RuntimeDirInspectionView) {
        *self
            .runtime_dir_inspection
            .lock()
            .expect("ops runtime-dir inspection should lock") = inspection;
    }

    pub fn health_view(&self) -> OpsHealthResponse {
        OpsHealthResponse {
            items: self.services.clone(),
        }
    }

    pub fn cluster_view(&self) -> ClusterView {
        let drain_status = self
            .drain_status
            .lock()
            .expect("ops drain status should lock")
            .clone();
        let rebalance_state = self
            .rebalance_state
            .lock()
            .expect("ops rebalance state should lock")
            .clone();
        let device_route_count = self
            .device_routes
            .lock()
            .expect("ops device routes should lock")
            .len();
        ClusterView {
            nodes: vec![ClusterNodeView {
                node_id: self.node_id.clone(),
                profile: self.profile.clone(),
                bind_addr: self.bind_addr.clone(),
                drain_status,
                rebalance_state,
                device_route_count,
                owned_scopes: self.owned_scopes.clone(),
                services: self.services.clone(),
            }],
        }
    }

    pub fn lag_view(&self) -> LagView {
        LagView {
            items: self.lag_items.clone(),
        }
    }

    pub fn runtime_dir_view(&self) -> RuntimeDirInspectionView {
        self.runtime_dir_inspection
            .lock()
            .expect("ops runtime-dir inspection should lock")
            .clone()
    }

    pub fn diagnostic_bundle(&self) -> DiagnosticBundle {
        let drain_status = self
            .drain_status
            .lock()
            .expect("ops drain status should lock")
            .clone();
        let rebalance_state = self
            .rebalance_state
            .lock()
            .expect("ops rebalance state should lock")
            .clone();
        let device_routes = self
            .device_routes
            .lock()
            .expect("ops device routes should lock")
            .clone();
        DiagnosticBundle {
            generated_at: utc_now_rfc3339_millis(),
            profile: self.profile.clone(),
            node_id: self.node_id.clone(),
            bind_addr: self.bind_addr.clone(),
            drain_status,
            rebalance_state,
            owned_scopes: self.owned_scopes.clone(),
            services: self.services.clone(),
            lag: self.lag_items.clone(),
            device_routes,
        }
    }
}

#[derive(Debug)]
pub struct OpsError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl From<AuthContextError> for OpsError {
    fn from(value: AuthContextError) -> Self {
        Self {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl OpsError {
    fn forbidden(required_permission: &'static str) -> Self {
        Self {
            status: axum::http::StatusCode::FORBIDDEN,
            code: "permission_denied",
            message: format!("missing required permission: {required_permission}"),
        }
    }
}

impl axum::response::IntoResponse for OpsError {
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
    build_app(Arc::new(OpsRuntime::default()))
}

pub fn build_public_app() -> Router {
    build_default_app().layer(middleware::from_fn(require_public_bearer_auth))
}

pub fn build_app(runtime: Arc<OpsRuntime>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/api/v1/ops/health", get(get_ops_health))
        .route("/api/v1/ops/cluster", get(get_cluster))
        .route("/api/v1/ops/lag", get(get_lag))
        .route("/api/v1/ops/runtime-dir", get(get_runtime_dir))
        .route("/api/v1/ops/diagnostics", get(get_diagnostics))
        .with_state(AppState { runtime })
}

async fn require_public_bearer_auth(request: Request<axum::body::Body>, next: Next) -> Response {
    match request.uri().path() {
        "/healthz" | "/readyz" => next.run(request).await,
        _ => match resolve_public_bearer_auth_context(request.headers()) {
            Ok(_) => next.run(request).await,
            Err(error) => OpsError::from(error).into_response(),
        },
    }
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "ops-service",
    })
}

async fn readyz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "ops-service",
    })
}

async fn get_ops_health(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<OpsHealthResponse>, OpsError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.health_view()))
}

async fn get_cluster(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ClusterView>, OpsError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.cluster_view()))
}

async fn get_lag(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<LagView>, OpsError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.lag_view()))
}

async fn get_runtime_dir(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<RuntimeDirInspectionView>, OpsError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.runtime_dir_view()))
}

async fn get_diagnostics(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<DiagnosticBundle>, OpsError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.diagnostic_bundle()))
}

fn ensure_ops_read_access(auth: &AuthContext) -> Result<(), OpsError> {
    if auth.has_permission("ops.read") {
        return Ok(());
    }

    Err(OpsError::forbidden("ops.read"))
}
