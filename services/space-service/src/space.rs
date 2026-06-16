//! Space API handlers.

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::http::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateSpaceRequest {
    pub space_name: String,
    pub space_type: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub max_members: Option<i32>,
    pub settings_json: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SpaceResponse {
    pub space_id: String,
    pub space_name: String,
    pub space_type: String,
    pub owner_user_id: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub max_members: i32,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSpaceRequest {
    pub space_name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub max_members: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

pub async fn create_space(
    State(_state): State<AppState>,
    Json(_request): Json<CreateSpaceRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok((StatusCode::CREATED, Json(serde_json::json!({"status": "created"}))))
}

pub async fn list_spaces(
    State(_state): State<AppState>,
    Query(_query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok(Json(Vec::<SpaceResponse>::new()))
}

pub async fn get_space(
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Err::<(), StatusCode>(StatusCode::NOT_FOUND)
}

pub async fn update_space(
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
    Json(_request): Json<UpdateSpaceRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_space(
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement
    Ok(StatusCode::NO_CONTENT)
}
