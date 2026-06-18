//! Space API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use im_app_context::AppContext;
use serde::{Deserialize, Serialize};

use im_adapters_social_postgres::organization_store::{SpaceRecord, SpaceStore};

use crate::http::AppState;
use crate::service_http::require_request_scope;

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

impl From<SpaceRecord> for SpaceResponse {
    fn from(record: SpaceRecord) -> Self {
        Self {
            space_id: record.space_id.to_string(),
            space_name: record.space_name,
            space_type: record.space_type,
            owner_user_id: record.owner_user_id,
            description: record.description,
            avatar_url: record.avatar_url,
            max_members: record.max_members,
            created_at: record.created_at,
        }
    }
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

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{}", duration.as_millis())
}

pub async fn create_space(
    State(state): State<AppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Json(request): Json<CreateSpaceRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;

    let space_id = generate_id();
    let now = chrono::Utc::now().to_rfc3339();

    let record = SpaceRecord {
        tenant_id: scope.tenant_id,
        organization_id: scope.organization_id,
        space_id: space_id.parse().unwrap_or(0),
        space_name: request.space_name,
        space_type: request
            .space_type
            .unwrap_or_else(|| "organization".to_string()),
        owner_user_id: scope.user_id,
        description: request.description,
        avatar_url: request.avatar_url,
        max_members: request.max_members.unwrap_or(10000),
        settings_json: request.settings_json.unwrap_or_else(|| "{}".to_string()),
        created_at: now.clone(),
        updated_at: now,
    };

    match state.space_store.insert(&record) {
        Ok(()) => {
            let response = SpaceResponse::from(record);
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_spaces(
    State(state): State<AppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Query(query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let limit = query.limit.unwrap_or(20);

    match state.space_store.list_by_owner(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        scope.user_id.as_str(),
        limit,
    ) {
        Ok(records) => {
            let response: Vec<SpaceResponse> =
                records.into_iter().map(SpaceResponse::from).collect();
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_space(
    State(state): State<AppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Path(space_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let sid: i64 = space_id.parse().unwrap_or(0);

    match state.space_store.get_by_id(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        sid,
    ) {
        Ok(Some(record)) => Ok(Json(SpaceResponse::from(record))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_space(
    State(state): State<AppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Path(space_id): Path<String>,
    Json(request): Json<UpdateSpaceRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let sid: i64 = space_id.parse().unwrap_or(0);
    let now = chrono::Utc::now().to_rfc3339();

    match state.space_store.get_by_id(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        sid,
    ) {
        Ok(Some(mut record)) => {
            if let Some(name) = request.space_name {
                record.space_name = name;
            }
            if let Some(desc) = request.description {
                record.description = Some(desc);
            }
            if let Some(url) = request.avatar_url {
                record.avatar_url = Some(url);
            }
            if let Some(max) = request.max_members {
                record.max_members = max;
            }
            record.updated_at = now;

            match state.space_store.update(&record) {
                Ok(()) => Ok(StatusCode::NO_CONTENT),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_space(
    State(state): State<AppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Path(space_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let sid: i64 = space_id.parse().unwrap_or(0);

    match state.space_store.delete(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        sid,
    ) {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
