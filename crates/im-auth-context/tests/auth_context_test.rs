use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::http::header::AUTHORIZATION;
use axum::http::{HeaderMap, HeaderValue};
use im_auth_context::{
    PUBLIC_BEARER_HS256_SECRET_ENV, resolve_auth_context, resolve_bearer_auth_context,
    resolve_public_bearer_auth_context,
};

const TEST_PUBLIC_SECRET: &str = "public-test-secret";

fn public_auth_guard() -> MutexGuard<'static, ()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn configure_public_bearer_secret() -> MutexGuard<'static, ()> {
    let guard = public_auth_guard();
    unsafe {
        std::env::set_var(PUBLIC_BEARER_HS256_SECRET_ENV, TEST_PUBLIC_SECRET);
    }
    guard
}

#[test]
fn test_resolve_trusted_headers_supports_device_id() {
    let mut headers = HeaderMap::new();
    headers.insert("x-tenant-id", HeaderValue::from_static("t_demo"));
    headers.insert("x-user-id", HeaderValue::from_static("u_demo"));
    headers.insert("x-session-id", HeaderValue::from_static("s_demo"));
    headers.insert("x-device-id", HeaderValue::from_static("d_demo"));

    let auth = resolve_auth_context(&headers).expect("trusted headers should resolve");

    assert_eq!(auth.tenant_id, "t_demo");
    assert_eq!(auth.actor_id, "u_demo");
    assert_eq!(auth.session_id.as_deref(), Some("s_demo"));
    assert_eq!(auth.device_id.as_deref(), Some("d_demo"));
}

#[test]
fn test_auth_context_projects_ccp_authority_fields() {
    let mut headers = HeaderMap::new();
    headers.insert("x-tenant-id", HeaderValue::from_static("t_demo"));
    headers.insert("x-user-id", HeaderValue::from_static("u_demo"));
    headers.insert("x-session-id", HeaderValue::from_static("s_demo"));
    headers.insert("x-device-id", HeaderValue::from_static("d_demo"));
    headers.insert("x-actor-kind", HeaderValue::from_static("user"));

    let auth = resolve_auth_context(&headers).expect("trusted headers should resolve");
    let authority = auth.ccp_authority();

    assert_eq!(authority.tenant_id, "t_demo");
    assert_eq!(authority.actor.actor_id, "u_demo");
    assert_eq!(authority.actor.actor_kind, "user");
    assert_eq!(authority.sender.principal_id, "u_demo");
    assert_eq!(authority.sender.device_id.as_deref(), Some("d_demo"));
    assert_eq!(authority.sender.session_id.as_deref(), Some("s_demo"));
    assert_eq!(authority.sender.sender_id(), "u_demo:d_demo");
}

#[test]
fn test_resolve_bearer_token_supports_device_claim() {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_static(
            "Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0ZW5hbnRfaWQiOiJ0X2RlbW8iLCJzdWIiOiJ1X2RlbW8iLCJzaWQiOiJzX2RlbW8iLCJkaWQiOiJkX2RlbW8ifQ.",
        ),
    );

    let auth = resolve_auth_context(&headers).expect("bearer token should resolve");

    assert_eq!(auth.tenant_id, "t_demo");
    assert_eq!(auth.actor_id, "u_demo");
    assert_eq!(auth.session_id.as_deref(), Some("s_demo"));
    assert_eq!(auth.device_id.as_deref(), Some("d_demo"));
}

#[test]
fn test_resolve_bearer_token_supports_permissions_claims() {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_static(
            "Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0ZW5hbnRfaWQiOiJ0X2RlbW8iLCJzdWIiOiJ1X2RlbW8iLCJwZXJtaXNzaW9ucyI6WyJvcHMucmVhZCIsImF1ZGl0LnJlYWQiXSwic2NvcGUiOiJtZWRpYS53cml0ZSJ9.",
        ),
    );

    let auth = resolve_auth_context(&headers).expect("bearer token should resolve");

    assert!(auth.has_permission("ops.read"));
    assert!(auth.has_permission("audit.read"));
    assert!(auth.has_permission("media.write"));
    assert!(!auth.has_permission("ops.write"));
}

#[test]
fn test_resolve_bearer_auth_context_rejects_trusted_headers_without_authorization() {
    let mut headers = HeaderMap::new();
    headers.insert("x-tenant-id", HeaderValue::from_static("t_demo"));
    headers.insert("x-user-id", HeaderValue::from_static("u_demo"));

    let error = resolve_bearer_auth_context(&headers)
        .expect_err("bearer-only auth context should reject trusted headers fallback");

    assert_eq!(error.code(), "auth_context_missing");
}

#[test]
fn test_resolve_public_bearer_auth_context_rejects_unsigned_tokens_when_secret_is_configured() {
    let _guard = configure_public_bearer_secret();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_static(
            "Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0ZW5hbnRfaWQiOiJ0X2RlbW8iLCJzdWIiOiJ1X2RlbW8ifQ.",
        ),
    );

    let error = resolve_public_bearer_auth_context(&headers)
        .expect_err("public bearer auth should reject unsigned tokens");

    assert_eq!(error.code(), "jwt_algorithm_invalid");
}

#[test]
fn test_resolve_public_bearer_auth_context_accepts_unexpired_signed_token() {
    let _guard = configure_public_bearer_secret();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_secs();
    let token = im_auth_context::encode_hs256_bearer_token(
        &serde_json::json!({
            "tenant_id": "t_demo",
            "sub": "u_demo",
            "exp": now + 300
        }),
        TEST_PUBLIC_SECRET,
    )
    .expect("signed token should encode");
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(format!("Bearer {token}").as_str())
            .expect("authorization header should be valid"),
    );

    let auth = resolve_public_bearer_auth_context(&headers)
        .expect("unexpired signed token should pass public bearer verification");
    assert_eq!(auth.tenant_id, "t_demo");
    assert_eq!(auth.actor_id, "u_demo");
}

#[test]
fn test_resolve_public_bearer_auth_context_rejects_expired_signed_token() {
    let _guard = configure_public_bearer_secret();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_secs();
    let token = im_auth_context::encode_hs256_bearer_token(
        &serde_json::json!({
            "tenant_id": "t_demo",
            "sub": "u_demo",
            "exp": now.saturating_sub(120)
        }),
        TEST_PUBLIC_SECRET,
    )
    .expect("signed token should encode");
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(format!("Bearer {token}").as_str())
            .expect("authorization header should be valid"),
    );

    let error = resolve_public_bearer_auth_context(&headers)
        .expect_err("expired signed token should fail public bearer verification");
    assert_eq!(error.code(), "jwt_expired");
}

#[test]
fn test_resolve_public_bearer_auth_context_rejects_not_yet_valid_signed_token() {
    let _guard = configure_public_bearer_secret();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_secs();
    let token = im_auth_context::encode_hs256_bearer_token(
        &serde_json::json!({
            "tenant_id": "t_demo",
            "sub": "u_demo",
            "nbf": now + 300
        }),
        TEST_PUBLIC_SECRET,
    )
    .expect("signed token should encode");
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(format!("Bearer {token}").as_str())
            .expect("authorization header should be valid"),
    );

    let error = resolve_public_bearer_auth_context(&headers)
        .expect_err("token with future nbf should fail public bearer verification");
    assert_eq!(error.code(), "jwt_not_yet_valid");
}

#[test]
fn test_resolve_public_bearer_auth_context_rejects_signed_token_with_future_iat() {
    let _guard = configure_public_bearer_secret();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_secs();
    let token = im_auth_context::encode_hs256_bearer_token(
        &serde_json::json!({
            "tenant_id": "t_demo",
            "sub": "u_demo",
            "iat": now + 300
        }),
        TEST_PUBLIC_SECRET,
    )
    .expect("signed token should encode");
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(format!("Bearer {token}").as_str())
            .expect("authorization header should be valid"),
    );

    let error = resolve_public_bearer_auth_context(&headers)
        .expect_err("token with future iat should fail public bearer verification");
    assert_eq!(error.code(), "jwt_issued_at_invalid");
}
