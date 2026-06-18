use std::sync::Arc;

use axum::Router;
use axum::middleware;
use axum::routing::{get, post};
use im_app_context::inject_app_request_context_middleware;

use crate::block;
use crate::direct_chat;
use crate::external;
use crate::friendship::{self, AppState};
use crate::runtime::SocialRuntime;
use crate::shared_channel;

pub fn build_embedded_app(social_runtime: Arc<SocialRuntime>) -> Router {
    build_social_api_routes(AppState { social_runtime })
}

pub fn build_app(social_runtime: Arc<SocialRuntime>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .merge(build_embedded_app(social_runtime))
}

fn build_social_api_routes(state: AppState) -> Router {
    Router::new()
        .route(
            "/backend/v3/api/control/social/friend_requests",
            get(friendship::list_friend_requests).post(friendship::submit_friend_request),
        )
        .route(
            "/backend/v3/api/control/social/friend_requests/{request_id}",
            get(friendship::friend_request_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/friend_requests/{request_id}/accept",
            post(friendship::accept_friend_request),
        )
        .route(
            "/backend/v3/api/control/social/friend_requests/{request_id}/decline",
            post(friendship::decline_friend_request),
        )
        .route(
            "/backend/v3/api/control/social/friend_requests/{request_id}/cancel",
            post(friendship::cancel_friend_request),
        )
        .route(
            "/backend/v3/api/control/social/friendships",
            post(friendship::activate_friendship),
        )
        .route(
            "/backend/v3/api/control/social/friendships/{friendship_id}",
            get(friendship::friendship_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/friendships/{friendship_id}/remove",
            post(friendship::remove_friendship),
        )
        .route(
            "/backend/v3/api/control/social/user_blocks",
            post(block::block_user),
        )
        .route(
            "/backend/v3/api/control/social/user_blocks/{block_id}",
            get(block::user_block_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/direct_chats/bindings",
            post(direct_chat::bind_direct_chat),
        )
        .route(
            "/backend/v3/api/control/social/direct_chats/{direct_chat_id}",
            get(direct_chat::direct_chat_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/external_connections",
            post(external::establish_external_connection),
        )
        .route(
            "/backend/v3/api/control/social/external_connections/{connection_id}",
            get(external::external_connection_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/external_member_links",
            post(external::bind_external_member_link),
        )
        .route(
            "/backend/v3/api/control/social/external_member_links/{link_id}",
            get(external::external_member_link_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/shared_channel_policies",
            post(shared_channel::apply_shared_channel_policy),
        )
        .route(
            "/backend/v3/api/control/social/shared_channel_policies/{policy_id}",
            get(shared_channel::shared_channel_policy_snapshot),
        )
        .with_state(state)
}

async fn healthz() -> &'static str {
    "ok"
}

async fn readyz() -> &'static str {
    "ok"
}

pub fn build_public_app(social_runtime: Arc<SocialRuntime>) -> Router {
    build_app(social_runtime).layer(middleware::from_fn(inject_app_request_context_middleware))
}

pub fn build_public_app_with_postgres_extension(
    social_runtime: Arc<SocialRuntime>,
    postgres_state: Option<crate::postgres::PostgresAppState>,
) -> Router {
    let app = build_public_app(social_runtime);
    match postgres_state {
        Some(state) => app.merge(crate::postgres::build_supplemental_public_app(state)),
        None => app,
    }
}

/// Deprecated alias retained for migration scripts.
pub fn build_public_app_with_contact_extension(
    social_runtime: Arc<SocialRuntime>,
    postgres_state: Option<crate::postgres::PostgresAppState>,
) -> Router {
    build_public_app_with_postgres_extension(social_runtime, postgres_state)
}
