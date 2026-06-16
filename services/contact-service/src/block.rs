//! Block API handlers.

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::http::AppState;

#[derive(Debug, Deserialize)]
pub struct BlockUserRequest {
    pub blocked_user_id: String,
    pub scope: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BlockResponse {
    pub block_id: String,
    pub blocker_user_id: String,
    pub blocked_user_id: String,
    pub scope: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

pub async fn block_user(
    State(_state): State<AppState>,
    Json(_request): Json<BlockUserRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok((StatusCode::CREATED, Json(serde_json::json!({"status": "blocked"}))))
}

pub async fn list_blocks(
    State(_state): State<AppState>,
    Query(_query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok(Json(Vec::<BlockResponse>::new()))
}

pub async fn get_block(
    State(_state): State<AppState>,
    Path(_block_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Err::<(), StatusCode>(StatusCode::NOT_FOUND)
}

pub async fn unblock_user(
    State(_state): State<AppState>,
    Path(_block_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok(StatusCode::NO_CONTENT)
}
