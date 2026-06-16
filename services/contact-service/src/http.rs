//! Contact Service HTTP routes.

use std::sync::Arc;

use axum::Router;
use axum::routing::{delete, get, patch, post};

use crate::block;
use crate::direct_chat;
use crate::friend_request;
use crate::friendship;
use crate::user_profile;
use crate::user_settings;

/// Shared state for contact service handlers.
#[derive(Clone)]
pub struct AppState {
    pub friend_request_store: Arc<dyn im_adapters_social_postgres::friend_request_store::FriendRequestStore>,
    pub friendship_store: Arc<dyn im_adapters_social_postgres::friendship_store::FriendshipStore>,
    pub user_block_store: Arc<dyn im_adapters_social_postgres::user_block_store::UserBlockStore>,
    pub direct_chat_store: Arc<dyn im_adapters_social_postgres::direct_chat_store::DirectChatStore>,
}

pub fn build_app(state: AppState) -> Router {

    Router::new()
        // Friend requests
        .route(
            "/api/v1/contacts/friend-requests",
            post(friend_request::create_friend_request)
                .get(friend_request::list_friend_requests),
        )
        .route(
            "/api/v1/contacts/friend-requests/{request_id}",
            get(friend_request::get_friend_request),
        )
        .route(
            "/api/v1/contacts/friend-requests/{request_id}/accept",
            post(friend_request::accept_friend_request),
        )
        .route(
            "/api/v1/contacts/friend-requests/{request_id}/decline",
            post(friend_request::decline_friend_request),
        )
        .route(
            "/api/v1/contacts/friend-requests/{request_id}/cancel",
            post(friend_request::cancel_friend_request),
        )
        // Friends
        .route(
            "/api/v1/contacts/friends",
            get(friendship::list_friends),
        )
        .route(
            "/api/v1/contacts/friends/{friendship_id}",
            get(friendship::get_friendship),
        )
        .route(
            "/api/v1/contacts/friends/{friendship_id}",
            delete(friendship::remove_friendship),
        )
        // Blocks
        .route(
            "/api/v1/contacts/blocks",
            post(block::block_user).get(block::list_blocks),
        )
        .route(
            "/api/v1/contacts/blocks/{block_id}",
            get(block::get_block),
        )
        .route(
            "/api/v1/contacts/blocks/{block_id}",
            delete(block::unblock_user),
        )
        // Direct chats
        .route(
            "/api/v1/contacts/direct-chats",
            post(direct_chat::create_direct_chat)
                .get(direct_chat::list_direct_chats),
        )
        .route(
            "/api/v1/contacts/direct-chats/{direct_chat_id}",
            get(direct_chat::get_direct_chat),
        )
        .route(
            "/api/v1/contacts/direct-chats/{direct_chat_id}",
            patch(direct_chat::update_direct_chat),
        )
        // User profiles
        .route(
            "/api/v1/contacts/users/{user_id}/profile",
            get(user_profile::get_user_profile),
        )
        .route(
            "/api/v1/contacts/users/{user_id}/profile",
            patch(user_profile::update_user_profile),
        )
        // User settings
        .route(
            "/api/v1/contacts/users/{user_id}/settings",
            get(user_settings::get_user_settings),
        )
        .route(
            "/api/v1/contacts/users/{user_id}/settings",
            patch(user_settings::update_user_settings),
        )
        .with_state(state)
}

pub fn build_public_app(state: AppState) -> Router {
    build_app(state)
}
