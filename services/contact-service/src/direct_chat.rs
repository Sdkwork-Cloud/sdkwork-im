//! Direct chat API handlers.

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::http::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateDirectChatRequest {
    pub target_user_id: String,
}

#[derive(Debug, Serialize)]
pub struct DirectChatResponse {
    pub direct_chat_id: String,
    pub left_actor_id: String,
    pub right_actor_id: String,
    pub status: String,
    pub conversation_id: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDirectChatRequest {
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

pub async fn create_direct_chat(
    State(_state): State<AppState>,
    Json(_request): Json<CreateDirectChatRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok((StatusCode::CREATED, Json(serde_json::json!({"status": "created"}))))
}

pub async fn list_direct_chats(
    State(_state): State<AppState>,
    Query(_query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok(Json(Vec::<DirectChatResponse>::new()))
}

pub async fn get_direct_chat(
    State(_state): State<AppState>,
    Path(_direct_chat_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Err::<(), StatusCode>(StatusCode::NOT_FOUND)
}

pub async fn update_direct_chat(
    State(_state): State<AppState>,
    Path(_direct_chat_id): Path<String>,
    Json(_request): Json<UpdateDirectChatRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok(StatusCode::NO_CONTENT)
}
