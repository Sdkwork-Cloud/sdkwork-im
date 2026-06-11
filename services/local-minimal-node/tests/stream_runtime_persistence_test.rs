use im_app_context::DualTokenRequestBuilderExt;
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
    std::env::temp_dir().join(format!("craw_chat_stream_runtime_recovery_{unique}"))
}

#[tokio::test]
async fn test_default_local_minimal_profile_restores_stream_runtime_state_after_rebuild() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_before = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let create_conversation = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_stream_restart",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let open_stream = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_stream_restart",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_restart",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let append_frame = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_stream_restart/frames")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame should succeed");
    assert_eq!(append_frame.status(), StatusCode::OK);

    let stream_state_file = runtime_dir.join("state").join("stream-state.json");
    assert!(
        stream_state_file.exists(),
        "default local-minimal runtime should persist stream state under the runtime state dir"
    );

    let app_after = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let frames_after_restart = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/streams/st_stream_restart/frames?afterFrameSeq=0&limit=10")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone_new")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list stream frames after restart should succeed");
    assert_eq!(frames_after_restart.status(), StatusCode::OK);
    let frames_after_restart_body = frames_after_restart
        .into_body()
        .collect()
        .await
        .expect("frame list body should collect")
        .to_bytes();
    let frames_after_restart_json: serde_json::Value =
        serde_json::from_slice(&frames_after_restart_body)
            .expect("frame list should be valid json");
    let items = frames_after_restart_json["items"]
        .as_array()
        .expect("items should be an array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["frameSeq"], 1);

    let complete_stream = app_after
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_stream_restart/complete")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone_new")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 2,
                        "resultMessageId":"msg_stream_restart_result"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete stream after restart should succeed");
    assert_eq!(complete_stream.status(), StatusCode::OK);
    let complete_stream_body = complete_stream
        .into_body()
        .collect()
        .await
        .expect("complete body should collect")
        .to_bytes();
    let complete_stream_json: serde_json::Value = serde_json::from_slice(&complete_stream_body)
        .expect("complete response should be valid json");
    assert_eq!(complete_stream_json["state"], "completed");
    assert_eq!(complete_stream_json["lastFrameSeq"], 2);
    assert_eq!(
        complete_stream_json["resultMessageId"],
        "msg_stream_restart_result"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}
