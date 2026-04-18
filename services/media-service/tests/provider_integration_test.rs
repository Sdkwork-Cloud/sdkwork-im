use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_auth_context::AuthContext;
use im_domain_core::media::{MediaResource, MediaResourceType};
use im_domain_events::CommitEnvelope;
use im_platform_contracts::{
    CommitJournal, CommitPosition, ContractError, ObjectStorageDownloadUrlRequest,
    ObjectStorageObjectDescriptor, ObjectStorageProvider, ObjectStoragePutRequest,
    ObjectStorageUploadSession, ObjectStorageUploadUrlRequest, ProviderDomain,
    ProviderHealthSnapshot, ProviderPluginDescriptor, StaticProviderRegistry,
};
use tower::ServiceExt;

#[test]
fn test_complete_upload_uses_deployment_selected_object_storage_provider() {
    let runtime = media_service::MediaRuntime::default();
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: None,
        permissions: BTreeSet::new(),
    };

    runtime
        .create_upload(
            &auth,
            media_service::CreateUploadRequest {
                media_asset_id: "ma_provider_runtime".into(),
                bucket: None,
                object_key: None,
                expires_in_seconds: None,
                resource: MediaResource {
                    id: None,
                    uuid: Some("res_provider_runtime".into()),
                    url: None,
                    bytes: None,
                    local_file: None,
                    base64: None,
                    resource_type: Some(MediaResourceType::Video),
                    mime_type: Some("video/mp4".into()),
                    size: Some(2048),
                    name: Some("demo.mp4".into()),
                    extension: Some("mp4".into()),
                    tags: None,
                    metadata: None,
                    prompt: None,
                },
            },
        )
        .expect("create upload should succeed");

    let completed = runtime
        .complete_upload(
            &auth,
            "ma_provider_runtime",
            media_service::CompleteUploadRequest {
                bucket: "media-demo".into(),
                object_key: "tenant/t_demo/ma_provider_runtime/demo.mp4".into(),
                storage_provider: None,
                url: "https://ignored.example/demo.mp4".into(),
                checksum: Some("sha256:demo".into()),
            },
        )
        .expect("complete upload should succeed");

    assert_eq!(
        completed.storage_provider.as_deref(),
        Some("object-storage-volcengine")
    );
    let download_url = completed
        .resource
        .url
        .as_deref()
        .expect("provider-backed download url should exist");
    assert!(download_url.contains("object-storage-volcengine"));
    assert!(download_url.contains("expires=3600"));
}

#[tokio::test]
async fn test_get_media_download_url_over_http() {
    let app = media_service::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "mediaAssetId":"ma_provider_http",
                        "resource":{
                            "uuid":"res_provider_http",
                            "type":"file",
                            "mimeType":"application/pdf",
                            "size":512,
                            "name":"demo.pdf",
                            "extension":"pdf"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create upload should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let complete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads/ma_provider_http/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "bucket":"media-demo",
                        "objectKey":"tenant/t_demo/ma_provider_http/demo.pdf",
                        "url":"https://ignored.example/demo.pdf"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete upload should succeed");
    assert_eq!(complete_response.status(), StatusCode::OK);

    let download_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/media/ma_provider_http/download-url?expiresInSeconds=900")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("download url request should return response");

    assert_eq!(download_response.status(), StatusCode::OK);
    let body = download_response
        .into_body()
        .collect()
        .await
        .expect("download url body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("download url response should be valid json");

    assert_eq!(json["mediaAssetId"], "ma_provider_http");
    assert_eq!(json["storageProvider"], "object-storage-volcengine");
    assert_eq!(json["expiresInSeconds"], 900);
    let download_url = json["downloadUrl"]
        .as_str()
        .expect("downloadUrl should be returned");
    assert!(download_url.contains("object-storage-volcengine"));
    assert!(download_url.contains("expires=900"));
}

#[tokio::test]
async fn test_get_media_download_url_rejects_zero_ttl_over_http() {
    let app = media_service::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "mediaAssetId":"ma_provider_zero_ttl",
                        "resource":{
                            "uuid":"res_provider_zero_ttl",
                            "type":"file",
                            "mimeType":"application/pdf",
                            "size":512,
                            "name":"demo.pdf",
                            "extension":"pdf"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create upload should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let complete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads/ma_provider_zero_ttl/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "bucket":"media-demo",
                        "objectKey":"tenant/t_demo/ma_provider_zero_ttl/demo.pdf",
                        "url":"https://ignored.example/demo.pdf"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete upload should succeed");
    assert_eq!(complete_response.status(), StatusCode::OK);

    let download_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/media/ma_provider_zero_ttl/download-url?expiresInSeconds=0")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("download url request should return response");

    assert_eq!(download_response.status(), StatusCode::BAD_REQUEST);
    let body = download_response
        .into_body()
        .collect()
        .await
        .expect("rejection body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("rejection body should be valid json");
    assert_eq!(json["code"], "invalid_expires_in_seconds");
}

#[tokio::test]
async fn test_get_media_provider_health_over_http() {
    let app = media_service::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/media/provider-health")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("provider health request should return response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("provider health body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("provider health response should be valid json");

    assert_eq!(json["pluginId"], "object-storage-volcengine");
    assert_eq!(json["status"], "healthy");
    assert_eq!(json["details"]["providerKind"], "volcengine");
    assert!(json["checkedAt"].as_str().is_some());
}

#[test]
fn test_duplicate_complete_upload_retry_uses_existing_asset_without_reinvoking_provider() {
    let provider = VariableSignedUrlObjectStorageProvider::new(
        "object-storage-volcengine",
        "https://storage.variable.local",
    );
    let provider_registry = Arc::new(
        StaticProviderRegistry::platform_default()
            .with_deployment_profile(ProviderDomain::ObjectStorage, "object-storage-volcengine"),
    );
    let runtime = media_service::MediaRuntime::with_journal_and_provider_registry(
        Arc::new(NoopJournal),
        provider_registry,
        vec![(
            "object-storage-volcengine".into(),
            Arc::new(provider.clone()) as Arc<dyn ObjectStorageProvider>,
        )],
    );
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: None,
        permissions: BTreeSet::new(),
    };

    runtime
        .create_upload(
            &auth,
            media_service::CreateUploadRequest {
                media_asset_id: "ma_retry_complete".into(),
                bucket: None,
                object_key: None,
                expires_in_seconds: None,
                resource: MediaResource {
                    id: None,
                    uuid: Some("res_retry_complete".into()),
                    url: None,
                    bytes: None,
                    local_file: None,
                    base64: None,
                    resource_type: Some(MediaResourceType::Image),
                    mime_type: Some("image/png".into()),
                    size: Some(42),
                    name: Some("retry.png".into()),
                    extension: Some("png".into()),
                    tags: None,
                    metadata: None,
                    prompt: None,
                },
            },
        )
        .expect("create upload should succeed");

    let request = media_service::CompleteUploadRequest {
        bucket: "media-demo".into(),
        object_key: "tenant/t_demo/ma_retry_complete/retry.png".into(),
        storage_provider: None,
        url: "https://ignored.example/retry.png".into(),
        checksum: Some("sha256:retry".into()),
    };

    let first_complete = runtime
        .complete_upload(&auth, "ma_retry_complete", request.clone())
        .expect("first complete upload should succeed");
    let retry_complete = runtime
        .complete_upload(&auth, "ma_retry_complete", request)
        .expect("idempotent retry should reuse existing completed asset");

    assert_eq!(retry_complete, first_complete);
    assert_eq!(provider.put_requests().len(), 1);
    assert_eq!(provider.signed_requests().len(), 1);
}

#[derive(Default)]
struct NoopJournal;

impl CommitJournal for NoopJournal {
    fn append(&self, _envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        Ok(CommitPosition::new("noop", 0))
    }
}

#[derive(Clone)]
struct VariableSignedUrlObjectStorageProvider {
    plugin_id: String,
    endpoint: String,
    put_requests: Arc<Mutex<Vec<ObjectStoragePutRequest>>>,
    signed_requests: Arc<Mutex<Vec<ObjectStorageDownloadUrlRequest>>>,
    signed_counter: Arc<AtomicUsize>,
}

impl VariableSignedUrlObjectStorageProvider {
    fn new(plugin_id: impl Into<String>, endpoint: impl Into<String>) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            endpoint: endpoint.into(),
            put_requests: Arc::new(Mutex::new(Vec::new())),
            signed_requests: Arc::new(Mutex::new(Vec::new())),
            signed_counter: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn put_requests(&self) -> Vec<ObjectStoragePutRequest> {
        self.put_requests
            .lock()
            .expect("tracking provider should lock")
            .clone()
    }

    fn signed_requests(&self) -> Vec<ObjectStorageDownloadUrlRequest> {
        self.signed_requests
            .lock()
            .expect("tracking provider should lock")
            .clone()
    }
}

impl ObjectStorageProvider for VariableSignedUrlObjectStorageProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            self.plugin_id.clone(),
            ProviderDomain::ObjectStorage,
            "test",
            "Variable Signed URL Storage",
        )
        .with_default_selected(true)
        .with_required_capabilities(["s3", "presign"])
    }

    fn put_object(
        &self,
        request: ObjectStoragePutRequest,
    ) -> Result<ObjectStorageObjectDescriptor, ContractError> {
        self.put_requests
            .lock()
            .expect("tracking provider should lock")
            .push(request.clone());
        Ok(ObjectStorageObjectDescriptor {
            bucket: request.bucket,
            object_key: request.object_key,
            content_length: request.content_length,
            etag: Some("etag-demo".into()),
        })
    }

    fn signed_upload_url(
        &self,
        request: ObjectStorageUploadUrlRequest,
    ) -> Result<ObjectStorageUploadSession, ContractError> {
        Ok(ObjectStorageUploadSession {
            method: "PUT".into(),
            url: format!(
                "{}/{}/{}?provider={}&expires={}&upload=1",
                self.endpoint.trim_end_matches('/'),
                request.bucket,
                request.object_key,
                self.plugin_id,
                request.expires_in_seconds
            ),
            headers: BTreeMap::new(),
            expires_at: "2026-04-16T00:10:00.000Z".into(),
        })
    }

    fn signed_download_url(
        &self,
        request: ObjectStorageDownloadUrlRequest,
    ) -> Result<String, ContractError> {
        self.signed_requests
            .lock()
            .expect("tracking provider should lock")
            .push(request.clone());
        let signature = self.signed_counter.fetch_add(1, Ordering::SeqCst) + 1;
        Ok(format!(
            "{}/{}/{}?provider={}&expires={}&signature=s{}",
            self.endpoint.trim_end_matches('/'),
            request.bucket,
            request.object_key,
            self.plugin_id,
            request.expires_in_seconds,
            signature
        ))
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot::healthy(self.plugin_id.clone(), "2026-04-12T00:00:00Z")
    }
}
