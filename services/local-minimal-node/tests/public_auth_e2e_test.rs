use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_auth_context::{
    PUBLIC_BEARER_HS256_SECRET_ENV, PUBLIC_BEARER_MAX_TTL_SECONDS_ENV,
    PUBLIC_BEARER_REQUIRE_EXP_ENV, PUBLIC_BEARER_REQUIRED_AUD_ENV, PUBLIC_BEARER_REQUIRED_ISS_ENV,
    encode_hs256_bearer_token,
};
use serde_json::json;
use tokio::sync::{Mutex, MutexGuard};
use tower::ServiceExt;

const TEST_PUBLIC_SECRET: &str = "public-test-secret";
const UNSIGNED_DEMO_BEARER: &str = "Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0ZW5hbnRfaWQiOiJ0X2RlbW8iLCJzdWIiOiJ1X2RlbW8iLCJzaWQiOiJzX2RlbW8ifQ.";

struct ScopedEnvVar {
    name: &'static str,
    previous: Option<String>,
}

impl ScopedEnvVar {
    fn set(name: &'static str, value: &str) -> Self {
        let previous = std::env::var(name).ok();
        unsafe {
            std::env::set_var(name, value);
        }
        Self { name, previous }
    }
}

impl Drop for ScopedEnvVar {
    fn drop(&mut self) {
        if let Some(previous) = &self.previous {
            unsafe {
                std::env::set_var(self.name, previous);
            }
            return;
        }

        unsafe {
            std::env::remove_var(self.name);
        }
    }
}

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
        "sid": "s_demo"
    }))
}

fn owner_bearer() -> String {
    bearer(json!({
        "tenant_id": "t_demo",
        "sub": "u_owner",
        "sid": "s_owner"
    }))
}

async fn create_group_conversation(
    app: axum::Router,
    authorization: String,
    conversation_id: &str,
) -> axum::response::Response {
    app.oneshot(
        Request::builder()
            .method("POST")
            .uri("/api/v1/conversations")
            .header("authorization", authorization)
            .header("content-type", "application/json")
            .body(Body::from(format!(
                r#"{{
                        "conversationId":"{conversation_id}",
                        "conversationType":"group"
                    }}"#
            )))
            .expect("conversation request should build"),
    )
    .await
    .expect("conversation route should return response")
}

#[tokio::test]
async fn test_public_app_rejects_trusted_headers_without_bearer() {
    let _guard = configure_public_bearer_secret().await;
    let app = local_minimal_node::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_public_auth_reject",
                        "conversationType":"group"
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
async fn test_public_app_accepts_bearer_for_app_facing_routes() {
    let _guard = configure_public_bearer_secret().await;
    let app = local_minimal_node::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", demo_bearer())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_public_auth_accept",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("public app should return response");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_public_app_rejects_unsigned_bearer_when_public_verifier_is_configured() {
    let _guard = configure_public_bearer_secret().await;
    let app = local_minimal_node::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", UNSIGNED_DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_public_auth_unsigned_reject",
                        "conversationType":"group"
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
    assert_eq!(json["code"], "jwt_algorithm_invalid");
}

#[tokio::test]
async fn test_public_app_rejects_unprivileged_bearer_for_ops_and_audit_routes() {
    let _guard = configure_public_bearer_secret().await;
    let app = local_minimal_node::build_public_app();

    let audit_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/export")
                .header("authorization", demo_bearer())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("audit route should return response");

    assert_eq!(audit_response.status(), StatusCode::FORBIDDEN);
    let audit_body = audit_response
        .into_body()
        .collect()
        .await
        .expect("audit body should collect")
        .to_bytes();
    let audit_json: serde_json::Value =
        serde_json::from_slice(&audit_body).expect("audit body should be valid json");
    assert_eq!(audit_json["code"], "permission_denied");

    let ops_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/diagnostics")
                .header("authorization", demo_bearer())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops route should return response");

    assert_eq!(ops_response.status(), StatusCode::FORBIDDEN);
    let ops_body = ops_response
        .into_body()
        .collect()
        .await
        .expect("ops body should collect")
        .to_bytes();
    let ops_json: serde_json::Value =
        serde_json::from_slice(&ops_body).expect("ops body should be valid json");
    assert_eq!(ops_json["code"], "permission_denied");
}

#[tokio::test]
async fn test_public_app_rejects_cross_recipient_notification_request_without_permission() {
    let _guard = configure_public_bearer_secret().await;
    let app = local_minimal_node::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/notifications/requests")
                .header("authorization", demo_bearer())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_cross_recipient",
                        "sourceEventId":"evt_cross_recipient",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"u_target",
                        "title":"New message",
                        "body":"hello"
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
async fn test_public_app_accepts_self_notification_request() {
    let _guard = configure_public_bearer_secret().await;
    let app = local_minimal_node::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/notifications/requests")
                .header("authorization", demo_bearer())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_self",
                        "sourceEventId":"evt_self",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"u_demo",
                        "title":"New message",
                        "body":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("public app should return response");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_public_app_rejects_unprivileged_bearer_for_automation_execution() {
    let _guard = configure_public_bearer_secret().await;
    let app = local_minimal_node::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("authorization", demo_bearer())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_unprivileged",
                        "triggerType":"webhook.manual",
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
async fn test_public_app_rejects_cross_principal_media_attach() {
    let _guard = configure_public_bearer_secret().await;
    let app = local_minimal_node::build_public_app();

    let create_upload = app
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
    assert_eq!(create_upload.status(), StatusCode::OK);

    let complete_upload = app
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
                        "storageProvider":"local",
                        "url":"https://cdn.example.com/ma_cross_principal/demo.png",
                        "checksum":"sha256:demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete upload should return response");
    assert_eq!(complete_upload.status(), StatusCode::OK);

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", demo_bearer())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_cross_principal_media",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let attach_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/ma_cross_principal/attach")
                .header("authorization", demo_bearer())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_cross_principal_media",
                        "clientMsgId":"client_attach_foreign_media",
                        "summary":"foreign media",
                        "text":"should not attach"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("attach media should return response");

    assert_eq!(attach_response.status(), StatusCode::NOT_FOUND);
    let body = attach_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "media_asset_not_found");
}

#[tokio::test]
async fn test_public_app_rejects_bearer_without_exp_when_exp_requirement_is_enabled() {
    let _guard = configure_public_bearer_secret().await;
    let _exp_requirement = ScopedEnvVar::set(PUBLIC_BEARER_REQUIRE_EXP_ENV, "true");
    let app = local_minimal_node::build_public_app();
    let bearer_without_exp = bearer(json!({
        "tenant_id": "t_demo",
        "sub": "u_demo",
        "sid": "s_demo"
    }));

    let response =
        create_group_conversation(app, bearer_without_exp, "c_public_auth_require_exp_guard").await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "jwt_exp_required");
}

#[tokio::test]
async fn test_public_app_rejects_bearer_exceeding_max_ttl_when_ttl_guard_is_enabled() {
    let _guard = configure_public_bearer_secret().await;
    let _ttl_guard = ScopedEnvVar::set(PUBLIC_BEARER_MAX_TTL_SECONDS_ENV, "60");
    let app = local_minimal_node::build_public_app();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_secs();
    let long_ttl_bearer = bearer(json!({
        "tenant_id": "t_demo",
        "sub": "u_demo",
        "sid": "s_demo",
        "iat": now,
        "exp": now + 3600
    }));

    let response = create_group_conversation(app, long_ttl_bearer, "c_public_auth_ttl_guard").await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "jwt_ttl_exceeded");
}

#[tokio::test]
async fn test_public_app_rejects_bearer_when_required_issuer_or_audience_do_not_match() {
    let _guard = configure_public_bearer_secret().await;
    let _required_issuer = ScopedEnvVar::set(PUBLIC_BEARER_REQUIRED_ISS_ENV, "craw-chat");
    let _required_audience = ScopedEnvVar::set(PUBLIC_BEARER_REQUIRED_AUD_ENV, "craw-chat-public");
    let app = local_minimal_node::build_public_app();
    let missing_contract_claims_bearer = bearer(json!({
        "tenant_id": "t_demo",
        "sub": "u_demo",
        "sid": "s_demo"
    }));

    let response = create_group_conversation(
        app.clone(),
        missing_contract_claims_bearer,
        "c_public_auth_required_issuer_guard",
    )
    .await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "jwt_issuer_invalid");

    let wrong_audience_bearer = bearer(json!({
        "tenant_id": "t_demo",
        "sub": "u_demo",
        "sid": "s_demo",
        "iss": "craw-chat",
        "aud": "another-audience"
    }));
    let response = create_group_conversation(
        app,
        wrong_audience_bearer,
        "c_public_auth_required_audience_guard",
    )
    .await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "jwt_audience_invalid");
}

#[tokio::test]
async fn test_public_app_accepts_bearer_when_required_issuer_and_audience_match() {
    let _guard = configure_public_bearer_secret().await;
    let _required_issuer = ScopedEnvVar::set(PUBLIC_BEARER_REQUIRED_ISS_ENV, "craw-chat");
    let _required_audience = ScopedEnvVar::set(PUBLIC_BEARER_REQUIRED_AUD_ENV, "craw-chat-public");
    let app = local_minimal_node::build_public_app();
    let contract_compliant_bearer = bearer(json!({
        "tenant_id": "t_demo",
        "sub": "u_demo",
        "sid": "s_demo",
        "iss": "craw-chat",
        "aud": ["craw-chat-public", "fallback-audience"]
    }));

    let response = create_group_conversation(
        app,
        contract_compliant_bearer,
        "c_public_auth_issuer_audience_ok",
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
}
