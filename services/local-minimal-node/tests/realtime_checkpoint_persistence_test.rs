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
                .uri("/api/v1/conversations")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone")
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
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad")
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
                .uri("/api/v1/realtime/subscriptions/sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad")
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
                .uri("/api/v1/conversations/c_checkpoint_restart/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone")
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
                .uri("/api/v1/realtime/events/ack")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad")
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
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad_new")
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
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad_new")
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
                .uri("/api/v1/realtime/subscriptions/sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad_new")
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
                .uri("/api/v1/conversations/c_checkpoint_restart/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone_new")
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
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad_new")
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
