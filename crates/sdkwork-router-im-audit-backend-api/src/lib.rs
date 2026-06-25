mod manifest;
mod paths;
mod routes;
mod web_bootstrap;

pub use manifest::{route_manifest, API_SURFACE};
pub use paths::PREFIX;

use axum::Router;

pub fn build_public_app() -> Router {
    web_bootstrap::wrap_router(audit_service::apply_public_http_guardrails(
        routes::build_api_router(),
    ))
}
