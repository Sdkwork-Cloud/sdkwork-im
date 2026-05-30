use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_local_minimal_profile_gets_media_provider_health_over_http() {
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/media/provider_health")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
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
}

#[tokio::test]
async fn test_local_minimal_profile_gets_media_download_url_over_http() {
    let app = local_minimal_node::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/media/uploads")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "mediaAssetId":"ma_local_provider_http",
                        "bucket":"media-demo",
                        "resource":{
                            "uuid":"res_local_provider_http",
                            "type":"video",
                            "mimeType":"video/mp4",
                            "size":2048,
                            "name":"demo.mp4",
                            "extension":"mp4"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create upload should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("create upload body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(&create_body).expect("create upload response should be valid json");
    assert_eq!(create_json["mediaAssetId"], "ma_local_provider_http");
    assert_eq!(create_json["processingState"], "pendingUpload");
    assert_eq!(create_json["principalId"], "u_demo");
    assert_eq!(create_json["principalKind"], "user");
    assert_eq!(create_json["bucket"], "media-demo");
    assert_eq!(
        create_json["objectKey"],
        "tenant/t_demo/ma_local_provider_http/demo.mp4"
    );
    assert_eq!(
        create_json["upload"]["storageProvider"],
        "object-storage-volcengine"
    );
    assert_eq!(create_json["deliveryStatus"], "applied");
    let upload_url = create_json["upload"]["url"]
        .as_str()
        .expect("uploadUrl should be present");
    assert!(upload_url.contains("object-storage-volcengine"));
    assert!(upload_url.contains("expires=3600"));
    assert_eq!(create_json["upload"]["assetId"], "ma_local_provider_http");

    let complete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/media/uploads/ma_local_provider_http/complete")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "bucket":"media-demo",
                        "objectKey":"tenant/t_demo/ma_local_provider_http/demo.mp4",
                        "storageProvider":"object-storage-volcengine",
                        "url":"https://ignored.example/demo.mp4"
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
                .uri("/im/v3/api/media/ma_local_provider_http/download_url?expiresInSeconds=1200")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
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

    assert_eq!(json["mediaAssetId"], "ma_local_provider_http");
    assert_eq!(json["storageProvider"], "object-storage-volcengine");
    assert_eq!(json["expiresInSeconds"], 1200);
    let download_url = json["downloadUrl"]
        .as_str()
        .expect("downloadUrl should be present");
    assert!(download_url.contains("object-storage-volcengine"));
    assert!(download_url.contains("expires=1200"));
}
