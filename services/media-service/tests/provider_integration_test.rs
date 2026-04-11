use std::collections::BTreeSet;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_auth_context::AuthContext;
use im_domain_core::media::{MediaResource, MediaResourceType};
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
