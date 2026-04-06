use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::extract::{Path, State};
use axum::http::{HeaderMap, Request};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use im_auth_context::{
    AuthContext, AuthContextError, resolve_auth_context, resolve_public_bearer_auth_context,
};
use im_domain_core::media::{MediaAsset, MediaProcessingState, MediaResource};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::{CommitJournal, CommitPosition, ContractError};
use im_time::utc_now_rfc3339_millis;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct AppState {
    runtime: Arc<MediaRuntime>,
}

pub struct MediaRuntime {
    assets: Mutex<HashMap<String, MediaAsset>>,
    journal: Arc<dyn CommitJournal + Send + Sync>,
}

#[derive(Default)]
struct NoopJournal;

impl CommitJournal for NoopJournal {
    fn append(&self, _envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        Ok(CommitPosition::new("noop", 0))
    }
}

impl Default for MediaRuntime {
    fn default() -> Self {
        Self::with_journal(Arc::new(NoopJournal))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUploadRequest {
    pub media_asset_id: String,
    pub resource: MediaResource,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteUploadRequest {
    pub bucket: String,
    pub object_key: String,
    pub storage_provider: Option<String>,
    pub url: String,
    pub checksum: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Debug)]
pub struct MediaError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl MediaError {
    pub fn status(&self) -> axum::http::StatusCode {
        self.status
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    fn not_found(media_asset_id: &str) -> Self {
        Self {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "media_asset_not_found",
            message: format!("media asset not found: {media_asset_id}"),
        }
    }

    fn already_exists(media_asset_id: &str) -> Self {
        Self {
            status: axum::http::StatusCode::CONFLICT,
            code: "media_asset_already_exists",
            message: format!("media asset already exists: {media_asset_id}"),
        }
    }

    fn conflict(media_asset_id: &str) -> Self {
        Self {
            status: axum::http::StatusCode::CONFLICT,
            code: "media_asset_conflict",
            message: format!("media asset request conflicts with existing state: {media_asset_id}"),
        }
    }
}

impl From<ContractError> for MediaError {
    fn from(_value: ContractError) -> Self {
        Self {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code: "journal_unavailable",
            message: "commit journal unavailable".into(),
        }
    }
}

impl From<AuthContextError> for MediaError {
    fn from(value: AuthContextError) -> Self {
        Self {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl axum::response::IntoResponse for MediaError {
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

impl MediaRuntime {
    pub fn with_journal<J>(journal: Arc<J>) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
    {
        Self {
            assets: Mutex::new(HashMap::new()),
            journal,
        }
    }

    pub fn create_upload(
        &self,
        auth: &AuthContext,
        request: CreateUploadRequest,
    ) -> Result<MediaAsset, MediaError> {
        let scope = media_scope_key(auth.tenant_id.as_str(), request.media_asset_id.as_str());
        if let Some(existing) = self
            .assets
            .lock()
            .expect("media runtime should lock")
            .get(scope.as_str())
            .cloned()
        {
            if is_asset_owner(&existing, auth) {
                if create_upload_matches_existing(&existing, &request) {
                    return Ok(existing);
                }

                return Err(MediaError::conflict(request.media_asset_id.as_str()));
            }

            return Err(MediaError::already_exists(request.media_asset_id.as_str()));
        }

        let created_at = utc_now_rfc3339_millis();
        let asset = MediaAsset {
            tenant_id: auth.tenant_id.clone(),
            principal_id: auth.actor_id.clone(),
            principal_kind: auth.actor_kind.clone(),
            media_asset_id: request.media_asset_id.clone(),
            bucket: None,
            object_key: None,
            storage_provider: None,
            checksum: None,
            processing_state: MediaProcessingState::PendingUpload,
            resource: request.resource,
            created_at,
            completed_at: None,
        };

        self.assets
            .lock()
            .expect("media runtime should lock")
            .insert(scope, asset.clone());

        Ok(asset)
    }

    pub fn complete_upload(
        &self,
        auth: &AuthContext,
        media_asset_id: &str,
        request: CompleteUploadRequest,
    ) -> Result<MediaAsset, MediaError> {
        let normalized_storage_provider = request
            .storage_provider
            .clone()
            .unwrap_or_else(|| "local".into());
        let mut assets = self.assets.lock().expect("media runtime should lock");
        let asset = assets
            .get_mut(media_scope_key(auth.tenant_id.as_str(), media_asset_id).as_str())
            .ok_or_else(|| MediaError::not_found(media_asset_id))?;
        if !is_asset_owner(asset, auth) {
            return Err(MediaError::not_found(media_asset_id));
        }

        if asset.processing_state == MediaProcessingState::Ready {
            if complete_upload_matches_existing(
                asset,
                &request,
                normalized_storage_provider.as_str(),
            ) {
                return Ok(asset.clone());
            }

            return Err(MediaError::conflict(media_asset_id));
        }

        let should_emit_event = asset.processing_state != MediaProcessingState::Ready;

        let completed_at = utc_now_rfc3339_millis();
        asset.bucket = Some(request.bucket);
        asset.object_key = Some(request.object_key);
        asset.storage_provider = Some(normalized_storage_provider);
        asset.checksum = request.checksum;
        asset.processing_state = MediaProcessingState::Ready;
        asset.resource.url = Some(request.url);
        asset.completed_at = Some(completed_at);
        let completed_asset = asset.clone();
        drop(assets);

        if should_emit_event {
            self.append_media_asset_created(auth, &completed_asset)?;
        }

        Ok(completed_asset)
    }

    pub fn get_asset(
        &self,
        auth: &AuthContext,
        media_asset_id: &str,
    ) -> Result<MediaAsset, MediaError> {
        let asset = self
            .assets
            .lock()
            .expect("media runtime should lock")
            .get(media_scope_key(auth.tenant_id.as_str(), media_asset_id).as_str())
            .cloned()
            .ok_or_else(|| MediaError::not_found(media_asset_id))?;
        if !is_asset_owner(&asset, auth) {
            return Err(MediaError::not_found(media_asset_id));
        }
        Ok(asset)
    }

    fn append_media_asset_created(
        &self,
        auth: &AuthContext,
        asset: &MediaAsset,
    ) -> Result<(), MediaError> {
        let committed_at = asset
            .completed_at
            .clone()
            .unwrap_or_else(utc_now_rfc3339_millis);
        let envelope = CommitEnvelope {
            event_id: format!("evt_{}_created", asset.media_asset_id),
            tenant_id: auth.tenant_id.clone(),
            event_type: "media.asset.created".into(),
            event_version: 1,
            aggregate_type: AggregateType::MediaAsset,
            aggregate_id: asset.media_asset_id.clone(),
            scope_type: "media_asset".into(),
            scope_id: asset.media_asset_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                auth.tenant_id.as_str(),
                asset.media_asset_id.as_str(),
            ),
            ordering_seq: 1,
            causation_id: None,
            correlation_id: None,
            idempotency_key: None,
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: committed_at.clone(),
            committed_at,
            payload_schema: Some("media.asset.created.v1".into()),
            payload: serde_json::to_string(asset)
                .expect("media asset payload should serialize into commit envelope"),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };
        self.journal.append(envelope)?;
        Ok(())
    }
}

pub fn build_default_app() -> Router {
    build_app(Arc::new(MediaRuntime::default()))
}

pub fn build_public_app() -> Router {
    build_default_app().layer(middleware::from_fn(require_public_bearer_auth))
}

pub fn build_app(runtime: Arc<MediaRuntime>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/api/v1/media/uploads", post(create_upload))
        .route(
            "/api/v1/media/uploads/{media_asset_id}/complete",
            post(complete_upload),
        )
        .route("/api/v1/media/{media_asset_id}", get(get_media))
        .with_state(AppState { runtime })
}

async fn require_public_bearer_auth(request: Request<axum::body::Body>, next: Next) -> Response {
    match request.uri().path() {
        "/healthz" | "/readyz" => next.run(request).await,
        _ => match resolve_public_bearer_auth_context(request.headers()) {
            Ok(_) => next.run(request).await,
            Err(error) => MediaError::from(error).into_response(),
        },
    }
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "media-service",
    })
}

async fn readyz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "media-service",
    })
}

async fn create_upload(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateUploadRequest>,
) -> Result<Json<MediaAsset>, MediaError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.create_upload(&auth, request)?))
}

async fn complete_upload(
    Path(media_asset_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CompleteUploadRequest>,
) -> Result<Json<MediaAsset>, MediaError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.complete_upload(
        &auth,
        media_asset_id.as_str(),
        request,
    )?))
}

async fn get_media(
    Path(media_asset_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<MediaAsset>, MediaError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state.runtime.get_asset(&auth, media_asset_id.as_str())?,
    ))
}

fn media_scope_key(tenant_id: &str, media_asset_id: &str) -> String {
    format!("{tenant_id}:{media_asset_id}")
}

fn is_asset_owner(asset: &MediaAsset, auth: &AuthContext) -> bool {
    asset.principal_id == auth.actor_id && asset.principal_kind == auth.actor_kind
}

fn create_upload_matches_existing(asset: &MediaAsset, request: &CreateUploadRequest) -> bool {
    asset.resource == request.resource
}

fn complete_upload_matches_existing(
    asset: &MediaAsset,
    request: &CompleteUploadRequest,
    normalized_storage_provider: &str,
) -> bool {
    asset.bucket.as_deref() == Some(request.bucket.as_str())
        && asset.object_key.as_deref() == Some(request.object_key.as_str())
        && asset.storage_provider.as_deref() == Some(normalized_storage_provider)
        && asset.checksum == request.checksum
        && asset.resource.url.as_deref() == Some(request.url.as_str())
}
