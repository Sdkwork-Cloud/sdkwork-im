//! Friendship API handlers.

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::http::AppState;

#[derive(Debug, Serialize)]
pub struct FriendshipResponse {
    pub friendship_id: String,
    pub user_low_id: String,
    pub user_high_id: String,
    pub status: String,
    pub established_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

pub async fn list_friends(
    State(_state): State<AppState>,
    Query(_query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok(Json(Vec::<FriendshipResponse>::new()))
}

pub async fn get_friendship(
    State(_state): State<AppState>,
    Path(_friendship_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Err::<(), StatusCode>(StatusCode::NOT_FOUND)
}

pub async fn remove_friendship(
    State(_state): State<AppState>,
    Path(_friendship_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok(StatusCode::NO_CONTENT)
}
