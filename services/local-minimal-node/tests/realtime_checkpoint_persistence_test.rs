use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_realtime_checkpoint_runtime_{unique}"))
}

#[tokio::test]
async fn test_ops_commercial_readiness_reports_current_blockers_without_exposing_payloads() {
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/commercial_readiness")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_ops")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops commercial readiness should return response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("ops commercial readiness body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("ops commercial readiness should be valid json");

    assert_eq!(json["status"], "blocked");
    let blockers = json["blockers"]
        .as_array()
        .expect("commercial readiness blockers should be an array");
    assert!(
        blockers
            .iter()
            .any(|blocker| blocker["code"] == "postgres_runtime_adapter_contract_only"),
        "ops commercial readiness should expose PostgreSQL contract-only blocker: {json}"
    );
    assert!(
        blockers
            .iter()
            .any(|blocker| blocker["code"] == "step11_pre_release_gate_not_passed"),
        "ops commercial readiness should expose Step 11 pre-release blocker: {json}"
    );
    assert!(
        json.get("payload").is_none(),
        "ops commercial readiness must not expose message/event payloads"
    );
}

#[tokio::test]
async fn test_ops_commercial_readiness_requires_ops_read_permission() {
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/commercial_readiness")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_viewer")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops commercial readiness should return controlled permission response");

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("permission body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("permission response should be valid json");
    assert_eq!(json["code"], "permission_denied");
}

#[tokio::test]
async fn test_default_local_minimal_profile_persists_realtime_checkpoint_across_rebuild_via_runtime_dir()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_before = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let create_conversation = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_checkpoint_restart",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let register_pad = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register pad should succeed");
    assert_eq!(register_pad.status(), StatusCode::OK);

    let sync_subscriptions = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_checkpoint_restart",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let post_first_message = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_checkpoint_restart/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_checkpoint_restart_1",
                        "summary":"first",
                        "text":"first"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first post should succeed");
    assert_eq!(post_first_message.status(), StatusCode::OK);

    let ack_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/events/ack")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"ackedSeq":1}"#))
                .unwrap(),
        )
        .await
        .expect("ack request should succeed");
    assert_eq!(ack_response.status(), StatusCode::OK);

    let checkpoint_file = runtime_dir.join("state").join("realtime-checkpoints.json");
    assert!(
        checkpoint_file.exists(),
        "default local-minimal runtime should persist realtime checkpoints under the runtime state dir"
    );

    let app_after = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let register_pad_after = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad_new")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register pad after restart should succeed");
    assert_eq!(register_pad_after.status(), StatusCode::OK);

    let after_restart_before_publish = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad_new")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events after restart should succeed");
    assert_eq!(after_restart_before_publish.status(), StatusCode::OK);
    let after_restart_before_publish_body = after_restart_before_publish
        .into_body()
        .collect()
        .await
        .expect("events body should collect")
        .to_bytes();
    let after_restart_before_publish_json: serde_json::Value =
        serde_json::from_slice(&after_restart_before_publish_body)
            .expect("events body should be valid json");
    assert_eq!(after_restart_before_publish_json["ackedThroughSeq"], 1);
    assert_eq!(after_restart_before_publish_json["trimmedThroughSeq"], 1);
    assert_eq!(
        after_restart_before_publish_json["items"]
            .as_array()
            .expect("items should be an array")
            .len(),
        0
    );

    let resync_subscriptions_after_restart = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad_new")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_checkpoint_restart",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription resync after restart should succeed");
    assert_eq!(resync_subscriptions_after_restart.status(), StatusCode::OK);

    let post_second_message = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_checkpoint_restart/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone_new")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_checkpoint_restart_2",
                        "summary":"second",
                        "text":"second"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second post should succeed");
    assert_eq!(post_second_message.status(), StatusCode::OK);

    let after_restart_after_publish = app_after
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad_new")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events after publish should succeed");
    assert_eq!(after_restart_after_publish.status(), StatusCode::OK);
    let after_restart_after_publish_body = after_restart_after_publish
        .into_body()
        .collect()
        .await
        .expect("events after publish body should collect")
        .to_bytes();
    let after_restart_after_publish_json: serde_json::Value =
        serde_json::from_slice(&after_restart_after_publish_body)
            .expect("events after publish body should be valid json");
    let items = after_restart_after_publish_json["items"]
        .as_array()
        .expect("items should be an array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["realtimeSeq"], 2);
    assert_eq!(after_restart_after_publish_json["ackedThroughSeq"], 1);
    assert_eq!(after_restart_after_publish_json["trimmedThroughSeq"], 1);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_default_local_minimal_profile_restores_unacked_realtime_events_after_runtime_rebuild()
{
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_before = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let create_conversation = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_realtime_inbox_restart",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let register_pad = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register pad should succeed");
    assert_eq!(register_pad.status(), StatusCode::OK);

    let sync_subscriptions = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_realtime_inbox_restart",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let post_message = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_realtime_inbox_restart/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_realtime_inbox_restart_1",
                        "summary":"durable inbox",
                        "text":"durable inbox"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let app_after = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let register_pad_after = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad_restarted")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register pad after restart should succeed");
    assert_eq!(register_pad_after.status(), StatusCode::OK);

    let restored_events = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad_restarted")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("restored realtime events should succeed");
    assert_eq!(restored_events.status(), StatusCode::OK);
    let restored_events_body = restored_events
        .into_body()
        .collect()
        .await
        .expect("restored realtime body should collect")
        .to_bytes();
    let restored_events_json: serde_json::Value = serde_json::from_slice(&restored_events_body)
        .expect("restored realtime body should be valid json");
    let restored_items = restored_events_json["items"]
        .as_array()
        .expect("restored items should be an array");
    assert_eq!(
        restored_items.len(),
        1,
        "unacked device realtime event must survive runtime rebuild"
    );
    assert_eq!(restored_items[0]["realtimeSeq"], 1);
    assert_eq!(restored_items[0]["scopeId"], "c_realtime_inbox_restart");
    assert_eq!(restored_items[0]["eventType"], "message.posted");
    assert_eq!(restored_events_json["ackedThroughSeq"], 0);
    assert_eq!(restored_events_json["trimmedThroughSeq"], 0);

    let ack_response = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/events/ack")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad_restarted")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"ackedSeq":1}"#))
                .unwrap(),
        )
        .await
        .expect("ack restored event should succeed");
    assert_eq!(ack_response.status(), StatusCode::OK);

    let after_ack = app_after
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad_restarted")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("events after ack should succeed");
    assert_eq!(after_ack.status(), StatusCode::OK);
    let after_ack_body = after_ack
        .into_body()
        .collect()
        .await
        .expect("after ack body should collect")
        .to_bytes();
    let after_ack_json: serde_json::Value =
        serde_json::from_slice(&after_ack_body).expect("after ack body should be valid json");
    assert_eq!(after_ack_json["items"].as_array().unwrap().len(), 0);
    assert_eq!(after_ack_json["ackedThroughSeq"], 1);
    assert_eq!(after_ack_json["trimmedThroughSeq"], 1);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_default_local_minimal_ops_diagnostics_exposes_durable_realtime_inbox_backlog() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_realtime_inbox_diagnostics",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let register_pad = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register pad should succeed");
    assert_eq!(register_pad.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_realtime_inbox_diagnostics",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_realtime_inbox_diagnostics/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_realtime_inbox_diagnostics_1",
                        "summary":"diagnostics",
                        "text":"diagnostics"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let diagnostics_with_backlog = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/diagnostics")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops diagnostics with backlog should return response");
    assert_eq!(diagnostics_with_backlog.status(), StatusCode::OK);
    let diagnostics_with_backlog_body = diagnostics_with_backlog
        .into_body()
        .collect()
        .await
        .expect("diagnostics with backlog body should collect")
        .to_bytes();
    let diagnostics_with_backlog_json: serde_json::Value =
        serde_json::from_slice(&diagnostics_with_backlog_body)
            .expect("diagnostics with backlog should be valid json");
    assert_eq!(
        diagnostics_with_backlog_json["realtimeInbox"]["status"],
        "degraded"
    );
    assert_eq!(
        diagnostics_with_backlog_json["realtimeInbox"]["deviceWindowCount"],
        1
    );
    assert_eq!(
        diagnostics_with_backlog_json["realtimeInbox"]["pendingEventCount"],
        1
    );
    assert_eq!(
        diagnostics_with_backlog_json["realtimeInbox"]["maxClientRouteWindowEventCount"],
        1
    );
    assert_eq!(
        diagnostics_with_backlog_json["realtimeInbox"]["deviceWindowCapacity"],
        1000
    );
    assert_eq!(
        diagnostics_with_backlog_json["realtimeInbox"]["maxClientRouteWindowUsagePermille"],
        1
    );
    assert_eq!(
        diagnostics_with_backlog_json["realtimeInbox"]["maxTrimmedThroughSeq"],
        0
    );
    assert_eq!(
        diagnostics_with_backlog_json["realtimeInbox"]["capacityTrimmedEventCount"],
        0
    );
    assert_eq!(
        diagnostics_with_backlog_json["realtimeInbox"]["maxCapacityTrimmedThroughSeq"],
        0
    );
    assert!(
        diagnostics_with_backlog_json["realtimeInbox"]["lastCapacityTrimmedAt"].is_null(),
        "ops diagnostics should not report capacity trimming for a normal one-event backlog"
    );
    assert!(
        diagnostics_with_backlog_json["realtimeInbox"]["oldestPendingOccurredAt"].is_string(),
        "ops diagnostics should expose the oldest unacked realtime event timestamp"
    );
    assert_eq!(
        diagnostics_with_backlog_json["realtimeInbox"]["highRiskWindows"]
            .as_array()
            .expect("highRiskWindows should be an array")
            .len(),
        1
    );
    assert_eq!(
        diagnostics_with_backlog_json["realtimeInbox"]["highRiskWindows"][0]["tenantId"],
        "t_demo"
    );
    assert_eq!(
        diagnostics_with_backlog_json["realtimeInbox"]["highRiskWindows"][0]["principalKind"],
        "user"
    );
    assert_eq!(
        diagnostics_with_backlog_json["realtimeInbox"]["highRiskWindows"][0]["principalId"],
        "u_demo"
    );
    assert_eq!(
        diagnostics_with_backlog_json["realtimeInbox"]["highRiskWindows"][0]["deviceId"],
        "d_pad"
    );
    assert_eq!(
        diagnostics_with_backlog_json["realtimeInbox"]["highRiskWindows"][0]["pendingEventCount"],
        1
    );
    assert!(
        diagnostics_with_backlog_json["realtimeInbox"]["highRiskWindows"][0]
            .get("payload")
            .is_none(),
        "ops realtime inbox diagnostics must not expose event payloads"
    );

    let health_with_backlog = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/health")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops health with backlog should return response");
    assert_eq!(health_with_backlog.status(), StatusCode::OK);
    let health_with_backlog_body = health_with_backlog
        .into_body()
        .collect()
        .await
        .expect("health with backlog body should collect")
        .to_bytes();
    let health_with_backlog_json: serde_json::Value =
        serde_json::from_slice(&health_with_backlog_body)
            .expect("health with backlog should be valid json");
    assert_eq!(
        health_with_backlog_json["realtimeInbox"]["status"],
        "degraded"
    );
    assert_eq!(
        health_with_backlog_json["realtimeInbox"]["pendingEventCount"],
        1
    );
    assert_eq!(
        health_with_backlog_json["realtimeInbox"]["maxClientRouteWindowUsagePermille"],
        1
    );

    let ack_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/events/ack")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"ackedSeq":1}"#))
                .unwrap(),
        )
        .await
        .expect("ack request should succeed");
    assert_eq!(ack_response.status(), StatusCode::OK);

    let diagnostics_after_ack = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/diagnostics")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops diagnostics after ack should return response");
    assert_eq!(diagnostics_after_ack.status(), StatusCode::OK);
    let diagnostics_after_ack_body = diagnostics_after_ack
        .into_body()
        .collect()
        .await
        .expect("diagnostics after ack body should collect")
        .to_bytes();
    let diagnostics_after_ack_json: serde_json::Value =
        serde_json::from_slice(&diagnostics_after_ack_body)
            .expect("diagnostics after ack should be valid json");
    assert_eq!(diagnostics_after_ack_json["realtimeInbox"]["status"], "ok");
    assert_eq!(
        diagnostics_after_ack_json["realtimeInbox"]["deviceWindowCount"],
        1
    );
    assert_eq!(
        diagnostics_after_ack_json["realtimeInbox"]["pendingEventCount"],
        0
    );
    assert_eq!(
        diagnostics_after_ack_json["realtimeInbox"]["maxClientRouteWindowEventCount"],
        0
    );
    assert_eq!(
        diagnostics_after_ack_json["realtimeInbox"]["deviceWindowCapacity"],
        1000
    );
    assert_eq!(
        diagnostics_after_ack_json["realtimeInbox"]["maxClientRouteWindowUsagePermille"],
        0
    );
    assert_eq!(
        diagnostics_after_ack_json["realtimeInbox"]["maxTrimmedThroughSeq"],
        1
    );
    assert_eq!(
        diagnostics_after_ack_json["realtimeInbox"]["capacityTrimmedEventCount"],
        0
    );
    assert_eq!(
        diagnostics_after_ack_json["realtimeInbox"]["maxCapacityTrimmedThroughSeq"],
        0
    );
    assert!(
        diagnostics_after_ack_json["realtimeInbox"]["lastCapacityTrimmedAt"].is_null(),
        "ACK trimming should not be reported as capacity pressure"
    );
    assert!(
        diagnostics_after_ack_json["realtimeInbox"]["oldestPendingOccurredAt"].is_null(),
        "ops diagnostics should clear the oldest pending timestamp after ack trim"
    );
    assert_eq!(
        diagnostics_after_ack_json["realtimeInbox"]["highRiskWindows"]
            .as_array()
            .expect("highRiskWindows after ack should be an array")
            .len(),
        0
    );

    let health_after_ack = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/health")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops health after ack should return response");
    assert_eq!(health_after_ack.status(), StatusCode::OK);
    let health_after_ack_body = health_after_ack
        .into_body()
        .collect()
        .await
        .expect("health after ack body should collect")
        .to_bytes();
    let health_after_ack_json: serde_json::Value = serde_json::from_slice(&health_after_ack_body)
        .expect("health after ack should be valid json");
    assert_eq!(health_after_ack_json["realtimeInbox"]["status"], "ok");
    assert_eq!(
        health_after_ack_json["realtimeInbox"]["pendingEventCount"],
        0
    );

    let _ = fs::remove_dir_all(runtime_dir);
}
