mod manifest;
mod paths;
mod routes;
mod web_bootstrap;

pub use manifest::{route_manifest, API_SURFACE};
pub use paths::PREFIX;

use std::sync::Arc;

use axum::Router;
use projection_service::TimelineProjectionService;

pub fn build_public_app() -> Router {
    web_bootstrap::wrap_router(projection_service::http::apply_public_http_guardrails(
        routes::build_api_router(),
    ))
}

pub fn build_public_app_with_service(service: Arc<TimelineProjectionService>) -> Router {
    web_bootstrap::wrap_router(projection_service::http::apply_public_http_guardrails(
        routes::build_api_router_with_service(service),
    ))
}
