//! Sdkwork IM database pool bootstrap through `sdkwork-database`.
//!
//! R2D2-backed adapters may continue to use `sdkwork-database-config` directly until
//! they migrate to async sqlx pools. New async services should use this crate.

use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_sqlx::{DatabasePool, PoolError, create_pool_from_config};

const IM_DATABASE_SERVICE_NAME: &str = "IM";

/// Create the canonical IM sqlx pool from `SDKWORK_IM_DATABASE_*` environment variables.
pub async fn create_im_database_pool_from_env() -> Result<DatabasePool, PoolError> {
    let config = DatabaseConfig::from_env(IM_DATABASE_SERVICE_NAME)?;
    create_pool_from_config(config).await
}
