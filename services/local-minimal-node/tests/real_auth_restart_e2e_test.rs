use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_auth_context::PUBLIC_BEARER_HS256_SECRET_ENV;
use serde_json::{Value, json};
use tokio::sync::{Mutex, MutexGuard};
use tokio::time::timeout;
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
    std::env::temp_dir().join(format!("craw_chat_real_auth_restart_runtime_{unique}"))
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

async fn login(
    app: &axum::Router,
    login_id: &str,
    password: &str,
    device_id: &str,
    session_id: &str,
) -> Value {
    let response = post_json(
        app,
        "/api/v1/auth/login",
        json!({
            "tenantId": "t_demo",
            "login": login_id,
            "password": password,
            "clientKind": "im_user",
            "deviceId": device_id,
            "sessionId": session_id
        }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
    read_json(response).await
}

#[tokio::test]
async fn test_restarted_public_app_keeps_real_auth_message_writes_healthy() {
    let (_guard, _secret) = configure_public_bearer_secret().await;
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_before = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());
    let owner_before = login(&app_before, "u_owner", "Owner#2026", "d_owner_before", "s_owner_before").await;
    let owner_before_bearer = owner_before["accessToken"]
        .as_str()
        .expect("owner login should return access token")
        .to_owned();

    let create_before = post_json_with_bearer(
        &app_before,
        "/api/v1/conversations",
        owner_before_bearer.as_str(),
        json!({
            "conversationId": "c_real_auth_restart_demo",
            "conversationType": "group"
        }),
    )
    .await;
    assert_eq!(create_before.status(), StatusCode::OK);

    let add_before = post_json_with_bearer(
        &app_before,
        "/api/v1/conversations/c_real_auth_restart_demo/members/add",
        owner_before_bearer.as_str(),
        json!({
            "principalId": "u_guest",
            "principalKind": "user",
            "role": "member"
        }),
    )
    .await;
    assert_eq!(add_before.status(), StatusCode::OK);

    let first_message = post_json_with_bearer(
        &app_before,
        "/api/v1/conversations/c_real_auth_restart_demo/messages",
        owner_before_bearer.as_str(),
        json!({
            "clientMsgId": "restart_before_msg",
            "summary": "before restart",
            "text": "before restart"
        }),
    )
    .await;
    assert_eq!(first_message.status(), StatusCode::OK);

    let app_after = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());
    let owner_after = login(&app_after, "u_owner", "Owner#2026", "d_owner_after", "s_owner_after").await;
    let owner_after_bearer = owner_after["accessToken"]
        .as_str()
        .expect("owner login after restart should return access token")
        .to_owned();

    let second_message = timeout(
        Duration::from_secs(5),
        post_json_with_bearer(
            &app_after,
            "/api/v1/conversations/c_real_auth_restart_demo/messages",
            owner_after_bearer.as_str(),
            json!({
                "clientMsgId": "restart_after_msg",
                "summary": "after restart",
                "text": "after restart"
            }),
        ),
    )
    .await
    .expect("send-message after rebuilding runtime should not hang");
    assert_eq!(second_message.status(), StatusCode::OK);

    let _ = fs::remove_dir_all(runtime_dir);
}
