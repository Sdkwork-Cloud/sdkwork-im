use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

fn marker(parts: &[&str]) -> String {
    parts.concat()
}

async fn post_json(
    app: &axum::Router,
    path: impl AsRef<str>,
    body: &'static str,
) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(path.as_ref())
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-session-id", "sdkwork_iam_session_demo")
                .header("x-sdkwork-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .expect("request should build"),
        )
        .await
        .expect("route should return response")
}

async fn post_json_with_authorization_only(
    app: &axum::Router,
    path: impl AsRef<str>,
    body: &'static str,
) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(path.as_ref())
                .header(axum::http::header::AUTHORIZATION, "external-sdkwork-token")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .expect("request should build"),
        )
        .await
        .expect("route should return response")
}

async fn request_json(
    app: &axum::Router,
    method: Method,
    path: impl AsRef<str>,
    body: &'static str,
) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path.as_ref())
                .header(axum::http::header::AUTHORIZATION, "Bearer auth-token")
                .header("access-token", "access-token")
                .header("x-sdkwork-app-id", "sdkwork-craw-chat-pc")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-session-id", "sdkwork_iam_session_demo")
                .header("x-sdkwork-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .expect("request should build"),
        )
        .await
        .expect("route should return response")
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

#[tokio::test]
async fn test_public_app_does_not_expose_craw_chat_private_app_api_bootstrap_routes() {
    let app = local_minimal_node::build_public_app();

    for (method, path, body) in [
        (
            Method::GET,
            "/app/v3/api/system/iam/verification_policy",
            "",
        ),
        (Method::POST, "/app/v3/api/oauth/authorization_urls", "{}"),
        (
            Method::POST,
            "/app/v3/api/oauth/device_authorizations",
            "{}",
        ),
        (
            Method::GET,
            "/app/v3/api/oauth/device_authorizations/bootstrap-session",
            "",
        ),
        (
            Method::POST,
            "/app/v3/api/oauth/device_authorizations/bootstrap-session/scans",
            "{}",
        ),
        (
            Method::POST,
            "/app/v3/api/oauth/device_authorizations/bootstrap-session/password_completions",
            "{}",
        ),
        (Method::POST, "/app/v3/api/oauth/sessions", "{}"),
    ] {
        let response = request_json(&app, method, path, body).await;
        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "{path} must not be reimplemented by Craw Chat; /app/v3/api is provided by sdkwork-appbase"
        );
    }
}

#[tokio::test]
async fn test_im_v3_api_uses_presence_realtime_namespace_and_does_not_expose_identity_paths() {
    let app = local_minimal_node::build_default_app();

    let authorization_only = post_json_with_authorization_only(
        &app,
        "/im/v3/api/presence/heartbeat",
        r#"{"deviceId":"d_demo"}"#,
    )
    .await;
    assert_eq!(
        authorization_only.status(),
        StatusCode::UNAUTHORIZED,
        "craw-chat must require the trusted SDKWork AppContext projection; an authorization header alone is not enough"
    );

    let heartbeat = post_json(
        &app,
        "/im/v3/api/presence/heartbeat",
        r#"{"deviceId":"d_demo"}"#,
    )
    .await;
    assert_eq!(
        heartbeat.status(),
        StatusCode::OK,
        "IM API presence heartbeat should be exposed"
    );
    let heartbeat_body = read_json(heartbeat).await;
    assert_eq!(heartbeat_body["tenantId"], "t_demo");
    assert_eq!(heartbeat_body["principalId"], "u_demo");
    assert_eq!(heartbeat_body["currentDeviceId"], "d_demo");

    for app_device_session_path in [
        "/app/v3/api/device/sessions/resume",
        "/app/v3/api/device/sessions/disconnect",
    ] {
        let response = post_json(&app, app_device_session_path, "{}").await;
        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "{app_device_session_path} must stay outside the app IAM API surface"
        );
    }

    for (method, path, body) in [
        (
            Method::POST,
            "/app/v3/api/auth/sessions",
            r#"{"username":"sdkwork-user","password":"secret","deviceId":"d_demo"}"#,
        ),
        (Method::GET, "/app/v3/api/auth/sessions/current", ""),
        (Method::PATCH, "/app/v3/api/auth/sessions/current", "{}"),
        (Method::DELETE, "/app/v3/api/auth/sessions/current", ""),
        (
            Method::POST,
            "/app/v3/api/auth/registrations",
            r#"{"username":"new-user","password":"secret"}"#,
        ),
        (
            Method::POST,
            "/app/v3/api/auth/sessions/refresh",
            r#"{"refreshToken":"refresh-token"}"#,
        ),
        (
            Method::POST,
            "/app/v3/api/auth/verification_codes",
            r#"{"target":"user@example.com"}"#,
        ),
        (
            Method::POST,
            "/app/v3/api/auth/verification_codes/verify",
            r#"{"code":"000000"}"#,
        ),
        (Method::GET, "/app/v3/api/system/iam/runtime", ""),
        (
            Method::GET,
            "/app/v3/api/system/iam/verification_policy",
            "",
        ),
        (
            Method::POST,
            "/app/v3/api/oauth/device_authorizations",
            "{}",
        ),
        (Method::POST, "/app/v3/api/oauth/authorization_urls", "{}"),
        (Method::POST, "/app/v3/api/oauth/sessions", "{}"),
    ] {
        let response = request_json(&app, method, path, body).await;
        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "{path} must not be reimplemented by Craw Chat; sdkwork-appbase owns /app/v3/api"
        );
    }

    let removed_paths = vec![
        marker(&["/api", "/v1", "/sessions/resume"]),
        marker(&["/api", "/v1", "/sessions/disconnect"]),
        marker(&["/api", "/v1", "/chat", "-runtime/sessions/resume"]),
        marker(&["/api", "/v1", "/chat", "-runtime/sessions/disconnect"]),
        marker(&["/im/v3/api/device", "/sessions/resume"]),
        marker(&["/im/v3/api/device", "/sessions/disconnect"]),
        marker(&["/im/v3/api/devices", "/register"]),
        marker(&["/im/v3/api/devices", "/d_demo/sync_feed"]),
        marker(&["/api", "/v1", "/auth", "/login"]),
        marker(&["/api", "/v1", "/auth/refresh"]),
        marker(&["/api", "/v1", "/auth", "/me"]),
        marker(&["/api/app", "/v1", "/user", "-center/session/login"]),
        marker(&["/api/app", "/v1", "/user", "-center/session/refresh"]),
        marker(&["/api/app", "/v1", "/user", "-center/profile"]),
        marker(&["/api", "/v1", "/control/social/friend-requests"]),
        marker(&["/backend/v3/api/device", "/sessions/resume"]),
        marker(&["/backend/v3/api/device", "/sessions/disconnect"]),
        marker(&["/backend/v3/api/auth", "/sessions"]),
    ];
    for removed_path in removed_paths {
        let response = post_json(&app, removed_path.as_str(), "{}").await;
        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "{removed_path} must not be exposed by craw-chat; only SDKWork app v3 IAM routes belong here"
        );
    }
}
