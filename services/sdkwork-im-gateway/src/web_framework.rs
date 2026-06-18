use axum::Router;
use sdkwork_iam_web_adapter::IamDatabaseWebRequestContextResolver;
use sdkwork_web_axum::with_web_request_context;
use sdkwork_web_bootstrap::{
    SecurityPolicy, WebEnvironment, WebFramework, WebRequestContextProfile, service_router,
};

const IM_APP_API_PREFIX: &str = "/im/v3/api";
const IM_REALTIME_WEBSOCKET_PATH: &str = "/im/v3/api/realtime/ws";

/// Wrap the product gateway router with the canonical SDKWork HTTP interceptor pipeline.
///
/// Health, readiness, and metrics are owned by `sdkwork-web-bootstrap::service_router`.
pub fn wrap_gateway_router(router: Router) -> Router {
    let profile = WebRequestContextProfile {
        gateway_api_prefixes: vec![IM_APP_API_PREFIX.to_owned()],
        public_path_prefixes: vec![
            "/health".to_owned(),
            "/healthz".to_owned(),
            "/readyz".to_owned(),
            "/metrics".to_owned(),
            "/openapi.json".to_owned(),
            "/openapi/".to_owned(),
            "/docs".to_owned(),
            "/app/v3/api/auth/".to_owned(),
            "/app/v3/api/oauth/".to_owned(),
            // Realtime websocket upgrades authenticate through the first `auth.init` frame
            // (see docs/架构/20-WebSocket实时传输绑定标准.md), not HTTP Authorization.
            IM_REALTIME_WEBSOCKET_PATH.to_owned(),
            // Registry-owned websocket proxy routes (RouteProtocol::Websocket).
            "/ws/".to_owned(),
            "/admin/".to_owned(),
            "/api/".to_owned(),
            "/".to_owned(),
        ],
        environment: WebEnvironment::Dev,
        ..WebRequestContextProfile::default()
    };
    let mut security_policy = SecurityPolicy::default();
    security_policy.cors.allow_all_origins = true;
    security_policy
        .cross_site
        .reject_untrusted_state_changing_origins = false;
    security_policy.cross_site.reject_cookie_auth_without_origin = false;
    let framework = WebFramework::builder(IamDatabaseWebRequestContextResolver::new(None))
        .profile(profile)
        .security_policy(security_policy)
        .build();
    service_router(
        with_web_request_context(router, framework.layer().clone()),
        framework.service_router_config().with_always_ready(),
    )
}
