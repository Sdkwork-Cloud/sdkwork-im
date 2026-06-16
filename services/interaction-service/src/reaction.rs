//! Reaction API handlers.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::http::AppState;

#[derive(Debug, Serialize)]
pub struct ReactionResponse {
    pub user_id: String,
    pub reaction_type: String,
    pub created_at: String,
}

pub async fn add_reaction(
    State(_state): State<AppState>,
    Path((_conversation_id, _message_id, _emoji)): Path<(String, String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}

pub async fn remove_reaction(
    State(_state): State<AppState>,
    Path((_conversation_id, _message_id, _emoji)): Path<(String, String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_reactions(
    State(_state): State<AppState>,
    Path((_conversation_id, _message_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(Vec::<ReactionResponse>::new()))
}
