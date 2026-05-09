use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::Json;
use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderMap, Request, StatusCode};
use axum::routing::{get, post};
use axum::{Router, response::IntoResponse};
use http_body_util::BodyExt;
use im_auth_context::{PUBLIC_BEARER_HS256_SECRET_ENV, encode_hs256_bearer_token};
use serde_json::{Value, json};
use tokio::net::TcpListener;
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

    fn remove(name: &'static str) -> Self {
        let previous = std::env::var(name).ok();
        unsafe {
            std::env::remove_var(name);
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

async fn runtime_env_guard() -> MutexGuard<'static, ()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(())).lock().await
}

async fn configure_public_bearer_secret() -> (MutexGuard<'static, ()>, ScopedEnvVar) {
    let guard = runtime_env_guard().await;
    let scoped = ScopedEnvVar::set(PUBLIC_BEARER_HS256_SECRET_ENV, TEST_PUBLIC_SECRET);
    (guard, scoped)
}

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_user_center_auth_runtime_{unique}"))
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

#[derive(Clone, Default)]
struct UpstreamCapture {
    requests: Arc<Mutex<Vec<UpstreamCapturedRequest>>>,
}

#[derive(Clone, Debug)]
struct UpstreamCapturedRequest {
    method: String,
    path: String,
    headers: BTreeMap<String, String>,
    body: Option<Value>,
}

#[derive(Clone)]
struct UpstreamState {
    capture: UpstreamCapture,
    login_payload: Value,
    profile_payload: Value,
}

async fn spawn_mock_user_center_upstream(
    login_payload: Value,
    profile_payload: Value,
) -> (String, UpstreamCapture, tokio::task::JoinHandle<()>) {
    let capture = UpstreamCapture::default();
    let state = UpstreamState {
        capture: capture.clone(),
        login_payload,
        profile_payload,
    };
    let app = Router::new()
        .route(
            "/bridge/api/app/v1/user-center/session/login",
            post(mock_upstream_login),
        )
        .route(
            "/bridge/api/app/v1/user-center/profile",
            get(mock_upstream_profile),
        )
        .with_state(state);
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("mock upstream listener should bind");
    let address = listener
        .local_addr()
        .expect("mock upstream listener should expose address");
    let handle = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("mock upstream server should serve");
    });
    (format!("http://{address}/bridge"), capture, handle)
}

async fn mock_upstream_login(
    headers: HeaderMap,
    State(state): State<UpstreamState>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    capture_upstream_request(
        &state.capture,
        "POST",
        "/bridge/api/app/v1/user-center/session/login",
        &headers,
        Some(payload),
    )
    .await;
    Json(state.login_payload)
}

async fn mock_upstream_profile(
    headers: HeaderMap,
    State(state): State<UpstreamState>,
) -> impl IntoResponse {
    capture_upstream_request(
        &state.capture,
        "GET",
        "/bridge/api/app/v1/user-center/profile",
        &headers,
        None,
    )
    .await;
    Json(state.profile_payload)
}

async fn capture_upstream_request(
    capture: &UpstreamCapture,
    method: &str,
    path: &str,
    headers: &HeaderMap,
    body: Option<Value>,
) {
    let normalized_headers = headers
        .iter()
        .filter_map(|(name, value)| {
            value
                .to_str()
                .ok()
                .map(|value| (name.as_str().to_ascii_lowercase(), value.to_owned()))
        })
        .collect::<BTreeMap<_, _>>();
    capture.requests.lock().await.push(UpstreamCapturedRequest {
        method: method.to_owned(),
        path: path.to_owned(),
        headers: normalized_headers,
        body,
    });
}

fn issue_remote_auth_token() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_secs();
    encode_hs256_bearer_token(
        &json!({
            "tenant_id": "t_demo",
            "sub": "u_remote",
            "actor_kind": "user",
            "sid": "s_remote",
            "did": "d_remote",
            "client_kind": "im_user",
            "permissions": ["conversation.*", "realtime.*", "rtc.*", "media.*", "stream.*"],
            "iss": "craw-chat",
            "aud": "craw-chat-public",
            "iat": now,
            "exp": now + 3600
        }),
        TEST_PUBLIC_SECRET,
    )
    .expect("remote auth token should encode")
}

#[tokio::test]
async fn test_standard_user_center_session_login_alias_returns_dual_tokens_and_profile_view() {
    let (_guard, _secret) = configure_public_bearer_secret().await;
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());

    let login = post_json(
        &app,
        "/api/app/v1/user-center/session/login",
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
    assert!(login_body["authToken"].as_str().is_some());
    assert!(login_body["accessToken"].as_str().is_some());
    assert!(login_body["refreshToken"].as_str().is_some());
    let auth_token = login_body["authToken"]
        .as_str()
        .expect("auth token should be present")
        .to_owned();

    let profile =
        get_json_with_bearer(&app, "/api/app/v1/user-center/profile", auth_token.as_str()).await;
    assert_eq!(profile.status(), StatusCode::OK);
    let profile_body = read_json(profile).await;
    assert_eq!(profile_body["user"]["id"], "u_guest");
    assert_eq!(profile_body["tenantId"], "t_demo");

    let create_conversation = post_json_with_bearer(
        &app,
        "/api/v1/conversations",
        auth_token.as_str(),
        json!({
            "conversationId": "c_user_center_auth_token_demo",
            "conversationType": "group"
        }),
    )
    .await;
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_remote_user_center_mode_without_shared_secret_fails_closed_instead_of_falling_back_local()
 {
    let (_guard, _secret) = configure_public_bearer_secret().await;
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let _mode = ScopedEnvVar::set("SDKWORK_USER_CENTER_MODE", "sdkwork-cloud-app-api");
    let _base_url = ScopedEnvVar::set(
        "SDKWORK_USER_CENTER_APP_API_BASE_URL",
        "https://app-api.sdkwork.local/craw",
    );
    let _provider_key = ScopedEnvVar::set("SDKWORK_USER_CENTER_PROVIDER_KEY", "craw-app-api");
    let _secret_id = ScopedEnvVar::set("SDKWORK_USER_CENTER_SECRET_ID", "secret-501");
    let _shared_secret = ScopedEnvVar::remove("SDKWORK_USER_CENTER_SHARED_SECRET");

    let app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());
    let login = post_json(
        &app,
        "/api/app/v1/user-center/session/login",
        json!({
            "tenantId": "t_demo",
            "login": "u_guest",
            "password": "Guest#2026",
            "clientKind": "im_user"
        }),
    )
    .await;

    assert_eq!(login.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = read_json(login).await;
    assert_eq!(body["code"], "auth_authority_unavailable");
    assert!(
        body["message"]
            .as_str()
            .is_some_and(|message| message.contains("SDKWORK_USER_CENTER_SHARED_SECRET")),
        "fail-closed auth authority error should mention the missing shared secret env. actual body: {body}"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_remote_user_center_mode_proxies_to_upstream_app_api_and_preserves_token_interop() {
    let (_guard, _secret) = configure_public_bearer_secret().await;
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let remote_auth_token = issue_remote_auth_token();
    let remote_access_token = "remote-access-token";
    let remote_refresh_token = "remote-refresh-token";
    let (upstream_base_url, capture, server_handle) = spawn_mock_user_center_upstream(
        json!({
            "authToken": remote_auth_token,
            "accessToken": remote_access_token,
            "refreshToken": remote_refresh_token,
            "expiresAt": 4_102_444_800u64,
            "user": {
                "id": "u_remote",
                "login": "u_remote",
                "name": "Remote Demo",
                "role": "Remote User",
                "email": "remote@example.com",
                "actorKind": "user",
                "clientKind": "im_user",
                "permissions": ["conversation.*", "realtime.*", "rtc.*", "media.*", "stream.*"]
            },
            "workspace": null
        }),
        json!({
            "tenantId": "t_demo",
            "user": {
                "id": "u_remote",
                "login": "u_remote",
                "name": "Remote Demo",
                "role": "Remote User",
                "email": "remote@example.com",
                "actorKind": "user",
                "clientKind": "im_user",
                "permissions": ["conversation.*", "realtime.*", "rtc.*", "media.*", "stream.*"]
            },
            "workspace": null
        }),
    )
    .await;
    let _mode = ScopedEnvVar::set("SDKWORK_USER_CENTER_MODE", "sdkwork-cloud-app-api");
    let _base_url = ScopedEnvVar::set(
        "SDKWORK_USER_CENTER_APP_API_BASE_URL",
        upstream_base_url.as_str(),
    );
    let _provider_key = ScopedEnvVar::set("SDKWORK_USER_CENTER_PROVIDER_KEY", "craw-app-api");
    let _app_id = ScopedEnvVar::set("SDKWORK_USER_CENTER_APP_ID", "craw-chat");
    let _secret_id = ScopedEnvVar::set("SDKWORK_USER_CENTER_SECRET_ID", "secret-501");
    let _shared_secret =
        ScopedEnvVar::set("SDKWORK_USER_CENTER_SHARED_SECRET", "shared-secret-501");

    let app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());
    let login = post_json(
        &app,
        "/api/app/v1/user-center/session/login",
        json!({
            "tenantId": "t_demo",
            "login": "u_remote",
            "password": "Remote#2026",
            "clientKind": "im_user",
            "deviceId": "d_remote",
            "sessionId": "s_remote"
        }),
    )
    .await;
    assert_eq!(login.status(), StatusCode::OK);
    let login_body = read_json(login).await;
    assert_eq!(login_body["authToken"], remote_auth_token);
    assert_eq!(login_body["accessToken"], remote_access_token);
    assert_eq!(login_body["refreshToken"], remote_refresh_token);

    let profile = get_json_with_bearer(
        &app,
        "/api/app/v1/user-center/profile",
        remote_auth_token.as_str(),
    )
    .await;
    assert_eq!(profile.status(), StatusCode::OK);
    let profile_body = read_json(profile).await;
    assert_eq!(profile_body["user"]["id"], "u_remote");

    let create_conversation = post_json_with_bearer(
        &app,
        "/api/v1/conversations",
        remote_auth_token.as_str(),
        json!({
            "conversationId": "c_remote_user_center_auth_token_demo",
            "conversationType": "group"
        }),
    )
    .await;
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let requests = capture.requests.lock().await.clone();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].method, "POST");
    assert_eq!(
        requests[0].path,
        "/bridge/api/app/v1/user-center/session/login"
    );
    assert_eq!(
        requests[0]
            .body
            .as_ref()
            .and_then(|body| body.get("tenantId")),
        Some(&json!("t_demo"))
    );
    assert_eq!(
        requests[0]
            .headers
            .get("x-sdkwork-user-center-provider-key")
            .map(String::as_str),
        Some("craw-app-api")
    );
    assert_eq!(
        requests[0]
            .headers
            .get("x-sdkwork-user-center-handshake-mode")
            .map(String::as_str),
        Some("provider-shared-secret")
    );
    assert_eq!(
        requests[0]
            .headers
            .get("x-sdkwork-user-center-secret-id")
            .map(String::as_str),
        Some("secret-501")
    );
    assert_eq!(
        requests[0]
            .headers
            .get("x-sdkwork-app-id")
            .map(String::as_str),
        Some("craw-chat")
    );
    assert!(
        requests[0]
            .headers
            .get("x-sdkwork-user-center-signature")
            .is_some_and(|value| !value.trim().is_empty())
    );
    assert_eq!(requests[1].method, "GET");
    assert_eq!(requests[1].path, "/bridge/api/app/v1/user-center/profile");
    assert!(requests[1].body.is_none());
    assert_eq!(
        requests[1].headers.get("authorization").map(String::as_str),
        Some(format!("Bearer {remote_auth_token}").as_str())
    );

    server_handle.abort();
    let _ = fs::remove_dir_all(runtime_dir);
}
