//! Organization API handlers for spaces, groups, and channels.

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::friendship::{AppState, SocialServiceError};

// ---------------------------------------------------------------------------
// Request/Response Types
// ---------------------------------------------------------------------------

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
pub struct CreateGroupRequest {
    pub group_name: String,
    pub space_id: Option<String>,
    pub group_type: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub max_members: Option<i32>,
    pub settings_json: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GroupResponse {
    pub group_id: String,
    pub space_id: Option<String>,
    pub group_name: String,
    pub group_type: String,
    pub owner_user_id: String,
    pub conversation_id: Option<String>,
    pub max_members: i32,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateChannelRequest {
    pub channel_name: String,
    pub space_id: String,
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
pub struct AddMemberRequest {
    pub user_id: String,
    pub role: Option<String>,
    pub nickname: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MemberResponse {
    pub user_id: String,
    pub role: String,
    pub nickname: Option<String>,
    pub joined_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{}", duration.as_millis())
}

// ---------------------------------------------------------------------------
// Space Handlers
// ---------------------------------------------------------------------------

pub async fn create_space(
    State(_state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<CreateSpaceRequest>,
) -> Result<impl IntoResponse, SocialServiceError> {
    let auth = crate::friendship::resolve_auth_from_headers(&headers)?;
    let tenant_id = auth.tenant_id.as_str();
    let org_id = auth.organization_id.as_str();
    let user_id = auth.user_id.as_str();

    let space_id = generate_id();
    let now = chrono::Utc::now().to_rfc3339();

    let _record = im_adapters_social_postgres::organization_store::SpaceRecord {
        tenant_id: tenant_id.to_string(),
        organization_id: org_id.to_string(),
        space_id: space_id.parse().unwrap_or(0),
        space_name: request.space_name,
        space_type: request.space_type.unwrap_or_else(|| "organization".to_string()),
        owner_user_id: user_id.to_string(),
        description: request.description,
        avatar_url: request.avatar_url,
        max_members: request.max_members.unwrap_or(10000),
        settings_json: request.settings_json.unwrap_or_else(|| "{}".to_string()),
        created_at: now.clone(),
        updated_at: now,
    };

    // TODO: Insert into database via store

    let response = SpaceResponse {
        space_id,
        space_name: _record.space_name,
        space_type: _record.space_type,
        owner_user_id: _record.owner_user_id,
        description: _record.description,
        avatar_url: _record.avatar_url,
        max_members: _record.max_members,
        created_at: _record.created_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_space(
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
) -> Result<impl IntoResponse, SocialServiceError> {
    // TODO: Fetch from database via store
    Err(SocialServiceError::not_found("space not found"))
}

pub async fn list_spaces(
    State(_state): State<AppState>,
    Query(_query): Query<ListQuery>,
) -> Result<impl IntoResponse, SocialServiceError> {
    // TODO: Fetch from database via store
    Ok(Json(Vec::<SpaceResponse>::new()))
}

pub async fn update_space(
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
    Json(_request): Json<CreateSpaceRequest>,
) -> Result<impl IntoResponse, SocialServiceError> {
    // TODO: Update in database via store
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_space(
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
) -> Result<impl IntoResponse, SocialServiceError> {
    // TODO: Delete from database via store
    Ok(StatusCode::NO_CONTENT)
}

// ---------------------------------------------------------------------------
// Group Handlers
// ---------------------------------------------------------------------------

pub async fn create_group(
    State(_state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<CreateGroupRequest>,
) -> Result<impl IntoResponse, SocialServiceError> {
    let auth = crate::friendship::resolve_auth_from_headers(&headers)?;
    let tenant_id = auth.tenant_id.as_str();
    let org_id = auth.organization_id.as_str();
    let user_id = auth.user_id.as_str();

    let group_id = generate_id();
    let now = chrono::Utc::now().to_rfc3339();

    let _record = im_adapters_social_postgres::organization_store::GroupRecord {
        tenant_id: tenant_id.to_string(),
        organization_id: org_id.to_string(),
        group_id: group_id.parse().unwrap_or(0),
        space_id: request.space_id.and_then(|s| s.parse().ok()),
        group_name: request.group_name,
        group_type: request.group_type.unwrap_or_else(|| "normal".to_string()),
        owner_user_id: user_id.to_string(),
        conversation_id: None,
        max_members: request.max_members.unwrap_or(500),
        description: request.description,
        avatar_url: request.avatar_url,
        announcement: None,
        settings_json: request.settings_json.unwrap_or_else(|| "{}".to_string()),
        created_at: now.clone(),
        updated_at: now,
    };

    // TODO: Insert into database via store

    let response = GroupResponse {
        group_id,
        space_id: _record.space_id.map(|s| s.to_string()),
        group_name: _record.group_name,
        group_type: _record.group_type,
        owner_user_id: _record.owner_user_id,
        conversation_id: _record.conversation_id,
        max_members: _record.max_members,
        description: _record.description,
        avatar_url: _record.avatar_url,
        created_at: _record.created_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_group(
    State(_state): State<AppState>,
    Path(_group_id): Path<String>,
) -> Result<impl IntoResponse, SocialServiceError> {
    // TODO: Fetch from database via store
    Err(SocialServiceError::not_found("group not found"))
}

pub async fn list_groups(
    State(_state): State<AppState>,
    Query(_query): Query<ListQuery>,
) -> Result<impl IntoResponse, SocialServiceError> {
    // TODO: Fetch from database via store
    Ok(Json(Vec::<GroupResponse>::new()))
}

pub async fn update_group(
    State(_state): State<AppState>,
    Path(_group_id): Path<String>,
    Json(_request): Json<CreateGroupRequest>,
) -> Result<impl IntoResponse, SocialServiceError> {
    // TODO: Update in database via store
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_group(
    State(_state): State<AppState>,
    Path(_group_id): Path<String>,
) -> Result<impl IntoResponse, SocialServiceError> {
    // TODO: Delete from database via store
    Ok(StatusCode::NO_CONTENT)
}

// ---------------------------------------------------------------------------
// Channel Handlers
// ---------------------------------------------------------------------------

pub async fn create_channel(
    State(_state): State<AppState>,
    Json(request): Json<CreateChannelRequest>,
) -> Result<impl IntoResponse, SocialServiceError> {
    let _tenant_id = "default";
    let _org_id = "default";

    let channel_id = generate_id();
    let now = chrono::Utc::now().to_rfc3339();

    let _record = im_adapters_social_postgres::organization_store::ChannelRecord {
        tenant_id: _tenant_id.to_string(),
        organization_id: _org_id.to_string(),
        channel_id: channel_id.parse().unwrap_or(0),
        space_id: request.space_id.parse().unwrap_or(0),
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

    // TODO: Insert into database via store

    let response = ChannelResponse {
        channel_id,
        space_id: _record.space_id.to_string(),
        channel_name: _record.channel_name,
        channel_type: _record.channel_type,
        description: _record.description,
        conversation_id: _record.conversation_id,
        position: _record.position,
        topic: _record.topic,
        created_at: _record.created_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_channel(
    State(_state): State<AppState>,
    Path(_channel_id): Path<String>,
) -> Result<impl IntoResponse, SocialServiceError> {
    // TODO: Fetch from database via store
    Err(SocialServiceError::not_found("channel not found"))
}

pub async fn list_channels(
    State(_state): State<AppState>,
    Query(_query): Query<ListQuery>,
) -> Result<impl IntoResponse, SocialServiceError> {
    // TODO: Fetch from database via store
    Ok(Json(Vec::<ChannelResponse>::new()))
}

pub async fn update_channel(
    State(_state): State<AppState>,
    Path(_channel_id): Path<String>,
    Json(_request): Json<CreateChannelRequest>,
) -> Result<impl IntoResponse, SocialServiceError> {
    // TODO: Update in database via store
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_channel(
    State(_state): State<AppState>,
    Path(_channel_id): Path<String>,
) -> Result<impl IntoResponse, SocialServiceError> {
    // TODO: Delete from database via store
    Ok(StatusCode::NO_CONTENT)
}

// ---------------------------------------------------------------------------
// Member Handlers
// ---------------------------------------------------------------------------

pub async fn add_space_member(
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
    Json(_request): Json<AddMemberRequest>,
) -> Result<impl IntoResponse, SocialServiceError> {
    // TODO: Add member to space
    Ok(StatusCode::CREATED)
}

pub async fn list_space_members(
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
    Query(_query): Query<ListQuery>,
) -> Result<impl IntoResponse, SocialServiceError> {
    // TODO: Fetch members from database
    Ok(Json(Vec::<MemberResponse>::new()))
}

pub async fn remove_space_member(
    State(_state): State<AppState>,
    Path((_space_id, _user_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, SocialServiceError> {
    // TODO: Remove member from space
    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_group_member(
    State(_state): State<AppState>,
    Path(_group_id): Path<String>,
    Json(_request): Json<AddMemberRequest>,
) -> Result<impl IntoResponse, SocialServiceError> {
    // TODO: Add member to group
    Ok(StatusCode::CREATED)
}

pub async fn list_group_members(
    State(_state): State<AppState>,
    Path(_group_id): Path<String>,
    Query(_query): Query<ListQuery>,
) -> Result<impl IntoResponse, SocialServiceError> {
    // TODO: Fetch members from database
    Ok(Json(Vec::<MemberResponse>::new()))
}

pub async fn remove_group_member(
    State(_state): State<AppState>,
    Path((_group_id, _user_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, SocialServiceError> {
    // TODO: Remove member from group
    Ok(StatusCode::NO_CONTENT)
}
