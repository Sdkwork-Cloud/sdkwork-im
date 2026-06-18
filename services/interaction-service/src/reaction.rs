//! Reaction API handlers.

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;

use crate::http::AppState;

#[derive(Debug, Serialize)]
pub struct ReactionResponse {
    pub user_id: String,
    pub reaction_type: String,
    pub created_at: String,
}

pub async fn add_reaction(
    State(_state): State<AppState>,
    Path((conversation_id, message_id, emoji)): Path<(String, String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement database insert
    // For now, return success
    tracing::info!(
        "Adding reaction {} to message {} in conversation {}",
        emoji,
        message_id,
        conversation_id
    );
    Ok(StatusCode::NO_CONTENT)
}

pub async fn remove_reaction(
    State(_state): State<AppState>,
    Path((conversation_id, message_id, emoji)): Path<(String, String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement database delete
    tracing::info!(
        "Removing reaction {} from message {} in conversation {}",
        emoji,
        message_id,
        conversation_id
    );
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_reactions(
    State(_state): State<AppState>,
    Path((conversation_id, message_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement database query
    tracing::info!(
        "Listing reactions for message {} in conversation {}",
        message_id,
        conversation_id
    );
    Ok(Json(Vec::<ReactionResponse>::new()))
}
