//! Conversation settings API handlers.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::http::AppState;

#[derive(Debug, Serialize)]
pub struct ConversationSettingsResponse {
    pub conversation_id: String,
    pub is_muted: bool,
    pub mute_until: Option<String>,
    pub is_pinned: bool,
    pub is_archived: bool,
    pub is_blocked: bool,
    pub notification_level: String,
    pub custom_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateConversationSettingsRequest {
    pub is_muted: Option<bool>,
    pub mute_until: Option<String>,
    pub is_pinned: Option<bool>,
    pub is_archived: Option<bool>,
    pub is_blocked: Option<bool>,
    pub notification_level: Option<String>,
    pub custom_name: Option<String>,
}

pub async fn get_conversation_settings(
    State(_state): State<AppState>,
    Path(_conversation_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Get user_id from auth context
    Err::<(), StatusCode>(StatusCode::NOT_FOUND)
}

pub async fn update_conversation_settings(
    State(_state): State<AppState>,
    Path(_conversation_id): Path<String>,
    Json(_request): Json<UpdateConversationSettingsRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Get user_id from auth context
    Ok(StatusCode::NO_CONTENT)
}
