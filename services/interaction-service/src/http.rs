//! Interaction Service HTTP routes.

use axum::Router;
use axum::routing::{delete, get, patch, post, put};

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
        // Reactions
        .route(
            "/api/v1/interactions/conversations/{conversation_id}/messages/{message_id}/reactions/{emoji}",
            put(reaction::add_reaction).delete(reaction::remove_reaction),
        )
        .route(
            "/api/v1/interactions/conversations/{conversation_id}/messages/{message_id}/reactions",
            get(reaction::list_reactions),
        )
        // Pins
        .route(
            "/api/v1/interactions/conversations/{conversation_id}/pins",
            post(pin::pin_message).get(pin::list_pins),
        )
        .route(
            "/api/v1/interactions/conversations/{conversation_id}/pins/{message_id}",
            delete(pin::unpin_message),
        )
        // Threads
        .route(
            "/api/v1/interactions/conversations/{conversation_id}/threads",
            post(thread::create_thread).get(thread::list_threads),
        )
        .route(
            "/api/v1/interactions/conversations/{conversation_id}/threads/{thread_id}",
            get(thread::get_thread),
        )
        .route(
            "/api/v1/interactions/conversations/{conversation_id}/threads/{thread_id}/messages",
            post(thread::send_thread_message)
                .get(thread::list_thread_messages),
        )
        // Conversation settings
        .route(
            "/api/v1/interactions/conversations/{conversation_id}/settings",
            get(conversation_settings::get_conversation_settings)
                .patch(conversation_settings::update_conversation_settings),
        )
        .with_state(state)
}

pub fn build_public_app() -> Router {
    build_app()
}
