use im_app_context::DualTokenRequestBuilderExt;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_local_minimal_profile_gets_media_provider_health_over_http() {
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/media/provider_health")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("provider health request should return response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("provider health body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("provider health response should be valid json");

    assert_eq!(json["pluginId"], "sdkwork-drive");
    assert_eq!(json["status"], "healthy");
    assert_eq!(json["details"]["storageAuthority"], "sdkwork-drive");
    assert_eq!(json["details"]["uploadLifecycle"], "delegated-to-drive");
}

#[tokio::test]
async fn test_local_minimal_profile_does_not_expose_app_local_media_lifecycle_routes() {
    let app = local_minimal_node::build_default_app();

    for (method, path) in [
        ("POST", "/im/v3/api/media/uploads"),
        (
            "POST",
            "/im/v3/api/media/uploads/ma_local_provider_http/complete",
        ),
        ("GET", "/im/v3/api/media/ma_local_provider_http"),
        (
            "GET",
            "/im/v3/api/media/ma_local_provider_http/download_url?expiresInSeconds=1200",
        ),
        ("POST", "/im/v3/api/media/ma_local_provider_http/attach"),
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(path)
                    .with_dual_token_tenant("t_demo")
                    .with_dual_token_user("u_demo")
                    .with_dual_token_actor_kind("user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("media lifecycle request should return response");

        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "{method} {path} must be removed because Drive owns upload/download lifecycle"
        );
    }
}
