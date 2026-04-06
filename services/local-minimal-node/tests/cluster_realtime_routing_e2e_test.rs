use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use session_gateway::RealtimeClusterBridge;
use tower::ServiceExt;

#[tokio::test]
async fn test_local_minimal_profile_routes_realtime_events_to_remote_owner_node() {
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

    let create_conversation = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_cluster_realtime",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_cluster_realtime/members/add")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_other_demo",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member should succeed");
    assert_eq!(add_member.status(), StatusCode::OK);

    let register_remote_device = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_remote")
                .header("x-session-id", "s_remote")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register remote device should succeed");
    assert_eq!(register_remote_device.status(), StatusCode::OK);

    let sync_remote_subscriptions = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/realtime/subscriptions/sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_remote")
                .header("x-session-id", "s_remote")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_cluster_realtime",
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

    let remote_cluster = app_b
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/cluster")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_remote")
                .header("x-session-id", "s_remote")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("remote ops cluster should succeed");
    assert_eq!(remote_cluster.status(), StatusCode::OK);
    let remote_cluster_body = remote_cluster
        .into_body()
        .collect()
        .await
        .expect("remote cluster body should collect")
        .to_bytes();
    let remote_cluster_json: serde_json::Value =
        serde_json::from_slice(&remote_cluster_body).expect("remote cluster should be valid json");
    assert_eq!(remote_cluster_json["nodes"][0]["nodeId"], "node_b");
    assert_eq!(remote_cluster_json["nodes"][0]["deviceRouteCount"], 1);

    let remote_diagnostics = app_b
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/diagnostics")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_remote")
                .header("x-session-id", "s_remote")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("remote ops diagnostics should succeed");
    assert_eq!(remote_diagnostics.status(), StatusCode::OK);
    let remote_diagnostics_body = remote_diagnostics
        .into_body()
        .collect()
        .await
        .expect("remote diagnostics body should collect")
        .to_bytes();
    let remote_diagnostics_json: serde_json::Value =
        serde_json::from_slice(&remote_diagnostics_body)
            .expect("remote diagnostics should be valid json");
    assert_eq!(
        remote_diagnostics_json["deviceRoutes"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        remote_diagnostics_json["deviceRoutes"][0]["deviceId"],
        "d_remote"
    );
    assert_eq!(
        remote_diagnostics_json["deviceRoutes"][0]["ownerNodeId"],
        "node_b"
    );

    let post_message = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_cluster_realtime/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_cluster_route_1",
                        "summary":"cluster hello",
                        "text":"cluster hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let remote_events = app_b
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_remote")
                .header("x-session-id", "s_remote")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("remote realtime events should succeed");
    assert_eq!(remote_events.status(), StatusCode::OK);
    let remote_events_body = remote_events
        .into_body()
        .collect()
        .await
        .expect("remote realtime body should collect")
        .to_bytes();
    let remote_events_json: serde_json::Value = serde_json::from_slice(&remote_events_body)
        .expect("remote realtime events should be valid json");
    assert_eq!(remote_events_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(
        remote_events_json["items"][0]["scopeId"],
        "c_cluster_realtime"
    );
    assert_eq!(
        remote_events_json["items"][0]["eventType"],
        "message.posted"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_stale_disconnect_after_cross_node_resume_takeover() {
    let projection_service = Arc::new(projection_service::TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());

    let app_a = local_minimal_node::build_app_with_dependencies(
        "node_a",
        "127.0.0.1:18111",
        projection_service.clone(),
        realtime_cluster.clone(),
    );
    let app_b = local_minimal_node::build_app_with_dependencies(
        "node_b",
        "127.0.0.1:18112",
        projection_service.clone(),
        realtime_cluster.clone(),
    );

    let resume_old = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_resume")
                .header("x-session-id", "s_old")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("old resume should succeed");
    assert_eq!(resume_old.status(), StatusCode::OK);

    let resume_new = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_resume")
                .header("x-session-id", "s_new")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("new resume should succeed");
    assert_eq!(resume_new.status(), StatusCode::OK);

    let diagnostics_before = app_b
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/diagnostics")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_resume")
                .header("x-session-id", "s_new")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("diagnostics before stale disconnect should succeed");
    assert_eq!(diagnostics_before.status(), StatusCode::OK);
    let diagnostics_before_body = diagnostics_before
        .into_body()
        .collect()
        .await
        .expect("diagnostics before body should collect")
        .to_bytes();
    let diagnostics_before_json: serde_json::Value =
        serde_json::from_slice(&diagnostics_before_body)
            .expect("diagnostics before should be valid json");
    assert_eq!(
        diagnostics_before_json["deviceRoutes"][0]["ownerNodeId"],
        "node_b"
    );

    let stale_disconnect = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/disconnect")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_resume")
                .header("x-session-id", "s_old")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("stale disconnect should return response");
    assert_eq!(stale_disconnect.status(), StatusCode::CONFLICT);
    let stale_disconnect_body = stale_disconnect
        .into_body()
        .collect()
        .await
        .expect("stale disconnect body should collect")
        .to_bytes();
    let stale_disconnect_json: serde_json::Value = serde_json::from_slice(&stale_disconnect_body)
        .expect("stale disconnect body should be valid json");
    assert_eq!(stale_disconnect_json["code"], "stale_session");

    let diagnostics_after = app_b
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/diagnostics")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_resume")
                .header("x-session-id", "s_new")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("diagnostics after stale disconnect should succeed");
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
        diagnostics_after_json["deviceRoutes"][0]["ownerNodeId"],
        "node_b"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_sessionless_rebind_after_cross_node_resume_takeover() {
    let projection_service = Arc::new(projection_service::TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());

    let app_a = local_minimal_node::build_app_with_dependencies(
        "node_a",
        "127.0.0.1:18113",
        projection_service.clone(),
        realtime_cluster.clone(),
    );
    let app_b = local_minimal_node::build_app_with_dependencies(
        "node_b",
        "127.0.0.1:18114",
        projection_service.clone(),
        realtime_cluster.clone(),
    );

    let resume_old = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_resume")
                .header("x-session-id", "s_old")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("old resume should succeed");
    assert_eq!(resume_old.status(), StatusCode::OK);

    let resume_new = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_resume")
                .header("x-session-id", "s_new")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("new resume should succeed");
    assert_eq!(resume_new.status(), StatusCode::OK);

    let sessionless_register = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_resume")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("sessionless register should return response");
    assert_eq!(sessionless_register.status(), StatusCode::CONFLICT);
    let sessionless_register_body = sessionless_register
        .into_body()
        .collect()
        .await
        .expect("sessionless register body should collect")
        .to_bytes();
    let sessionless_register_json: serde_json::Value =
        serde_json::from_slice(&sessionless_register_body)
            .expect("sessionless register body should be valid json");
    assert_eq!(sessionless_register_json["code"], "session_id_required");

    let diagnostics_after = app_b
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/diagnostics")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_resume")
                .header("x-session-id", "s_new")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("diagnostics after sessionless register should succeed");
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
        diagnostics_after_json["deviceRoutes"][0]["ownerNodeId"],
        "node_b"
    );
}
