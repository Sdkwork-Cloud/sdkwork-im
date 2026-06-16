//! User settings API handlers.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::http::AppState;

#[derive(Debug, Serialize)]
pub struct UserSettingsResponse {
    pub settings: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserSettingsRequest {
    pub settings: HashMap<String, serde_json::Value>,
}

pub async fn get_user_settings(
    State(_state): State<AppState>,
    Path(_user_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok(Json(UserSettingsResponse {
        settings: HashMap::new(),
    }))
}

pub async fn update_user_settings(
    State(_state): State<AppState>,
    Path(_user_id): Path<String>,
    Json(_request): Json<UpdateUserSettingsRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok(StatusCode::NO_CONTENT)
}
