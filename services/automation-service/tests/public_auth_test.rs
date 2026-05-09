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

fn executor_bearer() -> String {
    bearer(json!({
        "tenant_id": "t_demo",
        "sub": "u_demo",
        "actor_kind": "user",
        "sid": "s_demo",
        "permissions": ["automation.execute", "automation.read"]
    }))
}

fn execute_only_bearer() -> String {
    bearer(json!({
        "tenant_id": "t_demo",
        "sub": "u_demo",
        "actor_kind": "user",
        "sid": "s_demo",
        "permissions": ["automation.execute"]
    }))
}

fn owner_automation_bearer() -> String {
    bearer(json!({
        "tenant_id": "t_demo",
        "sub": "u_owner",
        "actor_kind": "user",
        "sid": "s_owner",
        "permissions": ["automation.execute", "automation.read"]
    }))
}

fn other_automation_bearer() -> String {
    bearer(json!({
        "tenant_id": "t_demo",
        "sub": "u_other",
        "actor_kind": "user",
        "sid": "s_other",
        "permissions": ["automation.read"]
    }))
}

#[tokio::test]
async fn test_public_app_rejects_trusted_headers_for_execution_request() {
    let _guard = configure_public_bearer_secret().await;
    let app = automation_service::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_public_auth",
                        "triggerType":"webhook",
                        "targetKind":"workflow",
                        "targetRef":"wf_demo",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
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
async fn test_public_app_rejects_unprivileged_bearer_for_execution_request() {
    let _guard = configure_public_bearer_secret().await;
    let app = automation_service::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("authorization", demo_bearer())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_public_auth_unprivileged",
                        "triggerType":"webhook",
                        "targetKind":"workflow",
                        "targetRef":"wf_demo",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
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
async fn test_public_app_accepts_bearer_with_execute_permission_for_execution_request() {
    let _guard = configure_public_bearer_secret().await;
    let app = automation_service::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("authorization", executor_bearer())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_public_auth_bearer",
                        "triggerType":"webhook",
                        "targetKind":"workflow",
                        "targetRef":"wf_demo",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("public app should return response");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_public_app_rejects_execution_lookup_without_read_permission() {
    let _guard = configure_public_bearer_secret().await;
    let app = automation_service::build_public_app();

    let create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("authorization", execute_only_bearer())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_read_permission_required",
                        "triggerType":"webhook",
                        "targetKind":"workflow",
                        "targetRef":"wf_demo",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create execution should return response");
    assert_eq!(create.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/automation/executions/ae_read_permission_required")
                .header("authorization", execute_only_bearer())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get execution should return response");

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
async fn test_public_app_hides_cross_principal_execution_lookup() {
    let _guard = configure_public_bearer_secret().await;
    let app = automation_service::build_public_app();

    let create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("authorization", owner_automation_bearer())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_cross_principal",
                        "triggerType":"webhook",
                        "targetKind":"workflow",
                        "targetRef":"wf_demo",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create execution should return response");
    assert_eq!(create.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/automation/executions/ae_cross_principal")
                .header("authorization", other_automation_bearer())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("cross principal get should return response");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "automation_execution_not_found");
}
