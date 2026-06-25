//! Health-only router for the deprecated interaction-service workspace member.
//! Canonical client paths live under `/im/v3/api/chat/` in `sdkwork-im-im.openapi.yaml`.

use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;

pub fn build_app() -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
}

async fn healthz() -> &'static str {
    "ok"
}

async fn readyz() -> impl IntoResponse {
    let status = sdkwork_im_service_readiness::im_service_readiness_status_label();
    let code = if status == "ok" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    (code, status)
}

pub fn build_public_app() -> Router {
    build_app()
}
