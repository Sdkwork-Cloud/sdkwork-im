use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use hex::encode as hex_encode;
use http_body_util::BodyExt;
use im_auth_context::PUBLIC_BEARER_HS256_SECRET_ENV;
use pbkdf2::pbkdf2_hmac_array;
use serde_json::{Value, json};
use sha2::Sha256;
use tokio::sync::{Mutex, MutexGuard};
use tower::ServiceExt;

const TEST_PUBLIC_SECRET: &str = "public-test-secret";
const TEST_PASSWORD_ITERATIONS: u32 = 120_000;

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

async fn configure_public_bearer_secret() -> (MutexGuard<'static, ()>, ScopedEnvVar) {
    let guard = public_auth_guard().await;
    let scoped = ScopedEnvVar::set(PUBLIC_BEARER_HS256_SECRET_ENV, TEST_PUBLIC_SECRET);
    (guard, scoped)
}

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_real_auth_runtime_{unique}"))
}

async fn read_json(response: axum::response::Response) -> Value {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    serde_json::from_slice(&bytes).expect("response body should be valid json")
}

async fn post_json(app: &axum::Router, path: &str, payload: Value) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(path)
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&payload).expect("payload should serialize"),
                ))
                .expect("request should build"),
        )
        .await
        .expect("route should return response")
}

async fn post_json_with_bearer(
    app: &axum::Router,
    path: &str,
    bearer: &str,
    payload: Value,
) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(path)
                .header("authorization", format!("Bearer {bearer}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&payload).expect("payload should serialize"),
                ))
                .expect("request should build"),
        )
        .await
        .expect("route should return response")
}

async fn get_json(app: &axum::Router, path: &str) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::builder()
                .uri(path)
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("route should return response")
}

async fn get_json_with_bearer(
    app: &axum::Router,
    path: &str,
    bearer: &str,
) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::builder()
                .uri(path)
                .header("authorization", format!("Bearer {bearer}"))
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("route should return response")
}

fn auth_accounts_path(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("state").join("auth-accounts.json")
}

fn auth_refresh_sessions_path(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("state").join("auth-refresh-sessions.json")
}

fn auth_sidecar_paths(runtime_dir: &Path, prefix: &str) -> Vec<PathBuf> {
    let state_dir = runtime_dir.join("state");
    let Ok(entries) = fs::read_dir(&state_dir) else {
        return Vec::new();
    };

    let mut paths = entries
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| {
            path.file_name()
                .and_then(|value| value.to_str())
                .map(|name| name.starts_with(prefix))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

fn read_auth_accounts(runtime_dir: &Path) -> Vec<Value> {
    let path = auth_accounts_path(runtime_dir);
    serde_json::from_str(
        &fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("auth accounts file should exist: {}", path.display())),
    )
    .expect("auth accounts json should parse")
}

fn write_auth_accounts(runtime_dir: &Path, accounts: &[Value]) {
    let path = auth_accounts_path(runtime_dir);
    fs::write(
        &path,
        serde_json::to_string_pretty(accounts).expect("auth accounts should serialize"),
    )
    .unwrap_or_else(|_| panic!("auth accounts file should be writable: {}", path.display()));
}

fn read_auth_refresh_sessions(runtime_dir: &Path) -> Vec<Value> {
    let path = auth_refresh_sessions_path(runtime_dir);
    serde_json::from_str(&fs::read_to_string(&path).unwrap_or_else(|_| {
        panic!(
            "auth refresh sessions file should exist: {}",
            path.display()
        )
    }))
    .expect("auth refresh sessions json should parse")
}

fn write_auth_refresh_sessions(runtime_dir: &Path, sessions: &[Value]) {
    let path = auth_refresh_sessions_path(runtime_dir);
    fs::write(
        &path,
        serde_json::to_string_pretty(sessions).expect("auth refresh sessions should serialize"),
    )
    .unwrap_or_else(|_| {
        panic!(
            "auth refresh sessions should be writable: {}",
            path.display()
        )
    });
}

fn derive_password_hash(password: &str, salt: &[u8], iterations: u32) -> String {
    let derived = pbkdf2_hmac_array::<Sha256, 32>(password.as_bytes(), salt, iterations);
    hex_encode(derived)
}

fn provision_portal_operator_account(runtime_dir: &Path, login: &str, password: &str) {
    provision_portal_operator_account_for_tenant(runtime_dir, "t_demo", login, password);
}

fn provision_portal_operator_account_for_tenant(
    runtime_dir: &Path,
    tenant_id: &str,
    login: &str,
    password: &str,
) {
    let mut accounts = read_auth_accounts(runtime_dir);
    let salt = [0x42_u8; 16];

    accounts.push(json!({
        "tenantId": tenant_id,
        "accountId": format!("acct_{login}"),
        "login": login,
        "clientKind": "portal_operator",
        "actorId": login,
        "actorKind": "user",
        "name": "Provisioned Portal Operator",
        "role": "Tenant Operations Lead",
        "email": format!("{login}@nebula-commerce.example"),
        "passwordHash": derive_password_hash(password, &salt, TEST_PASSWORD_ITERATIONS),
        "passwordSalt": hex_encode(salt),
        "passwordIterations": TEST_PASSWORD_ITERATIONS,
        "permissions": ["portal.access", "portal.read", "ops.read", "audit.read"],
        "disabled": false
    }));

    write_auth_accounts(runtime_dir, &accounts);
}

#[tokio::test]
async fn test_bootstrap_prunes_expired_refresh_sessions_from_runtime_store() {
    let (_guard, _secret) = configure_public_bearer_secret().await;
    let runtime_dir = unique_runtime_dir();
    let state_dir = runtime_dir.join("state");
    fs::create_dir_all(&state_dir).expect("runtime state dir should be created");
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_secs();

    write_auth_refresh_sessions(
        &runtime_dir,
        &[
            json!({
                "refreshToken": "rt_expired_bootstrap",
                "tenantId": "t_demo",
                "accountId": "acct_demo_guest",
                "actorId": "u_guest",
                "clientKind": "im_user",
                "sessionId": "s_expired_bootstrap",
                "deviceId": "d_expired_bootstrap",
                "expiresAt": now.saturating_sub(30)
            }),
            json!({
                "refreshToken": "rt_active_bootstrap",
                "tenantId": "t_demo",
                "accountId": "acct_demo_guest",
                "actorId": "u_guest",
                "clientKind": "im_user",
                "sessionId": "s_active_bootstrap",
                "deviceId": "d_active_bootstrap",
                "expiresAt": now + 300
            }),
        ],
    );

    let _app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());
    let sessions = read_auth_refresh_sessions(&runtime_dir);
    let refresh_tokens = sessions
        .iter()
        .filter_map(|session| session["refreshToken"].as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        refresh_tokens,
        vec!["rt_active_bootstrap"],
        "bootstrap should evict expired refresh sessions and preserve active ones"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_bootstrap_quarantines_invalid_auth_accounts_before_reseeding_defaults() {
    let (_guard, _secret) = configure_public_bearer_secret().await;
    let runtime_dir = unique_runtime_dir();
    let state_dir = runtime_dir.join("state");
    fs::create_dir_all(&state_dir).expect("runtime state dir should be created");
    let accounts_path = auth_accounts_path(&runtime_dir);
    fs::write(&accounts_path, "{invalid-auth-accounts")
        .expect("invalid auth accounts fixture should be written");

    let _app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());

    let accounts = read_auth_accounts(&runtime_dir);
    assert!(
        accounts.iter().any(|account| account["login"] == "u_guest"),
        "bootstrap should reseed default auth accounts after quarantining invalid content"
    );

    let invalid_backups = auth_sidecar_paths(&runtime_dir, "auth-accounts.json.invalid-");
    assert_eq!(
        invalid_backups.len(),
        1,
        "bootstrap should quarantine exactly one invalid auth accounts file"
    );
    assert_eq!(
        fs::read_to_string(&invalid_backups[0]).expect("invalid auth accounts backup should exist"),
        "{invalid-auth-accounts"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_bootstrap_recovers_refresh_sessions_from_pending_tmp_file() {
    let (_guard, _secret) = configure_public_bearer_secret().await;
    let runtime_dir = unique_runtime_dir();
    let state_dir = runtime_dir.join("state");
    fs::create_dir_all(&state_dir).expect("runtime state dir should be created");
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_secs();
    let pending_tmp_path = state_dir.join("auth-refresh-sessions.json.tmp");
    fs::write(
        &pending_tmp_path,
        serde_json::to_string_pretty(&vec![json!({
            "refreshToken": "rt_pending_tmp",
            "tenantId": "t_demo",
            "accountId": "acct_demo_guest",
            "actorId": "u_guest",
            "clientKind": "im_user",
            "sessionId": "s_pending_tmp",
            "deviceId": "d_pending_tmp",
            "expiresAt": now + 300
        })])
        .expect("pending tmp refresh sessions should serialize"),
    )
    .expect("pending tmp refresh sessions should be written");

    let _app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());

    let sessions = read_auth_refresh_sessions(&runtime_dir);
    let refresh_tokens = sessions
        .iter()
        .filter_map(|session| session["refreshToken"].as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        refresh_tokens,
        vec!["rt_pending_tmp"],
        "bootstrap should promote a pending tmp refresh session file into the live auth store"
    );
    assert!(
        !pending_tmp_path.exists(),
        "bootstrap should clean up the pending tmp file after promotion"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_seeded_im_user_can_log_in_refresh_and_fetch_me_profile() {
    let (_guard, _secret) = configure_public_bearer_secret().await;
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());

    let login = post_json(
        &app,
        "/api/v1/auth/login",
        json!({
            "tenantId": "t_demo",
            "login": "u_guest",
            "password": "Guest#2026",
            "clientKind": "im_user",
            "deviceId": "d_guest",
            "sessionId": "s_guest"
        }),
    )
    .await;

    assert_eq!(login.status(), StatusCode::OK);
    let login_body = read_json(login).await;
    assert_eq!(login_body["user"]["id"], "u_guest");
    assert_eq!(login_body["user"]["clientKind"], "im_user");
    assert!(login_body["accessToken"].as_str().is_some());
    assert!(login_body["refreshToken"].as_str().is_some());
    assert!(
        runtime_dir
            .join("state")
            .join("auth-accounts.json")
            .exists()
    );
    assert!(
        runtime_dir
            .join("state")
            .join("auth-refresh-sessions.json")
            .exists()
    );

    let access_token = login_body["accessToken"]
        .as_str()
        .expect("access token should be present");
    let refresh_token = login_body["refreshToken"]
        .as_str()
        .expect("refresh token should be present")
        .to_owned();

    let me = get_json_with_bearer(&app, "/api/v1/auth/me", access_token).await;
    assert_eq!(me.status(), StatusCode::OK);
    let me_body = read_json(me).await;
    assert_eq!(me_body["tenantId"], "t_demo");
    assert_eq!(me_body["user"]["id"], "u_guest");
    assert_eq!(me_body["user"]["actorKind"], "user");
    assert_eq!(me_body["workspace"], Value::Null);

    let refreshed = post_json(
        &app,
        "/api/v1/auth/refresh",
        json!({
            "refreshToken": refresh_token,
            "deviceId": "d_guest",
            "sessionId": "s_guest"
        }),
    )
    .await;
    assert_eq!(refreshed.status(), StatusCode::OK);
    let refreshed_body = read_json(refreshed).await;
    let rotated_refresh = refreshed_body["refreshToken"]
        .as_str()
        .expect("rotated refresh token should be present");
    assert_ne!(
        rotated_refresh,
        login_body["refreshToken"]
            .as_str()
            .expect("original refresh token should be present")
    );

    let stale_refresh = post_json(
        &app,
        "/api/v1/auth/refresh",
        json!({
            "refreshToken": login_body["refreshToken"],
            "deviceId": "d_guest",
            "sessionId": "s_guest"
        }),
    )
    .await;
    assert_eq!(stale_refresh.status(), StatusCode::UNAUTHORIZED);
    let stale_refresh_body = read_json(stale_refresh).await;
    assert_eq!(stale_refresh_body["code"], "auth_refresh_invalid");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_refresh_rejects_disabled_account_loaded_from_runtime_store() {
    let (_guard, _secret) = configure_public_bearer_secret().await;
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());

    let login = post_json(
        &app,
        "/api/v1/auth/login",
        json!({
            "tenantId": "t_demo",
            "login": "u_guest",
            "password": "Guest#2026",
            "clientKind": "im_user",
            "deviceId": "d_guest",
            "sessionId": "s_guest"
        }),
    )
    .await;
    assert_eq!(login.status(), StatusCode::OK);
    let login_body = read_json(login).await;
    let refresh_token = login_body["refreshToken"]
        .as_str()
        .expect("refresh token should be present")
        .to_owned();

    let mut accounts = read_auth_accounts(&runtime_dir);
    let account = accounts
        .iter_mut()
        .find(|candidate| candidate["login"] == "u_guest" && candidate["clientKind"] == "im_user")
        .expect("seeded guest account should exist");
    account["disabled"] = Value::Bool(true);
    write_auth_accounts(&runtime_dir, &accounts);

    let reloaded_app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());
    let refreshed = post_json(
        &reloaded_app,
        "/api/v1/auth/refresh",
        json!({
            "refreshToken": refresh_token,
            "deviceId": "d_guest",
            "sessionId": "s_guest"
        }),
    )
    .await;

    assert_eq!(refreshed.status(), StatusCode::FORBIDDEN);
    let refreshed_body = read_json(refreshed).await;
    assert_eq!(refreshed_body["code"], "auth_account_disabled");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_me_prefers_token_client_kind_when_actor_ids_collide() {
    let (_guard, _secret) = configure_public_bearer_secret().await;
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let _seeded_app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());

    let mut accounts = read_auth_accounts(&runtime_dir);
    let guest_account = accounts
        .iter()
        .find(|candidate| candidate["login"] == "u_guest" && candidate["clientKind"] == "im_user")
        .cloned()
        .expect("seeded guest account should exist");
    let mut shadow_account = guest_account;
    shadow_account["accountId"] = Value::String("acct_shadow_guest_portal".into());
    shadow_account["login"] = Value::String("u_guest_portal_shadow".into());
    shadow_account["clientKind"] = Value::String("portal_operator".into());
    shadow_account["name"] = Value::String("Guest Portal Shadow".into());
    shadow_account["role"] = Value::String("Shadow Portal Operator".into());
    shadow_account["email"] = Value::String("guest-shadow@nebula-commerce.example".into());
    shadow_account["permissions"] = json!(["portal.access", "portal.read", "audit.read"]);
    accounts.insert(0, shadow_account);
    write_auth_accounts(&runtime_dir, &accounts);

    let app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());
    let login = post_json(
        &app,
        "/api/v1/auth/login",
        json!({
            "tenantId": "t_demo",
            "login": "u_guest",
            "password": "Guest#2026",
            "clientKind": "im_user",
            "deviceId": "d_guest",
            "sessionId": "s_guest"
        }),
    )
    .await;
    assert_eq!(login.status(), StatusCode::OK);
    let login_body = read_json(login).await;
    let access_token = login_body["accessToken"]
        .as_str()
        .expect("access token should be present")
        .to_owned();

    let me = get_json_with_bearer(&app, "/api/v1/auth/me", access_token.as_str()).await;
    assert_eq!(me.status(), StatusCode::OK);
    let me_body = read_json(me).await;
    assert_eq!(me_body["user"]["id"], "u_guest");
    assert_eq!(me_body["user"]["clientKind"], "im_user");
    assert_eq!(me_body["workspace"], Value::Null);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_login_rejects_invalid_password() {
    let (_guard, _secret) = configure_public_bearer_secret().await;
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());

    let response = post_json(
        &app,
        "/api/v1/auth/login",
        json!({
            "tenantId": "t_demo",
            "login": "u_guest",
            "password": "wrong-password",
            "clientKind": "im_user"
        }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = read_json(response).await;
    assert_eq!(body["code"], "auth_login_invalid");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_login_rejects_oversized_device_id() {
    let (_guard, _secret) = configure_public_bearer_secret().await;
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());

    let response = post_json(
        &app,
        "/api/v1/auth/login",
        json!({
            "tenantId": "t_demo",
            "login": "u_guest",
            "password": "Guest#2026",
            "clientKind": "im_user",
            "deviceId": "d".repeat(257),
            "sessionId": "s_guest"
        }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = read_json(response).await;
    assert_eq!(body["code"], "payload_too_large");
    assert!(
        body["message"]
            .as_str()
            .expect("message should be present")
            .contains("deviceId")
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_refresh_rejects_oversized_session_id() {
    let (_guard, _secret) = configure_public_bearer_secret().await;
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());

    let login = post_json(
        &app,
        "/api/v1/auth/login",
        json!({
            "tenantId": "t_demo",
            "login": "u_guest",
            "password": "Guest#2026",
            "clientKind": "im_user",
            "deviceId": "d_guest",
            "sessionId": "s_guest"
        }),
    )
    .await;
    assert_eq!(login.status(), StatusCode::OK);
    let login_body = read_json(login).await;
    let refresh_token = login_body["refreshToken"]
        .as_str()
        .expect("refresh token should be present")
        .to_owned();

    let refreshed = post_json(
        &app,
        "/api/v1/auth/refresh",
        json!({
            "refreshToken": refresh_token,
            "deviceId": "d_guest",
            "sessionId": "s".repeat(257)
        }),
    )
    .await;

    assert_eq!(refreshed.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let refreshed_body = read_json(refreshed).await;
    assert_eq!(refreshed_body["code"], "payload_too_large");
    assert!(
        refreshed_body["message"]
            .as_str()
            .expect("message should be present")
            .contains("sessionId")
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_portal_public_snapshots_are_open_but_workspace_requires_operator_token() {
    let (_guard, _secret) = configure_public_bearer_secret().await;
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());

    let home = get_json(&app, "/api/v1/portal/home").await;
    assert_eq!(home.status(), StatusCode::OK);
    let home_body = read_json(home).await;
    assert!(home_body["hero"]["title"].is_string());

    let auth = get_json(&app, "/api/v1/portal/auth").await;
    assert_eq!(auth.status(), StatusCode::OK);
    let auth_body = read_json(auth).await;
    assert!(auth_body["title"].is_string());
    let auth_body_text = auth_body.to_string();
    assert!(
        !auth_body_text.contains("ops_demo"),
        "portal auth snapshot must not leak demo operator logins: {auth_body_text}"
    );
    assert!(
        !auth_body_text.contains("Portal#2026"),
        "portal auth snapshot must not leak demo password hints: {auth_body_text}"
    );
    assert!(
        !auth_body_text.contains("seeded operator account"),
        "portal auth snapshot must not instruct users to sign in with seeded demo credentials: {auth_body_text}"
    );
    assert!(
        auth_body.get("defaultTenantId").is_none(),
        "portal auth snapshot must not prefill a tenant id: {auth_body_text}"
    );
    assert!(
        auth_body.get("defaultLogin").is_none(),
        "portal auth snapshot must not prefill a login: {auth_body_text}"
    );
    assert!(
        auth_body.get("passwordHint").is_none(),
        "portal auth snapshot must not publish password hints: {auth_body_text}"
    );

    let unauthenticated_workspace = get_json(&app, "/api/v1/portal/workspace").await;
    assert_eq!(unauthenticated_workspace.status(), StatusCode::UNAUTHORIZED);

    let im_login = post_json(
        &app,
        "/api/v1/auth/login",
        json!({
            "tenantId": "t_demo",
            "login": "u_guest",
            "password": "Guest#2026",
            "clientKind": "im_user",
            "deviceId": "d_guest",
            "sessionId": "s_guest"
        }),
    )
    .await;
    assert_eq!(im_login.status(), StatusCode::OK);
    let im_body = read_json(im_login).await;
    let im_access_token = im_body["accessToken"]
        .as_str()
        .expect("im access token should be present")
        .to_owned();

    let forbidden_workspace =
        get_json_with_bearer(&app, "/api/v1/portal/workspace", im_access_token.as_str()).await;
    assert_eq!(forbidden_workspace.status(), StatusCode::FORBIDDEN);
    let forbidden_body = read_json(forbidden_workspace).await;
    assert_eq!(forbidden_body["code"], "permission_denied");

    let default_portal_login = post_json(
        &app,
        "/api/v1/auth/login",
        json!({
            "tenantId": "t_demo",
            "login": "ops_demo",
            "password": "Portal#2026",
            "clientKind": "portal_operator",
            "deviceId": "d_portal_demo",
            "sessionId": "s_portal_demo"
        }),
    )
    .await;

    assert_eq!(default_portal_login.status(), StatusCode::UNAUTHORIZED);
    let default_portal_login_body = read_json(default_portal_login).await;
    assert_eq!(default_portal_login_body["code"], "auth_login_invalid");

    provision_portal_operator_account(
        runtime_dir.as_path(),
        "ops_portal",
        "ProvisionedPortal#2026",
    );
    let provisioned_app =
        local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());

    let portal_login = post_json(
        &provisioned_app,
        "/api/v1/auth/login",
        json!({
            "tenantId": "t_demo",
            "login": "ops_portal",
            "password": "ProvisionedPortal#2026",
            "clientKind": "portal_operator",
            "deviceId": "d_portal_demo",
            "sessionId": "s_portal_demo"
        }),
    )
    .await;

    assert_eq!(portal_login.status(), StatusCode::OK);
    let portal_body = read_json(portal_login).await;
    assert_eq!(portal_body["user"]["clientKind"], "portal_operator");
    assert!(portal_body["workspace"].is_object());

    let portal_access_token = portal_body["accessToken"]
        .as_str()
        .expect("portal access token should be present");
    let workspace = get_json_with_bearer(
        &provisioned_app,
        "/api/v1/portal/workspace",
        portal_access_token,
    )
    .await;
    assert_eq!(workspace.status(), StatusCode::OK);
    let workspace_body = read_json(workspace).await;
    assert_eq!(workspace_body["slug"], "nebula-commerce-im");

    let dashboard = get_json_with_bearer(
        &provisioned_app,
        "/api/v1/portal/dashboard",
        portal_access_token,
    )
    .await;
    assert_eq!(dashboard.status(), StatusCode::OK);
    let dashboard_body = read_json(dashboard).await;
    assert!(dashboard_body["hero"]["title"].is_string());

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_portal_workspace_snapshot_tracks_provisioned_operator_tenant() {
    let (_guard, _secret) = configure_public_bearer_secret().await;
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let _ = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());

    provision_portal_operator_account_for_tenant(
        runtime_dir.as_path(),
        "tenant-acme",
        "ops_acme",
        "ProvisionedPortal#2026",
    );
    let app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());

    let portal_login = post_json(
        &app,
        "/api/v1/auth/login",
        json!({
            "tenantId": "tenant-acme",
            "login": "ops_acme",
            "password": "ProvisionedPortal#2026",
            "clientKind": "portal_operator",
            "deviceId": "d_portal_acme",
            "sessionId": "s_portal_acme"
        }),
    )
    .await;

    assert_eq!(portal_login.status(), StatusCode::OK);
    let portal_body = read_json(portal_login).await;
    assert_eq!(portal_body["workspace"]["name"], "Acme Commerce IM");
    assert_eq!(portal_body["workspace"]["slug"], "acme-commerce-im");
    assert_ne!(portal_body["workspace"]["slug"], "nebula-commerce-im");

    let portal_access_token = portal_body["accessToken"]
        .as_str()
        .expect("portal access token should be present");

    let me = get_json_with_bearer(&app, "/api/v1/auth/me", portal_access_token).await;
    assert_eq!(me.status(), StatusCode::OK);
    let me_body = read_json(me).await;
    assert_eq!(me_body["tenantId"], "tenant-acme");
    assert_eq!(me_body["workspace"]["name"], "Acme Commerce IM");
    assert_eq!(me_body["workspace"]["slug"], "acme-commerce-im");

    let workspace =
        get_json_with_bearer(&app, "/api/v1/portal/workspace", portal_access_token).await;
    assert_eq!(workspace.status(), StatusCode::OK);
    let workspace_body = read_json(workspace).await;
    assert_eq!(workspace_body["name"], "Acme Commerce IM");
    assert_eq!(workspace_body["slug"], "acme-commerce-im");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_chat_routes_accept_access_tokens_issued_by_login() {
    let (_guard, _secret) = configure_public_bearer_secret().await;
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());

    let login = post_json(
        &app,
        "/api/v1/auth/login",
        json!({
            "tenantId": "t_demo",
            "login": "u_owner",
            "password": "Owner#2026",
            "clientKind": "im_user",
            "deviceId": "d_owner",
            "sessionId": "s_owner"
        }),
    )
    .await;
    assert_eq!(login.status(), StatusCode::OK);
    let login_body = read_json(login).await;
    let access_token = login_body["accessToken"]
        .as_str()
        .expect("access token should be present")
        .to_owned();

    let create_conversation = post_json_with_bearer(
        &app,
        "/api/v1/conversations",
        access_token.as_str(),
        json!({
            "conversationId": "c_login_token_demo",
            "conversationType": "group"
        }),
    )
    .await;
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_message = post_json_with_bearer(
        &app,
        "/api/v1/conversations/c_login_token_demo/messages",
        access_token.as_str(),
        json!({
            "clientMsgId": "client_login_token_demo",
            "summary": "hello with real login",
            "text": "hello with real login"
        }),
    )
    .await;
    assert_eq!(post_message.status(), StatusCode::OK);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_rtc_routes_accept_real_login_tokens_for_owner_and_guest_flow() {
    let (_guard, _secret) = configure_public_bearer_secret().await;
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());

    let owner_login = post_json(
        &app,
        "/api/v1/auth/login",
        json!({
            "tenantId": "t_demo",
            "login": "u_owner",
            "password": "Owner#2026",
            "clientKind": "im_user",
            "deviceId": "d_owner",
            "sessionId": "s_owner"
        }),
    )
    .await;
    assert_eq!(owner_login.status(), StatusCode::OK);
    let owner_body = read_json(owner_login).await;
    let owner_access_token = owner_body["accessToken"]
        .as_str()
        .expect("owner access token should be present")
        .to_owned();

    let guest_login = post_json(
        &app,
        "/api/v1/auth/login",
        json!({
            "tenantId": "t_demo",
            "login": "u_guest",
            "password": "Guest#2026",
            "clientKind": "im_user",
            "deviceId": "d_guest",
            "sessionId": "s_guest"
        }),
    )
    .await;
    assert_eq!(guest_login.status(), StatusCode::OK);
    let guest_body = read_json(guest_login).await;
    let guest_access_token = guest_body["accessToken"]
        .as_str()
        .expect("guest access token should be present")
        .to_owned();

    let create_conversation = post_json_with_bearer(
        &app,
        "/api/v1/conversations",
        owner_access_token.as_str(),
        json!({
            "conversationId": "c_rtc_login_token_demo",
            "conversationType": "group"
        }),
    )
    .await;
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_guest = post_json_with_bearer(
        &app,
        "/api/v1/conversations/c_rtc_login_token_demo/members/add",
        owner_access_token.as_str(),
        json!({
            "principalId": "u_guest",
            "principalKind": "user",
            "role": "member"
        }),
    )
    .await;
    assert_eq!(add_guest.status(), StatusCode::OK);

    let create_rtc = post_json_with_bearer(
        &app,
        "/api/v1/rtc/sessions",
        owner_access_token.as_str(),
        json!({
            "rtcSessionId": "rtc_login_token_demo",
            "conversationId": "c_rtc_login_token_demo",
            "rtcMode": "voice"
        }),
    )
    .await;
    assert_eq!(create_rtc.status(), StatusCode::OK);

    let invite_rtc = post_json_with_bearer(
        &app,
        "/api/v1/rtc/sessions/rtc_login_token_demo/invite",
        owner_access_token.as_str(),
        json!({
            "signalingStreamId": "st_login_token_demo"
        }),
    )
    .await;
    assert_eq!(invite_rtc.status(), StatusCode::OK);

    let guest_signal = post_json_with_bearer(
        &app,
        "/api/v1/rtc/sessions/rtc_login_token_demo/signals",
        guest_access_token.as_str(),
        json!({
            "signalType": "rtc.offer",
            "schemaRef": "webrtc.offer.v1",
            "payload": "{\"sdp\":\"login-demo\"}"
        }),
    )
    .await;
    assert_eq!(guest_signal.status(), StatusCode::OK);

    let accept_rtc = post_json_with_bearer(
        &app,
        "/api/v1/rtc/sessions/rtc_login_token_demo/accept",
        guest_access_token.as_str(),
        json!({
            "artifactMessageId": "msg_login_rtc_accept"
        }),
    )
    .await;
    assert_eq!(accept_rtc.status(), StatusCode::OK);

    let end_rtc = post_json_with_bearer(
        &app,
        "/api/v1/rtc/sessions/rtc_login_token_demo/end",
        owner_access_token.as_str(),
        json!({
            "artifactMessageId": "msg_login_rtc_end"
        }),
    )
    .await;
    assert_eq!(end_rtc.status(), StatusCode::OK);

    let timeline = get_json_with_bearer(
        &app,
        "/api/v1/conversations/c_rtc_login_token_demo/messages",
        owner_access_token.as_str(),
    )
    .await;
    assert_eq!(timeline.status(), StatusCode::OK);
    let timeline_body = read_json(timeline).await;
    let items = timeline_body["items"]
        .as_array()
        .expect("timeline items should be an array");
    let summaries = items
        .iter()
        .map(|item| item["summary"].as_str().unwrap_or_default())
        .collect::<Vec<_>>();
    assert_eq!(
        summaries,
        vec!["rtc.invite", "rtc.offer", "rtc.accept", "rtc.end"]
    );

    let _ = fs::remove_dir_all(runtime_dir);
}
