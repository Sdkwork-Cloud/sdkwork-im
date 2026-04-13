use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_auth_context::PUBLIC_BEARER_HS256_SECRET_ENV;
use serde_json::{Value, json};
use tokio::sync::{Mutex, MutexGuard};
use tower::ServiceExt;

const TEST_PUBLIC_SECRET: &str = "public-test-secret";

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

async fn post_json(
    app: &axum::Router,
    path: &str,
    payload: Value,
) -> axum::response::Response {
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
    assert!(runtime_dir.join("state").join("auth-accounts.json").exists());
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

    let portal_login = post_json(
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
    assert_eq!(portal_login.status(), StatusCode::OK);
    let portal_body = read_json(portal_login).await;
    assert_eq!(portal_body["user"]["clientKind"], "portal_operator");
    assert!(portal_body["workspace"].is_object());

    let portal_access_token = portal_body["accessToken"]
        .as_str()
        .expect("portal access token should be present");
    let workspace =
        get_json_with_bearer(&app, "/api/v1/portal/workspace", portal_access_token).await;
    assert_eq!(workspace.status(), StatusCode::OK);
    let workspace_body = read_json(workspace).await;
    assert_eq!(workspace_body["slug"], "nebula-commerce-im");

    let dashboard =
        get_json_with_bearer(&app, "/api/v1/portal/dashboard", portal_access_token).await;
    assert_eq!(dashboard.status(), StatusCode::OK);
    let dashboard_body = read_json(dashboard).await;
    assert!(dashboard_body["hero"]["title"].is_string());

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
    assert_eq!(summaries, vec!["rtc.invite", "rtc.offer", "rtc.accept", "rtc.end"]);

    let _ = fs::remove_dir_all(runtime_dir);
}
