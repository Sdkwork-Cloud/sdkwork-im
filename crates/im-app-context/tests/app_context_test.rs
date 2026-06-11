use axum::http::{HeaderMap, HeaderValue, header};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use im_app_context::{
    AppContextSignatureConfig, DualTokenRequestBuilderExt, build_dual_token_headers_for_context,
    local_service_app_context, resolve_app_context, resolve_app_context_for_request,
    resolve_app_context_with_signature_config,
};
use serde_json::{Value, json};

fn local_token(claims: Value) -> String {
    let header = URL_SAFE_NO_PAD.encode(r#"{"alg":"none","typ":"JWT"}"#);
    let payload = URL_SAFE_NO_PAD.encode(claims.to_string());
    format!("{header}.{payload}.local")
}

fn token_headers() -> HeaderMap {
    let claims = json!({
        "tenant_id": "t_demo",
        "organization_id": "o_demo",
        "login_scope": "ORGANIZATION",
        "user_id": "u_demo",
        "session_id": "as_demo",
        "device_id": "d_demo",
        "app_id": "craw-chat",
        "environment": "dev",
        "deployment_mode": "private",
        "auth_level": "password",
        "actor_id": "u_demo",
        "actor_kind": "user",
        "permission_scope": ["ops.read", "audit.*", "media.write"],
        "data_scope": ["tenant"]
    });
    let token = local_token(claims);
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(format!("Bearer {token}").as_str()).expect("auth header"),
    );
    headers.insert(
        "Access-Token",
        HeaderValue::from_str(token.as_str()).expect("access token header"),
    );
    headers
}

#[test]
fn test_build_dual_token_headers_for_context_emits_only_dual_token_credentials() {
    let context = local_service_app_context(
        "t_demo",
        "u_demo",
        "user",
        Some("d_demo"),
        ["chat.write", "chat.read"],
    );
    let headers = build_dual_token_headers_for_context(&context, ["chat.write"]);

    assert!(headers.contains_key(header::AUTHORIZATION));
    assert!(headers.contains_key("Access-Token"));
    assert_eq!(headers.len(), 2);
}

#[test]
fn test_dual_token_permission_scope_builder_splits_comma_and_whitespace() {
    let request = axum::http::Request::builder()
        .with_dual_token_context("t_demo", "u_demo", "user", Some("d_demo"), ["chat.read"])
        .with_dual_token_permission_scope("ops.read, audit.*\nmedia.write")
        .body(())
        .expect("request should build");

    let context = resolve_app_context(request.headers()).expect("app context should resolve");

    assert!(context.has_permission("ops.read"));
    assert!(context.has_permission("audit.write"));
    assert!(context.has_permission("media.write"));
    assert!(!context.has_permission("chat.read"));
}

#[test]
fn test_dual_token_builder_accepts_owned_string_values() {
    let request = axum::http::Request::builder()
        .with_dual_token_context("t_demo", "u_demo", "user", None, ["chat.read"])
        .with_dual_token_session(format!("s_{}", "owned"))
        .with_dual_token_device(format!("d_{}", "owned"))
        .with_dual_token_permission_scope("chat.write".to_owned())
        .body(())
        .expect("request should build");

    let context = resolve_app_context(request.headers()).expect("app context should resolve");

    assert_eq!(context.session_id.as_deref(), Some("s_owned"));
    assert_eq!(context.device_id.as_deref(), Some("d_owned"));
    assert!(context.has_permission("chat.write"));
}

#[test]
fn test_resolve_app_context_uses_dual_token_claims() {
    let headers = token_headers();

    let context = resolve_app_context(&headers).expect("app context should resolve");

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
fn test_signature_config_compatibility_resolves_dual_token_claims() {
    let headers = token_headers();

    let context = resolve_app_context_with_signature_config(
        &headers,
        AppContextSignatureConfig {
            require_signature: true,
            shared_secret: Some("gateway-signing-secret".to_owned()),
        },
    )
    .expect("legacy signature config callers should still resolve dual token context");

    assert_eq!(context.tenant_id, "t_demo");
    assert_eq!(context.user_id, "u_demo");
    assert_eq!(context.session_id.as_deref(), Some("as_demo"));
}

#[test]
fn test_resolve_app_context_for_request_exposes_appbase_context() {
    let headers = token_headers();

    let resolved =
        resolve_app_context_for_request(&headers, "/app/v3/api/messages", "POST").expect("context");

    assert_eq!(resolved.app_request_context.path, "/app/v3/api/messages");
    assert_eq!(resolved.app_request_context.method, "POST");
    assert!(resolved.app_request_context.auth_token_present);
    assert!(resolved.app_request_context.access_token_present);
    let principal = resolved
        .app_request_context
        .principal
        .as_ref()
        .expect("principal");
    assert_eq!(principal.tenant_id, "t_demo");
    assert_eq!(principal.organization_id.as_deref(), Some("o_demo"));
    assert_eq!(principal.user_id, "u_demo");
    assert_eq!(principal.app_id, "craw-chat");
}

#[test]
fn test_resolve_app_context_rejects_missing_access_token() {
    let mut headers = token_headers();
    headers.remove("Access-Token");

    let error = resolve_app_context(&headers).expect_err("access token must be required");

    assert_eq!(error.code(), "app_context_missing");
    assert!(error.message().contains("Access-Token"));
}

#[test]
fn test_resolve_app_context_rejects_mismatched_user() {
    let mut headers = HeaderMap::new();
    let auth = local_token(json!({
        "tenant_id": "t_demo",
        "user_id": "u_auth",
        "session_id": "as_demo",
        "app_id": "craw-chat"
    }));
    let access = local_token(json!({
        "tenant_id": "t_demo",
        "user_id": "u_access",
        "session_id": "as_demo",
        "app_id": "craw-chat"
    }));
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(format!("Bearer {auth}").as_str()).expect("auth header"),
    );
    headers.insert(
        "Access-Token",
        HeaderValue::from_str(access.as_str()).expect("access token header"),
    );

    let error = resolve_app_context(&headers).expect_err("mismatch must fail");

    assert_eq!(error.code(), "app_context_invalid");
    assert!(error.message().contains("user_id"));
}

#[test]
fn test_app_context_projects_ccp_authority_fields() {
    let headers = token_headers();

    let context = resolve_app_context(&headers).expect("app context should resolve");
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
    let headers = token_headers();

    let context = resolve_app_context(&headers).expect("app context should resolve");

    assert!(context.has_permission("ops.read"));
    assert!(context.has_permission("audit.read"));
    assert!(context.has_permission("media.write"));
    assert!(!context.has_permission("ops.write"));
}
