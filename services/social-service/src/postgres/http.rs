//! Postgres-backed supplemental social routes merged into comms-social-service.

use std::sync::Arc;

use axum::Router;
use axum::middleware;
use axum::routing::{delete, get, post};
use im_app_context::inject_app_request_context_middleware;

use super::block;
use super::direct_chat;
use super::friendship;
use super::user_profile;
use super::user_settings;

/// Shared state for Postgres supplemental social handlers.
#[derive(Clone)]
pub struct PostgresAppState {
    pub friend_request_store:
        Arc<dyn im_adapters_social_postgres::friend_request_store::FriendRequestStore>,
    pub friendship_store: Arc<dyn im_adapters_social_postgres::friendship_store::FriendshipStore>,
    pub user_block_store: Arc<dyn im_adapters_social_postgres::user_block_store::UserBlockStore>,
    pub direct_chat_store: Arc<dyn im_adapters_social_postgres::direct_chat_store::DirectChatStore>,
    pub presence_cache: Option<im_adapters_redis_cache::presence_cache::RedisPresenceCache>,
    pub session_cache: Option<im_adapters_redis_cache::session_cache::RedisSessionCache>,
}

pub fn build_supplemental_app(state: PostgresAppState) -> Router {
    Router::new()
        .route(
            "/im/v3/api/social/friendships",
            get(friendship::list_friends),
        )
        .route("/im/v3/api/social/user_blocks", get(block::list_blocks))
        .route(
            "/im/v3/api/social/user_blocks/{block_id}",
            delete(block::unblock_user),
        )
        .route(
            "/im/v3/api/social/direct_chats",
            post(direct_chat::create_direct_chat).get(direct_chat::list_direct_chats),
        )
        .route(
            "/im/v3/api/social/direct_chats/{direct_chat_id}",
            get(direct_chat::get_direct_chat).patch(direct_chat::update_direct_chat),
        )
        .route(
            "/im/v3/api/social/users/{user_id}/profile",
            get(user_profile::get_user_profile).patch(user_profile::update_user_profile),
        )
        .route(
            "/im/v3/api/social/users/{user_id}/settings",
            get(user_settings::get_user_settings).patch(user_settings::update_user_settings),
        )
        .with_state(state)
}

pub fn build_supplemental_public_app(state: PostgresAppState) -> Router {
    build_supplemental_app(state).layer(middleware::from_fn(inject_app_request_context_middleware))
}
