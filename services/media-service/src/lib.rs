use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, Request};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use craw_chat_contract_core::ContractError;
use craw_chat_contract_message::{CommitJournal, CommitPosition};
use im_adapter_object_storage_s3::{
    ALIYUN_OBJECT_STORAGE_PLUGIN_ID, AWS_OBJECT_STORAGE_PLUGIN_ID, GOOGLE_OBJECT_STORAGE_PLUGIN_ID,
    MICROSOFT_OBJECT_STORAGE_PLUGIN_ID, S3CompatibleObjectStorageProvider,
    TENCENT_OBJECT_STORAGE_PLUGIN_ID, VOLCENGINE_OBJECT_STORAGE_PLUGIN_ID,
};
use im_auth_context::{
    AuthContext, AuthContextError, resolve_auth_context, resolve_public_bearer_auth_context,
};
use im_domain_core::media::{MediaAsset, MediaProcessingState, MediaResource};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::{
    EffectiveProviderBinding, ObjectStorageDownloadUrlRequest, ObjectStorageProvider,
    ObjectStoragePutRequest, ProviderDomain, ProviderHealthSnapshot, ProviderRegistry,
    StaticProviderRegistry,
};
use im_time::utc_now_rfc3339_millis;
use serde::{Deserialize, Serialize};

const DEFAULT_MEDIA_DOWNLOAD_URL_TTL_SECONDS: u32 = 3600;

#[derive(Clone)]
struct AppState {
    runtime: Arc<MediaRuntime>,
}

pub struct MediaRuntime {
    assets: Mutex<HashMap<String, MediaAsset>>,
    journal: Arc<dyn CommitJournal + Send + Sync>,
    provider_registry: Arc<dyn ProviderRegistry>,
    object_storage_providers: HashMap<String, Arc<dyn ObjectStorageProvider>>,
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

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadUrlQuery {
    pub expires_in_seconds: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaDownloadUrlResponse {
    pub media_asset_id: String,
    pub storage_provider: String,
    pub download_url: String,
    pub expires_in_seconds: u32,
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

    fn not_ready(media_asset_id: &str) -> Self {
        Self {
            status: axum::http::StatusCode::BAD_REQUEST,
            code: "media_asset_not_ready",
            message: format!("media asset not ready: {media_asset_id}"),
        }
    }

    fn provider_binding_missing(message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code: "media_provider_binding_missing",
            message: message.into(),
        }
    }

    fn object_storage_provider(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "media_provider_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                code: "media_provider_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                status: axum::http::StatusCode::NOT_IMPLEMENTED,
                code: "media_provider_unsupported",
                message,
            },
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
        let provider_registry = Arc::new(
            StaticProviderRegistry::platform_default().with_deployment_profile(
                ProviderDomain::ObjectStorage,
                VOLCENGINE_OBJECT_STORAGE_PLUGIN_ID,
            ),
        );
        let object_storage_providers = built_in_object_storage_providers();
        Self::with_journal_and_provider_registry(
            journal,
            provider_registry,
            object_storage_providers,
        )
    }

    pub fn with_journal_and_provider_registry<J, I>(
        journal: Arc<J>,
        provider_registry: Arc<dyn ProviderRegistry>,
        object_storage_providers: I,
    ) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
        I: IntoIterator<Item = (String, Arc<dyn ObjectStorageProvider>)>,
    {
        Self {
            assets: Mutex::new(HashMap::new()),
            journal,
            provider_registry,
            object_storage_providers: object_storage_providers.into_iter().collect(),
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
        let provider_plugin_id = self.selected_provider_plugin_id(auth.tenant_id.as_str(), None)?;
        let provider = self.object_storage_provider(provider_plugin_id.as_str())?;
        let mut assets = self.assets.lock().expect("media runtime should lock");
        let asset = assets
            .get_mut(media_scope_key(auth.tenant_id.as_str(), media_asset_id).as_str())
            .ok_or_else(|| MediaError::not_found(media_asset_id))?;
        if !is_asset_owner(asset, auth) {
            return Err(MediaError::not_found(media_asset_id));
        }

        let object_descriptor = provider
            .put_object(ObjectStoragePutRequest {
                bucket: request.bucket.clone(),
                object_key: request.object_key.clone(),
                content_length: asset.resource.size.unwrap_or_default(),
                content_type: asset.resource.mime_type.clone(),
                storage_class: None,
            })
            .map_err(MediaError::object_storage_provider)?;
        let signed_url = provider
            .signed_download_url(ObjectStorageDownloadUrlRequest {
                bucket: object_descriptor.bucket.clone(),
                object_key: object_descriptor.object_key.clone(),
                expires_in_seconds: DEFAULT_MEDIA_DOWNLOAD_URL_TTL_SECONDS,
            })
            .map_err(MediaError::object_storage_provider)?;

        if asset.processing_state == MediaProcessingState::Ready {
            if complete_upload_matches_existing(
                asset,
                object_descriptor.bucket.as_str(),
                object_descriptor.object_key.as_str(),
                provider_plugin_id.as_str(),
                request.checksum.as_ref(),
                signed_url.as_str(),
            ) {
                return Ok(asset.clone());
            }

            return Err(MediaError::conflict(media_asset_id));
        }

        let should_emit_event = asset.processing_state != MediaProcessingState::Ready;

        let completed_at = utc_now_rfc3339_millis();
        asset.bucket = Some(object_descriptor.bucket);
        asset.object_key = Some(object_descriptor.object_key);
        asset.storage_provider = Some(provider_plugin_id);
        asset.checksum = request.checksum;
        asset.processing_state = MediaProcessingState::Ready;
        asset.resource.url = Some(signed_url);
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

    pub fn download_url(
        &self,
        auth: &AuthContext,
        media_asset_id: &str,
        expires_in_seconds: u32,
    ) -> Result<MediaDownloadUrlResponse, MediaError> {
        let asset = self.get_asset(auth, media_asset_id)?;
        if asset.processing_state != MediaProcessingState::Ready {
            return Err(MediaError::not_ready(media_asset_id));
        }
        let bucket = asset
            .bucket
            .clone()
            .ok_or_else(|| MediaError::not_ready(media_asset_id))?;
        let object_key = asset
            .object_key
            .clone()
            .ok_or_else(|| MediaError::not_ready(media_asset_id))?;
        let provider_plugin_id = self.selected_provider_plugin_id(
            auth.tenant_id.as_str(),
            asset.storage_provider.as_deref(),
        )?;
        let provider = self.object_storage_provider(provider_plugin_id.as_str())?;
        let download_url = provider
            .signed_download_url(ObjectStorageDownloadUrlRequest {
                bucket,
                object_key,
                expires_in_seconds,
            })
            .map_err(MediaError::object_storage_provider)?;
        Ok(MediaDownloadUrlResponse {
            media_asset_id: asset.media_asset_id,
            storage_provider: provider_plugin_id,
            download_url,
            expires_in_seconds,
        })
    }

    pub fn provider_health_snapshot(
        &self,
        tenant_id: &str,
    ) -> Result<ProviderHealthSnapshot, MediaError> {
        let provider = self
            .object_storage_provider(self.selected_provider_plugin_id(tenant_id, None)?.as_str())?;
        Ok(provider.provider_health_snapshot())
    }

    pub fn provider_binding(
        &self,
        tenant_id: Option<&str>,
    ) -> Result<EffectiveProviderBinding, MediaError> {
        self.provider_registry
            .effective_binding(ProviderDomain::ObjectStorage, tenant_id)
            .ok_or_else(|| {
                MediaError::provider_binding_missing(
                    "object storage provider binding is missing for the current scope",
                )
            })
    }

    fn selected_provider_plugin_id(
        &self,
        tenant_id: &str,
        frozen_plugin_id: Option<&str>,
    ) -> Result<String, MediaError> {
        if let Some(plugin_id) = frozen_plugin_id.filter(|value| !value.trim().is_empty()) {
            return Ok(plugin_id.to_string());
        }

        let binding = self
            .provider_registry
            .effective_binding(ProviderDomain::ObjectStorage, Some(tenant_id))
            .ok_or_else(|| {
                MediaError::provider_binding_missing(
                    "object storage provider binding is missing for the current tenant",
                )
            })?;
        binding
            .selected_plugin_id
            .or(binding.default_plugin_id)
            .ok_or_else(|| {
                MediaError::provider_binding_missing(
                    "object storage provider selection is missing for the current tenant",
                )
            })
    }

    fn object_storage_provider(
        &self,
        plugin_id: &str,
    ) -> Result<Arc<dyn ObjectStorageProvider>, MediaError> {
        self.object_storage_providers
            .get(plugin_id)
            .cloned()
            .ok_or_else(|| {
                MediaError::provider_binding_missing(format!(
                    "object storage provider is not installed in runtime: {plugin_id}"
                ))
            })
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
        .route("/api/v1/media/provider-health", get(get_provider_health))
        .route(
            "/api/v1/media/{media_asset_id}/download-url",
            get(get_download_url),
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

async fn get_download_url(
    Path(media_asset_id): Path<String>,
    Query(query): Query<DownloadUrlQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<MediaDownloadUrlResponse>, MediaError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state.runtime.download_url(
            &auth,
            media_asset_id.as_str(),
            query
                .expires_in_seconds
                .unwrap_or(DEFAULT_MEDIA_DOWNLOAD_URL_TTL_SECONDS),
        )?,
    ))
}

async fn get_provider_health(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ProviderHealthSnapshot>, MediaError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state
            .runtime
            .provider_health_snapshot(auth.tenant_id.as_str())?,
    ))
}

fn built_in_object_storage_providers() -> Vec<(String, Arc<dyn ObjectStorageProvider>)> {
    vec![
        (
            ALIYUN_OBJECT_STORAGE_PLUGIN_ID.into(),
            Arc::new(S3CompatibleObjectStorageProvider::aliyun_default()),
        ),
        (
            TENCENT_OBJECT_STORAGE_PLUGIN_ID.into(),
            Arc::new(S3CompatibleObjectStorageProvider::tencent_default()),
        ),
        (
            VOLCENGINE_OBJECT_STORAGE_PLUGIN_ID.into(),
            Arc::new(S3CompatibleObjectStorageProvider::volcengine_default()),
        ),
        (
            AWS_OBJECT_STORAGE_PLUGIN_ID.into(),
            Arc::new(S3CompatibleObjectStorageProvider::aws_default()),
        ),
        (
            GOOGLE_OBJECT_STORAGE_PLUGIN_ID.into(),
            Arc::new(S3CompatibleObjectStorageProvider::google_default()),
        ),
        (
            MICROSOFT_OBJECT_STORAGE_PLUGIN_ID.into(),
            Arc::new(S3CompatibleObjectStorageProvider::microsoft_default()),
        ),
    ]
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
    bucket: &str,
    object_key: &str,
    storage_provider: &str,
    checksum: Option<&String>,
    download_url: &str,
) -> bool {
    asset.bucket.as_deref() == Some(bucket)
        && asset.object_key.as_deref() == Some(object_key)
        && asset.storage_provider.as_deref() == Some(storage_provider)
        && asset.checksum.as_ref() == checksum
        && asset.resource.url.as_deref() == Some(download_url)
}
