//! Sdkwork IM HTTP service bootstrap through `sdkwork-web-framework`.
//!
//! Upstream IM services that own `/im/v3/api/*` (and optional `/backend/v3/api/*`) routes
//! must mount the standard interceptor chain instead of the legacy `im-app-context`
//! middleware.

use std::sync::Arc;

use axum::Router;
use im_app_context::app_context_from_web_request;
use sdkwork_iam_web_adapter::{
    iam_database_resolver_from_env, IamAuthorizationPolicy, IamDatabaseWebRequestContextResolver,
};
use sdkwork_web_axum::{with_web_request_context, WebFrameworkLayer};
use sdkwork_web_core::{
    DomainContextInjector, HttpRouteManifest, WebEnvironment, WebRequestContext,
    WebRequestContextProfile,
};

#[derive(Clone, Default)]
struct ImAppContextInjector;

impl DomainContextInjector for ImAppContextInjector {
    fn inject(&self, request: &mut axum::extract::Request, context: &WebRequestContext) {
        request.extensions_mut().insert(context.clone());
        if let Some(app_context) = app_context_from_web_request(context) {
            request.extensions_mut().insert(app_context);
        }
    }
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
        "/im/v3/api/realtime/ws".to_owned(),
    ]
}

/// Profile for IM-owned open-api ingress (`/im/v3/api/*`) with default backend-api prefix.
pub fn im_service_context_profile() -> WebRequestContextProfile {
    WebRequestContextProfile {
        open_api_prefixes: vec!["/im/v3/api".to_owned()],
        public_path_prefixes: im_service_public_path_prefixes(),
        gateway_api_prefixes: Vec::new(),
        environment: WebEnvironment::Dev,
        ..WebRequestContextProfile::default()
    }
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
    let authorization_policy = Arc::new(IamAuthorizationPolicy::new(route_manifest));
    let layer = WebFrameworkLayer::new(resolver)
        .with_profile(im_service_context_profile())
        .with_route_manifest(route_manifest)
        .with_authorization_policy(authorization_policy)
        .with_domain_injector(Arc::new(ImAppContextInjector));
    with_web_request_context(router, layer)
}

/// Bootstrap from environment (split-deploy service processes with IAM database lookup).
pub async fn wrap_im_open_api_service_router_from_env(
    route_manifest: HttpRouteManifest,
    router: Router,
) -> Router {
    let resolver = iam_database_resolver_from_env().await;
    wrap_im_open_api_service_router_with_resolver(resolver, route_manifest, router)
}
