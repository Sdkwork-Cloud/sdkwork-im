use std::panic::AssertUnwindSafe;
use std::sync::OnceLock;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use tokio::sync::{Mutex, MutexGuard};
use tower::ServiceExt;

const PUBLIC_BROWSER_ORIGINS_ENV: &str = "CRAW_CHAT_BROWSER_ORIGINS";

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

async fn browser_origin_env_guard() -> MutexGuard<'static, ()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(())).lock().await
}

async fn run_preflight(origin: &str) -> axum::response::Response {
    let app = local_minimal_node::build_public_app();
    app.oneshot(
        Request::builder()
            .method("OPTIONS")
            .uri("/im/v3/api/portal/home")
            .header("origin", origin)
            .header("access-control-request-method", "GET")
            .header(
                "access-control-request-headers",
                "authorization,content-type",
            )
            .body(Body::empty())
            .expect("preflight request should build"),
    )
    .await
    .expect("public app should return preflight response")
}

async fn run_iam_app_context_preflight(origin: &str) -> axum::response::Response {
    let app = local_minimal_node::build_public_app();
    app.oneshot(
        Request::builder()
            .method("OPTIONS")
            .uri("/app/v3/api/iam/users/current")
            .header("origin", origin)
            .header("access-control-request-method", "GET")
            .header(
                "access-control-request-headers",
                [
                    "authorization",
                    "access-token",
                    "content-type",
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
                    "x-sdkwork-context-signature",
                ]
                .join(",")
                .as_str(),
            )
            .body(Body::empty())
            .expect("IAM AppContext preflight request should build"),
    )
    .await
    .expect("public app should return IAM AppContext preflight response")
}

#[tokio::test]
async fn test_public_app_preflight_allows_default_preview_origins() {
    let _guard = browser_origin_env_guard().await;
    let _origins = ScopedEnvVar::remove(PUBLIC_BROWSER_ORIGINS_ENV);

    for origin in ["http://127.0.0.1:4176", "http://localhost:4176"] {
        let response = run_preflight(origin).await;
        assert!(matches!(
            response.status(),
            StatusCode::OK | StatusCode::NO_CONTENT
        ));
        assert_eq!(
            response
                .headers()
                .get("access-control-allow-origin")
                .expect("allowed origin should be echoed"),
            origin
        );
    }
}

#[tokio::test]
async fn test_public_app_preflight_uses_configured_browser_origins() {
    let _guard = browser_origin_env_guard().await;
    let _origins = ScopedEnvVar::set(
        PUBLIC_BROWSER_ORIGINS_ENV,
        "https://portal.example.com, tauri://localhost/, https://portal.example.com",
    );

    for origin in ["https://portal.example.com", "tauri://localhost"] {
        let response = run_preflight(origin).await;
        assert!(matches!(
            response.status(),
            StatusCode::OK | StatusCode::NO_CONTENT
        ));
        assert_eq!(
            response
                .headers()
                .get("access-control-allow-origin")
                .expect("configured origin should be echoed"),
            origin
        );
    }

    let disallowed_response = run_preflight("http://localhost:4176").await;
    assert!(
        disallowed_response
            .headers()
            .get("access-control-allow-origin")
            .is_none(),
        "default preview origin should not remain allowed after explicit override"
    );
}

#[tokio::test]
async fn test_public_app_preflight_allows_dual_token_and_signed_app_context_headers() {
    let _guard = browser_origin_env_guard().await;
    let _origins = ScopedEnvVar::remove(PUBLIC_BROWSER_ORIGINS_ENV);

    let response = run_iam_app_context_preflight("http://127.0.0.1:4176").await;
    assert!(matches!(
        response.status(),
        StatusCode::OK | StatusCode::NO_CONTENT
    ));
    let allow_headers = response
        .headers()
        .get("access-control-allow-headers")
        .and_then(|value| value.to_str().ok())
        .expect("IAM AppContext preflight should declare allowed headers")
        .to_ascii_lowercase();

    for expected_header in [
        "authorization",
        "access-token",
        "content-type",
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
        "x-sdkwork-context-signature",
    ] {
        assert!(
            allow_headers.contains(expected_header),
            "public app CORS preflight must allow {expected_header}, got {allow_headers}"
        );
    }
}

#[tokio::test]
async fn test_public_app_fails_fast_for_invalid_browser_origin_config() {
    let _guard = browser_origin_env_guard().await;
    let _origins = ScopedEnvVar::set(PUBLIC_BROWSER_ORIGINS_ENV, "https://portal.example.com/app");

    let result = std::panic::catch_unwind(AssertUnwindSafe(local_minimal_node::build_public_app));
    assert!(result.is_err());
}
