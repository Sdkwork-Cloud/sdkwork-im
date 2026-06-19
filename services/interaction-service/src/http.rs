//! Health-only router for the deprecated interaction-service workspace member.
//! Canonical client paths live under `/im/v3/api/chat/` in `sdkwork-im-im.openapi.yaml`.

use axum::Router;
use axum::middleware;
use axum::routing::get;
use im_app_context::inject_app_request_context_middleware;

pub fn build_app() -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
}

async fn healthz() -> &'static str {
    "ok"
}

async fn readyz() -> &'static str {
    "ok"
}

pub fn build_public_app() -> Router {
    build_app().layer(middleware::from_fn(inject_app_request_context_middleware))
}
