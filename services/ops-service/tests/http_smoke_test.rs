use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_public_app_exports_live_openapi_json() {
    let app = ops_service::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/openapi.json")
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

    assert_eq!(value["openapi"], "3.1.0");
    assert_eq!(value["info"]["title"], "Craw Chat Ops Service API");
    assert!(value["paths"]["/api/v1/ops/health"].is_object());
}

#[tokio::test]
async fn test_public_app_serves_docs_page_for_live_openapi() {
    let app = ops_service::build_public_app();

    let response = app
        .oneshot(Request::builder().uri("/docs").body(Body::empty()).unwrap())
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let html = String::from_utf8(body.to_vec()).expect("docs should be valid utf-8");

    assert!(html.contains("OpenAPI 3.1"));
    assert!(html.contains("Craw Chat Ops Service API"));
    assert!(html.contains("/openapi.json"));
}

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
    let health_body = health_response
        .into_body()
        .collect()
        .await
        .expect("health body should collect")
        .to_bytes();
    let health_json: serde_json::Value =
        serde_json::from_slice(&health_body).expect("health body should be valid json");
    assert_eq!(health_json["projectionPlane"]["status"], "idle");
    assert_eq!(
        health_json["projectionPlane"]["metrics"]["conversationSnapshotPersist"]["successCount"],
        0
    );
    assert_eq!(health_json["projectionPlane"]["replay"]["backlogSize"], 0);
    assert_eq!(
        health_json["projectionPlane"]["replay"]["replayedEventCount"],
        0
    );
    assert_eq!(health_json["projectionPlane"]["replay"]["durationMs"], 0);
    assert_eq!(health_json["projectionPlane"]["rebuildDurationMs"], 0);
    assert_eq!(
        health_json["projectionPlane"]["updateDelay"]["timelineMs"],
        0
    );
    assert_eq!(health_json["projectionPlane"]["updateDelay"]["inboxMs"], 0);

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
    let lag_body = lag_response
        .into_body()
        .collect()
        .await
        .expect("lag body should collect")
        .to_bytes();
    let lag_json: serde_json::Value =
        serde_json::from_slice(&lag_body).expect("lag body should be valid json");
    assert!(
        lag_json["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["component"] == "projection_replay" && item["lag"] == 0),
        "ops lag should expose the default projection replay lag item"
    );

    let replay_status_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/replay-status")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops replay-status should succeed");
    assert_eq!(replay_status_response.status(), StatusCode::OK);
    let replay_status_body = replay_status_response
        .into_body()
        .collect()
        .await
        .expect("replay-status body should collect")
        .to_bytes();
    let replay_status_json: serde_json::Value = serde_json::from_slice(&replay_status_body)
        .expect("replay-status body should be valid json");
    assert_eq!(replay_status_json["status"], "idle");
    assert_eq!(replay_status_json["replay"]["backlogSize"], 0);
    assert_eq!(replay_status_json["replay"]["replayedEventCount"], 0);
    assert_eq!(replay_status_json["replay"]["durationMs"], 0);
    assert_eq!(replay_status_json["replayThroughputPerSecond"], 0);
    assert!(
        replay_status_json["lag"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["component"] == "projection_replay" && item["lag"] == 0),
        "ops replay-status should expose the default projection replay lag item"
    );

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

    let provider_bindings_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/provider-bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops provider-bindings should succeed");
    assert_eq!(provider_bindings_response.status(), StatusCode::OK);
    let provider_bindings_body = provider_bindings_response
        .into_body()
        .collect()
        .await
        .expect("provider-bindings body should collect")
        .to_bytes();
    let provider_bindings_json: serde_json::Value = serde_json::from_slice(&provider_bindings_body)
        .expect("provider-bindings body should be valid json");
    assert_eq!(provider_bindings_json["items"].as_array().unwrap().len(), 0);

    let provider_binding_drift_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/provider-bindings/drift")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops provider-bindings drift should succeed");
    assert_eq!(provider_binding_drift_response.status(), StatusCode::OK);
    let provider_binding_drift_body = provider_binding_drift_response
        .into_body()
        .collect()
        .await
        .expect("provider-bindings drift body should collect")
        .to_bytes();
    let provider_binding_drift_json: serde_json::Value =
        serde_json::from_slice(&provider_binding_drift_body)
            .expect("provider-bindings drift body should be valid json");
    assert_eq!(
        provider_binding_drift_json["items"]
            .as_array()
            .unwrap()
            .len(),
        0
    );

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
    assert_eq!(diagnostics_json["projectionPlane"]["status"], "idle");
    assert_eq!(
        diagnostics_json["projectionPlane"]["replay"]["backlogSize"],
        0
    );
    assert_eq!(
        diagnostics_json["projectionPlane"]["replay"]["replayedEventCount"],
        0
    );
    assert_eq!(
        diagnostics_json["projectionPlane"]["replay"]["durationMs"],
        0
    );
    assert_eq!(diagnostics_json["projectionPlane"]["rebuildDurationMs"], 0);
    assert_eq!(
        diagnostics_json["projectionPlane"]["updateDelay"]["timelineMs"],
        0
    );
    assert_eq!(
        diagnostics_json["projectionPlane"]["updateDelay"]["inboxMs"],
        0
    );
    assert_eq!(
        diagnostics_json["projectionPlane"]["traces"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert_eq!(
        diagnostics_json["providerBindings"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert_eq!(
        diagnostics_json["providerBindingDrift"]["items"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
}
