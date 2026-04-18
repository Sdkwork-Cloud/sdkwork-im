use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use std::thread::sleep;
use std::time::Duration;
use tower::ServiceExt;

#[tokio::test]
async fn test_create_complete_and_get_media_asset_over_http() {
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
                        "mediaAssetId":"ma_demo",
                        "resource":{
                            "uuid":"res_demo",
                            "type":"image",
                            "mimeType":"image/png",
                            "size":42,
                            "name":"demo.png",
                            "extension":"png",
                            "metadata":{"origin":"test"},
                            "prompt":"poster"
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
        .expect("create body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(&create_body).expect("create should be valid json");
    assert_eq!(create_json["mediaAssetId"], "ma_demo");
    assert_eq!(create_json["principalId"], "u_demo");
    assert_eq!(create_json["principalKind"], "user");
    assert_eq!(create_json["processingState"], "pendingUpload");
    assert_eq!(create_json["resource"]["type"], "image");
    assert_eq!(create_json["upload"]["method"], "PUT");
    assert_eq!(create_json["upload"]["headers"], serde_json::json!({}));
    let upload_url = create_json["upload"]["url"]
        .as_str()
        .expect("create response should include upload url");
    assert!(upload_url.contains("object-storage-volcengine"));
    assert!(upload_url.contains("expires=3600"));
    assert_eq!(
        create_json["upload"]["assetId"],
        "ma_demo"
    );
    assert_eq!(
        create_json["upload"]["storageProvider"],
        "object-storage-volcengine"
    );
    assert!(
        create_json["upload"]["expiresAt"].as_str().is_some(),
        "create response should include upload expiry timestamp"
    );

    let complete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads/ma_demo/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "bucket":"local-media",
                        "objectKey":"tenant/t_demo/ma_demo/demo.png",
                        "storageProvider":"local",
                        "url":"https://cdn.example.com/ma_demo/demo.png",
                        "checksum":"sha256:demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete upload should succeed");
    assert_eq!(complete_response.status(), StatusCode::OK);
    let complete_body = complete_response
        .into_body()
        .collect()
        .await
        .expect("complete body should collect")
        .to_bytes();
    let complete_json: serde_json::Value =
        serde_json::from_slice(&complete_body).expect("complete should be valid json");
    assert_eq!(complete_json["processingState"], "ready");
    assert_eq!(complete_json["bucket"], "local-media");
    assert_eq!(
        complete_json["storageProvider"],
        "object-storage-volcengine"
    );
    let download_url = complete_json["resource"]["url"]
        .as_str()
        .expect("complete response should include provider download url");
    assert!(download_url.contains("object-storage-volcengine"));
    assert!(download_url.contains("expires=3600"));

    let get_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/media/ma_demo")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get media should succeed");
    assert_eq!(get_response.status(), StatusCode::OK);
    let get_body = get_response
        .into_body()
        .collect()
        .await
        .expect("get body should collect")
        .to_bytes();
    let get_json: serde_json::Value =
        serde_json::from_slice(&get_body).expect("get should be valid json");
    assert_eq!(get_json["mediaAssetId"], "ma_demo");
    assert_eq!(get_json["principalId"], "u_demo");
    assert_eq!(get_json["principalKind"], "user");
    assert_eq!(get_json["resource"]["name"], "demo.png");
}

#[tokio::test]
async fn test_media_asset_timestamps_advance_between_create_and_complete_requests() {
    let app = media_service::build_default_app();

    let create_first = app
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
                        "mediaAssetId":"ma_time_one",
                        "resource":{
                            "uuid":"res_time_one",
                            "type":"image",
                            "mimeType":"image/png",
                            "size":42,
                            "name":"one.png",
                            "extension":"png"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first create should succeed");
    assert_eq!(create_first.status(), StatusCode::OK);
    let create_first_body = create_first
        .into_body()
        .collect()
        .await
        .expect("first create body should collect")
        .to_bytes();
    let create_first_json: serde_json::Value =
        serde_json::from_slice(&create_first_body).expect("first create should be valid json");
    let created_first_at = create_first_json["createdAt"]
        .as_str()
        .expect("createdAt should be present")
        .to_owned();

    sleep(Duration::from_millis(20));

    let create_second = app
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
                        "mediaAssetId":"ma_time_two",
                        "resource":{
                            "uuid":"res_time_two",
                            "type":"image",
                            "mimeType":"image/png",
                            "size":42,
                            "name":"two.png",
                            "extension":"png"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second create should succeed");
    assert_eq!(create_second.status(), StatusCode::OK);
    let create_second_body = create_second
        .into_body()
        .collect()
        .await
        .expect("second create body should collect")
        .to_bytes();
    let create_second_json: serde_json::Value =
        serde_json::from_slice(&create_second_body).expect("second create should be valid json");
    let created_second_at = create_second_json["createdAt"]
        .as_str()
        .expect("createdAt should be present")
        .to_owned();

    sleep(Duration::from_millis(20));

    let complete_first = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads/ma_time_one/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "bucket":"local-media",
                        "objectKey":"tenant/t_demo/ma_time_one/one.png",
                        "storageProvider":"local",
                        "url":"https://cdn.example.com/ma_time_one/one.png"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first complete should succeed");
    assert_eq!(complete_first.status(), StatusCode::OK);
    let complete_first_body = complete_first
        .into_body()
        .collect()
        .await
        .expect("first complete body should collect")
        .to_bytes();
    let complete_first_json: serde_json::Value =
        serde_json::from_slice(&complete_first_body).expect("first complete should be valid json");
    let completed_first_at = complete_first_json["completedAt"]
        .as_str()
        .expect("completedAt should be present")
        .to_owned();

    sleep(Duration::from_millis(20));

    let complete_second = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads/ma_time_two/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "bucket":"local-media",
                        "objectKey":"tenant/t_demo/ma_time_two/two.png",
                        "storageProvider":"local",
                        "url":"https://cdn.example.com/ma_time_two/two.png"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second complete should succeed");
    assert_eq!(complete_second.status(), StatusCode::OK);
    let complete_second_body = complete_second
        .into_body()
        .collect()
        .await
        .expect("second complete body should collect")
        .to_bytes();
    let complete_second_json: serde_json::Value = serde_json::from_slice(&complete_second_body)
        .expect("second complete should be valid json");
    let completed_second_at = complete_second_json["completedAt"]
        .as_str()
        .expect("completedAt should be present")
        .to_owned();

    assert!(created_first_at < created_second_at);
    assert!(created_second_at < completed_first_at);
    assert!(completed_first_at < completed_second_at);
}

#[tokio::test]
async fn test_duplicate_create_upload_rejects_conflicting_resource_for_same_owner() {
    let app = media_service::build_default_app();

    let first_create = app
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
                        "mediaAssetId":"ma_conflicting_create",
                        "resource":{
                            "uuid":"res_conflicting_create_one",
                            "type":"image",
                            "mimeType":"image/png",
                            "size":42,
                            "name":"one.png",
                            "extension":"png"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first create should succeed");
    assert_eq!(first_create.status(), StatusCode::OK);

    let second_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "mediaAssetId":"ma_conflicting_create",
                        "resource":{
                            "uuid":"res_conflicting_create_two",
                            "type":"image",
                            "mimeType":"image/png",
                            "size":128,
                            "name":"two.png",
                            "extension":"png"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second create should return response");

    assert_eq!(second_create.status(), StatusCode::CONFLICT);
    let body = second_create
        .into_body()
        .collect()
        .await
        .expect("conflict body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("conflict body should be valid json");
    assert_eq!(json["code"], "media_asset_conflict");
}

#[tokio::test]
async fn test_duplicate_complete_upload_rejects_conflicting_storage_target() {
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
                        "mediaAssetId":"ma_conflicting_complete",
                        "resource":{
                            "uuid":"res_conflicting_complete",
                            "type":"image",
                            "mimeType":"image/png",
                            "size":42,
                            "name":"demo.png",
                            "extension":"png"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let first_complete = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads/ma_conflicting_complete/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "bucket":"local-media",
                        "objectKey":"tenant/t_demo/ma_conflicting_complete/demo.png",
                        "storageProvider":"local",
                        "url":"https://cdn.example.com/ma_conflicting_complete/demo.png",
                        "checksum":"sha256:one"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first complete should succeed");
    assert_eq!(first_complete.status(), StatusCode::OK);

    let second_complete = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads/ma_conflicting_complete/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "bucket":"remote-media",
                        "objectKey":"tenant/t_demo/ma_conflicting_complete/other.png",
                        "storageProvider":"s3",
                        "url":"https://cdn.example.com/ma_conflicting_complete/other.png",
                        "checksum":"sha256:two"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second complete should return response");

    assert_eq!(second_complete.status(), StatusCode::CONFLICT);
    let body = second_complete
        .into_body()
        .collect()
        .await
        .expect("conflict body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("conflict body should be valid json");
    assert_eq!(json["code"], "media_asset_conflict");
}

#[tokio::test]
async fn test_duplicate_media_upload_requests_expose_delivery_proof_over_http() {
    let app = media_service::build_default_app();

    let create_request = r#"{
        "mediaAssetId":"ma_delivery_proof",
        "resource":{
            "uuid":"res_delivery_proof",
            "type":"image",
            "mimeType":"image/png",
            "size":42,
            "name":"proof.png",
            "extension":"png"
        }
    }"#;

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(create_request))
                .unwrap(),
        )
        .await
        .expect("first create should return response");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value =
        serde_json::from_slice(&first_create_body).expect("first create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert_eq!(
        first_create_json["proofVersion"],
        "media.upload.delivery-proof.v1"
    );

    let duplicate_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(create_request))
                .unwrap(),
        )
        .await
        .expect("duplicate create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::OK);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate create should be valid json");
    assert_eq!(duplicate_create_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        duplicate_create_json["proofVersion"],
        first_create_json["proofVersion"]
    );

    let complete_request = r#"{
        "bucket":"local-media",
        "objectKey":"tenant/t_demo/ma_delivery_proof/proof.png",
        "storageProvider":"local",
        "url":"https://cdn.example.com/ma_delivery_proof/proof.png",
        "checksum":"sha256:proof"
    }"#;

    let first_complete = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads/ma_delivery_proof/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(complete_request))
                .unwrap(),
        )
        .await
        .expect("first complete should return response");
    assert_eq!(first_complete.status(), StatusCode::OK);
    let first_complete_body = first_complete
        .into_body()
        .collect()
        .await
        .expect("first complete body should collect")
        .to_bytes();
    let first_complete_json: serde_json::Value =
        serde_json::from_slice(&first_complete_body).expect("first complete should be valid json");
    assert_eq!(first_complete_json["deliveryStatus"], "applied");
    assert_eq!(
        first_complete_json["proofVersion"],
        "media.upload.delivery-proof.v1"
    );

    let duplicate_complete = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads/ma_delivery_proof/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(complete_request))
                .unwrap(),
        )
        .await
        .expect("duplicate complete should return response");
    assert_eq!(duplicate_complete.status(), StatusCode::OK);
    let duplicate_complete_body = duplicate_complete
        .into_body()
        .collect()
        .await
        .expect("duplicate complete body should collect")
        .to_bytes();
    let duplicate_complete_json: serde_json::Value =
        serde_json::from_slice(&duplicate_complete_body)
            .expect("duplicate complete should be valid json");
    assert_eq!(duplicate_complete_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_complete_json["requestKey"],
        first_complete_json["requestKey"]
    );
    assert_eq!(
        duplicate_complete_json["proofVersion"],
        first_complete_json["proofVersion"]
    );
}

#[tokio::test]
async fn test_create_upload_rejects_oversized_media_asset_id_over_http() {
    let app = media_service::build_default_app();
    let oversized_media_asset_id = "m".repeat(1024);
    let request_body = serde_json::json!({
        "mediaAssetId": oversized_media_asset_id,
        "resource": {
            "uuid": "res_oversized_media_asset_id",
            "type": "image",
            "mimeType": "image/png",
            "size": 42,
            "name": "demo.png",
            "extension": "png"
        }
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("create upload should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "payload_too_large");
    assert!(
        json["message"]
            .as_str()
            .expect("message should be present")
            .contains("mediaAssetId")
    );
}

#[tokio::test]
async fn test_complete_upload_rejects_oversized_object_key_over_http() {
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
                        "mediaAssetId":"ma_oversized_object_key",
                        "resource":{
                            "uuid":"res_oversized_object_key",
                            "type":"image",
                            "mimeType":"image/png",
                            "size":42,
                            "name":"demo.png",
                            "extension":"png"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create upload should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let oversized_object_key = format!("tenant/t_demo/{}", "k".repeat(4096));
    let request_body = serde_json::json!({
        "bucket": "local-media",
        "objectKey": oversized_object_key,
        "storageProvider": "local",
        "url": "https://cdn.example.com/oversized-object-key/demo.png",
        "checksum": "sha256:oversized-object-key"
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads/ma_oversized_object_key/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("complete upload should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "payload_too_large");
    assert!(
        json["message"]
            .as_str()
            .expect("message should be present")
            .contains("objectKey")
    );
}

#[tokio::test]
async fn test_complete_upload_rejects_oversized_path_media_asset_id_over_http() {
    let app = media_service::build_default_app();
    let oversized_media_asset_id = "m".repeat(1024);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/v1/media/uploads/{oversized_media_asset_id}/complete"
                ))
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "bucket":"local-media",
                        "objectKey":"tenant/t_demo/ma_oversized_path/demo.png",
                        "storageProvider":"local",
                        "url":"https://cdn.example.com/ma_oversized_path/demo.png",
                        "checksum":"sha256:demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete upload should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "payload_too_large");
    assert!(
        json["message"]
            .as_str()
            .expect("message should be present")
            .contains("mediaAssetId")
    );
}
