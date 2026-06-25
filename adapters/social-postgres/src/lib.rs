//! PostgreSQL adapter for social-service persistence.
//!
//! Provides database-backed storage for friend requests, friendships,
//! user blocks, direct chats, external connections, and shared channel policies.

pub mod config;
pub mod direct_chat_store;
pub mod external_store;
pub mod friend_request_store;
pub mod friendship_store;
pub mod organization_store;
pub mod shared_channel_store;
pub mod user_block_store;
pub mod user_profile_store;
pub mod user_settings_store;

pub use config::SocialPostgresConfig;

use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
pub use r2d2_postgres::postgres::NoTls;

/// Shared connection pool wrapper for all social stores.
#[derive(Clone)]
pub struct SocialPostgresPool {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl SocialPostgresPool {
    pub fn new(pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        Self { pool }
    }

    pub fn inner(&self) -> &Pool<PostgresConnectionManager<NoTls>> {
        &self.pool
    }
}

/// Run a blocking PostgreSQL operation, bridging to async if needed.
pub(crate) fn run_postgres_io<F, T>(f: F) -> Result<T, im_platform_contracts::ContractError>
where
    F: FnOnce() -> Result<T, im_platform_contracts::ContractError> + Send + 'static,
    T: Send + 'static,
{
    f()
}

/// Map a postgres error to ContractError, redacting the connection URL.
pub(crate) fn postgres_unavailable(
    operation: &str,
    error: postgres::Error,
) -> im_platform_contracts::ContractError {
    im_platform_contracts::ContractError::Unavailable(format!(
        "postgres {operation} failed: {error}"
    ))
}

/// Get a client from the pool.
pub(crate) fn postgres_pool_client(
    pool: &Pool<PostgresConnectionManager<NoTls>>,
    operation: &str,
) -> Result<
    r2d2::PooledConnection<PostgresConnectionManager<NoTls>>,
    im_platform_contracts::ContractError,
> {
    pool.get().map_err(|error| {
        im_platform_contracts::ContractError::Unavailable(format!(
            "postgres pool get for {operation} failed: {error}"
        ))
    })
}
