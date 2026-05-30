use axum::body::Body;
use axum::http::{Request, StatusCode};
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
async fn test_im_v3_api_uses_device_session_namespace_and_does_not_expose_identity_paths() {
    let app = local_minimal_node::build_default_app();

    let authorization_only = post_json_with_authorization_only(
        &app,
        "/im/v3/api/device/sessions/resume",
        r#"{"deviceId":"d_demo","lastSeenSyncSeq":0}"#,
    )
    .await;
    assert_eq!(
        authorization_only.status(),
        StatusCode::UNAUTHORIZED,
        "craw-chat must require the trusted SDKWork AppContext projection; an authorization header alone is not enough"
    );

    for api_prefix in ["/im/v3/api", "/app/v3/api"] {
        let resume = post_json(
            &app,
            format!("{api_prefix}/device/sessions/resume"),
            r#"{"deviceId":"d_demo","lastSeenSyncSeq":0}"#,
        )
        .await;
        assert_eq!(
            resume.status(),
            StatusCode::OK,
            "{api_prefix} device session resume should be exposed"
        );
        let resume_body = read_json(resume).await;
        assert_eq!(resume_body["tenantId"], "t_demo");
        assert_eq!(resume_body["actorId"], "u_demo");

        let disconnect = post_json(
            &app,
            format!("{api_prefix}/device/sessions/disconnect"),
            r#"{"deviceId":"d_demo"}"#,
        )
        .await;
        assert_eq!(
            disconnect.status(),
            StatusCode::OK,
            "{api_prefix} device session disconnect should be exposed"
        );
    }

    let removed_paths = vec![
        marker(&["/api", "/v1", "/sessions/resume"]),
        marker(&["/api", "/v1", "/sessions/disconnect"]),
        marker(&["/api", "/v1", "/chat", "-runtime/sessions/resume"]),
        marker(&["/api", "/v1", "/chat", "-runtime/sessions/disconnect"]),
        marker(&["/im/v3/api/device", "-sessions/resume"]),
        marker(&["/im/v3/api/device", "-sessions/disconnect"]),
        marker(&["/api", "/v1", "/auth", "/login"]),
        marker(&["/api", "/v1", "/auth/refresh"]),
        marker(&["/api", "/v1", "/auth", "/me"]),
        marker(&["/api/app", "/v1", "/user", "-center/session/login"]),
        marker(&["/api/app", "/v1", "/user", "-center/session/refresh"]),
        marker(&["/api/app", "/v1", "/user", "-center/profile"]),
        marker(&["/api", "/v1", "/control/social/friend-requests"]),
        marker(&["/app/v3/api/auth", "/sessions"]),
        marker(&["/app/v3/api/auth", "/sessions/refresh"]),
        marker(&["/app/v3/api/auth", "/sessions/current"]),
        marker(&["/backend/v3/api/device", "/sessions/resume"]),
        marker(&["/backend/v3/api/device", "/sessions/disconnect"]),
        marker(&["/backend/v3/api/auth", "/sessions"]),
    ];
    for removed_path in removed_paths {
        let response = post_json(&app, removed_path.as_str(), "{}").await;
        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "{removed_path} must not be exposed by craw-chat; login belongs outside this service"
        );
    }
}
