//! Block API handlers.

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use im_adapters_social_postgres::user_block_store::{UserBlockRecord, UserBlockStore};

use crate::http::AppState;

#[derive(Debug, Deserialize)]
pub struct BlockUserRequest {
    pub blocked_user_id: String,
    pub scope: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BlockResponse {
    pub block_id: String,
    pub blocker_user_id: String,
    pub blocked_user_id: String,
    pub scope: String,
    pub created_at: String,
}

impl From<UserBlockRecord> for BlockResponse {
    fn from(record: UserBlockRecord) -> Self {
        Self {
            block_id: record.block_id.to_string(),
            blocker_user_id: record.blocker_user_id,
            blocked_user_id: record.blocked_user_id,
            scope: record.scope,
            created_at: record.created_at,
        }
    }
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

pub async fn block_user(
    State(state): State<AppState>,
    Json(request): Json<BlockUserRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "default";
    let org_id = "default";
    let blocker_user_id = "system"; // TODO: Extract from auth context

    let block_id = generate_id();
    let now = chrono::Utc::now().to_rfc3339();

    let record = UserBlockRecord {
        tenant_id: tenant_id.to_string(),
        organization_id: org_id.to_string(),
        block_id: block_id.parse().unwrap_or(0),
        blocker_user_id: blocker_user_id.to_string(),
        blocked_user_id: request.blocked_user_id,
        scope: request.scope.unwrap_or_else(|| "all".to_string()),
        direct_chat_id: None,
        reason: request.reason,
        expires_at: None,
        created_at: now.clone(),
        updated_at: now,
    };

    match state.user_block_store.insert(&record) {
        Ok(()) => {
            let response = BlockResponse::from(record);
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_blocks(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "default";
    let org_id = "default";
    let user_id = "system"; // TODO: Extract from auth context
    let limit = query.limit.unwrap_or(20);

    match state
        .user_block_store
        .list_by_blocker(tenant_id, org_id, user_id, limit)
    {
        Ok(records) => {
            let response: Vec<BlockResponse> =
                records.into_iter().map(BlockResponse::from).collect();
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_block(
    State(state): State<AppState>,
    Path(block_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "default";
    let org_id = "default";
    let bid: i64 = block_id.parse().unwrap_or(0);

    match state.user_block_store.get_by_id(tenant_id, org_id, bid) {
        Ok(Some(record)) => Ok(Json(BlockResponse::from(record))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn unblock_user(
    State(_state): State<AppState>,
    Path(_block_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement unblock logic
    Ok(StatusCode::NO_CONTENT)
}
