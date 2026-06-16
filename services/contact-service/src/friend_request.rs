//! Friend request API handlers.

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use im_adapters_social_postgres::friend_request_store::{FriendRequestRecord, FriendRequestStore};

use crate::http::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateFriendRequestRequest {
    pub target_user_id: String,
    pub request_message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FriendRequestResponse {
    pub request_id: String,
    pub requester_user_id: String,
    pub target_user_id: String,
    pub status: String,
    pub request_message: Option<String>,
    pub created_at: String,
}

impl From<FriendRequestRecord> for FriendRequestResponse {
    fn from(record: FriendRequestRecord) -> Self {
        Self {
            request_id: record.request_id.to_string(),
            requester_user_id: record.requester_user_id,
            target_user_id: record.target_user_id,
            status: record.status,
            request_message: record.request_message,
            created_at: record.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub status: Option<String>,
    pub limit: Option<i64>,
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{}", duration.as_millis())
}

pub async fn create_friend_request(
    State(state): State<AppState>,
    Json(request): Json<CreateFriendRequestRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "default";
    let org_id = "default";
    let requester_user_id = "system"; // TODO: Extract from auth context

    let request_id = generate_id();
    let now = chrono::Utc::now().to_rfc3339();

    let record = FriendRequestRecord {
        tenant_id: tenant_id.to_string(),
        organization_id: org_id.to_string(),
        request_id: request_id.parse().unwrap_or(0),
        requester_user_id: requester_user_id.to_string(),
        target_user_id: request.target_user_id,
        request_message: request.request_message,
        status: "pending".to_string(),
        expired_at: None,
        created_at: now.clone(),
        updated_at: now,
    };

    match state.friend_request_store.insert(&record) {
        Ok(()) => {
            let response = FriendRequestResponse::from(record);
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_friend_requests(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "default";
    let org_id = "default";
    let user_id = "system"; // TODO: Extract from auth context
    let status = query.status.as_deref().unwrap_or("pending");
    let limit = query.limit.unwrap_or(20);

    match state
        .friend_request_store
        .list_by_target(tenant_id, org_id, user_id, status, limit)
    {
        Ok(records) => {
            let response: Vec<FriendRequestResponse> =
                records.into_iter().map(FriendRequestResponse::from).collect();
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_friend_request(
    State(state): State<AppState>,
    Path(request_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "default";
    let org_id = "default";
    let rid: i64 = request_id.parse().unwrap_or(0);

    match state.friend_request_store.get_by_id(tenant_id, org_id, rid) {
        Ok(Some(record)) => Ok(Json(FriendRequestResponse::from(record))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn accept_friend_request(
    State(state): State<AppState>,
    Path(request_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "default";
    let org_id = "default";
    let rid: i64 = request_id.parse().unwrap_or(0);
    let now = chrono::Utc::now().to_rfc3339();

    match state
        .friend_request_store
        .update_status(tenant_id, org_id, rid, "accepted", &now)
    {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn decline_friend_request(
    State(state): State<AppState>,
    Path(request_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "default";
    let org_id = "default";
    let rid: i64 = request_id.parse().unwrap_or(0);
    let now = chrono::Utc::now().to_rfc3339();

    match state
        .friend_request_store
        .update_status(tenant_id, org_id, rid, "declined", &now)
    {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn cancel_friend_request(
    State(state): State<AppState>,
    Path(request_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "default";
    let org_id = "default";
    let rid: i64 = request_id.parse().unwrap_or(0);
    let now = chrono::Utc::now().to_rfc3339();

    match state
        .friend_request_store
        .update_status(tenant_id, org_id, rid, "canceled", &now)
    {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
