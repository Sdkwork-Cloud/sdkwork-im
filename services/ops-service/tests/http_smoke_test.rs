use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_cluster_lag_health_runtime_dir_and_diagnostics_over_http() {
    let app = ops_service::build_default_app();

    let health_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/health")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops health should succeed");
    assert_eq!(health_response.status(), StatusCode::OK);

    let cluster_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/cluster")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops cluster should succeed");
    assert_eq!(cluster_response.status(), StatusCode::OK);
    let cluster_body = cluster_response
        .into_body()
        .collect()
        .await
        .expect("cluster body should collect")
        .to_bytes();
    let cluster_json: serde_json::Value =
        serde_json::from_slice(&cluster_body).expect("cluster body should be valid json");
    assert_eq!(cluster_json["nodes"][0]["profile"], "standalone");
    assert_eq!(cluster_json["nodes"][0]["deviceRouteCount"], 0);

    let lag_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/lag")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops lag should succeed");
    assert_eq!(lag_response.status(), StatusCode::OK);

    let runtime_dir_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/runtime-dir")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops runtime-dir inspection should succeed");
    assert_eq!(runtime_dir_response.status(), StatusCode::OK);
    let runtime_dir_body = runtime_dir_response
        .into_body()
        .collect()
        .await
        .expect("runtime-dir body should collect")
        .to_bytes();
    let runtime_dir_json: serde_json::Value =
        serde_json::from_slice(&runtime_dir_body).expect("runtime-dir body should be valid json");
    assert_eq!(runtime_dir_json["status"], "unmanaged");
    assert_eq!(runtime_dir_json["files"].as_array().unwrap().len(), 0);

    let diagnostics_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/diagnostics")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops diagnostics should succeed");
    assert_eq!(diagnostics_response.status(), StatusCode::OK);
    let diagnostics_body = diagnostics_response
        .into_body()
        .collect()
        .await
        .expect("diagnostics body should collect")
        .to_bytes();
    let diagnostics_json: serde_json::Value =
        serde_json::from_slice(&diagnostics_body).expect("diagnostics body should be valid json");
    assert_eq!(diagnostics_json["profile"], "standalone");
    assert_eq!(
        diagnostics_json["deviceRoutes"].as_array().unwrap().len(),
        0
    );
}
