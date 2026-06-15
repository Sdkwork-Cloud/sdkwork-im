use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};

use crate::block;
use crate::direct_chat;
use crate::external;
use crate::friendship::{self, AppState};
use crate::runtime::SocialRuntime;
use crate::shared_channel;

pub fn build_app(social_runtime: Arc<SocialRuntime>) -> Router {
    let state = AppState { social_runtime };

    Router::new()
        .route(
            "/social/friend_requests",
            get(friendship::list_friend_requests).post(friendship::submit_friend_request),
        )
        .route(
            "/social/friend_requests/{request_id}",
            get(friendship::friend_request_snapshot),
        )
        .route(
            "/social/friend_requests/{request_id}/accept",
            post(friendship::accept_friend_request),
        )
        .route(
            "/social/friend_requests/{request_id}/decline",
            post(friendship::decline_friend_request),
        )
        .route(
            "/social/friend_requests/{request_id}/cancel",
            post(friendship::cancel_friend_request),
        )
        .route("/social/friendships", post(friendship::activate_friendship))
        .route(
            "/social/friendships/{friendship_id}",
            get(friendship::friendship_snapshot),
        )
        .route(
            "/social/friendships/{friendship_id}/remove",
            post(friendship::remove_friendship),
        )
        .route("/social/user_blocks", post(block::block_user))
        .route(
            "/social/user_blocks/{block_id}",
            get(block::user_block_snapshot),
        )
        .route(
            "/social/direct_chats/bindings",
            post(direct_chat::bind_direct_chat),
        )
        .route(
            "/social/direct_chats/{direct_chat_id}",
            get(direct_chat::direct_chat_snapshot),
        )
        .route(
            "/social/external_connections",
            post(external::establish_external_connection),
        )
        .route(
            "/social/external_connections/{connection_id}",
            get(external::external_connection_snapshot),
        )
        .route(
            "/social/external_member_links",
            post(external::bind_external_member_link),
        )
        .route(
            "/social/external_member_links/{link_id}",
            get(external::external_member_link_snapshot),
        )
        .route(
            "/social/shared_channel_policies",
            post(shared_channel::apply_shared_channel_policy),
        )
        .route(
            "/social/shared_channel_policies/{policy_id}",
            get(shared_channel::shared_channel_policy_snapshot),
        )
        .with_state(state)
}

pub fn build_public_app(social_runtime: Arc<SocialRuntime>) -> Router {
    build_app(social_runtime)
}
