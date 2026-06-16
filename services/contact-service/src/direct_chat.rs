//! Direct chat API handlers.

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use im_adapters_social_postgres::direct_chat_store::{DirectChatRecord, DirectChatStore};

use crate::http::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateDirectChatRequest {
    pub target_user_id: String,
}

#[derive(Debug, Serialize)]
pub struct DirectChatResponse {
    pub direct_chat_id: String,
    pub left_actor_id: String,
    pub right_actor_id: String,
    pub status: String,
    pub conversation_id: Option<String>,
    pub created_at: String,
}

impl From<DirectChatRecord> for DirectChatResponse {
    fn from(record: DirectChatRecord) -> Self {
        Self {
            direct_chat_id: record.direct_chat_id.to_string(),
            left_actor_id: record.left_actor_id,
            right_actor_id: record.right_actor_id,
            status: record.status,
            conversation_id: record.conversation_id,
            created_at: record.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateDirectChatRequest {
    pub status: Option<String>,
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

fn normalize_pair_hash(left: &str, right: &str) -> String {
    use sha2::{Sha256, Digest};
    let (low, high) = if left < right { (left, right) } else { (right, left) };
    let mut hasher = Sha256::new();
    hasher.update(format!("{low}:{high}"));
    format!("{:x}", hasher.finalize())
}

pub async fn create_direct_chat(
    State(state): State<AppState>,
    Json(request): Json<CreateDirectChatRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "default";
    let org_id = "default";
    let user_id = "system"; // TODO: Extract from auth context

    let direct_chat_id = generate_id();
    let now = chrono::Utc::now().to_rfc3339();
    let pair_hash = normalize_pair_hash(user_id, &request.target_user_id);

    let record = DirectChatRecord {
        tenant_id: tenant_id.to_string(),
        organization_id: org_id.to_string(),
        direct_chat_id: direct_chat_id.parse().unwrap_or(0),
        left_actor_kind: "user".to_string(),
        left_actor_id: user_id.to_string(),
        right_actor_kind: "user".to_string(),
        right_actor_id: request.target_user_id,
        pair_hash,
        status: "active".to_string(),
        conversation_id: None,
        created_at: now.clone(),
        updated_at: now,
    };

    match state.direct_chat_store.insert(&record) {
        Ok(()) => {
            let response = DirectChatResponse::from(record);
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_direct_chats(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "default";
    let org_id = "default";
    let user_id = "system"; // TODO: Extract from auth context
    let limit = query.limit.unwrap_or(20);

    match state
        .direct_chat_store
        .list_by_actor(tenant_id, org_id, user_id, "active", limit)
    {
        Ok(records) => {
            let response: Vec<DirectChatResponse> =
                records.into_iter().map(DirectChatResponse::from).collect();
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_direct_chat(
    State(state): State<AppState>,
    Path(direct_chat_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "default";
    let org_id = "default";
    let dcid: i64 = direct_chat_id.parse().unwrap_or(0);

    match state.direct_chat_store.get_by_id(tenant_id, org_id, dcid) {
        Ok(Some(record)) => Ok(Json(DirectChatResponse::from(record))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_direct_chat(
    State(state): State<AppState>,
    Path(direct_chat_id): Path<String>,
    Json(request): Json<UpdateDirectChatRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "default";
    let org_id = "default";
    let dcid: i64 = direct_chat_id.parse().unwrap_or(0);
    let now = chrono::Utc::now().to_rfc3339();
    let status = request.status.as_deref().unwrap_or("active");

    match state
        .direct_chat_store
        .update_status(tenant_id, org_id, dcid, status, &now)
    {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
