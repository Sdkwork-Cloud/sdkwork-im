//! Channel API handlers.

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::http::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateChannelRequest {
    pub channel_name: String,
    pub channel_type: Option<String>,
    pub description: Option<String>,
    pub position: Option<i32>,
    pub topic: Option<String>,
    pub settings_json: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChannelResponse {
    pub channel_id: String,
    pub space_id: String,
    pub channel_name: String,
    pub channel_type: String,
    pub description: Option<String>,
    pub conversation_id: Option<String>,
    pub position: i32,
    pub topic: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateChannelRequest {
    pub channel_name: Option<String>,
    pub description: Option<String>,
    pub position: Option<i32>,
    pub topic: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

pub async fn create_channel(
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
    Json(_request): Json<CreateChannelRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok((StatusCode::CREATED, Json(serde_json::json!({"status": "created"}))))
}

pub async fn list_channels(
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
    Query(_query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(Vec::<ChannelResponse>::new()))
}

pub async fn get_channel(
    State(_state): State<AppState>,
    Path((_space_id, _channel_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    Err::<(), StatusCode>(StatusCode::NOT_FOUND)
}

pub async fn update_channel(
    State(_state): State<AppState>,
    Path((_space_id, _channel_id)): Path<(String, String)>,
    Json(_request): Json<UpdateChannelRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_channel(
    State(_state): State<AppState>,
    Path((_space_id, _channel_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}
