use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, Request};
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use craw_chat_api_registry::HttpMethod;
use craw_chat_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};
use im_auth_context::{AuthContextError, resolve_auth_context, resolve_public_bearer_auth_context};
use im_domain_core::conversation::{ConversationInboxEntry, ConversationReadCursorView};
use serde::{Deserialize, Serialize};

use super::{
    ContactView, ConversationMemberDirectoryEntry, ConversationSummaryView,
    MessageInteractionSummaryView, ProjectionAccessError, RegisteredDeviceView,
    TimelineProjectionService, TimelineWindowView,
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
    limit: Option<usize>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct TimelineQuery {
    after_seq: Option<u64>,
    limit: Option<usize>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

type TimelineResponse = TimelineWindowView;

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

type DeviceSyncFeedResponse = super::DeviceSyncFeedWindowView;

#[derive(Debug)]
pub struct ProjectionApiError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl ProjectionApiError {
    fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code,
            message: message.into(),
        }
    }
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
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs))
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
        "/healthz" | "/readyz" | "/openapi.json" | "/docs" => next.run(request).await,
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

async fn openapi_json() -> Result<Json<serde_json::Value>, ProjectionApiError> {
    Ok(Json(build_projection_service_openapi_document().map_err(
        |message| ProjectionApiError::internal("openapi_export_failed", message),
    )?))
}

async fn docs() -> Html<String> {
    Html(render_docs_html(&projection_service_openapi_spec()))
}

fn build_projection_service_openapi_document() -> Result<serde_json::Value, String> {
    let routes = extract_routes_from_function(
        include_str!("http.rs"),
        "build_app",
        &[],
        &["/openapi.json", "/docs"],
    )?;

    Ok(build_openapi_document(
        &projection_service_openapi_spec(),
        &routes,
        projection_service_tag,
        projection_service_requires_bearer,
        projection_service_summary,
    ))
}

fn projection_service_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Craw Chat Projection Service API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the projection-service router for inbox, timeline, contacts, read cursor, sync-feed, and interaction summary queries.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn projection_service_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" | "/readyz" => "system".to_owned(),
        path if path.starts_with("/api/v1/devices/") => "devices".to_owned(),
        "/api/v1/contacts" => "contacts".to_owned(),
        "/api/v1/inbox" => "inbox".to_owned(),
        _ => "conversations".to_owned(),
    }
}

fn projection_service_requires_bearer(path: &str, _method: HttpMethod) -> bool {
    !matches!(path, "/healthz" | "/readyz")
}

fn projection_service_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check projection service health".to_owned(),
        ("/readyz", HttpMethod::Get) => "Check projection service readiness".to_owned(),
        _ => format!(
            "{} {}",
            projection_service_method_display(method),
            path.trim_matches('/').replace('/', " ")
        ),
    }
}

fn projection_service_method_display(method: HttpMethod) -> &'static str {
    match method {
        HttpMethod::Delete => "Delete",
        HttpMethod::Get => "Get",
        HttpMethod::Head => "Head",
        HttpMethod::Options => "Options",
        HttpMethod::Patch => "Patch",
        HttpMethod::Post => "Post",
        HttpMethod::Put => "Put",
    }
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
    Ok(Json(service.device_sync_feed_window_from_auth_context(
        &auth,
        device_id.as_str(),
        query.after_seq,
        query.limit,
    )?))
}

async fn get_timeline(
    Path(conversation_id): Path<String>,
    Query(query): Query<TimelineQuery>,
    headers: HeaderMap,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Result<Json<TimelineResponse>, ProjectionApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(service.timeline_window_from_auth_context(
        &auth,
        conversation_id.as_str(),
        query.after_seq,
        query.limit,
    )?))
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
        items: service.contacts_from_auth_context(&auth)?,
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
