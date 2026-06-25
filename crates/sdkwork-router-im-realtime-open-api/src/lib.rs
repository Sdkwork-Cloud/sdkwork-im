mod manifest;
mod paths;
mod routes;
mod web_bootstrap;

pub use manifest::{route_manifest, API_SURFACE};
pub use paths::PREFIX;

use axum::Router;

pub fn build_public_app() -> Router {
    web_bootstrap::wrap_router(session_gateway::apply_public_http_guardrails(
        routes::build_api_router(),
    ))
}

pub fn build_public_app_with_realtime_plane(
    assembly: session_gateway::RealtimePlaneAssembly,
    node_id: &str,
) -> Router {
    web_bootstrap::wrap_router(
        session_gateway::build_public_app_with_realtime_plane(assembly, node_id),
    )
}

pub fn build_public_app_with_realtime_bootstrap(
    bootstrap: &session_gateway::RealtimePlaneBootstrap,
) -> Router {
    web_bootstrap::wrap_router(session_gateway::build_public_app_with_realtime_bootstrap(
        bootstrap,
    ))
}
