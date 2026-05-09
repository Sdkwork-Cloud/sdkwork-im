use std::sync::OnceLock;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_auth_context::{PUBLIC_BEARER_HS256_SECRET_ENV, encode_hs256_bearer_token};
use serde_json::json;
use tokio::sync::{Mutex, MutexGuard};
use tower::ServiceExt;

const TEST_PUBLIC_SECRET: &str = "public-test-secret";

async fn public_auth_guard() -> MutexGuard<'static, ()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(())).lock().await
}

async fn configure_public_bearer_secret() -> MutexGuard<'static, ()> {
    let guard = public_auth_guard().await;
    unsafe {
        std::env::set_var(PUBLIC_BEARER_HS256_SECRET_ENV, TEST_PUBLIC_SECRET);
    }
    guard
}

fn bearer(claims: serde_json::Value) -> String {
    format!(
        "Bearer {}",
        encode_hs256_bearer_token(&claims, TEST_PUBLIC_SECRET)
            .expect("signed bearer token should encode")
    )
}

fn demo_bearer() -> String {
    bearer(json!({
        "tenant_id": "t_demo",
        "sub": "u_demo",
        "actor_kind": "user",
        "sid": "s_demo"
    }))
}

fn owner_bearer() -> String {
    bearer(json!({
        "tenant_id": "t_demo",
        "sub": "u_owner",
        "actor_kind": "user",
        "sid": "s_owner"
    }))
}

#[tokio::test]
async fn test_public_app_rejects_trusted_headers_for_media_upload() {
    let _guard = configure_public_bearer_secret().await;
    let app = media_service::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "mediaAssetId":"ma_public_auth",
                        "bucket":"local-media",
                        "resource":{
                            "uuid":"res_public_auth",
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
        .expect("public app should return response");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "auth_context_missing");
}

#[tokio::test]
async fn test_public_app_accepts_bearer_for_media_upload() {
    let _guard = configure_public_bearer_secret().await;
    let app = media_service::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads")
                .header("authorization", demo_bearer())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "mediaAssetId":"ma_public_auth_bearer",
                        "bucket":"local-media",
                        "resource":{
                            "uuid":"res_public_auth_bearer",
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
        .expect("public app should return response");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_public_app_rejects_cross_principal_media_read() {
    let _guard = configure_public_bearer_secret().await;
    let app = media_service::build_public_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads")
                .header("authorization", owner_bearer())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "mediaAssetId":"ma_cross_principal",
                        "bucket":"local-media",
                        "resource":{
                            "uuid":"res_cross_principal",
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
        .expect("create upload should return response");
    assert_eq!(create_response.status(), StatusCode::OK);

    let complete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads/ma_cross_principal/complete")
                .header("authorization", owner_bearer())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "bucket":"local-media",
                        "objectKey":"tenant/t_demo/ma_cross_principal/demo.png",
                        "storageProvider":"object-storage-volcengine",
                        "url":"https://cdn.example.com/ma_cross_principal/demo.png",
                        "checksum":"sha256:demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete upload should return response");
    assert_eq!(complete_response.status(), StatusCode::OK);

    let get_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/media/ma_cross_principal")
                .header("authorization", demo_bearer())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get media should return response");

    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
    let body = get_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "media_asset_not_found");
}

#[tokio::test]
async fn test_public_app_rejects_cross_principal_media_asset_id_collision() {
    let _guard = configure_public_bearer_secret().await;
    let app = media_service::build_public_app();

    let owner_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads")
                .header("authorization", owner_bearer())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "mediaAssetId":"ma_collision",
                        "bucket":"local-media",
                        "resource":{
                            "uuid":"res_collision_owner",
                            "type":"image",
                            "mimeType":"image/png",
                            "size":42,
                            "name":"owner.png",
                            "extension":"png",
                            "metadata":{"origin":"test"},
                            "prompt":"poster"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("owner create should return response");
    assert_eq!(owner_create.status(), StatusCode::OK);

    let intruder_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads")
                .header("authorization", demo_bearer())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "mediaAssetId":"ma_collision",
                        "bucket":"local-media",
                        "resource":{
                            "uuid":"res_collision_intruder",
                            "type":"image",
                            "mimeType":"image/png",
                            "size":42,
                            "name":"intruder.png",
                            "extension":"png",
                            "metadata":{"origin":"test"},
                            "prompt":"poster"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("intruder create should return response");

    assert_eq!(intruder_create.status(), StatusCode::CONFLICT);
    let body = intruder_create
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "media_asset_already_exists");
}
