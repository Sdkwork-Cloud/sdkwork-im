//! Group API handlers.

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::http::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateGroupRequest {
    pub group_name: String,
    pub group_type: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub max_members: Option<i32>,
    pub settings_json: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GroupResponse {
    pub group_id: String,
    pub space_id: String,
    pub group_name: String,
    pub group_type: String,
    pub owner_user_id: String,
    pub conversation_id: Option<String>,
    pub max_members: i32,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGroupRequest {
    pub group_name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub announcement: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

pub async fn create_group(
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
    Json(_request): Json<CreateGroupRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok((StatusCode::CREATED, Json(serde_json::json!({"status": "created"}))))
}

pub async fn list_groups(
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
    Query(_query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(Vec::<GroupResponse>::new()))
}

pub async fn get_group(
    State(_state): State<AppState>,
    Path((_space_id, _group_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    Err::<(), StatusCode>(StatusCode::NOT_FOUND)
}

pub async fn update_group(
    State(_state): State<AppState>,
    Path((_space_id, _group_id)): Path<(String, String)>,
    Json(_request): Json<UpdateGroupRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_group(
    State(_state): State<AppState>,
    Path((_space_id, _group_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}
