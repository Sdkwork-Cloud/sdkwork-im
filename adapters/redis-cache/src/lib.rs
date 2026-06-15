//! Redis cache adapter for hot data caching.
//!
//! Provides caching for presence, unread counts, recent messages,
//! conversation lists, typing indicators, and session state.

pub mod config;
pub mod presence_cache;
pub mod unread_cache;
pub mod timeline_cache;
pub mod inbox_cache;
pub mod typing_cache;
pub mod session_cache;

pub use config::RedisCacheConfig;

use redis::aio::ConnectionManager;
use redis::RedisResult;

/// Shared Redis connection manager wrapper.
#[derive(Clone)]
pub struct RedisCachePool {
    manager: ConnectionManager,
}

impl RedisCachePool {
    pub async fn new(url: &str) -> RedisResult<Self> {
        let client = redis::Client::open(url)?;
        let manager = ConnectionManager::new(client).await?;
        Ok(Self { manager })
    }

    pub fn inner(&self) -> &ConnectionManager {
        &self.manager
    }
}

/// Map a Redis error to ContractError.
pub(crate) fn redis_unavailable(operation: &str, error: redis::RedisError) -> im_platform_contracts::ContractError {
    im_platform_contracts::ContractError::Unavailable(format!(
        "redis {operation} failed: {error}"
    ))
}
