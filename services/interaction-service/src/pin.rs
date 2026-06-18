//! Pin API handlers.

#![allow(dead_code)]

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::http::AppState;

#[derive(Debug, Deserialize)]
pub struct PinMessageRequest {
    pub message_id: String,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PinResponse {
    pub message_id: String,
    pub pinned_by_user_id: String,
    pub reason: Option<String>,
    pub pinned_at: String,
}

pub async fn pin_message(
    State(_state): State<AppState>,
    Path(_conversation_id): Path<String>,
    Json(_request): Json<PinMessageRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::CREATED)
}

pub async fn list_pins(
    State(_state): State<AppState>,
    Path(_conversation_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(Vec::<PinResponse>::new()))
}

pub async fn unpin_message(
    State(_state): State<AppState>,
    Path((_conversation_id, _message_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}
