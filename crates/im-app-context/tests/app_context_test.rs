use axum::http::{HeaderMap, HeaderValue};
use base64::Engine;
use hmac::{Hmac, Mac};
use im_app_context::{
    AppContextSignatureConfig, resolve_app_context, resolve_app_context_with_signature_config,
};
use sha2::Sha256;

const SIGNED_HEADER_NAMES: &[&str] = &[
    "x-sdkwork-app-id",
    "x-sdkwork-tenant-id",
    "x-sdkwork-organization-id",
    "x-sdkwork-user-id",
    "x-sdkwork-session-id",
    "x-sdkwork-environment",
    "x-sdkwork-deployment-mode",
    "x-sdkwork-auth-level",
    "x-sdkwork-data-scope",
    "x-sdkwork-permission-scope",
    "x-sdkwork-actor-id",
    "x-sdkwork-actor-kind",
    "x-sdkwork-device-id",
];

fn canonicalize_signed_headers(headers: &HeaderMap) -> String {
    SIGNED_HEADER_NAMES
        .iter()
        .map(|name| {
            let value = headers
                .get(*name)
                .and_then(|value| value.to_str().ok())
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .unwrap_or("");
            format!("{name}:{value}")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn sign_headers(headers: &HeaderMap, secret: &str) -> String {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("hmac secret should be valid");
    mac.update(canonicalize_signed_headers(headers).as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes())
}

fn projection_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("x-sdkwork-tenant-id", HeaderValue::from_static("t_demo"));
    headers.insert(
        "x-sdkwork-organization-id",
        HeaderValue::from_static("o_demo"),
    );
    headers.insert("x-sdkwork-user-id", HeaderValue::from_static("u_demo"));
    headers.insert("x-sdkwork-session-id", HeaderValue::from_static("as_demo"));
    headers.insert("x-sdkwork-device-id", HeaderValue::from_static("d_demo"));
    headers.insert("x-sdkwork-app-id", HeaderValue::from_static("craw-chat"));
    headers.insert("x-sdkwork-environment", HeaderValue::from_static("dev"));
    headers.insert("x-sdkwork-auth-level", HeaderValue::from_static("password"));
    headers.insert(
        "x-sdkwork-deployment-mode",
        HeaderValue::from_static("private"),
    );
    headers
}

#[test]
fn test_resolve_app_context_projection_supports_sdkwork_scope_fields() {
    let headers = projection_headers();

    let context = resolve_app_context(&headers).expect("app context projection should resolve");

    assert_eq!(context.tenant_id, "t_demo");
    assert_eq!(context.organization_id.as_deref(), Some("o_demo"));
    assert_eq!(context.user_id, "u_demo");
    assert_eq!(context.actor_id, "u_demo");
    assert_eq!(context.actor_kind, "user");
    assert_eq!(context.session_id.as_deref(), Some("as_demo"));
    assert_eq!(context.device_id.as_deref(), Some("d_demo"));
    assert_eq!(context.app_id.as_deref(), Some("craw-chat"));
    assert_eq!(context.environment.as_deref(), Some("dev"));
    assert_eq!(context.deployment_mode.as_deref(), Some("private"));
    assert_eq!(context.auth_level.as_deref(), Some("password"));
}

#[test]
fn test_resolve_app_context_projection_rejects_missing_user_id() {
    let mut headers = HeaderMap::new();
    headers.insert("x-sdkwork-tenant-id", HeaderValue::from_static("t_demo"));
    headers.insert("x-sdkwork-session-id", HeaderValue::from_static("as_demo"));
    headers.insert("x-sdkwork-app-id", HeaderValue::from_static("craw-chat"));
    headers.insert("x-sdkwork-environment", HeaderValue::from_static("dev"));
    headers.insert(
        "x-sdkwork-deployment-mode",
        HeaderValue::from_static("private"),
    );
    headers.insert("x-sdkwork-auth-level", HeaderValue::from_static("password"));

    let error = resolve_app_context(&headers).expect_err("app context must require user id");

    assert_eq!(error.code(), "app_context_missing");
    assert!(error.message().contains("x-sdkwork-user-id"));
}

#[test]
fn test_resolve_app_context_signature_required_rejects_missing_signature() {
    let headers = projection_headers();

    let error = resolve_app_context_with_signature_config(
        &headers,
        AppContextSignatureConfig {
            require_signature: true,
            shared_secret: Some("demo-secret".to_owned()),
        },
    )
    .expect_err("signature should be required");

    assert_eq!(error.code(), "app_context_invalid");
    assert!(error.message().contains("x-sdkwork-context-signature"));
}

#[test]
fn test_resolve_app_context_signature_required_rejects_invalid_signature() {
    let mut headers = projection_headers();
    headers.insert(
        "x-sdkwork-context-signature",
        HeaderValue::from_static("invalid-signature"),
    );

    let error = resolve_app_context_with_signature_config(
        &headers,
        AppContextSignatureConfig {
            require_signature: true,
            shared_secret: Some("demo-secret".to_owned()),
        },
    )
    .expect_err("invalid signature should be rejected");

    assert_eq!(error.code(), "app_context_invalid");
    assert!(error.message().contains("signature validation failed"));
}

#[test]
fn test_resolve_app_context_signature_required_accepts_valid_signature() {
    let mut headers = projection_headers();
    let signature = sign_headers(&headers, "demo-secret");
    headers.insert(
        "x-sdkwork-context-signature",
        HeaderValue::from_str(signature.as_str()).expect("signature header should be valid"),
    );

    let context = resolve_app_context_with_signature_config(
        &headers,
        AppContextSignatureConfig {
            require_signature: true,
            shared_secret: Some("demo-secret".to_owned()),
        },
    )
    .expect("valid signature should pass");

    assert_eq!(context.tenant_id, "t_demo");
    assert_eq!(context.user_id, "u_demo");
}

#[test]
fn test_app_context_projects_ccp_authority_fields() {
    let mut headers = HeaderMap::new();
    headers.insert("x-sdkwork-tenant-id", HeaderValue::from_static("t_demo"));
    headers.insert("x-sdkwork-user-id", HeaderValue::from_static("u_demo"));
    headers.insert("x-sdkwork-session-id", HeaderValue::from_static("as_demo"));
    headers.insert("x-sdkwork-app-id", HeaderValue::from_static("craw-chat"));
    headers.insert("x-sdkwork-environment", HeaderValue::from_static("dev"));
    headers.insert(
        "x-sdkwork-deployment-mode",
        HeaderValue::from_static("private"),
    );
    headers.insert("x-sdkwork-auth-level", HeaderValue::from_static("password"));
    headers.insert("x-sdkwork-device-id", HeaderValue::from_static("d_demo"));
    headers.insert("x-sdkwork-actor-kind", HeaderValue::from_static("user"));

    let context = resolve_app_context(&headers).expect("app context projection should resolve");
    let authority = context.ccp_authority();

    assert_eq!(authority.tenant_id, "t_demo");
    assert_eq!(authority.actor.actor_id, "u_demo");
    assert_eq!(authority.actor.actor_kind, "user");
    assert_eq!(authority.sender.principal_id, "u_demo");
    assert_eq!(authority.sender.device_id.as_deref(), Some("d_demo"));
    assert_eq!(authority.sender.session_id.as_deref(), Some("as_demo"));
    assert_eq!(authority.sender.sender_id(), "u_demo:d_demo");
}

#[test]
fn test_app_context_permissions_support_exact_and_wildcard_matches() {
    let mut headers = HeaderMap::new();
    headers.insert("x-sdkwork-tenant-id", HeaderValue::from_static("t_demo"));
    headers.insert("x-sdkwork-user-id", HeaderValue::from_static("u_demo"));
    headers.insert("x-sdkwork-session-id", HeaderValue::from_static("as_demo"));
    headers.insert("x-sdkwork-app-id", HeaderValue::from_static("craw-chat"));
    headers.insert("x-sdkwork-environment", HeaderValue::from_static("dev"));
    headers.insert(
        "x-sdkwork-deployment-mode",
        HeaderValue::from_static("private"),
    );
    headers.insert("x-sdkwork-auth-level", HeaderValue::from_static("password"));
    headers.insert(
        "x-sdkwork-permission-scope",
        HeaderValue::from_static("ops.read audit.* media.write"),
    );

    let context = resolve_app_context(&headers).expect("app context projection should resolve");

    assert!(context.has_permission("ops.read"));
    assert!(context.has_permission("audit.read"));
    assert!(context.has_permission("media.write"));
    assert!(!context.has_permission("ops.write"));
}

#[test]
fn test_app_context_ignores_legacy_scope_headers() {
    let mut headers = HeaderMap::new();
    headers.insert("x-sdkwork-tenant-id", HeaderValue::from_static("t_demo"));
    headers.insert("x-sdkwork-user-id", HeaderValue::from_static("u_demo"));
    headers.insert("x-scope", HeaderValue::from_static("tenant.admin"));
    headers.insert("x-scopes", HeaderValue::from_static("*"));

    let context = resolve_app_context(&headers).expect("app context projection should resolve");

    assert!(
        !context.has_permission("notification.write"),
        "permissions must come from sdkwork AppContext projection, not legacy scope headers"
    );
}
