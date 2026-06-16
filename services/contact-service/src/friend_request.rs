//! Friend request API handlers.

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::http::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateFriendRequestRequest {
    pub target_user_id: String,
    pub request_message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FriendRequestResponse {
    pub request_id: String,
    pub requester_user_id: String,
    pub target_user_id: String,
    pub status: String,
    pub request_message: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub status: Option<String>,
    pub limit: Option<i64>,
}

pub async fn create_friend_request(
    State(_state): State<AppState>,
    Json(_request): Json<CreateFriendRequestRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok((StatusCode::CREATED, Json(serde_json::json!({"status": "created"}))))
}

pub async fn list_friend_requests(
    State(_state): State<AppState>,
    Query(_query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok(Json(Vec::<FriendRequestResponse>::new()))
}

pub async fn get_friend_request(
    State(_state): State<AppState>,
    Path(_request_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Err::<(), StatusCode>(StatusCode::NOT_FOUND)
}

pub async fn accept_friend_request(
    State(_state): State<AppState>,
    Path(_request_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok(StatusCode::NO_CONTENT)
}

pub async fn decline_friend_request(
    State(_state): State<AppState>,
    Path(_request_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok(StatusCode::NO_CONTENT)
}

pub async fn cancel_friend_request(
    State(_state): State<AppState>,
    Path(_request_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok(StatusCode::NO_CONTENT)
}
