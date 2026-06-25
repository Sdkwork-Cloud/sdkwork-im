//! Social Service HTTP helpers retained for in-process health probes only.

use std::sync::Arc;

use axum::Router;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;

use crate::runtime::SocialRuntime;
use crate::render_shared_channel_sync_prometheus_from_env;

pub fn build_app(_social_runtime: Arc<SocialRuntime>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz_probe))
        .route("/metrics", get(metrics))
}

async fn metrics() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
        render_shared_channel_sync_prometheus_from_env(),
    )
}

async fn healthz() -> &'static str {
    "ok"
}

pub async fn readyz_probe() -> impl IntoResponse {
    let status = sdkwork_im_service_readiness::im_service_readiness_status_label();
    let code = if status == "ok" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    (code, status)
}
