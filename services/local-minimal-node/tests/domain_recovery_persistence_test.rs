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
    std::env::temp_dir().join(format!("craw_chat_domain_recovery_runtime_{unique}"))
}

#[tokio::test]
async fn test_default_local_minimal_profile_rebuild_restores_conversation_domain_state() {
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
                        "conversationId":"c_domain_restart",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_first_message = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_domain_restart/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_domain_restart_1",
                        "summary":"first",
                        "text":"first"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first post should succeed");
    assert_eq!(post_first_message.status(), StatusCode::OK);

    let app_after = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let summary_after_restart = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_domain_restart")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone_new")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("conversation summary after restart should return a response");
    assert_eq!(summary_after_restart.status(), StatusCode::OK);
    let summary_after_restart_body = summary_after_restart
        .into_body()
        .collect()
        .await
        .expect("conversation summary after restart body should collect")
        .to_bytes();
    let summary_after_restart_json: serde_json::Value =
        serde_json::from_slice(&summary_after_restart_body)
            .expect("conversation summary after restart should be valid json");
    assert_eq!(summary_after_restart_json["messageCount"], 1);
    assert_eq!(
        summary_after_restart_json["lastMessageId"],
        "msg_c_domain_restart_1"
    );

    let members_after_restart = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_domain_restart/members")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone_new")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("members after restart should return a response");
    assert_eq!(members_after_restart.status(), StatusCode::OK);

    let post_second_message = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_domain_restart/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone_new")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_domain_restart_2",
                        "summary":"second",
                        "text":"second"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second post after restart should return a response");
    assert_eq!(post_second_message.status(), StatusCode::OK);

    let timeline_after_restart = app_after
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_domain_restart/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone_new")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline after restart should return a response");
    assert_eq!(timeline_after_restart.status(), StatusCode::OK);
    let timeline_after_restart_body = timeline_after_restart
        .into_body()
        .collect()
        .await
        .expect("timeline after restart body should collect")
        .to_bytes();
    let timeline_after_restart_json: serde_json::Value =
        serde_json::from_slice(&timeline_after_restart_body)
            .expect("timeline after restart should be valid json");
    let items = timeline_after_restart_json["items"]
        .as_array()
        .expect("timeline items should be an array");
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["messageId"], "msg_c_domain_restart_1");
    assert_eq!(items[1]["messageId"], "msg_c_domain_restart_2");

    let _ = fs::remove_dir_all(runtime_dir);
}
