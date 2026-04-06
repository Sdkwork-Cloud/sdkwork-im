use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

const DEMO_BEARER: &str = "Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0ZW5hbnRfaWQiOiJ0X2RlbW8iLCJzdWIiOiJ1X2RlbW8iLCJzaWQiOiJzX2RlbW8ifQ.";

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "craw_chat_live_subscription_recovery_runtime_{unique}"
    ))
}

#[tokio::test]
async fn test_default_local_minimal_profile_restores_live_subscriptions_after_rebuild_with_fresh_resume()
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
                        "conversationId":"c_live_sub_restart",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let resume_before = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"deviceId":"d_pad","lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("resume before restart should succeed");
    assert_eq!(resume_before.status(), StatusCode::OK);

    let sync_before = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/realtime/subscriptions/sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_live_sub_restart",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync before restart should succeed");
    assert_eq!(sync_before.status(), StatusCode::OK);

    let app_after = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let resume_after = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"deviceId":"d_pad","lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("fresh resume after restart should succeed");
    assert_eq!(resume_after.status(), StatusCode::OK);

    let post_message = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_live_sub_restart/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone_new")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_live_sub_restart_1",
                        "summary":"first",
                        "text":"first"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post after restart should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let realtime_events = app_after
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events after restart should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    let items = realtime_events_json["items"]
        .as_array()
        .expect("items should be an array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["scopeId"], "c_live_sub_restart");
    assert_eq!(items[0]["eventType"], "message.posted");

    let _ = fs::remove_dir_all(runtime_dir);
}
