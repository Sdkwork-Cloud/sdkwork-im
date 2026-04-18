use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex, MutexGuard};

use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, Request};
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use craw_chat_api_registry::HttpMethod;
use craw_chat_contract_core::ContractError;
use craw_chat_contract_message::{CommitJournal, CommitPosition};
use craw_chat_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};
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
    ObjectStoragePutRequest, ObjectStorageUploadSession as ProviderUploadSession,
    ObjectStorageUploadUrlRequest, ProviderDomain, ProviderHealthSnapshot, ProviderRegistry,
    StaticProviderRegistry,
};
use im_time::utc_now_rfc3339_millis;
use serde::{Deserialize, Serialize};

const DEFAULT_MEDIA_DOWNLOAD_URL_TTL_SECONDS: u32 = 3600;
const DEFAULT_MEDIA_UPLOAD_URL_TTL_SECONDS: u32 = 3600;
const MEDIA_UPLOAD_DELIVERY_PROOF_VERSION: &str = "media.upload.delivery-proof.v1";
const DEFAULT_MEDIA_UPLOAD_BUCKET: &str = "media-assets";
const MEDIA_MAX_ASSET_ID_BYTES: usize = 256;
const MEDIA_MAX_BUCKET_BYTES: usize = 256;
const MEDIA_MAX_OBJECT_KEY_BYTES: usize = 1024;
const MEDIA_MAX_STORAGE_PROVIDER_BYTES: usize = 128;
const MEDIA_MAX_URL_BYTES: usize = 2048;
const MEDIA_MAX_CHECKSUM_BYTES: usize = 256;
const MEDIA_MAX_RESOURCE_UUID_BYTES: usize = 256;
const MEDIA_MAX_RESOURCE_LOCAL_FILE_BYTES: usize = 1024;
const MEDIA_MAX_RESOURCE_INLINE_BYTES: usize = 256 * 1024;
const MEDIA_MAX_RESOURCE_BASE64_BYTES: usize = 256 * 1024;
const MEDIA_MAX_RESOURCE_MIME_TYPE_BYTES: usize = 128;
const MEDIA_MAX_RESOURCE_NAME_BYTES: usize = 256;
const MEDIA_MAX_RESOURCE_EXTENSION_BYTES: usize = 32;
const MEDIA_MAX_RESOURCE_PROMPT_BYTES: usize = 8 * 1024;
const MEDIA_MAX_RESOURCE_TAGS_BYTES: usize = 16 * 1024;
const MEDIA_MAX_RESOURCE_METADATA_BYTES: usize = 64 * 1024;

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
    pub bucket: Option<String>,
    pub object_key: Option<String>,
    pub expires_in_seconds: Option<u32>,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MediaUploadMutationOutcome {
    pub asset: MediaAsset,
    pub applied: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaUploadDeliveryStatus {
    Applied,
    Replayed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaUploadMutationResponse {
    #[serde(flatten)]
    pub asset: MediaAsset,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload: Option<MediaUploadSession>,
    pub request_key: String,
    pub delivery_status: MediaUploadDeliveryStatus,
    pub proof_version: String,
}

impl MediaUploadMutationResponse {
    pub fn from_outcome(
        outcome: MediaUploadMutationOutcome,
        request_key: String,
        upload: Option<MediaUploadSession>,
    ) -> Self {
        Self {
            asset: outcome.asset,
            upload,
            request_key,
            delivery_status: if outcome.applied {
                MediaUploadDeliveryStatus::Applied
            } else {
                MediaUploadDeliveryStatus::Replayed
            },
            proof_version: MEDIA_UPLOAD_DELIVERY_PROOF_VERSION.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaUploadSession {
    pub asset_id: String,
    pub storage_provider: String,
    pub bucket: String,
    pub object_key: String,
    pub method: String,
    pub url: String,
    pub headers: BTreeMap<String, String>,
    pub expires_at: String,
}

impl MediaUploadSession {
    fn from_provider_session(
        asset_id: String,
        storage_provider: String,
        bucket: String,
        object_key: String,
        session: ProviderUploadSession,
    ) -> Self {
        Self {
            asset_id,
            storage_provider,
            bucket,
            object_key,
            method: session.method,
            url: session.url,
            headers: session.headers,
            expires_at: session.expires_at,
        }
    }
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
    fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code,
            message: message.into(),
        }
    }

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

    fn invalid_expires_in_seconds(expires_in_seconds: u32) -> Self {
        Self {
            status: axum::http::StatusCode::BAD_REQUEST,
            code: "invalid_expires_in_seconds",
            message: format!("expiresInSeconds must be greater than zero: {expires_in_seconds}"),
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

    fn payload_too_large(field: &'static str, max_bytes: usize, actual_bytes: usize) -> Self {
        Self {
            status: axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
            ),
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
        Ok(self.create_upload_with_outcome(auth, request)?.asset)
    }

    pub fn create_upload_with_outcome(
        &self,
        auth: &AuthContext,
        request: CreateUploadRequest,
    ) -> Result<MediaUploadMutationOutcome, MediaError> {
        validate_create_upload_request_payload_size(&request)?;
        let provider_plugin_id = self.selected_provider_plugin_id(auth.tenant_id.as_str(), None)?;
        let bucket = request
            .bucket
            .clone()
            .unwrap_or_else(default_media_upload_bucket);
        let object_key = resolve_media_object_key(auth.tenant_id.as_str(), &request);
        let scope = media_scope_key(auth.tenant_id.as_str(), request.media_asset_id.as_str());
        if let Some(existing) = self
            .lock_assets("create_upload")
            .get(scope.as_str())
            .cloned()
        {
            if is_asset_owner(&existing, auth) {
                if create_upload_matches_existing(
                    &existing,
                    &request,
                    bucket.as_str(),
                    object_key.as_str(),
                    provider_plugin_id.as_str(),
                ) {
                    return Ok(MediaUploadMutationOutcome {
                        asset: existing,
                        applied: false,
                    });
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
            bucket: Some(bucket),
            object_key: Some(object_key),
            storage_provider: Some(provider_plugin_id),
            checksum: None,
            processing_state: MediaProcessingState::PendingUpload,
            resource: request.resource,
            created_at,
            completed_at: None,
        };

        self.lock_assets("create_upload")
            .insert(scope, asset.clone());

        Ok(MediaUploadMutationOutcome {
            asset,
            applied: true,
        })
    }

    pub fn prepare_upload_session(
        &self,
        auth: &AuthContext,
        asset: &MediaAsset,
        expires_in_seconds: Option<u32>,
    ) -> Result<MediaUploadSession, MediaError> {
        let storage_provider = self.selected_provider_plugin_id(
            auth.tenant_id.as_str(),
            asset.storage_provider.as_deref(),
        )?;
        let provider = self.object_storage_provider(storage_provider.as_str())?;
        let bucket = asset
            .bucket
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(default_media_upload_bucket);
        let object_key = asset
            .object_key
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| default_media_upload_object_key(auth.tenant_id.as_str(), asset));
        let resolved_expires_in_seconds =
            expires_in_seconds.unwrap_or(DEFAULT_MEDIA_UPLOAD_URL_TTL_SECONDS);
        validate_download_url_expires_in_seconds(resolved_expires_in_seconds)?;
        let session = provider
            .signed_upload_url(ObjectStorageUploadUrlRequest {
                bucket: bucket.clone(),
                object_key: object_key.clone(),
                expires_in_seconds: resolved_expires_in_seconds,
                content_type: asset.resource.mime_type.clone(),
                content_length: asset.resource.size,
            })
            .map_err(MediaError::object_storage_provider)?;

        Ok(MediaUploadSession::from_provider_session(
            asset.media_asset_id.clone(),
            storage_provider,
            bucket,
            object_key,
            session,
        ))
    }

    pub fn complete_upload(
        &self,
        auth: &AuthContext,
        media_asset_id: &str,
        request: CompleteUploadRequest,
    ) -> Result<MediaAsset, MediaError> {
        Ok(self
            .complete_upload_with_outcome(auth, media_asset_id, request)?
            .asset)
    }

    pub fn complete_upload_with_outcome(
        &self,
        auth: &AuthContext,
        media_asset_id: &str,
        request: CompleteUploadRequest,
    ) -> Result<MediaUploadMutationOutcome, MediaError> {
        validate_media_asset_id(media_asset_id)?;
        validate_complete_upload_request_payload_size(&request)?;
        let mut assets = self.lock_assets("complete_upload");
        let asset = assets
            .get_mut(media_scope_key(auth.tenant_id.as_str(), media_asset_id).as_str())
            .ok_or_else(|| MediaError::not_found(media_asset_id))?;
        if !is_asset_owner(asset, auth) {
            return Err(MediaError::not_found(media_asset_id));
        }
        let provider_plugin_id = self.selected_provider_plugin_id(
            auth.tenant_id.as_str(),
            asset.storage_provider.as_deref(),
        )?;
        let provider = self.object_storage_provider(provider_plugin_id.as_str())?;

        if asset.processing_state == MediaProcessingState::Ready {
            if complete_upload_request_matches_existing(
                asset,
                request.bucket.as_str(),
                request.object_key.as_str(),
                provider_plugin_id.as_str(),
                request.checksum.as_ref(),
            ) {
                return Ok(MediaUploadMutationOutcome {
                    asset: asset.clone(),
                    applied: false,
                });
            }

            return Err(MediaError::conflict(media_asset_id));
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

        Ok(MediaUploadMutationOutcome {
            asset: completed_asset,
            applied: true,
        })
    }

    pub fn get_asset(
        &self,
        auth: &AuthContext,
        media_asset_id: &str,
    ) -> Result<MediaAsset, MediaError> {
        validate_media_asset_id(media_asset_id)?;
        let asset = self
            .lock_assets("get_asset")
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
        validate_media_asset_id(media_asset_id)?;
        validate_download_url_expires_in_seconds(expires_in_seconds)?;
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

    fn lock_assets(&self, operation: &'static str) -> MutexGuard<'_, HashMap<String, MediaAsset>> {
        match self.assets.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!(
                    "warning: recovering poisoned media-service assets lock during {operation}"
                );
                poisoned.into_inner()
            }
        }
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
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs))
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
        "/healthz" | "/readyz" | "/openapi.json" | "/docs" => next.run(request).await,
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

async fn openapi_json() -> Result<Json<serde_json::Value>, MediaError> {
    Ok(Json(
        build_media_service_openapi_document()
            .map_err(|message| MediaError::internal("openapi_export_failed", message))?,
    ))
}

async fn docs() -> Html<String> {
    Html(render_docs_html(&media_service_openapi_spec()))
}

fn build_media_service_openapi_document() -> Result<serde_json::Value, String> {
    let routes = extract_routes_from_function(
        include_str!("lib.rs"),
        "build_app",
        &[],
        &["/openapi.json", "/docs"],
    )?;

    Ok(build_openapi_document(
        &media_service_openapi_spec(),
        &routes,
        media_service_tag,
        media_service_requires_bearer,
        media_service_summary,
    ))
}

fn media_service_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Craw Chat Media Service API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the media-service router for upload creation, upload completion, asset lookup, download URL issue, and provider health flows.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn media_service_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" | "/readyz" => "system".to_owned(),
        path if path.contains("provider-health") => "providers".to_owned(),
        _ => "media".to_owned(),
    }
}

fn media_service_requires_bearer(path: &str, _method: HttpMethod) -> bool {
    !matches!(path, "/healthz" | "/readyz")
}

fn media_service_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check media service health".to_owned(),
        ("/readyz", HttpMethod::Get) => "Check media service readiness".to_owned(),
        _ => format!(
            "{} {}",
            media_service_method_display(method),
            path.trim_matches('/').replace('/', " ")
        ),
    }
}

fn media_service_method_display(method: HttpMethod) -> &'static str {
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

async fn create_upload(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateUploadRequest>,
) -> Result<Json<MediaUploadMutationResponse>, MediaError> {
    let auth = resolve_auth_context(&headers)?;
    let expires_in_seconds = request.expires_in_seconds;
    let request_key = media_create_upload_request_key(&auth, request.media_asset_id.as_str());
    let outcome = state.runtime.create_upload_with_outcome(&auth, request)?;
    let upload = state
        .runtime
        .prepare_upload_session(&auth, &outcome.asset, expires_in_seconds)?;
    Ok(Json(MediaUploadMutationResponse::from_outcome(
        outcome,
        request_key,
        Some(upload),
    )))
}

async fn complete_upload(
    Path(media_asset_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CompleteUploadRequest>,
) -> Result<Json<MediaUploadMutationResponse>, MediaError> {
    let auth = resolve_auth_context(&headers)?;
    validate_media_asset_id(media_asset_id.as_str())?;
    let request_key = media_complete_upload_request_key(&auth, media_asset_id.as_str());
    Ok(Json(MediaUploadMutationResponse::from_outcome(
        state
            .runtime
            .complete_upload_with_outcome(&auth, media_asset_id.as_str(), request)?,
        request_key,
        None,
    )))
}

async fn get_media(
    Path(media_asset_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<MediaAsset>, MediaError> {
    let auth = resolve_auth_context(&headers)?;
    validate_media_asset_id(media_asset_id.as_str())?;
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
    validate_media_asset_id(media_asset_id.as_str())?;
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

fn validate_payload_size(
    field: &'static str,
    payload: &str,
    max_bytes: usize,
) -> Result<(), MediaError> {
    let payload_len = payload.len();
    if payload_len > max_bytes {
        return Err(MediaError::payload_too_large(field, max_bytes, payload_len));
    }
    Ok(())
}

fn validate_optional_payload_size(
    field: &'static str,
    payload: Option<&str>,
    max_bytes: usize,
) -> Result<(), MediaError> {
    if let Some(payload) = payload {
        validate_payload_size(field, payload, max_bytes)?;
    }
    Ok(())
}

fn validate_map_payload_size(
    field: &'static str,
    payload: Option<&std::collections::BTreeMap<String, String>>,
    max_bytes: usize,
) -> Result<(), MediaError> {
    let payload_len = payload
        .into_iter()
        .flat_map(|entries| entries.iter())
        .map(|(key, value)| key.len() + value.len())
        .sum::<usize>();
    if payload_len > max_bytes {
        return Err(MediaError::payload_too_large(field, max_bytes, payload_len));
    }
    Ok(())
}

fn validate_media_asset_id(media_asset_id: &str) -> Result<(), MediaError> {
    validate_payload_size("mediaAssetId", media_asset_id, MEDIA_MAX_ASSET_ID_BYTES)
}

fn validate_download_url_expires_in_seconds(expires_in_seconds: u32) -> Result<(), MediaError> {
    if expires_in_seconds == 0 {
        return Err(MediaError::invalid_expires_in_seconds(expires_in_seconds));
    }
    Ok(())
}

fn validate_media_resource_payload_size(resource: &MediaResource) -> Result<(), MediaError> {
    validate_optional_payload_size(
        "resource.uuid",
        resource.uuid.as_deref(),
        MEDIA_MAX_RESOURCE_UUID_BYTES,
    )?;
    validate_optional_payload_size("resource.url", resource.url.as_deref(), MEDIA_MAX_URL_BYTES)?;
    if let Some(bytes) = resource.bytes.as_ref() {
        let payload_len = bytes.len();
        if payload_len > MEDIA_MAX_RESOURCE_INLINE_BYTES {
            return Err(MediaError::payload_too_large(
                "resource.bytes",
                MEDIA_MAX_RESOURCE_INLINE_BYTES,
                payload_len,
            ));
        }
    }
    validate_optional_payload_size(
        "resource.localFile",
        resource.local_file.as_deref(),
        MEDIA_MAX_RESOURCE_LOCAL_FILE_BYTES,
    )?;
    validate_optional_payload_size(
        "resource.base64",
        resource.base64.as_deref(),
        MEDIA_MAX_RESOURCE_BASE64_BYTES,
    )?;
    validate_optional_payload_size(
        "resource.mimeType",
        resource.mime_type.as_deref(),
        MEDIA_MAX_RESOURCE_MIME_TYPE_BYTES,
    )?;
    validate_optional_payload_size(
        "resource.name",
        resource.name.as_deref(),
        MEDIA_MAX_RESOURCE_NAME_BYTES,
    )?;
    validate_optional_payload_size(
        "resource.extension",
        resource.extension.as_deref(),
        MEDIA_MAX_RESOURCE_EXTENSION_BYTES,
    )?;
    validate_map_payload_size(
        "resource.tags",
        resource.tags.as_ref(),
        MEDIA_MAX_RESOURCE_TAGS_BYTES,
    )?;
    validate_map_payload_size(
        "resource.metadata",
        resource.metadata.as_ref(),
        MEDIA_MAX_RESOURCE_METADATA_BYTES,
    )?;
    validate_optional_payload_size(
        "resource.prompt",
        resource.prompt.as_deref(),
        MEDIA_MAX_RESOURCE_PROMPT_BYTES,
    )?;
    Ok(())
}

fn validate_create_upload_request_payload_size(
    request: &CreateUploadRequest,
) -> Result<(), MediaError> {
    validate_media_asset_id(request.media_asset_id.as_str())?;
    validate_optional_payload_size("bucket", request.bucket.as_deref(), MEDIA_MAX_BUCKET_BYTES)?;
    validate_optional_payload_size(
        "objectKey",
        request.object_key.as_deref(),
        MEDIA_MAX_OBJECT_KEY_BYTES,
    )?;
    if let Some(expires_in_seconds) = request.expires_in_seconds {
        validate_download_url_expires_in_seconds(expires_in_seconds)?;
    }
    validate_media_resource_payload_size(&request.resource)?;
    Ok(())
}

fn validate_complete_upload_request_payload_size(
    request: &CompleteUploadRequest,
) -> Result<(), MediaError> {
    validate_payload_size("bucket", request.bucket.as_str(), MEDIA_MAX_BUCKET_BYTES)?;
    validate_payload_size(
        "objectKey",
        request.object_key.as_str(),
        MEDIA_MAX_OBJECT_KEY_BYTES,
    )?;
    validate_optional_payload_size(
        "storageProvider",
        request.storage_provider.as_deref(),
        MEDIA_MAX_STORAGE_PROVIDER_BYTES,
    )?;
    validate_payload_size("url", request.url.as_str(), MEDIA_MAX_URL_BYTES)?;
    validate_optional_payload_size(
        "checksum",
        request.checksum.as_deref(),
        MEDIA_MAX_CHECKSUM_BYTES,
    )?;
    Ok(())
}

fn media_scope_key(tenant_id: &str, media_asset_id: &str) -> String {
    format!("{tenant_id}:{media_asset_id}")
}

pub fn media_create_upload_request_key(auth: &AuthContext, media_asset_id: &str) -> String {
    format!(
        "{}:{}:{}:create:{}",
        auth.tenant_id, auth.actor_kind, auth.actor_id, media_asset_id
    )
}

pub fn media_complete_upload_request_key(auth: &AuthContext, media_asset_id: &str) -> String {
    format!(
        "{}:{}:{}:complete:{}",
        auth.tenant_id, auth.actor_kind, auth.actor_id, media_asset_id
    )
}

fn is_asset_owner(asset: &MediaAsset, auth: &AuthContext) -> bool {
    asset.principal_id == auth.actor_id && asset.principal_kind == auth.actor_kind
}

fn create_upload_matches_existing(
    asset: &MediaAsset,
    request: &CreateUploadRequest,
    bucket: &str,
    object_key: &str,
    storage_provider: &str,
) -> bool {
    asset.resource == request.resource
        && asset.bucket.as_deref() == Some(bucket)
        && asset.object_key.as_deref() == Some(object_key)
        && asset.storage_provider.as_deref() == Some(storage_provider)
}

fn complete_upload_request_matches_existing(
    asset: &MediaAsset,
    bucket: &str,
    object_key: &str,
    storage_provider: &str,
    checksum: Option<&String>,
) -> bool {
    asset.bucket.as_deref() == Some(bucket)
        && asset.object_key.as_deref() == Some(object_key)
        && asset.storage_provider.as_deref() == Some(storage_provider)
        && asset.checksum.as_ref() == checksum
}

fn default_media_upload_bucket() -> String {
    DEFAULT_MEDIA_UPLOAD_BUCKET.into()
}

fn resolve_media_object_key(tenant_id: &str, request: &CreateUploadRequest) -> String {
    if let Some(object_key) = request.object_key.as_ref() {
        return object_key.clone();
    }

    let file_name = request
        .resource
        .name
        .as_deref()
        .map(sanitize_media_object_path_segment)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| sanitize_media_object_path_segment(request.media_asset_id.as_str()));

    format!(
        "tenant/{tenant_id}/{}/{}",
        request.media_asset_id, file_name
    )
}

fn default_media_upload_object_key(tenant_id: &str, asset: &MediaAsset) -> String {
    let file_name = asset
        .resource
        .name
        .as_deref()
        .map(sanitize_media_object_path_segment)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "asset".into());

    format!("tenant/{tenant_id}/{}/{}", asset.media_asset_id, file_name)
}

fn sanitize_media_object_path_segment(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '-' | '_' => character,
            _ => '-',
        })
        .collect()
}
