use std::sync::OnceLock;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_auth_context::{PUBLIC_BEARER_HS256_SECRET_ENV, encode_hs256_bearer_token};
use serde_json::json;
use tokio::sync::{Mutex, MutexGuard};
use tower::ServiceExt;

const TEST_PUBLIC_SECRET: &str = "public-test-secret";
const SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS";
const SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS";

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

fn demo_bearer() -> String {
    signed_bearer(json!({
        "tenant_id": "t_demo",
        "sub": "u_demo",
        "actor_kind": "user",
        "sid": "s_demo"
    }))
}

fn signed_bearer(claims: serde_json::Value) -> String {
    format!(
        "Bearer {}",
        encode_hs256_bearer_token(&claims, TEST_PUBLIC_SECRET)
            .expect("signed bearer token should encode")
    )
}

#[tokio::test]
async fn test_public_app_rejects_trusted_headers_for_create_conversation() {
    let _guard = configure_public_bearer_secret().await;
    let app = conversation_runtime::build_public_app();

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
                        "conversationId":"c_public_auth",
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
async fn test_public_app_accepts_bearer_for_create_conversation() {
    let _guard = configure_public_bearer_secret().await;
    let app = conversation_runtime::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", demo_bearer())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_public_auth_bearer",
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
async fn test_public_app_rejects_shared_channel_sync_without_dedicated_permission() {
    let _guard = configure_public_bearer_secret().await;
    let app = conversation_runtime::build_public_app();
    let system_bearer = signed_bearer(json!({
        "tenant_id": "t_demo",
        "sub": "control-plane-sync",
        "actor_kind": "system"
    }));

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/shared-channel-links/sync")
                .header("authorization", system_bearer)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_shared_sync_permission_guard",
                        "sharedChannelPolicyId":"scp_permission_guard",
                        "externalConnectionId":"ec_permission_guard",
                        "localActorId":"u_remote_partner",
                        "localActorKind":"user",
                        "externalMemberId":"partner::permission-guard"
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
    assert_eq!(json["code"], "shared_channel_sync_permission_denied");
}

#[tokio::test]
async fn test_public_app_rejects_shared_channel_sync_for_non_control_plane_actor() {
    let _guard = configure_public_bearer_secret().await;
    let app = conversation_runtime::build_public_app();
    let system_bearer = signed_bearer(json!({
        "tenant_id": "t_demo",
        "sub": "svc_control",
        "actor_kind": "system",
        "permissions": ["conversation.shared_channel.sync"]
    }));

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/shared-channel-links/sync")
                .header("authorization", system_bearer)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_shared_sync_actor_guard",
                        "sharedChannelPolicyId":"scp_actor_guard",
                        "externalConnectionId":"ec_actor_guard",
                        "localActorId":"u_remote_partner",
                        "localActorKind":"user",
                        "externalMemberId":"partner::actor-guard"
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
    assert_eq!(json["code"], "shared_channel_sync_actor_invalid");
}

#[tokio::test]
async fn test_public_app_rate_limits_shared_channel_sync_requests() {
    let _guard = configure_public_bearer_secret().await;
    let _limit = ScopedEnvVar::set(SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS_ENV, "2");
    let _window = ScopedEnvVar::set(SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS_ENV, "300");
    let app = conversation_runtime::build_public_app();
    let system_bearer = signed_bearer(json!({
        "tenant_id": "t_demo",
        "sub": "control-plane-sync",
        "actor_kind": "system",
        "permissions": ["conversation.shared_channel.sync"]
    }));

    for index in 0..2 {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/conversations/shared-channel-links/sync")
                    .header("authorization", system_bearer.as_str())
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{
                            "conversationId":"c_shared_sync_rate_limit_guard",
                            "sharedChannelPolicyId":"scp_rate_limit_guard",
                            "externalConnectionId":"ec_rate_limit_guard",
                            "localActorId":"u_remote_partner",
                            "localActorKind":"user",
                            "externalMemberId":"partner::rate-limit-guard"
                        }"#,
                    ))
                    .unwrap(),
            )
            .await
            .expect("public app should return response");
        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "request {} should pass auth/permission checks and reach runtime before rate limit",
            index + 1
        );
    }

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/shared-channel-links/sync")
                .header("authorization", system_bearer.as_str())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_shared_sync_rate_limit_guard",
                        "sharedChannelPolicyId":"scp_rate_limit_guard",
                        "externalConnectionId":"ec_rate_limit_guard",
                        "localActorId":"u_remote_partner",
                        "localActorKind":"user",
                        "externalMemberId":"partner::rate-limit-guard"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("public app should return response");

    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "shared_channel_sync_rate_limited");
}
