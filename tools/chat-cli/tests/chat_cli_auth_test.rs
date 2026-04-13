use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::Router;
use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderMap, Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use craw_chat_cli::{CommandOutput, execute_command, parse_cli_args};
use im_auth_context::PUBLIC_BEARER_HS256_SECRET_ENV;
use serde_json::{Value, json};
use tokio::net::TcpListener;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Clone, Default)]
struct CaptureState {
    last_request: std::sync::Arc<std::sync::Mutex<Option<CapturedRequest>>>,
}

#[derive(Clone, Debug)]
struct CapturedRequest {
    authorization: Option<String>,
    body: Value,
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

async fn public_auth_guard() -> MutexGuard<'static, ()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(())).lock().await
}

async fn configure_public_bearer_secret() -> (MutexGuard<'static, ()>, ScopedEnvVar) {
    let guard = public_auth_guard().await;
    let scoped = ScopedEnvVar::set(PUBLIC_BEARER_HS256_SECRET_ENV, "public-test-secret");
    (guard, scoped)
}

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_cli_auth_runtime_{unique}"))
}

async fn spawn_server(app: Router) -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("listener should bind");
    let address = listener
        .local_addr()
        .expect("listener should expose local address");
    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.expect("server should run");
    });
    (format!("http://127.0.0.1:{}", address.port()), handle)
}

fn command_output_json(output: CommandOutput) -> Value {
    match output {
        CommandOutput::Json(value) => value,
        other => panic!("expected json output, got {other:?}"),
    }
}

async fn capture_login_request(
    State(state): State<CaptureState>,
    headers: HeaderMap,
    request: Request<Body>,
) -> impl IntoResponse {
    let authorization = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    let bytes = axum::body::to_bytes(request.into_body(), usize::MAX)
        .await
        .expect("request body should collect");
    let body = serde_json::from_slice::<Value>(&bytes).expect("request body should be valid json");

    *state
        .last_request
        .lock()
        .expect("capture state should remain available") = Some(CapturedRequest {
        authorization,
        body: body.clone(),
    });

    (
        StatusCode::OK,
        axum::Json(json!({
            "accessToken": "capture_access_token",
            "refreshToken": "capture_refresh_token",
            "expiresAt": 1893456000u64,
            "user": {
                "id": "u_guest",
                "clientKind": "im_user"
            },
            "workspace": null
        })),
    )
}

#[tokio::test]
async fn test_chat_cli_login_command_posts_credentials_without_authorization_header() {
    let state = CaptureState::default();
    let app = Router::new()
        .route("/api/v1/auth/login", post(capture_login_request))
        .with_state(state.clone());
    let (base_url, handle) = spawn_server(app).await;

    let output = execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url.as_str(),
            "--tenant-id",
            "t_demo",
            "--session-id",
            "s_guest",
            "--device-id",
            "d_guest",
            "login",
            "--login",
            "u_guest",
            "--password",
            "Guest#2026",
            "--client-kind",
            "im_user",
        ])
        .expect("login args should parse"),
    )
    .await
    .expect("login command should succeed");

    let json = command_output_json(output);
    assert_eq!(json["accessToken"], "capture_access_token");
    assert_eq!(json["refreshToken"], "capture_refresh_token");
    assert_eq!(json["user"]["id"], "u_guest");

    let captured = state
        .last_request
        .lock()
        .expect("capture state should remain available")
        .clone()
        .expect("login request should be captured");
    assert_eq!(captured.authorization, None);
    assert_eq!(
        captured.body,
        json!({
            "tenantId": "t_demo",
            "login": "u_guest",
            "password": "Guest#2026",
            "deviceId": "d_guest",
            "sessionId": "s_guest",
            "clientKind": "im_user"
        })
    );

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_chat_cli_login_command_works_against_local_minimal_node_real_auth() {
    let (_guard, _secret) = configure_public_bearer_secret().await;
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());
    let (base_url, handle) = spawn_server(app).await;

    let output = execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url.as_str(),
            "--tenant-id",
            "t_demo",
            "--session-id",
            "s_guest",
            "--device-id",
            "d_guest",
            "login",
            "--login",
            "u_guest",
            "--password",
            "Guest#2026",
            "--client-kind",
            "im_user",
        ])
        .expect("login args should parse"),
    )
    .await
    .expect("login command should succeed");

    let json = command_output_json(output);
    assert_eq!(json["user"]["id"], "u_guest");
    assert_eq!(json["user"]["clientKind"], "im_user");
    assert!(json["accessToken"].as_str().is_some());
    assert!(json["refreshToken"].as_str().is_some());

    handle.abort();
    let _ = handle.await;
    let _ = fs::remove_dir_all(runtime_dir);
}
