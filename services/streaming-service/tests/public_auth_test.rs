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

fn demo_bearer() -> String {
    format!(
        "Bearer {}",
        encode_hs256_bearer_token(
            &json!({
                "tenant_id": "t_demo",
                "sub": "u_demo",
                "sid": "s_demo"
            }),
            TEST_PUBLIC_SECRET,
        )
        .expect("signed bearer token should encode")
    )
}

#[tokio::test]
async fn test_public_app_rejects_trusted_headers_without_bearer() {
    let _guard = configure_public_bearer_secret().await;
    let app = streaming_service::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_public_auth",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
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
async fn test_public_app_accepts_bearer_for_stream_routes() {
    let _guard = configure_public_bearer_secret().await;
    let app = streaming_service::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("authorization", demo_bearer())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_public_auth_bearer",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("public app should return response");

    assert_eq!(response.status(), StatusCode::OK);
}
