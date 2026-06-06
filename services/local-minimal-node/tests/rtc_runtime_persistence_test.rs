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
    std::env::temp_dir().join(format!("craw_chat_rtc_runtime_recovery_{unique}"))
}

#[tokio::test]
async fn test_default_local_minimal_profile_restores_rtc_runtime_state_after_rebuild() {
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
                        "conversationId":"c_rtc_restart",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let create_rtc = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/rtc/sessions")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_restart_demo",
                        "conversationId":"c_rtc_restart",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session should succeed");
    assert_eq!(create_rtc.status(), StatusCode::OK);

    let invite_rtc = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/rtc/sessions/rtc_restart_demo/invite")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalingStreamId":"st_rtc_restart"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("invite rtc should succeed");
    assert_eq!(invite_rtc.status(), StatusCode::OK);

    let rtc_state_file = runtime_dir.join("state").join("rtc-state.json");
    assert!(
        rtc_state_file.exists(),
        "default local-minimal runtime should persist rtc state under the runtime state dir"
    );

    let app_after = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let accept_rtc = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/rtc/sessions/rtc_restart_demo/accept")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone_new")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_rtc_restart_accept"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("accept rtc after rebuild should return response");
    assert_eq!(accept_rtc.status(), StatusCode::OK);
    let accept_body = accept_rtc
        .into_body()
        .collect()
        .await
        .expect("accept body should collect")
        .to_bytes();
    let accept_json: serde_json::Value =
        serde_json::from_slice(&accept_body).expect("accept response should be valid json");
    assert_eq!(accept_json["state"], "accepted");
    assert_eq!(accept_json["signalingStreamId"], "st_rtc_restart");
    assert_eq!(accept_json["artifactMessageId"], "msg_rtc_restart_accept");

    let custom_signal = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/rtc/sessions/rtc_restart_demo/signals")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone_new")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalType":"rtc.answer",
                        "schemaRef":"webrtc.answer.v1",
                        "payload":"{\"sdp\":\"answer\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("rtc signal after rebuild should return response");
    assert_eq!(custom_signal.status(), StatusCode::OK);
    let custom_signal_body = custom_signal
        .into_body()
        .collect()
        .await
        .expect("custom signal body should collect")
        .to_bytes();
    let custom_signal_json: serde_json::Value = serde_json::from_slice(&custom_signal_body)
        .expect("custom signal response should be valid json");
    assert_eq!(custom_signal_json["signalType"], "rtc.answer");
    assert_eq!(custom_signal_json["signalingStreamId"], "st_rtc_restart");

    let end_rtc = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/rtc/sessions/rtc_restart_demo/end")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone_new")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_rtc_restart_end"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("end rtc after rebuild should return response");
    assert_eq!(end_rtc.status(), StatusCode::OK);

    let timeline = app_after
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_rtc_restart/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone_new")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline should return response");
    assert_eq!(timeline.status(), StatusCode::OK);
    let timeline_body = timeline
        .into_body()
        .collect()
        .await
        .expect("timeline body should collect")
        .to_bytes();
    let timeline_json: serde_json::Value =
        serde_json::from_slice(&timeline_body).expect("timeline should be valid json");
    let items = timeline_json["items"]
        .as_array()
        .expect("items should be an array");
    assert_eq!(items.len(), 4);
    assert_eq!(items[0]["summary"], "rtc.invite");
    assert_eq!(items[1]["summary"], "rtc.accept");
    assert_eq!(items[2]["summary"], "rtc.answer");
    assert_eq!(items[3]["summary"], "rtc.end");

    let _ = fs::remove_dir_all(runtime_dir);
}
