//! User profile API handlers.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::http::AppState;

#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub user_id: String,
    pub im_nickname: Option<String>,
    pub im_avatar_url: Option<String>,
    pub im_status_message: Option<String>,
    pub im_online_status: String,
    pub last_active_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserProfileRequest {
    pub im_nickname: Option<String>,
    pub im_avatar_url: Option<String>,
    pub im_status_message: Option<String>,
}

pub async fn get_user_profile(
    State(_state): State<AppState>,
    Path(_user_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Err::<(), StatusCode>(StatusCode::NOT_FOUND)
}

pub async fn update_user_profile(
    State(_state): State<AppState>,
    Path(_user_id): Path<String>,
    Json(_request): Json<UpdateUserProfileRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok(StatusCode::NO_CONTENT)
}
