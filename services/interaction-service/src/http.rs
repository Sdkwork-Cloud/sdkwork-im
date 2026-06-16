//! HTTP routes for the deprecated interaction-service scaffold.
//! Canonical client paths live under `/im/v3/api/chat/` in `sdkwork-im-im.openapi.yaml`.

use axum::Router;
use axum::middleware;
use axum::routing::{delete, get, patch, post, put};
use im_app_context::inject_app_request_context_middleware;

use crate::conversation_settings;
use crate::pin;
use crate::reaction;
use crate::thread;

/// Shared state for interaction service handlers.
#[derive(Clone)]
pub struct AppState {
    // TODO: Add database stores
}

pub fn build_app() -> Router {
    let state = AppState {};

    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        // Reactions
        .route(
            "/im/v3/api/interactions/conversations/{conversation_id}/messages/{message_id}/reactions/{emoji}",
            put(reaction::add_reaction).delete(reaction::remove_reaction),
        )
        .route(
            "/im/v3/api/interactions/conversations/{conversation_id}/messages/{message_id}/reactions",
            get(reaction::list_reactions),
        )
        // Pins
        .route(
            "/im/v3/api/interactions/conversations/{conversation_id}/pins",
            post(pin::pin_message).get(pin::list_pins),
        )
        .route(
            "/im/v3/api/interactions/conversations/{conversation_id}/pins/{message_id}",
            delete(pin::unpin_message),
        )
        // Threads
        .route(
            "/im/v3/api/interactions/conversations/{conversation_id}/threads",
            post(thread::create_thread).get(thread::list_threads),
        )
        .route(
            "/im/v3/api/interactions/conversations/{conversation_id}/threads/{thread_id}",
            get(thread::get_thread),
        )
        .route(
            "/im/v3/api/interactions/conversations/{conversation_id}/threads/{thread_id}/messages",
            post(thread::send_thread_message)
                .get(thread::list_thread_messages),
        )
        // Conversation settings
        .route(
            "/im/v3/api/interactions/conversations/{conversation_id}/settings",
            get(conversation_settings::get_conversation_settings)
                .patch(conversation_settings::update_conversation_settings),
        )
        .with_state(state)
}

async fn healthz() -> &'static str {
    "ok"
}

async fn readyz() -> &'static str {
    "ok"
}

pub fn build_public_app() -> Router {
    build_app().layer(middleware::from_fn(inject_app_request_context_middleware))
}
