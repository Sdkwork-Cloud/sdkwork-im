//! Channel API handlers.

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use im_adapters_social_postgres::organization_store::{ChannelRecord, ChannelStore};

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

impl From<ChannelRecord> for ChannelResponse {
    fn from(record: ChannelRecord) -> Self {
        Self {
            channel_id: record.channel_id.to_string(),
            space_id: record.space_id.to_string(),
            channel_name: record.channel_name,
            channel_type: record.channel_type,
            description: record.description,
            conversation_id: record.conversation_id,
            position: record.position,
            topic: record.topic,
            created_at: record.created_at,
        }
    }
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

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{}", duration.as_millis())
}

pub async fn create_channel(
    State(state): State<AppState>,
    Path(space_id): Path<String>,
    Json(request): Json<CreateChannelRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "default";
    let org_id = "default";

    let channel_id = generate_id();
    let now = chrono::Utc::now().to_rfc3339();

    let record = ChannelRecord {
        tenant_id: tenant_id.to_string(),
        organization_id: org_id.to_string(),
        channel_id: channel_id.parse().unwrap_or(0),
        space_id: space_id.parse().unwrap_or(0),
        channel_name: request.channel_name,
        channel_type: request.channel_type.unwrap_or_else(|| "text".to_string()),
        description: request.description,
        conversation_id: None,
        position: request.position.unwrap_or(0),
        is_nsfw: false,
        is_pinned: false,
        topic: request.topic,
        settings_json: request.settings_json.unwrap_or_else(|| "{}".to_string()),
        created_at: now.clone(),
        updated_at: now,
    };

    match state.channel_store.insert(&record) {
        Ok(()) => {
            let response = ChannelResponse::from(record);
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_channels(
    State(state): State<AppState>,
    Path(space_id): Path<String>,
    Query(query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "default";
    let org_id = "default";
    let sid: i64 = space_id.parse().unwrap_or(0);
    let limit = query.limit.unwrap_or(20);

    match state.channel_store.list_by_space(tenant_id, org_id, sid, limit) {
        Ok(records) => {
            let response: Vec<ChannelResponse> = records.into_iter().map(ChannelResponse::from).collect();
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_channel(
    State(state): State<AppState>,
    Path((_space_id, channel_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "default";
    let org_id = "default";
    let cid: i64 = channel_id.parse().unwrap_or(0);

    match state.channel_store.get_by_id(tenant_id, org_id, cid) {
        Ok(Some(record)) => Ok(Json(ChannelResponse::from(record))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_channel(
    State(state): State<AppState>,
    Path((_space_id, channel_id)): Path<(String, String)>,
    Json(request): Json<UpdateChannelRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "default";
    let org_id = "default";
    let cid: i64 = channel_id.parse().unwrap_or(0);
    let now = chrono::Utc::now().to_rfc3339();

    match state.channel_store.get_by_id(tenant_id, org_id, cid) {
        Ok(Some(mut record)) => {
            if let Some(name) = request.channel_name {
                record.channel_name = name;
            }
            if let Some(desc) = request.description {
                record.description = Some(desc);
            }
            if let Some(pos) = request.position {
                record.position = pos;
            }
            if let Some(topic) = request.topic {
                record.topic = Some(topic);
            }
            record.updated_at = now;

            match state.channel_store.update(&record) {
                Ok(()) => Ok(StatusCode::NO_CONTENT),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_channel(
    State(state): State<AppState>,
    Path((_space_id, channel_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "default";
    let org_id = "default";
    let cid: i64 = channel_id.parse().unwrap_or(0);

    match state.channel_store.delete(tenant_id, org_id, cid) {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
