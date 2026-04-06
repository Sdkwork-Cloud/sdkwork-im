use std::sync::{Mutex, MutexGuard, OnceLock};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_auth_context::{PUBLIC_BEARER_HS256_SECRET_ENV, encode_hs256_bearer_token};
use serde_json::json;
use tower::ServiceExt;

const TEST_PUBLIC_SECRET: &str = "public-test-secret";

fn public_auth_guard() -> MutexGuard<'static, ()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("public auth guard should lock")
}

fn configure_public_bearer_secret() -> MutexGuard<'static, ()> {
    let guard = public_auth_guard();
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
        "sid": "s_demo"
    }))
}

fn audit_bearer() -> String {
    bearer(json!({
        "tenant_id": "t_demo",
        "sub": "u_audit_demo",
        "sid": "s_audit_demo",
        "permissions": ["audit.read", "audit.write"]
    }))
}

#[tokio::test]
async fn test_public_app_rejects_trusted_headers_for_audit_export() {
    let _guard = configure_public_bearer_secret();
    let app = audit_service::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/export")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
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
async fn test_public_app_rejects_bearer_without_audit_read_permission() {
    let _guard = configure_public_bearer_secret();
    let app = audit_service::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/export")
                .header("authorization", demo_bearer())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("public app should return response");

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "permission_denied");
}

#[tokio::test]
async fn test_public_app_accepts_privileged_bearer_for_audit_export() {
    let _guard = configure_public_bearer_secret();
    let app = audit_service::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/export")
                .header("authorization", audit_bearer())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("public app should return response");

    assert_eq!(response.status(), StatusCode::OK);
}
