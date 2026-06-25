use axum::Router;

pub fn build_api_router() -> Router {
    session_gateway::build_domain_api_router(session_gateway::default_app_state())
}
