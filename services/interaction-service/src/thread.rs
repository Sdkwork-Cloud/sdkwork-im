//! Thread API handlers.

#![allow(dead_code)]

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::http::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateThreadRequest {
    pub root_message_id: String,
    pub title: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ThreadResponse {
    pub thread_id: String,
    pub conversation_id: String,
    pub root_message_id: String,
    pub title: Option<String>,
    pub reply_count: i32,
    pub last_reply_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
    pub message_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message_id: String,
    pub sender_id: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

pub async fn create_thread(
    State(_state): State<AppState>,
    Path(_conversation_id): Path<String>,
    Json(_request): Json<CreateThreadRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({"status": "created"})),
    ))
}

pub async fn list_threads(
    State(_state): State<AppState>,
    Path(_conversation_id): Path<String>,
    Query(_query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(Vec::<ThreadResponse>::new()))
}

pub async fn get_thread(
    State(_state): State<AppState>,
    Path((_conversation_id, _thread_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    Err::<(), StatusCode>(StatusCode::NOT_FOUND)
}

pub async fn send_thread_message(
    State(_state): State<AppState>,
    Path((_conversation_id, _thread_id)): Path<(String, String)>,
    Json(_request): Json<SendMessageRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({"status": "sent"})),
    ))
}

pub async fn list_thread_messages(
    State(_state): State<AppState>,
    Path((_conversation_id, _thread_id)): Path<(String, String)>,
    Query(_query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(Vec::<MessageResponse>::new()))
}
