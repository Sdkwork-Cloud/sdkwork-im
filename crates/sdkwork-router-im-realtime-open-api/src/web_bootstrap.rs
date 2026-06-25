use axum::Router;
use sdkwork_iam_web_adapter::IamDatabaseWebRequestContextResolver;
use sdkwork_im_web_bootstrap::wrap_im_open_api_service_router_with_resolver;

use crate::manifest::route_manifest;

pub fn wrap_router(router: Router) -> Router {
    wrap_im_open_api_service_router_with_resolver(
        IamDatabaseWebRequestContextResolver::new(None),
        route_manifest(),
        router,
    )
}
