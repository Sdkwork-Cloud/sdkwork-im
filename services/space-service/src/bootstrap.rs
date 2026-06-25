//! Bootstrap helpers for space-service process startup.

use std::sync::Arc;

use axum::Router;
use im_adapters_social_postgres::organization_store::{
    PostgresChannelStore, PostgresGroupStore, PostgresSpaceStore,
};
use im_adapters_social_postgres::{SocialPostgresConfig, SocialPostgresPool};
use im_platform_contracts::IdGenerator;

use crate::http::{AppState, build_embedded_app, build_public_app};
use crate::id::build_runtime_id_generator;

pub const DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";

pub fn app_state_from_postgres_pool(pool: SocialPostgresPool) -> AppState {
    let pool_arc = Arc::new(pool.inner().clone());
    AppState {
        space_store: Arc::new(PostgresSpaceStore::new(pool_arc.clone())),
        group_store: Arc::new(PostgresGroupStore::new(pool_arc.clone())),
        channel_store: Arc::new(PostgresChannelStore::new(pool_arc)),
        id_generator: build_runtime_id_generator(),
    }
}

/// Builds the embedded space router when `SDKWORK_IM_DATABASE_URL` is configured.
pub fn try_build_embedded_app_from_database_url_env() -> Option<Router> {
    let database_url = std::env::var(DATABASE_URL_ENV).ok()?;
    let pool = SocialPostgresConfig::new(database_url)
        .connect_pool()
        .ok()?;
    Some(build_embedded_app(app_state_from_postgres_pool(pool)))
}

/// Builds the public space router when `SDKWORK_IM_DATABASE_URL` is configured.
pub fn try_build_public_app_from_database_url_env() -> Option<Router> {
    let database_url = std::env::var(DATABASE_URL_ENV).ok()?;
    let pool = SocialPostgresConfig::new(database_url)
        .connect_pool()
        .ok()?;
    Some(build_public_app(app_state_from_postgres_pool(pool)))
}
