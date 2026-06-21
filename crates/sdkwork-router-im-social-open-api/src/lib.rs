//! IM social open-api routes (`/im/v3/api/social/*`).

pub const OPEN_API_PREFIX: &str = "/im/v3/api/social";

mod manifest;
mod routes;
mod web_bootstrap;

pub use manifest::{open_route_manifest, open_routes};
pub use routes::build_supplemental_app;
pub use web_bootstrap::wrap_router;

use axum::Router;
use social_service::PostgresAppState;

/// Postgres-backed social open-api router with standard web-framework wrapping.
pub fn build_supplemental_public_app(state: PostgresAppState) -> Router {
    web_bootstrap::wrap_router(routes::build_supplemental_app(state))
}
