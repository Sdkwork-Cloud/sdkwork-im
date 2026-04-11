use std::sync::{Arc, Mutex, MutexGuard, OnceLock};
use std::time::Duration;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::routing::post;
use axum::{Json, Router};
use control_plane_api::SharedChannelLinkedMemberSyncRequest;
use im_auth_context::{PUBLIC_BEARER_REQUIRED_AUD_ENV, PUBLIC_BEARER_REQUIRED_ISS_ENV};
use serde_json::json;
use tokio::net::TcpListener;

#[derive(Clone, Default)]
struct CapturedSyncRequest {
    authorization: Arc<Mutex<Option<String>>>,
}

fn decode_base64url(input: &str) -> Vec<u8> {
    let mut output = Vec::with_capacity((input.len() * 3) / 4 + 3);
    let mut buffer = 0u32;
    let mut bits = 0u8;

    for byte in input.bytes() {
        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'-' => 62,
            b'_' => 63,
            b'=' => continue,
            _ => panic!("jwt payload segment should be valid base64url"),
        } as u32;

        buffer = (buffer << 6) | value;
        bits += 6;

        while bits >= 8 {
            bits -= 8;
            output.push(((buffer >> bits) & 0xff) as u8);
        }
    }

    output
}

fn decode_claims_from_bearer(authorization: &str) -> serde_json::Value {
    let token = authorization
        .strip_prefix("Bearer ")
        .or_else(|| authorization.strip_prefix("bearer "))
        .expect("authorization should be bearer");
    let segments: Vec<&str> = token.split('.').collect();
    assert_eq!(
        segments.len(),
        3,
        "shared-channel sync bearer should contain jwt header/payload/signature"
    );

    let payload = decode_base64url(segments[1]);
    serde_json::from_slice(&payload).expect("jwt claims should be valid json")
}

async fn capture_sync_call(
    State(captured): State<CapturedSyncRequest>,
    headers: HeaderMap,
) -> (StatusCode, Json<serde_json::Value>) {
    let authorization = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    *captured
        .authorization
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = authorization;
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}

async fn capture_sync_call_with_delay(
    State(captured): State<CapturedSyncRequest>,
    headers: HeaderMap,
) -> (StatusCode, Json<serde_json::Value>) {
    let authorization = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    *captured
        .authorization
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = authorization;
    tokio::time::sleep(Duration::from_millis(200)).await;
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}

async fn capture_sync_call_with_large_error_body(
    State(captured): State<CapturedSyncRequest>,
    headers: HeaderMap,
) -> (StatusCode, String) {
    let authorization = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    *captured
        .authorization
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = authorization;
    (StatusCode::BAD_GATEWAY, "x".repeat(20_000))
}

fn insecure_http_guard() -> MutexGuard<'static, ()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn public_bearer_contract_guard() -> MutexGuard<'static, ()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

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

fn clear_insecure_http_override() {
    unsafe {
        std::env::remove_var(control_plane_api::ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP_ENV);
    }
}

fn clear_shared_channel_sync_timeout_override() {
    unsafe {
        std::env::remove_var(control_plane_api::SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_MILLIS_ENV);
    }
}

#[test]
fn test_public_shared_channel_sync_trigger_accepts_https_target() {
    let _guard = insecure_http_guard();
    clear_insecure_http_override();
    let trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        "https://sync.example.com",
        "secret",
    );
    assert!(
        trigger.is_ok(),
        "https target should be accepted for shared-channel sync trigger"
    );
}

#[test]
fn test_public_shared_channel_sync_trigger_accepts_localhost_http_target() {
    let _guard = insecure_http_guard();
    clear_insecure_http_override();
    let trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        "http://127.0.0.1:19080",
        "secret",
    );
    assert!(
        trigger.is_ok(),
        "localhost http target should remain available for local testing"
    );
}

#[test]
fn test_public_shared_channel_sync_trigger_rejects_remote_http_target() {
    let _guard = insecure_http_guard();
    clear_insecure_http_override();
    let error = match control_plane_api::build_public_shared_channel_sync_trigger(
        "http://sync.example.com",
        "secret",
    ) {
        Ok(_) => panic!("remote http target must be rejected"),
        Err(error) => error,
    };
    assert!(
        error.contains("https://"),
        "error should guide callers to use https transport, got: {error}"
    );
}

#[test]
fn test_public_shared_channel_sync_trigger_allows_remote_http_when_explicitly_enabled() {
    let _guard = insecure_http_guard();
    clear_insecure_http_override();
    unsafe {
        std::env::set_var(
            control_plane_api::ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP_ENV,
            "true",
        );
    }
    let trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        "http://sync.example.com",
        "secret",
    );
    clear_insecure_http_override();
    assert!(
        trigger.is_ok(),
        "explicitly enabled insecure mode should allow non-local http target"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_public_shared_channel_sync_trigger_embeds_dedicated_permission_claim() {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("shared-channel sync test listener should bind");
    let local_addr = listener
        .local_addr()
        .expect("shared-channel sync test listener should expose local addr");
    let captured = CapturedSyncRequest::default();
    let app = Router::new()
        .route(
            "/api/v1/conversations/shared-channel-links/sync",
            post(capture_sync_call),
        )
        .with_state(captured.clone());
    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("shared-channel sync test server should run");
    });

    let trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        format!("http://{local_addr}"),
        "test-shared-channel-secret",
    )
    .expect("shared-channel sync trigger should build against local http target");
    trigger
        .trigger(SharedChannelLinkedMemberSyncRequest {
            tenant_id: "t_demo".into(),
            conversation_id: "c_shared_sync_permission_claim".into(),
            shared_channel_policy_id: "scp_permission_claim".into(),
            external_connection_id: "ec_permission_claim".into(),
            local_actor_id: "u_local_actor".into(),
            local_actor_kind: "user".into(),
            external_member_id: "partner::permission-claim".into(),
        })
        .expect("shared-channel sync trigger should dispatch request");

    let authorization = captured
        .authorization
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .clone()
        .expect("sync request should include authorization header");
    let claims = decode_claims_from_bearer(authorization.as_str());
    let permissions = claims["permissions"]
        .as_array()
        .expect("shared-channel sync token should include permissions array");
    assert!(
        permissions
            .iter()
            .any(|item| item.as_str() == Some("conversation.shared_channel.sync")),
        "shared-channel sync token should include dedicated sync permission claim"
    );

    server.abort();
    let _ = server.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_public_shared_channel_sync_trigger_includes_required_issuer_and_audience_claims_when_configured()
 {
    let _guard = public_bearer_contract_guard();
    let _required_issuer = ScopedEnvVar::set(PUBLIC_BEARER_REQUIRED_ISS_ENV, "craw-chat");
    let _required_audience = ScopedEnvVar::set(PUBLIC_BEARER_REQUIRED_AUD_ENV, "craw-chat-public");
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("shared-channel sync test listener should bind");
    let local_addr = listener
        .local_addr()
        .expect("shared-channel sync test listener should expose local addr");
    let captured = CapturedSyncRequest::default();
    let app = Router::new()
        .route(
            "/api/v1/conversations/shared-channel-links/sync",
            post(capture_sync_call),
        )
        .with_state(captured.clone());
    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("shared-channel sync test server should run");
    });

    let trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        format!("http://{local_addr}"),
        "test-shared-channel-secret",
    )
    .expect("shared-channel sync trigger should build against local http target");
    trigger
        .trigger(SharedChannelLinkedMemberSyncRequest {
            tenant_id: "t_demo".into(),
            conversation_id: "c_shared_sync_issuer_audience_claim".into(),
            shared_channel_policy_id: "scp_issuer_audience_claim".into(),
            external_connection_id: "ec_issuer_audience_claim".into(),
            local_actor_id: "u_local_actor".into(),
            local_actor_kind: "user".into(),
            external_member_id: "partner::issuer-audience-claim".into(),
        })
        .expect("shared-channel sync trigger should dispatch request");

    let authorization = captured
        .authorization
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .clone()
        .expect("sync request should include authorization header");
    let claims = decode_claims_from_bearer(authorization.as_str());
    assert_eq!(claims["iss"], "craw-chat");
    assert_eq!(claims["aud"], "craw-chat-public");

    server.abort();
    let _ = server.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_public_shared_channel_sync_trigger_fails_fast_when_http_timeout_is_exceeded() {
    let _guard = insecure_http_guard();
    clear_shared_channel_sync_timeout_override();
    let _timeout_override = ScopedEnvVar::set(
        control_plane_api::SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_MILLIS_ENV,
        "20",
    );
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("shared-channel sync timeout test listener should bind");
    let local_addr = listener
        .local_addr()
        .expect("shared-channel sync timeout test listener should expose local addr");
    let captured = CapturedSyncRequest::default();
    let app = Router::new()
        .route(
            "/api/v1/conversations/shared-channel-links/sync",
            post(capture_sync_call_with_delay),
        )
        .with_state(captured);
    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("shared-channel sync timeout test server should run");
    });

    let trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        format!("http://{local_addr}"),
        "test-shared-channel-secret",
    )
    .expect("shared-channel sync trigger should build against local http target");
    let error = trigger
        .trigger(SharedChannelLinkedMemberSyncRequest {
            tenant_id: "t_demo".into(),
            conversation_id: "c_shared_sync_timeout_guard".into(),
            shared_channel_policy_id: "scp_timeout_guard".into(),
            external_connection_id: "ec_timeout_guard".into(),
            local_actor_id: "u_local_actor".into(),
            local_actor_kind: "user".into(),
            external_member_id: "partner::timeout-guard".into(),
        })
        .expect_err("shared-channel sync trigger should fail when response exceeds timeout");
    assert!(
        error.contains("timed out"),
        "timeout failure should mention timed out, got: {error}"
    );

    server.abort();
    let _ = server.await;
    clear_shared_channel_sync_timeout_override();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_public_shared_channel_sync_trigger_rejects_oversized_response_body() {
    let _guard = insecure_http_guard();
    clear_shared_channel_sync_timeout_override();
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("shared-channel sync oversized body test listener should bind");
    let local_addr = listener
        .local_addr()
        .expect("shared-channel sync oversized body test listener should expose local addr");
    let captured = CapturedSyncRequest::default();
    let app = Router::new()
        .route(
            "/api/v1/conversations/shared-channel-links/sync",
            post(capture_sync_call_with_large_error_body),
        )
        .with_state(captured);
    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("shared-channel sync oversized body test server should run");
    });

    let trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        format!("http://{local_addr}"),
        "test-shared-channel-secret",
    )
    .expect("shared-channel sync trigger should build against local http target");
    let error = trigger
        .trigger(SharedChannelLinkedMemberSyncRequest {
            tenant_id: "t_demo".into(),
            conversation_id: "c_shared_sync_body_limit_guard".into(),
            shared_channel_policy_id: "scp_body_limit_guard".into(),
            external_connection_id: "ec_body_limit_guard".into(),
            local_actor_id: "u_local_actor".into(),
            local_actor_kind: "user".into(),
            external_member_id: "partner::body-limit-guard".into(),
        })
        .expect_err("shared-channel sync trigger should fail when response body exceeds limit");
    assert!(
        error.contains("exceeds maximum body size"),
        "oversized response failure should mention body size guard, got: {error}"
    );

    server.abort();
    let _ = server.await;
}
