use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, Request};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use im_auth_context::{AuthContextError, resolve_auth_context, resolve_public_bearer_auth_context};
use im_domain_core::conversation::{
    ConversationInboxEntry, ConversationReadCursorView, DeviceSyncFeedEntry,
};
use serde::{Deserialize, Serialize};

use super::{
    ContactView, ConversationMemberDirectoryEntry, ConversationSummaryView,
    MessageInteractionSummaryView, ProjectionAccessError, RegisteredDeviceView,
    TimelineProjectionService, TimelineViewEntry,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterDeviceRequest {
    device_id: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct SyncFeedQuery {
    after_seq: Option<u64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TimelineResponse {
    items: Vec<TimelineViewEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct InboxResponse {
    items: Vec<ConversationInboxEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ContactsResponse {
    items: Vec<ContactView>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MemberDirectoryResponse {
    items: Vec<ConversationMemberDirectoryEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PinnedMessagesResponse {
    items: Vec<MessageInteractionSummaryView>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DeviceSyncFeedResponse {
    items: Vec<DeviceSyncFeedEntry>,
}

#[derive(Debug)]
pub struct ProjectionApiError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl From<AuthContextError> for ProjectionApiError {
    fn from(value: AuthContextError) -> Self {
        Self {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl From<ProjectionAccessError> for ProjectionApiError {
    fn from(value: ProjectionAccessError) -> Self {
        Self {
            status: value.status(),
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl IntoResponse for ProjectionApiError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status,
            Json(serde_json::json!({
                "code": self.code,
                "message": self.message
            })),
        )
            .into_response()
    }
}

pub fn build_default_app() -> Router {
    build_app(Arc::new(TimelineProjectionService::default()))
}

pub fn build_public_app() -> Router {
    build_default_app().layer(middleware::from_fn(require_public_bearer_auth))
}

pub fn build_public_app_with_service(service: Arc<TimelineProjectionService>) -> Router {
    build_app(service).layer(middleware::from_fn(require_public_bearer_auth))
}

pub fn build_app(service: Arc<TimelineProjectionService>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/api/v1/devices/register", post(register_device))
        .route(
            "/api/v1/devices/{device_id}/sync-feed",
            get(get_device_sync_feed),
        )
        .route("/api/v1/contacts", get(get_contacts))
        .route("/api/v1/inbox", get(get_inbox))
        .route(
            "/api/v1/conversations/{conversation_id}",
            get(get_conversation_summary),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/read-cursor",
            get(get_read_cursor),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/member-directory",
            get(get_member_directory),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/pins",
            get(get_pinned_messages),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/messages/{message_id}/interaction-summary",
            get(get_message_interaction_summary),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/messages",
            get(get_timeline),
        )
        .with_state(service)
}

async fn require_public_bearer_auth(request: Request<axum::body::Body>, next: Next) -> Response {
    match request.uri().path() {
        "/healthz" | "/readyz" => next.run(request).await,
        _ => match resolve_public_bearer_auth_context(request.headers()) {
            Ok(_) => next.run(request).await,
            Err(error) => ProjectionApiError::from(error).into_response(),
        },
    }
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "projection-service",
    })
}

async fn readyz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "projection-service",
    })
}

async fn register_device(
    headers: HeaderMap,
    State(service): State<Arc<TimelineProjectionService>>,
    Json(request): Json<RegisterDeviceRequest>,
) -> Result<Json<RegisteredDeviceView>, ProjectionApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(service.register_device_from_auth_context(
        &auth,
        request.device_id,
    )?))
}

async fn get_device_sync_feed(
    Path(device_id): Path<String>,
    Query(query): Query<SyncFeedQuery>,
    headers: HeaderMap,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Result<Json<DeviceSyncFeedResponse>, ProjectionApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(DeviceSyncFeedResponse {
        items: service.device_sync_feed_from_auth_context(
            &auth,
            device_id.as_str(),
            query.after_seq,
        )?,
    }))
}

async fn get_timeline(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Result<Json<TimelineResponse>, ProjectionApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(TimelineResponse {
        items: service.timeline_from_auth_context(&auth, conversation_id.as_str())?,
    }))
}

async fn get_inbox(
    headers: HeaderMap,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Result<Json<InboxResponse>, ProjectionApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(InboxResponse {
        items: service.inbox_from_auth_context(&auth),
    }))
}

async fn get_contacts(
    headers: HeaderMap,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Result<Json<ContactsResponse>, ProjectionApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(ContactsResponse {
        items: service.contacts_from_auth_context(&auth),
    }))
}

async fn get_conversation_summary(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Result<Json<ConversationSummaryView>, ProjectionApiError> {
    let auth = resolve_auth_context(&headers)?;
    let summary = service
        .conversation_summary_from_auth_context(&auth, conversation_id.as_str())?
        .ok_or_else(|| ProjectionApiError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "conversation_summary_not_found",
            message: format!("conversation summary not found: {conversation_id}"),
        })?;
    Ok(Json(summary))
}

async fn get_read_cursor(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Result<Json<ConversationReadCursorView>, ProjectionApiError> {
    let auth = resolve_auth_context(&headers)?;
    let cursor = service
        .read_cursor_from_auth_context(&auth, conversation_id.as_str())?
        .ok_or_else(|| ProjectionApiError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "conversation_read_cursor_not_found",
            message: format!("conversation read cursor not found: {conversation_id}"),
        })?;
    Ok(Json(cursor))
}

async fn get_member_directory(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Result<Json<MemberDirectoryResponse>, ProjectionApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(MemberDirectoryResponse {
        items: service.member_directory_from_auth_context(&auth, conversation_id.as_str())?,
    }))
}

async fn get_pinned_messages(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Result<Json<PinnedMessagesResponse>, ProjectionApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(PinnedMessagesResponse {
        items: service.pinned_messages_from_auth_context(&auth, conversation_id.as_str())?,
    }))
}

async fn get_message_interaction_summary(
    Path((conversation_id, message_id)): Path<(String, String)>,
    headers: HeaderMap,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Result<Json<MessageInteractionSummaryView>, ProjectionApiError> {
    let auth = resolve_auth_context(&headers)?;
    let summary = service
        .message_interaction_summary_from_auth_context(
            &auth,
            conversation_id.as_str(),
            message_id.as_str(),
        )?
        .ok_or_else(|| ProjectionApiError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "message_interaction_summary_not_found",
            message: format!(
                "message interaction summary not found: {conversation_id}/{message_id}"
            ),
        })?;
    Ok(Json(summary))
}
