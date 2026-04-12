use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_healthz_returns_ok_and_service_metadata() {
    let app = control_plane_api::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("body should be valid json");

    assert_eq!(value["status"], "ok");
    assert_eq!(value["service"], "control-plane-api");
}

#[tokio::test]
async fn test_delivered_shared_channel_sync_inventory_route_returns_snapshot() {
    let app = control_plane_api::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/delivered-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_control_reader")
                .header("x-actor-kind", "user")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(value["status"], "snapshot");
    assert_eq!(value["deliveredCount"], 0);
    assert_eq!(
        value["items"]
            .as_array()
            .expect("items should be an array")
            .len(),
        0
    );
}
