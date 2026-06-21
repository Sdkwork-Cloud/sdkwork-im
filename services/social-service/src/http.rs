//! Social Service HTTP helpers retained for in-process health probes only.

use std::sync::Arc;

use axum::Router;
use axum::routing::get;

use crate::runtime::SocialRuntime;

pub fn build_app(_social_runtime: Arc<SocialRuntime>) -> Router {
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
