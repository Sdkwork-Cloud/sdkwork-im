use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use session_gateway::RealtimeClusterBridge;
use tower::ServiceExt;

#[tokio::test]
async fn test_local_minimal_profile_drain_migrates_routes_and_preserves_realtime_delivery() {
    let projection_service = Arc::new(projection_service::TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());

    let app_a = local_minimal_node::build_app_with_dependencies(
        "node_a",
        "127.0.0.1:18101",
        projection_service.clone(),
        realtime_cluster.clone(),
    );
    let app_b = local_minimal_node::build_app_with_dependencies(
        "node_b",
        "127.0.0.1:18102",
        projection_service.clone(),
        realtime_cluster.clone(),
    );
    let control_app = control_plane_api::build_app_with_cluster(realtime_cluster.clone());

    let create_conversation = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_drain_demo",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_drain_demo/members/add")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_remote",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member should succeed");
    assert_eq!(add_member.status(), StatusCode::OK);

    let register_remote_device = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_remote")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_remote")
                .header("x-session-id", "s_remote")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register remote device should succeed");
    assert_eq!(register_remote_device.status(), StatusCode::OK);

    let sync_remote_subscriptions = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/realtime/subscriptions/sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_remote")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_remote")
                .header("x-session-id", "s_remote")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_drain_demo",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("sync remote subscriptions should succeed");
    assert_eq!(sync_remote_subscriptions.status(), StatusCode::OK);

    let drain_response = control_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/nodes/node_a/drain")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-actor-kind", "user")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("drain request should succeed");
    assert_eq!(drain_response.status(), StatusCode::OK);

    let migrate_response = control_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/nodes/node_a/routes/migrate")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-actor-kind", "user")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"targetNodeId":"node_b"}"#))
                .unwrap(),
        )
        .await
        .expect("migrate request should succeed");
    assert_eq!(migrate_response.status(), StatusCode::OK);

    let drained_pull = app_a
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_remote")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_remote")
                .header("x-session-id", "s_remote")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("drained pull should return response");
    assert_eq!(drained_pull.status(), StatusCode::CONFLICT);
    let drained_body = drained_pull
        .into_body()
        .collect()
        .await
        .expect("drained body should collect")
        .to_bytes();
    let drained_json: serde_json::Value =
        serde_json::from_slice(&drained_body).expect("drained body should be valid json");
    assert_eq!(drained_json["code"], "node_draining");

    let post_message = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_drain_demo/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_drain_route_1",
                        "summary":"drain hello",
                        "text":"drain hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let remote_events = app_b
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_remote")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_remote")
                .header("x-session-id", "s_remote")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("remote events should succeed");
    assert_eq!(remote_events.status(), StatusCode::OK);
    let remote_events_body = remote_events
        .into_body()
        .collect()
        .await
        .expect("remote events body should collect")
        .to_bytes();
    let remote_events_json: serde_json::Value =
        serde_json::from_slice(&remote_events_body).expect("remote events should be valid json");
    assert_eq!(remote_events_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(remote_events_json["items"][0]["scopeId"], "c_drain_demo");
    assert_eq!(
        remote_events_json["items"][0]["eventType"],
        "message.posted"
    );

    let source_cluster = app_a
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/cluster")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("source cluster should succeed");
    assert_eq!(source_cluster.status(), StatusCode::OK);
    let source_cluster_body = source_cluster
        .into_body()
        .collect()
        .await
        .expect("source cluster body should collect")
        .to_bytes();
    let source_cluster_json: serde_json::Value =
        serde_json::from_slice(&source_cluster_body).expect("source cluster should be valid json");
    assert_eq!(source_cluster_json["nodes"][0]["nodeId"], "node_a");
    assert_eq!(source_cluster_json["nodes"][0]["drainStatus"], "drained");
    assert_eq!(source_cluster_json["nodes"][0]["deviceRouteCount"], 0);

    let target_cluster = app_b
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/cluster")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("target cluster should succeed");
    assert_eq!(target_cluster.status(), StatusCode::OK);
    let target_cluster_body = target_cluster
        .into_body()
        .collect()
        .await
        .expect("target cluster body should collect")
        .to_bytes();
    let target_cluster_json: serde_json::Value =
        serde_json::from_slice(&target_cluster_body).expect("target cluster should be valid json");
    assert_eq!(target_cluster_json["nodes"][0]["nodeId"], "node_b");
    assert_eq!(target_cluster_json["nodes"][0]["drainStatus"], "active");
    assert_eq!(target_cluster_json["nodes"][0]["deviceRouteCount"], 2);

    let target_diagnostics = app_b
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/diagnostics")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("target diagnostics should succeed");
    assert_eq!(target_diagnostics.status(), StatusCode::OK);
    let target_diagnostics_body = target_diagnostics
        .into_body()
        .collect()
        .await
        .expect("target diagnostics body should collect")
        .to_bytes();
    let target_diagnostics_json: serde_json::Value =
        serde_json::from_slice(&target_diagnostics_body)
            .expect("target diagnostics should be valid json");
    assert_eq!(
        target_diagnostics_json["deviceRoutes"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|item| item["deviceId"] == "d_remote" && item["ownerNodeId"] == "node_b")
            .count(),
        1
    );
}

#[tokio::test]
async fn test_local_minimal_profile_disconnect_releases_route_before_drain() {
    let projection_service = Arc::new(projection_service::TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());

    let app_a = local_minimal_node::build_app_with_dependencies(
        "node_a",
        "127.0.0.1:18111",
        projection_service,
        realtime_cluster.clone(),
    );
    let control_app = control_plane_api::build_app_with_cluster(realtime_cluster);

    let resume = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("resume should succeed");
    assert_eq!(resume.status(), StatusCode::OK);

    let cluster_before = app_a
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/cluster")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_demo")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("cluster before disconnect should succeed");
    assert_eq!(cluster_before.status(), StatusCode::OK);
    let cluster_before_body = cluster_before
        .into_body()
        .collect()
        .await
        .expect("cluster before body should collect")
        .to_bytes();
    let cluster_before_json: serde_json::Value =
        serde_json::from_slice(&cluster_before_body).expect("cluster before should be valid json");
    assert_eq!(cluster_before_json["nodes"][0]["deviceRouteCount"], 1);

    let disconnect = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/disconnect")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("disconnect should succeed");
    assert_eq!(disconnect.status(), StatusCode::OK);

    let diagnostics_after = app_a
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/diagnostics")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_demo")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("diagnostics after disconnect should succeed");
    assert_eq!(diagnostics_after.status(), StatusCode::OK);
    let diagnostics_after_body = diagnostics_after
        .into_body()
        .collect()
        .await
        .expect("diagnostics after body should collect")
        .to_bytes();
    let diagnostics_after_json: serde_json::Value = serde_json::from_slice(&diagnostics_after_body)
        .expect("diagnostics after should be valid json");
    assert_eq!(
        diagnostics_after_json["deviceRoutes"]
            .as_array()
            .unwrap()
            .len(),
        0
    );

    let drain = control_app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/nodes/node_a/drain")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("drain should succeed");
    assert_eq!(drain.status(), StatusCode::OK);
    let drain_body = drain
        .into_body()
        .collect()
        .await
        .expect("drain body should collect")
        .to_bytes();
    let drain_json: serde_json::Value =
        serde_json::from_slice(&drain_body).expect("drain should be valid json");
    assert_eq!(drain_json["drainStatus"], "drained");
    assert_eq!(drain_json["rebalanceState"], "stable");
    assert_eq!(drain_json["ownedRouteCount"], 0);
}
