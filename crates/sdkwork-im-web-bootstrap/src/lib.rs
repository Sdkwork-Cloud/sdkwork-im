//! Sdkwork IM HTTP service bootstrap through `sdkwork-web-framework`.
//!
//! Upstream IM services that own `/im/v3/api/*` (and optional `/backend/v3/api/*`) routes
//! must mount the standard interceptor chain instead of the legacy `im-app-context`
//! middleware.

use std::sync::{Arc, OnceLock};

use axum::Router;
use im_app_context::{app_context_from_web_request, resolve_web_environment_from_process_env};
use sdkwork_iam_web_adapter::{
    iam_database_resolver_from_env, IamAuthorizationPolicy, IamDatabaseWebRequestContextResolver,
};
use sdkwork_im_realtime_api_paths::REALTIME_WS;
use sdkwork_web_axum::{with_web_request_context, WebFrameworkLayer};
use sdkwork_web_bootstrap::SecurityPolicy;
use sdkwork_web_core::{
    DomainContextInjector, EnforcePrincipalTenantIsolationPolicy, HttpMetricsDimensions,
    HttpMetricsRegistry, HttpRouteManifest, WebEnvironment, WebRequestContext,
    WebRequestContextProfile,
};

#[derive(Clone, Default)]
struct ImAppContextInjector;

static IM_HTTP_METRICS: OnceLock<Arc<HttpMetricsRegistry>> = OnceLock::new();

/// Shared HTTP metrics registry for IM service processes (`OBSERVABILITY_SPEC.md` §3).
pub fn im_service_http_metrics() -> Arc<HttpMetricsRegistry> {
    IM_HTTP_METRICS
        .get_or_init(|| {
            let environment = resolve_web_environment_from_process_env();
            let service = std::env::var("SDKWORK_IM_SERVICE_NAME")
                .or_else(|_| std::env::var("OTEL_SERVICE_NAME"))
                .ok()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "sdkwork-im-service".to_owned());
            let deployment_profile = std::env::var("SDKWORK_IM_DEPLOYMENT_PROFILE")
                .unwrap_or_else(|_| "standalone".to_owned());
            HttpMetricsRegistry::with_dimensions(
                HttpMetricsDimensions::from_profile_environment(environment)
                    .with_service(service)
                    .with_deployment_profile(deployment_profile)
                    .with_runtime_target("server"),
            )
        })
        .clone()
}

impl DomainContextInjector for ImAppContextInjector {
    fn inject(&self, request: &mut axum::extract::Request, context: &WebRequestContext) {
        request.extensions_mut().insert(context.clone());
        if let Some(app_context) = app_context_from_web_request(context) {
            request.extensions_mut().insert(app_context);
        }
    }
}

fn im_service_security_policy(environment: &WebEnvironment) -> SecurityPolicy {
    let mut security_policy = if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
        SecurityPolicy::default()
    } else {
        SecurityPolicy::production()
    };
    if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
        security_policy.cors.allow_all_origins = true;
        security_policy
            .cross_site
            .reject_untrusted_state_changing_origins = false;
        security_policy.cross_site.reject_cookie_auth_without_origin = false;
    }
    security_policy
}

/// Infra paths that stay anonymous across IM HTTP service processes.
pub fn im_service_public_path_prefixes() -> Vec<String> {
    vec![
        "/health".to_owned(),
        "/healthz".to_owned(),
        "/readyz".to_owned(),
        "/metrics".to_owned(),
        "/openapi.json".to_owned(),
        "/openapi/".to_owned(),
        "/docs".to_owned(),
        REALTIME_WS.to_owned(),
    ]
}

/// Profile for IM-owned open-api ingress (`/im/v3/api/*`) with default backend-api prefix.
pub fn im_service_context_profile() -> WebRequestContextProfile {
    WebRequestContextProfile {
        open_api_prefixes: vec!["/im/v3/api".to_owned()],
        public_path_prefixes: im_service_public_path_prefixes(),
        gateway_api_prefixes: Vec::new(),
        environment: resolve_web_environment_from_process_env(),
        ..WebRequestContextProfile::default()
    }
}

fn wrap_im_open_api_service_router_inner(
    resolver: IamDatabaseWebRequestContextResolver,
    route_manifest: HttpRouteManifest,
    router: Router,
) -> Router {
    let environment = resolve_web_environment_from_process_env();
    let security_policy = im_service_security_policy(&environment);
    let authorization_policy = Arc::new(IamAuthorizationPolicy::new(route_manifest.clone()));
    let tenant_isolation_policy = Arc::new(EnforcePrincipalTenantIsolationPolicy);
    let layer = WebFrameworkLayer::new(resolver)
        .with_profile(im_service_context_profile())
        .with_security_policy(security_policy)
        .with_route_manifest(route_manifest)
        .with_authorization_policy(authorization_policy)
        .with_tenant_isolation_policy(tenant_isolation_policy)
        .with_domain_injector(Arc::new(ImAppContextInjector))
        .with_metrics(im_service_http_metrics());
    with_web_request_context(router, layer)
}

/// Initialize structured logging and optional OTel export for IM HTTP service processes.
pub fn init_im_service_tracing_from_env() {
    sdkwork_web_bootstrap::init_tracing_from_env();
}

/// Wrap an IM HTTP service router with the canonical SDKWork interceptor pipeline.
pub fn wrap_im_open_api_service_router(router: Router) -> Router {
    wrap_im_open_api_service_router_with_resolver(
        IamDatabaseWebRequestContextResolver::new(None),
        HttpRouteManifest::new(&[]),
        router,
    )
}

/// Alias for IM HTTP service processes (open-api and backend-api prefixes).
pub fn wrap_im_service_router(router: Router) -> Router {
    wrap_im_open_api_service_router(router)
}

/// Wrap with an explicit resolver and route manifest (public routes from manifest + infra prefixes).
pub fn wrap_im_open_api_service_router_with_resolver(
    resolver: IamDatabaseWebRequestContextResolver,
    route_manifest: HttpRouteManifest,
    router: Router,
) -> Router {
    wrap_im_open_api_service_router_inner(resolver, route_manifest, router)
}

/// Bootstrap from environment (split-deploy service processes with IAM database lookup).
pub async fn wrap_im_open_api_service_router_from_env(
    route_manifest: HttpRouteManifest,
    router: Router,
) -> Router {
    let resolver = iam_database_resolver_from_env().await;
    wrap_im_open_api_service_router_inner(resolver, route_manifest, router)
}
