use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_adapters_local_memory::MemoryRealtimeDisconnectFenceStore;
use projection_service::TimelineProjectionService;
use std::sync::Arc;
use tower::ServiceExt;

const DEMO_BEARER: &str = "Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0ZW5hbnRfaWQiOiJ0X2RlbW8iLCJzdWIiOiJ1X2RlbW8iLCJzaWQiOiJzX2RlbW8ifQ.";
const OTHER_BEARER: &str = "Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0ZW5hbnRfaWQiOiJ0X290aGVyIiwic3ViIjoidV9vdGhlciIsInNpZCI6InNfb3RoZXIifQ.";

#[tokio::test]
async fn test_local_minimal_profile_runs_end_to_end_flow() {
    let app = local_minimal_node::build_default_app();

    let health = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("healthz should succeed");
    assert_eq!(health.status(), StatusCode::OK);

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_demo",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_demo/messages")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_demo",
                        "summary":"hello",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let timeline = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_demo/messages")
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline should succeed");
    assert_eq!(timeline.status(), StatusCode::OK);
    let timeline_body = timeline
        .into_body()
        .collect()
        .await
        .expect("timeline body should collect")
        .to_bytes();
    let timeline_json: serde_json::Value =
        serde_json::from_slice(&timeline_body).expect("timeline should be valid json");
    assert_eq!(timeline_json["items"][0]["messageId"], "msg_c_demo_1");
    assert_eq!(timeline_json["items"][0]["summary"], "hello");

    let conversation_summary = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_demo")
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("conversation summary should succeed");
    assert_eq!(conversation_summary.status(), StatusCode::OK);
    let conversation_summary_body = conversation_summary
        .into_body()
        .collect()
        .await
        .expect("conversation summary body should collect")
        .to_bytes();
    let conversation_summary_json: serde_json::Value =
        serde_json::from_slice(&conversation_summary_body)
            .expect("conversation summary should be valid json");
    assert_eq!(conversation_summary_json["lastMessageId"], "msg_c_demo_1");
    assert_eq!(conversation_summary_json["messageCount"], 1);

    let create_conversation_other_tenant = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", OTHER_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_demo",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation for other tenant should succeed");
    assert_eq!(create_conversation_other_tenant.status(), StatusCode::OK);

    let post_message_other_tenant = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_demo/messages")
                .header("authorization", OTHER_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_other",
                        "summary":"other",
                        "text":"other"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message for other tenant should succeed");
    assert_eq!(post_message_other_tenant.status(), StatusCode::OK);

    let other_timeline = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_demo/messages")
                .header("authorization", OTHER_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("other tenant timeline should succeed");
    assert_eq!(other_timeline.status(), StatusCode::OK);
    let other_timeline_body = other_timeline
        .into_body()
        .collect()
        .await
        .expect("other tenant timeline body should collect")
        .to_bytes();
    let other_timeline_json: serde_json::Value =
        serde_json::from_slice(&other_timeline_body).expect("other timeline should be valid json");
    assert_eq!(other_timeline_json["items"][0]["summary"], "other");
    assert_eq!(other_timeline_json["items"].as_array().unwrap().len(), 1);

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_demo",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);
    let open_stream_body = open_stream
        .into_body()
        .collect()
        .await
        .expect("open stream body should collect")
        .to_bytes();
    let open_stream_json: serde_json::Value =
        serde_json::from_slice(&open_stream_body).expect("open stream should be valid json");
    assert_eq!(open_stream_json["state"], "opened");

    let checkpoint_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_demo/checkpoint")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 3
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("checkpoint stream should succeed");
    assert_eq!(checkpoint_stream.status(), StatusCode::OK);

    let complete_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_demo/complete")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 5,
                        "resultMessageId": "msg_c_demo_1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete stream should succeed");
    assert_eq!(complete_stream.status(), StatusCode::OK);

    let open_abort_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_abort",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open abort stream should succeed");
    assert_eq!(open_abort_stream.status(), StatusCode::OK);

    let abort_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_abort/abort")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 2,
                        "reason": "client_cancelled"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("abort stream should succeed");
    assert_eq!(abort_stream.status(), StatusCode::OK);
    let abort_stream_body = abort_stream
        .into_body()
        .collect()
        .await
        .expect("abort stream body should collect")
        .to_bytes();
    let abort_stream_json: serde_json::Value =
        serde_json::from_slice(&abort_stream_body).expect("abort stream should be valid json");
    assert_eq!(abort_stream_json["state"], "aborted");
    assert_eq!(abort_stream_json["lastFrameSeq"], 2);
    assert_eq!(
        abort_stream_json["resultMessageId"],
        serde_json::Value::Null
    );

    let create_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_demo",
                        "conversationId":"c_demo",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session should succeed");
    assert_eq!(create_rtc.status(), StatusCode::OK);

    let invite_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_demo/invite")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalingStreamId": "st_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("invite rtc should succeed");
    assert_eq!(invite_rtc.status(), StatusCode::OK);

    let custom_signal = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_demo/signals")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalType":"rtc.offer",
                        "schemaRef":"webrtc.offer.v1",
                        "payload":"{\"sdp\":\"demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("custom signal should succeed");
    assert_eq!(custom_signal.status(), StatusCode::OK);

    let accept_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_demo/accept")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId": "msg_accept"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("accept rtc should succeed");
    assert_eq!(accept_rtc.status(), StatusCode::OK);

    let end_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_demo/end")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId": "msg_end"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("end rtc should succeed");
    assert_eq!(end_rtc.status(), StatusCode::OK);

    let create_media_upload = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "mediaAssetId":"ma_demo",
                        "resource":{
                            "uuid":"res_demo",
                            "type":"image",
                            "mimeType":"image/png",
                            "size":42,
                            "name":"demo.png",
                            "extension":"png",
                            "metadata":{"origin":"e2e"},
                            "prompt":"poster"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create media upload should succeed");
    assert_eq!(create_media_upload.status(), StatusCode::OK);

    let complete_media_upload = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads/ma_demo/complete")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "bucket":"local-media",
                        "objectKey":"tenant/t_demo/ma_demo/demo.png",
                        "storageProvider":"local",
                        "url":"https://cdn.example.com/ma_demo/demo.png",
                        "checksum":"sha256:demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete media upload should succeed");
    assert_eq!(complete_media_upload.status(), StatusCode::OK);

    let attach_media = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/ma_demo/attach")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_demo",
                        "clientMsgId":"client_attach",
                        "summary":"poster asset",
                        "text":"see attachment"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("attach media should succeed");
    assert_eq!(attach_media.status(), StatusCode::OK);

    let get_media = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/media/ma_demo")
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get media should succeed");
    assert_eq!(get_media.status(), StatusCode::OK);

    let conversation_summary_after_attach = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_demo")
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("conversation summary after attach should succeed");
    assert_eq!(conversation_summary_after_attach.status(), StatusCode::OK);
    let conversation_summary_after_attach_body = conversation_summary_after_attach
        .into_body()
        .collect()
        .await
        .expect("conversation summary after attach body should collect")
        .to_bytes();
    let conversation_summary_after_attach_json: serde_json::Value =
        serde_json::from_slice(&conversation_summary_after_attach_body)
            .expect("conversation summary after attach should be valid json");
    assert_eq!(
        conversation_summary_after_attach_json["lastMessageId"],
        "msg_c_demo_6"
    );
    assert_eq!(conversation_summary_after_attach_json["messageCount"], 6);
    assert_eq!(
        conversation_summary_after_attach_json["lastSummary"],
        "poster asset"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_surfaces_media_not_found_error() {
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/media/ma_missing")
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get missing media should return response");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let response_body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let response_json: serde_json::Value =
        serde_json::from_slice(&response_body).expect("response should be valid json");
    assert_eq!(response_json["code"], "media_asset_not_found");
}

#[tokio::test]
async fn test_local_minimal_profile_projects_rtc_state_changes_into_signal_messages() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_rtc_signal",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let create_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_signal_demo",
                        "conversationId":"c_rtc_signal",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session should succeed");
    assert_eq!(create_rtc.status(), StatusCode::OK);

    let invite_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_signal_demo/invite")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalingStreamId":"st_signal_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("invite rtc should succeed");
    assert_eq!(invite_rtc.status(), StatusCode::OK);

    let custom_signal = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_signal_demo/signals")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalType":"rtc.offer",
                        "schemaRef":"webrtc.offer.v1",
                        "payload":"{\"sdp\":\"demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("custom signal should succeed");
    assert_eq!(custom_signal.status(), StatusCode::OK);

    let accept_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_signal_demo/accept")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_rtc_accept"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("accept rtc should succeed");
    assert_eq!(accept_rtc.status(), StatusCode::OK);

    let end_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_signal_demo/end")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_rtc_end"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("end rtc should succeed");
    assert_eq!(end_rtc.status(), StatusCode::OK);

    let timeline = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_rtc_signal/messages")
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline should succeed");
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
    assert_eq!(items[1]["summary"], "rtc.offer");
    assert_eq!(items[2]["summary"], "rtc.accept");
    assert_eq!(items[3]["summary"], "rtc.end");

    let summary = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_rtc_signal")
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("summary should succeed");
    assert_eq!(summary.status(), StatusCode::OK);
    let summary_body = summary
        .into_body()
        .collect()
        .await
        .expect("summary body should collect")
        .to_bytes();
    let summary_json: serde_json::Value =
        serde_json::from_slice(&summary_body).expect("summary should be valid json");
    assert_eq!(summary_json["lastMessageId"], "msg_c_rtc_signal_4");
    assert_eq!(summary_json["messageCount"], 4);
    assert_eq!(summary_json["lastSummary"], "rtc.end");
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_rtc_session_create_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_demo",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_local_idempotent",
                        "conversationId":"c_demo",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first rtc create should succeed");
    assert_eq!(first_create.status(), StatusCode::OK);

    let accept_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_local_idempotent/accept")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_local_rtc_accept"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("accept rtc should succeed");
    assert_eq!(accept_response.status(), StatusCode::OK);

    let idempotent_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_local_idempotent",
                        "conversationId":"c_demo",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("idempotent rtc create should return response");
    assert_eq!(idempotent_create.status(), StatusCode::OK);
    let idempotent_body = idempotent_create
        .into_body()
        .collect()
        .await
        .expect("idempotent rtc create body should collect")
        .to_bytes();
    let idempotent_json: serde_json::Value = serde_json::from_slice(&idempotent_body)
        .expect("idempotent rtc create should be valid json");
    assert_eq!(idempotent_json["state"], "accepted");
    assert_eq!(idempotent_json["artifactMessageId"], "msg_local_rtc_accept");

    let conflicting_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_local_idempotent",
                        "conversationId":"c_other",
                        "rtcMode":"video"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting rtc create should return response");
    assert_eq!(conflicting_create.status(), StatusCode::CONFLICT);
    let conflicting_body = conflicting_create
        .into_body()
        .collect()
        .await
        .expect("conflicting rtc body should collect")
        .to_bytes();
    let conflicting_json: serde_json::Value = serde_json::from_slice(&conflicting_body)
        .expect("conflicting rtc create should be valid json");
    assert_eq!(conflicting_json["code"], "rtc_session_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_suppresses_duplicate_rtc_state_side_effects() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_rtc_state_side_effects",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let create_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_state_side_effects",
                        "conversationId":"c_rtc_state_side_effects",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session should succeed");
    assert_eq!(create_rtc.status(), StatusCode::OK);

    let first_accept = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_state_side_effects/accept")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_state_accept"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first accept should succeed");
    assert_eq!(first_accept.status(), StatusCode::OK);

    let duplicate_accept = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_state_side_effects/accept")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_state_accept"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate accept should return response");
    assert_eq!(duplicate_accept.status(), StatusCode::OK);

    let first_end = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_state_side_effects/end")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_state_end"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first end should succeed");
    assert_eq!(first_end.status(), StatusCode::OK);

    let duplicate_end = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_state_side_effects/end")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_state_end"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate end should return response");
    assert_eq!(duplicate_end.status(), StatusCode::OK);

    let conflicting_reject = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_state_side_effects/reject")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_state_reject_conflict"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting reject should return response");
    assert_eq!(conflicting_reject.status(), StatusCode::CONFLICT);
    let conflicting_reject_body = conflicting_reject
        .into_body()
        .collect()
        .await
        .expect("conflicting reject body should collect")
        .to_bytes();
    let conflicting_reject_json: serde_json::Value =
        serde_json::from_slice(&conflicting_reject_body)
            .expect("conflicting reject should be valid json");
    assert_eq!(
        conflicting_reject_json["code"],
        "rtc_session_state_conflict"
    );

    let timeline = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_rtc_state_side_effects/messages")
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline should succeed");
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
        .expect("timeline items should be array");
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["summary"], "rtc.accept");
    assert_eq!(items[1]["summary"], "rtc.end");
}

#[tokio::test]
async fn test_local_minimal_profile_exposes_conversation_member_management() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_members",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let initial_members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_members/members")
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list initial members should succeed");
    assert_eq!(initial_members.status(), StatusCode::OK);
    let initial_members_body = initial_members
        .into_body()
        .collect()
        .await
        .expect("initial members body should collect")
        .to_bytes();
    let initial_members_json: serde_json::Value = serde_json::from_slice(&initial_members_body)
        .expect("initial members response should be valid json");
    assert_eq!(initial_members_json["items"][0]["principalId"], "u_demo");
    assert_eq!(initial_members_json["items"][0]["role"], "owner");

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_members/members/add")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"ag_demo",
                        "principalKind":"agent",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member should succeed");
    assert_eq!(add_member.status(), StatusCode::OK);
    let add_member_body = add_member
        .into_body()
        .collect()
        .await
        .expect("add member body should collect")
        .to_bytes();
    let add_member_json: serde_json::Value =
        serde_json::from_slice(&add_member_body).expect("add member response should be valid json");
    assert_eq!(add_member_json["memberId"], "cm_c_members_ag_demo");
    assert_eq!(add_member_json["principalKind"], "agent");
    assert_eq!(add_member_json["state"], "joined");

    let members_after_add = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_members/members")
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list members after add should succeed");
    assert_eq!(members_after_add.status(), StatusCode::OK);
    let members_after_add_body = members_after_add
        .into_body()
        .collect()
        .await
        .expect("members after add body should collect")
        .to_bytes();
    let members_after_add_json: serde_json::Value = serde_json::from_slice(&members_after_add_body)
        .expect("members after add response should be valid json");
    assert_eq!(members_after_add_json["items"].as_array().unwrap().len(), 2);

    let remove_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_members/members/remove")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_members_ag_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("remove member should succeed");
    assert_eq!(remove_member.status(), StatusCode::OK);
    let remove_member_body = remove_member
        .into_body()
        .collect()
        .await
        .expect("remove member body should collect")
        .to_bytes();
    let remove_member_json: serde_json::Value = serde_json::from_slice(&remove_member_body)
        .expect("remove member response should be valid json");
    assert_eq!(remove_member_json["state"], "removed");

    let members_after_remove = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_members/members")
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list members after remove should succeed");
    assert_eq!(members_after_remove.status(), StatusCode::OK);
    let members_after_remove_body = members_after_remove
        .into_body()
        .collect()
        .await
        .expect("members after remove body should collect")
        .to_bytes();
    let members_after_remove_json: serde_json::Value =
        serde_json::from_slice(&members_after_remove_body)
            .expect("members after remove response should be valid json");
    assert_eq!(
        members_after_remove_json["items"].as_array().unwrap().len(),
        1
    );
    assert_eq!(
        members_after_remove_json["items"][0]["principalId"],
        "u_demo"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_exposes_read_cursor_and_unread_view() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_cursor",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    for (client_msg_id, summary) in [("client_1", "one"), ("client_2", "two")] {
        let post_message = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/conversations/c_cursor/messages")
                    .header("authorization", DEMO_BEARER)
                    .header("content-type", "application/json")
                    .body(Body::from(format!(
                        r#"{{
                            "clientMsgId":"{client_msg_id}",
                            "summary":"{summary}",
                            "text":"{summary}"
                        }}"#,
                    )))
                    .unwrap(),
            )
            .await
            .expect("post message should succeed");
        assert_eq!(post_message.status(), StatusCode::OK);
    }

    let initial_cursor = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_cursor/read-cursor")
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("initial read cursor should succeed");
    assert_eq!(initial_cursor.status(), StatusCode::OK);
    let initial_cursor_body = initial_cursor
        .into_body()
        .collect()
        .await
        .expect("initial cursor body should collect")
        .to_bytes();
    let initial_cursor_json: serde_json::Value =
        serde_json::from_slice(&initial_cursor_body).expect("initial cursor should be valid json");
    assert_eq!(initial_cursor_json["readSeq"], 0);
    assert_eq!(initial_cursor_json["unreadCount"], 2);

    let update_cursor = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_cursor/read-cursor")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "readSeq":1,
                        "lastReadMessageId":"msg_c_cursor_1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("update read cursor should succeed");
    assert_eq!(update_cursor.status(), StatusCode::OK);
    let update_cursor_body = update_cursor
        .into_body()
        .collect()
        .await
        .expect("update cursor body should collect")
        .to_bytes();
    let update_cursor_json: serde_json::Value =
        serde_json::from_slice(&update_cursor_body).expect("updated cursor should be valid json");
    assert_eq!(update_cursor_json["readSeq"], 1);
    assert_eq!(update_cursor_json["unreadCount"], 1);

    let regressed_cursor = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_cursor/read-cursor")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "readSeq":0,
                        "lastReadMessageId":"msg_c_cursor_0"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("regressed read cursor should succeed");
    assert_eq!(regressed_cursor.status(), StatusCode::OK);
    let regressed_cursor_body = regressed_cursor
        .into_body()
        .collect()
        .await
        .expect("regressed cursor body should collect")
        .to_bytes();
    let regressed_cursor_json: serde_json::Value = serde_json::from_slice(&regressed_cursor_body)
        .expect("regressed cursor response should be valid json");
    assert_eq!(regressed_cursor_json["readSeq"], 1);
    assert_eq!(regressed_cursor_json["unreadCount"], 1);
}

#[tokio::test]
async fn test_local_minimal_profile_exposes_inbox_view() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_inbox",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_inbox/messages")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_inbox_1",
                        "summary":"hello inbox",
                        "text":"hello inbox"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let inbox = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/inbox")
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get inbox should succeed");
    assert_eq!(inbox.status(), StatusCode::OK);
    let inbox_body = inbox
        .into_body()
        .collect()
        .await
        .expect("inbox body should collect")
        .to_bytes();
    let inbox_json: serde_json::Value =
        serde_json::from_slice(&inbox_body).expect("inbox should be valid json");
    assert_eq!(inbox_json["items"][0]["conversationId"], "c_inbox");
    assert_eq!(inbox_json["items"][0]["conversationType"], "group");
    assert_eq!(inbox_json["items"][0]["messageCount"], 1);
    assert_eq!(inbox_json["items"][0]["unreadCount"], 1);

    let update_cursor = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_inbox/read-cursor")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "readSeq":1,
                        "lastReadMessageId":"msg_c_inbox_1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("update read cursor should succeed");
    assert_eq!(update_cursor.status(), StatusCode::OK);

    let inbox_after_read = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/inbox")
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get inbox after read should succeed");
    assert_eq!(inbox_after_read.status(), StatusCode::OK);
    let inbox_after_read_body = inbox_after_read
        .into_body()
        .collect()
        .await
        .expect("inbox after read body should collect")
        .to_bytes();
    let inbox_after_read_json: serde_json::Value = serde_json::from_slice(&inbox_after_read_body)
        .expect("inbox after read should be valid json");
    assert_eq!(inbox_after_read_json["items"][0]["unreadCount"], 0);
}

#[tokio::test]
async fn test_local_minimal_profile_exposes_device_sync_feed_for_multi_device_resume() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_sync_feed",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    for device_id in ["d_phone", "d_pad"] {
        let register = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/devices/register")
                    .header("x-tenant-id", "t_demo")
                    .header("x-user-id", "u_demo")
                    .header("x-device-id", device_id)
                    .header("content-type", "application/json")
                    .body(Body::from(format!(r#"{{"deviceId":"{device_id}"}}"#)))
                    .unwrap(),
            )
            .await
            .expect("device register should succeed");
        assert_eq!(register.status(), StatusCode::OK);
    }

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_sync_feed/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_sync_feed_1",
                        "summary":"hello sync feed",
                        "text":"hello sync feed"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let update_cursor = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_sync_feed/read-cursor")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "readSeq":1,
                        "lastReadMessageId":"msg_c_sync_feed_1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("update read cursor should succeed");
    assert_eq!(update_cursor.status(), StatusCode::OK);

    let sync_feed = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/devices/d_pad/sync-feed?afterSeq=0")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("sync feed should succeed");
    assert_eq!(sync_feed.status(), StatusCode::OK);
    let sync_feed_body = sync_feed
        .into_body()
        .collect()
        .await
        .expect("sync feed body should collect")
        .to_bytes();
    let sync_feed_json: serde_json::Value =
        serde_json::from_slice(&sync_feed_body).expect("sync feed should be valid json");

    let items = sync_feed_json["items"]
        .as_array()
        .expect("items should be an array");
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["originEventType"], "message.posted");
    assert_eq!(items[0]["actorDeviceId"], "d_phone");
    assert_eq!(items[0]["messageId"], "msg_c_sync_feed_1");
    assert_eq!(
        items[1]["originEventType"],
        "conversation.read_cursor_updated"
    );
    assert_eq!(items[1]["readSeq"], 1);
}

#[tokio::test]
async fn test_local_minimal_profile_resumes_session_and_returns_presence_snapshot() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_resume",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    for device_id in ["d_phone", "d_pad"] {
        let register = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/devices/register")
                    .header("x-tenant-id", "t_demo")
                    .header("x-user-id", "u_demo")
                    .header("x-device-id", device_id)
                    .header("content-type", "application/json")
                    .body(Body::from(format!(r#"{{"deviceId":"{device_id}"}}"#)))
                    .unwrap(),
            )
            .await
            .expect("device register should succeed");
        assert_eq!(register.status(), StatusCode::OK);
    }

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_resume/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_phone")
                .header("x-device-id", "d_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_resume_1",
                        "summary":"resume hello",
                        "text":"resume hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let resume = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_pad")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "lastSeenSyncSeq":0
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("resume should succeed");
    assert_eq!(resume.status(), StatusCode::OK);
    let resume_body = resume
        .into_body()
        .collect()
        .await
        .expect("resume body should collect")
        .to_bytes();
    let resume_json: serde_json::Value =
        serde_json::from_slice(&resume_body).expect("resume should be valid json");
    assert_eq!(resume_json["deviceId"], "d_pad");
    assert_eq!(resume_json["resumeRequired"], true);
    assert_eq!(resume_json["resumeFromSyncSeq"], 1);
    assert_eq!(resume_json["latestSyncSeq"], 1);
    assert_eq!(resume_json["presence"]["currentDeviceId"], "d_pad");
    assert_eq!(
        resume_json["presence"]["devices"].as_array().unwrap().len(),
        2
    );
    assert_eq!(resume_json["presence"]["devices"][0]["deviceId"], "d_pad");
    assert_eq!(resume_json["presence"]["devices"][0]["status"], "online");
    assert_eq!(resume_json["presence"]["devices"][1]["status"], "offline");

    let presence = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/presence/me")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_pad")
                .header("x-device-id", "d_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("presence request should succeed");
    assert_eq!(presence.status(), StatusCode::OK);
    let presence_body = presence
        .into_body()
        .collect()
        .await
        .expect("presence body should collect")
        .to_bytes();
    let presence_json: serde_json::Value =
        serde_json::from_slice(&presence_body).expect("presence should be valid json");
    assert_eq!(presence_json["currentDeviceId"], "d_pad");
    assert_eq!(presence_json["devices"][0]["status"], "online");
}

#[tokio::test]
async fn test_local_minimal_profile_disconnects_presence_back_to_offline() {
    let app = local_minimal_node::build_default_app();

    let register = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"deviceId":"d_pad"}"#))
                .unwrap(),
        )
        .await
        .expect("device register should succeed");
    assert_eq!(register.status(), StatusCode::OK);

    let resume = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_pad")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("resume should succeed");
    assert_eq!(resume.status(), StatusCode::OK);

    let heartbeat = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/presence/heartbeat")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_pad")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("heartbeat should succeed");
    assert_eq!(heartbeat.status(), StatusCode::OK);

    let disconnect = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/disconnect")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_pad")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("disconnect should succeed");
    assert_eq!(disconnect.status(), StatusCode::OK);
    let disconnect_body = disconnect
        .into_body()
        .collect()
        .await
        .expect("disconnect body should collect")
        .to_bytes();
    let disconnect_json: serde_json::Value =
        serde_json::from_slice(&disconnect_body).expect("disconnect should be valid json");
    assert_eq!(disconnect_json["devices"][0]["status"], "offline");

    let presence = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/presence/me")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_pad")
                .header("x-device-id", "d_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("presence request should succeed");
    assert_eq!(presence.status(), StatusCode::OK);
    let presence_body = presence
        .into_body()
        .collect()
        .await
        .expect("presence body should collect")
        .to_bytes();
    let presence_json: serde_json::Value =
        serde_json::from_slice(&presence_body).expect("presence should be valid json");
    assert_eq!(presence_json["devices"][0]["status"], "offline");
}

#[tokio::test]
async fn test_local_minimal_profile_requires_fresh_resume_after_disconnect() {
    let app = local_minimal_node::build_default_app();

    let resume_old = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_old")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("resume should succeed");
    assert_eq!(resume_old.status(), StatusCode::OK);

    let disconnect = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/disconnect")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_old")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("disconnect should succeed");
    assert_eq!(disconnect.status(), StatusCode::OK);

    let stale_heartbeat = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/presence/heartbeat")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_old")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("stale heartbeat should return response");
    assert_eq!(stale_heartbeat.status(), StatusCode::CONFLICT);
    let stale_heartbeat_body = stale_heartbeat
        .into_body()
        .collect()
        .await
        .expect("stale heartbeat body should collect")
        .to_bytes();
    let stale_heartbeat_json: serde_json::Value = serde_json::from_slice(&stale_heartbeat_body)
        .expect("stale heartbeat should be valid json");
    assert_eq!(stale_heartbeat_json["code"], "reconnect_required");

    let resume_new = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_new")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("fresh resume should succeed");
    assert_eq!(resume_new.status(), StatusCode::OK);

    let fresh_heartbeat = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/presence/heartbeat")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_new")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("fresh heartbeat should succeed");
    assert_eq!(fresh_heartbeat.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_disconnect_as_idempotent_for_same_session() {
    let app = local_minimal_node::build_default_app();

    let resume = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("resume should succeed");
    assert_eq!(resume.status(), StatusCode::OK);

    let first_disconnect = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/disconnect")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("first disconnect should succeed");
    assert_eq!(first_disconnect.status(), StatusCode::OK);

    let duplicate_disconnect = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/disconnect")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("duplicate disconnect should return response");
    assert_eq!(duplicate_disconnect.status(), StatusCode::OK);
    let duplicate_disconnect_body = duplicate_disconnect
        .into_body()
        .collect()
        .await
        .expect("duplicate disconnect body should collect")
        .to_bytes();
    let duplicate_disconnect_json: serde_json::Value =
        serde_json::from_slice(&duplicate_disconnect_body)
            .expect("duplicate disconnect should be valid json");
    assert_eq!(duplicate_disconnect_json["devices"][0]["status"], "offline");
}

#[tokio::test]
async fn test_local_minimal_profile_rebuild_preserves_reconnect_required_fence_until_fresh_resume()
{
    let shared_store = Arc::new(MemoryRealtimeDisconnectFenceStore::default());
    let app_before = local_minimal_node::build_app_with_dependencies(
        "node_before_restart",
        "127.0.0.1:18090",
        Arc::new(TimelineProjectionService::default()),
        Arc::new(
            session_gateway::RealtimeClusterBridge::with_disconnect_fence_store(
                shared_store.clone(),
            ),
        ),
    );

    let resume_old = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_old")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("old resume should succeed before restart");
    assert_eq!(resume_old.status(), StatusCode::OK);

    let disconnect = app_before
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/disconnect")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_old")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("disconnect should succeed before restart");
    assert_eq!(disconnect.status(), StatusCode::OK);

    let app_after = local_minimal_node::build_app_with_dependencies(
        "node_after_restart",
        "127.0.0.1:18091",
        Arc::new(TimelineProjectionService::default()),
        Arc::new(session_gateway::RealtimeClusterBridge::with_disconnect_fence_store(shared_store)),
    );

    let stale_heartbeat = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/presence/heartbeat")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_old")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("stale heartbeat should return response after restart");
    assert_eq!(stale_heartbeat.status(), StatusCode::CONFLICT);
    let stale_heartbeat_body = stale_heartbeat
        .into_body()
        .collect()
        .await
        .expect("stale heartbeat body should collect")
        .to_bytes();
    let stale_heartbeat_json: serde_json::Value = serde_json::from_slice(&stale_heartbeat_body)
        .expect("stale heartbeat should be valid json");
    assert_eq!(stale_heartbeat_json["code"], "reconnect_required");

    let resume_new = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_new")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("fresh resume should clear restored fence");
    assert_eq!(resume_new.status(), StatusCode::OK);

    let fresh_heartbeat = app_after
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/presence/heartbeat")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_new")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("fresh heartbeat should succeed after restored fence clears");
    assert_eq!(fresh_heartbeat.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_local_minimal_profile_edits_and_recalls_message_with_sync_feed_projection() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_message_mutation",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    for device_id in ["d_phone", "d_pad"] {
        let register = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/devices/register")
                    .header("x-tenant-id", "t_demo")
                    .header("x-user-id", "u_demo")
                    .header("x-device-id", device_id)
                    .header("content-type", "application/json")
                    .body(Body::from(format!(r#"{{"deviceId":"{device_id}"}}"#)))
                    .unwrap(),
            )
            .await
            .expect("device register should succeed");
        assert_eq!(register.status(), StatusCode::OK);
    }

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_message_mutation/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_message_mutation",
                        "summary":"hello",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let edit_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/messages/msg_c_message_mutation_1/edit")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "summary":"edited",
                        "text":"edited"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("edit message should succeed");
    assert_eq!(edit_message.status(), StatusCode::OK);

    let timeline_after_edit = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_message_mutation/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline after edit should succeed");
    assert_eq!(timeline_after_edit.status(), StatusCode::OK);
    let timeline_after_edit_body = timeline_after_edit
        .into_body()
        .collect()
        .await
        .expect("timeline after edit body should collect")
        .to_bytes();
    let timeline_after_edit_json: serde_json::Value =
        serde_json::from_slice(&timeline_after_edit_body)
            .expect("timeline after edit should be valid json");
    assert_eq!(timeline_after_edit_json["items"][0]["summary"], "edited");

    let recall_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/messages/msg_c_message_mutation_1/recall")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("recall message should succeed");
    assert_eq!(recall_message.status(), StatusCode::OK);

    let timeline_after_recall = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_message_mutation/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline after recall should succeed");
    assert_eq!(timeline_after_recall.status(), StatusCode::OK);
    let timeline_after_recall_body = timeline_after_recall
        .into_body()
        .collect()
        .await
        .expect("timeline after recall body should collect")
        .to_bytes();
    let timeline_after_recall_json: serde_json::Value =
        serde_json::from_slice(&timeline_after_recall_body)
            .expect("timeline after recall should be valid json");
    assert_eq!(
        timeline_after_recall_json["items"][0]["summary"],
        "[recalled]"
    );

    let sync_feed = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/devices/d_pad/sync-feed?afterSeq=0")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("sync feed should succeed");
    assert_eq!(sync_feed.status(), StatusCode::OK);
    let sync_feed_body = sync_feed
        .into_body()
        .collect()
        .await
        .expect("sync feed body should collect")
        .to_bytes();
    let sync_feed_json: serde_json::Value =
        serde_json::from_slice(&sync_feed_body).expect("sync feed should be valid json");
    let items = sync_feed_json["items"]
        .as_array()
        .expect("items should be array");
    assert_eq!(items[0]["originEventType"], "message.posted");
    assert_eq!(items[1]["originEventType"], "message.edited");
    assert_eq!(items[2]["originEventType"], "message.recalled");
}

#[tokio::test]
async fn test_local_minimal_profile_fanouts_message_notifications_to_other_active_members_only() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_notification_fanout",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_notification_fanout/members/add")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_member",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member should succeed");
    assert_eq!(add_member.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_notification_fanout/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_notification_fanout",
                        "summary":"hello member",
                        "text":"hello member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let owner_notifications = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("owner notifications should succeed");
    assert_eq!(owner_notifications.status(), StatusCode::OK);
    let owner_notifications_body = owner_notifications
        .into_body()
        .collect()
        .await
        .expect("owner notifications body should collect")
        .to_bytes();
    let owner_notifications_json: serde_json::Value =
        serde_json::from_slice(&owner_notifications_body)
            .expect("owner notifications should be valid json");
    assert_eq!(
        owner_notifications_json["items"]
            .as_array()
            .expect("owner items should be array")
            .len(),
        0
    );

    let member_notifications = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_member")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("member notifications should succeed");
    assert_eq!(member_notifications.status(), StatusCode::OK);
    let member_notifications_body = member_notifications
        .into_body()
        .collect()
        .await
        .expect("member notifications body should collect")
        .to_bytes();
    let member_notifications_json: serde_json::Value =
        serde_json::from_slice(&member_notifications_body)
            .expect("member notifications should be valid json");
    let items = member_notifications_json["items"]
        .as_array()
        .expect("member items should be array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["category"], "message.new");
    assert_eq!(items[0]["recipientId"], "u_member");
    assert_eq!(items[0]["sourceEventType"], "message.posted");
    assert_eq!(items[0]["title"], "hello member");
}

#[tokio::test]
async fn test_local_minimal_profile_exposes_generic_stream_frame_transport() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_stream_frames",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_frames_demo",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_frames",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_frames_demo/frames")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hel\"}",
                        "attributes": {
                            "topic": "llm"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame should succeed");
    assert_eq!(append_frame.status(), StatusCode::OK);
    let append_frame_body = append_frame
        .into_body()
        .collect()
        .await
        .expect("append frame body should collect")
        .to_bytes();
    let append_frame_json: serde_json::Value =
        serde_json::from_slice(&append_frame_body).expect("append frame should be valid json");
    assert_eq!(append_frame_json["streamId"], "st_frames_demo");
    assert_eq!(append_frame_json["frameSeq"], 1);
    assert_eq!(append_frame_json["sender"]["id"], "u_demo");

    let second_append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_frames_demo/frames")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 2,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"lo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second append frame should succeed");
    assert_eq!(second_append_frame.status(), StatusCode::OK);

    let list_frames = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/streams/st_frames_demo/frames?afterFrameSeq=0&limit=10")
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list frames should succeed");
    assert_eq!(list_frames.status(), StatusCode::OK);
    let list_frames_body = list_frames
        .into_body()
        .collect()
        .await
        .expect("list frames body should collect")
        .to_bytes();
    let list_frames_json: serde_json::Value =
        serde_json::from_slice(&list_frames_body).expect("list frames should be valid json");
    assert_eq!(list_frames_json["items"].as_array().unwrap().len(), 2);
    assert_eq!(list_frames_json["items"][0]["frameSeq"], 1);
    assert_eq!(list_frames_json["items"][1]["frameSeq"], 2);
    assert_eq!(list_frames_json["items"][0]["attributes"]["topic"], "llm");
    assert_eq!(list_frames_json["nextAfterFrameSeq"], 2);
    assert_eq!(list_frames_json["hasMore"], false);
}

#[tokio::test]
async fn test_local_minimal_profile_delivers_realtime_events_to_subscribed_device_window() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
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
                        "conversationId":"c_realtime",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let register_phone = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register phone should succeed");
    assert_eq!(register_phone.status(), StatusCode::OK);

    let register_pad = app
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

    let sync_subscriptions = app
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
                                "scopeId":"c_realtime",
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
                .uri("/api/v1/conversations/c_realtime/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_realtime_1",
                        "summary":"hello realtime",
                        "text":"hello realtime"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let realtime_events = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    assert_eq!(realtime_events_json["deviceId"], "d_pad");
    assert_eq!(realtime_events_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(
        realtime_events_json["items"][0]["eventType"],
        "message.posted"
    );
    assert_eq!(
        realtime_events_json["items"][0]["scopeType"],
        "conversation"
    );
    assert_eq!(realtime_events_json["items"][0]["scopeId"], "c_realtime");
    let payload: serde_json::Value = serde_json::from_str(
        realtime_events_json["items"][0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["conversationId"], "c_realtime");
    assert_eq!(payload["messageType"], "standard");
    assert_eq!(realtime_events_json["nextAfterSeq"], 1);
    assert_eq!(realtime_events_json["hasMore"], false);
}

#[tokio::test]
async fn test_local_minimal_profile_disconnect_stops_new_realtime_delivery_and_preserves_sync_feed()
{
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
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
                        "conversationId":"c_disconnect_realtime",
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

    let sync_subscriptions = app
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
                                "scopeId":"c_disconnect_realtime",
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

    let disconnect = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/disconnect")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("disconnect should succeed");
    assert_eq!(disconnect.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_disconnect_realtime/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_disconnect_realtime_1",
                        "summary":"after disconnect",
                        "text":"after disconnect"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let stale_realtime_events = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("stale realtime events should return response");
    assert_eq!(stale_realtime_events.status(), StatusCode::CONFLICT);
    let stale_realtime_events_body = stale_realtime_events
        .into_body()
        .collect()
        .await
        .expect("stale realtime events body should collect")
        .to_bytes();
    let stale_realtime_events_json: serde_json::Value =
        serde_json::from_slice(&stale_realtime_events_body)
            .expect("stale realtime events should be valid json");
    assert_eq!(stale_realtime_events_json["code"], "reconnect_required");

    let sync_feed = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/devices/d_pad/sync-feed?afterSeq=0")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("sync feed should succeed");
    assert_eq!(sync_feed.status(), StatusCode::OK);
    let sync_feed_body = sync_feed
        .into_body()
        .collect()
        .await
        .expect("sync feed body should collect")
        .to_bytes();
    let sync_feed_json: serde_json::Value =
        serde_json::from_slice(&sync_feed_body).expect("sync feed should be valid json");
    let sync_items = sync_feed_json["items"].as_array().unwrap();
    assert_eq!(sync_items.len(), 1);
    assert_eq!(sync_items[0]["messageId"], "msg_c_disconnect_realtime_1");
    assert_eq!(sync_items[0]["originEventType"], "message.posted");

    let resume_fresh = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad_new")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":1}"#))
                .unwrap(),
        )
        .await
        .expect("fresh resume should succeed");
    assert_eq!(resume_fresh.status(), StatusCode::OK);

    let realtime_events_after_resume = app
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
        .expect("realtime events after resume should succeed");
    assert_eq!(realtime_events_after_resume.status(), StatusCode::OK);
    let realtime_events_after_resume_body = realtime_events_after_resume
        .into_body()
        .collect()
        .await
        .expect("realtime events after resume body should collect")
        .to_bytes();
    let realtime_events_after_resume_json: serde_json::Value =
        serde_json::from_slice(&realtime_events_after_resume_body)
            .expect("realtime events after resume should be valid json");
    assert_eq!(
        realtime_events_after_resume_json["items"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
}

#[tokio::test]
async fn test_local_minimal_profile_acks_and_trims_realtime_event_window() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
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
                        "conversationId":"c_realtime_ack",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let register_phone = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register phone should succeed");
    assert_eq!(register_phone.status(), StatusCode::OK);

    let register_pad = app
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

    let sync_subscriptions = app
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
                                "scopeId":"c_realtime_ack",
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
                .uri("/api/v1/conversations/c_realtime_ack/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_realtime_ack_1",
                        "summary":"ack me",
                        "text":"ack me"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let before_ack = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(before_ack.status(), StatusCode::OK);
    let before_ack_body = before_ack
        .into_body()
        .collect()
        .await
        .expect("before ack body should collect")
        .to_bytes();
    let before_ack_json: serde_json::Value =
        serde_json::from_slice(&before_ack_body).expect("before ack should be valid json");
    assert_eq!(before_ack_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(before_ack_json["ackedThroughSeq"], 0);
    assert_eq!(before_ack_json["trimmedThroughSeq"], 0);

    let ack_response = app
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
    let ack_body = ack_response
        .into_body()
        .collect()
        .await
        .expect("ack body should collect")
        .to_bytes();
    let ack_json: serde_json::Value =
        serde_json::from_slice(&ack_body).expect("ack response should be valid json");
    assert_eq!(ack_json["deviceId"], "d_pad");
    assert_eq!(ack_json["ackedThroughSeq"], 1);
    assert_eq!(ack_json["trimmedThroughSeq"], 1);
    assert_eq!(ack_json["retainedEventCount"], 0);

    let after_ack = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events after ack should succeed");
    assert_eq!(after_ack.status(), StatusCode::OK);
    let after_ack_body = after_ack
        .into_body()
        .collect()
        .await
        .expect("after ack body should collect")
        .to_bytes();
    let after_ack_json: serde_json::Value =
        serde_json::from_slice(&after_ack_body).expect("after ack should be valid json");
    assert_eq!(after_ack_json["items"].as_array().unwrap().len(), 0);
    assert_eq!(after_ack_json["ackedThroughSeq"], 1);
    assert_eq!(after_ack_json["trimmedThroughSeq"], 1);
    assert_eq!(after_ack_json["hasMore"], false);
}

#[tokio::test]
async fn test_local_minimal_profile_fanouts_conversation_stream_frames_to_other_member_subscribers()
{
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
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
                        "conversationId":"c_stream_realtime_fanout",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_stream_realtime_fanout/members/add")
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

    let register_other_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_other")
                .header("x-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register other device should succeed");
    assert_eq!(register_other_device.status(), StatusCode::OK);

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_stream_realtime_fanout",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_realtime_fanout",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/realtime/subscriptions/sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_other")
                .header("x-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"stream",
                                "scopeId":"st_stream_realtime_fanout",
                                "eventTypes":["stream.frame.appended"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_stream_realtime_fanout/frames")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
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

    let realtime_events = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_other")
                .header("x-session-id", "s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
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
        .expect("items should be array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["eventType"], "stream.frame.appended");
    assert_eq!(items[0]["scopeType"], "stream");
    assert_eq!(items[0]["scopeId"], "st_stream_realtime_fanout");

    let payload: serde_json::Value = serde_json::from_str(
        items[0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["streamId"], "st_stream_realtime_fanout");
    assert_eq!(payload["scopeKind"], "conversation");
    assert_eq!(payload["scopeId"], "c_stream_realtime_fanout");
    assert_eq!(payload["frameSeq"], 1);
}

#[tokio::test]
async fn test_local_minimal_profile_fanouts_conversation_stream_completion_to_other_member_subscribers()
 {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
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
                        "conversationId":"c_stream_completion_fanout",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_stream_completion_fanout/members/add")
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

    let register_other_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_other")
                .header("x-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register other device should succeed");
    assert_eq!(register_other_device.status(), StatusCode::OK);

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_stream_completion_fanout",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_completion_fanout",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/realtime/subscriptions/sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_other")
                .header("x-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"stream",
                                "scopeId":"st_stream_completion_fanout",
                                "eventTypes":["stream.completed"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_stream_completion_fanout/frames")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
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

    let complete_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_stream_completion_fanout/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "resultMessageId": "msg_result_stream_completion"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete stream should succeed");
    assert_eq!(complete_stream.status(), StatusCode::OK);

    let realtime_events = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_other")
                .header("x-session-id", "s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
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
        .expect("items should be array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["eventType"], "stream.completed");
    assert_eq!(items[0]["scopeType"], "stream");
    assert_eq!(items[0]["scopeId"], "st_stream_completion_fanout");

    let payload: serde_json::Value = serde_json::from_str(
        items[0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["streamId"], "st_stream_completion_fanout");
    assert_eq!(payload["scopeKind"], "conversation");
    assert_eq!(payload["scopeId"], "c_stream_completion_fanout");
    assert_eq!(payload["state"], "completed");
    assert_eq!(payload["lastFrameSeq"], 1);
    assert_eq!(payload["resultMessageId"], "msg_result_stream_completion");
}

#[tokio::test]
async fn test_local_minimal_profile_fanouts_conversation_stream_abort_to_other_member_subscribers()
{
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
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
                        "conversationId":"c_stream_abort_fanout",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_stream_abort_fanout/members/add")
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

    let register_other_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_other")
                .header("x-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register other device should succeed");
    assert_eq!(register_other_device.status(), StatusCode::OK);

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_stream_abort_fanout",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_abort_fanout",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/realtime/subscriptions/sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_other")
                .header("x-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"stream",
                                "scopeId":"st_stream_abort_fanout",
                                "eventTypes":["stream.aborted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_stream_abort_fanout/frames")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
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

    let abort_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_stream_abort_fanout/abort")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "reason": "user_cancelled"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("abort stream should succeed");
    assert_eq!(abort_stream.status(), StatusCode::OK);

    let realtime_events = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_other")
                .header("x-session-id", "s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
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
        .expect("items should be array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["eventType"], "stream.aborted");
    assert_eq!(items[0]["scopeType"], "stream");
    assert_eq!(items[0]["scopeId"], "st_stream_abort_fanout");

    let payload: serde_json::Value = serde_json::from_str(
        items[0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["streamId"], "st_stream_abort_fanout");
    assert_eq!(payload["scopeKind"], "conversation");
    assert_eq!(payload["scopeId"], "c_stream_abort_fanout");
    assert_eq!(payload["state"], "aborted");
    assert_eq!(payload["lastFrameSeq"], 1);
    assert_eq!(payload["reason"], "user_cancelled");
}

#[tokio::test]
async fn test_local_minimal_profile_fanouts_realtime_message_events_to_other_conversation_member() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
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
                        "conversationId":"c_realtime_fanout",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_realtime_fanout/members/add")
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

    let register_other_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_other")
                .header("x-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register other device should succeed");
    assert_eq!(register_other_device.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/realtime/subscriptions/sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_other")
                .header("x-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_realtime_fanout",
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
                .uri("/api/v1/conversations/c_realtime_fanout/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_realtime_fanout_1",
                        "summary":"fanout hello",
                        "text":"fanout hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let realtime_events = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_other")
                .header("x-session-id", "s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    assert_eq!(realtime_events_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(
        realtime_events_json["items"][0]["eventType"],
        "message.posted"
    );
    assert_eq!(
        realtime_events_json["items"][0]["scopeId"],
        "c_realtime_fanout"
    );
    let payload: serde_json::Value = serde_json::from_str(
        realtime_events_json["items"][0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["conversationId"], "c_realtime_fanout");
    assert_eq!(payload["summary"], "fanout hello");
}

#[tokio::test]
async fn test_local_minimal_profile_fanouts_message_mutation_realtime_events_to_other_conversation_member()
 {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
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
                        "conversationId":"c_realtime_mutation_fanout",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_realtime_mutation_fanout/members/add")
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

    let register_other_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_other")
                .header("x-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register other device should succeed");
    assert_eq!(register_other_device.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/realtime/subscriptions/sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_other")
                .header("x-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_realtime_mutation_fanout",
                                "eventTypes":["message.posted","message.edited","message.recalled"]
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
                .uri("/api/v1/conversations/c_realtime_mutation_fanout/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_realtime_mutation_fanout_1",
                        "summary":"posted hello",
                        "text":"posted hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let edit_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/messages/msg_c_realtime_mutation_fanout_1/edit")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "summary":"edited hello",
                        "text":"edited hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("edit message should succeed");
    assert_eq!(edit_message.status(), StatusCode::OK);

    let recall_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/messages/msg_c_realtime_mutation_fanout_1/recall")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("recall message should succeed");
    assert_eq!(recall_message.status(), StatusCode::OK);

    let realtime_events = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_other")
                .header("x-session-id", "s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
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
        .expect("items should be array");
    assert_eq!(items.len(), 3);
    assert_eq!(items[0]["eventType"], "message.posted");
    assert_eq!(items[1]["eventType"], "message.edited");
    assert_eq!(items[2]["eventType"], "message.recalled");

    let posted_payload: serde_json::Value = serde_json::from_str(
        items[0]["payload"]
            .as_str()
            .expect("posted payload should be string"),
    )
    .expect("posted payload should be valid json");
    assert_eq!(
        posted_payload["conversationId"],
        "c_realtime_mutation_fanout"
    );
    assert_eq!(posted_payload["summary"], "posted hello");

    let edited_payload: serde_json::Value = serde_json::from_str(
        items[1]["payload"]
            .as_str()
            .expect("edited payload should be string"),
    )
    .expect("edited payload should be valid json");
    assert_eq!(
        edited_payload["conversationId"],
        "c_realtime_mutation_fanout"
    );
    assert_eq!(edited_payload["summary"], "edited hello");
    assert_eq!(
        edited_payload["messageId"],
        "msg_c_realtime_mutation_fanout_1"
    );

    let recalled_payload: serde_json::Value = serde_json::from_str(
        items[2]["payload"]
            .as_str()
            .expect("recalled payload should be string"),
    )
    .expect("recalled payload should be valid json");
    assert_eq!(
        recalled_payload["conversationId"],
        "c_realtime_mutation_fanout"
    );
    assert_eq!(
        recalled_payload["messageId"],
        "msg_c_realtime_mutation_fanout_1"
    );
    assert_eq!(realtime_events_json["nextAfterSeq"], 3);
    assert_eq!(realtime_events_json["hasMore"], false);
}

#[tokio::test]
async fn test_local_minimal_profile_fanouts_member_governance_realtime_events_to_registered_owner_device()
 {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
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
                        "conversationId":"c_member_realtime",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let sync_subscriptions = app
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
                                "scopeId":"c_member_realtime",
                                "eventTypes":[
                                    "conversation.member_joined",
                                    "conversation.member_role_changed",
                                    "conversation.member_removed",
                                    "conversation.member_left"
                                ]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let add_other_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_member_realtime/members/add")
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
        .expect("add other member should succeed");
    assert_eq!(add_other_member.status(), StatusCode::OK);

    let change_other_role = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_member_realtime/members/change-role")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_member_realtime_u_other_demo",
                        "role":"admin"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("change role should succeed");
    assert_eq!(change_other_role.status(), StatusCode::OK);

    let remove_other_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_member_realtime/members/remove")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_member_realtime_u_other_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("remove other member should succeed");
    assert_eq!(remove_other_member.status(), StatusCode::OK);

    let add_leaver = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_member_realtime/members/add")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_leave_demo",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add leaver should succeed");
    assert_eq!(add_leaver.status(), StatusCode::OK);

    let leave_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_member_realtime/members/leave")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_leave_demo")
                .header("x-device-id", "d_leave")
                .header("x-session-id", "s_leave")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("leave conversation should succeed");
    assert_eq!(leave_conversation.status(), StatusCode::OK);

    let realtime_events = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
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
        .expect("items should be array");
    assert_eq!(items.len(), 5);
    assert_eq!(items[0]["eventType"], "conversation.member_joined");
    assert_eq!(items[1]["eventType"], "conversation.member_role_changed");
    assert_eq!(items[2]["eventType"], "conversation.member_removed");
    assert_eq!(items[3]["eventType"], "conversation.member_joined");
    assert_eq!(items[4]["eventType"], "conversation.member_left");

    let joined_payload: serde_json::Value = serde_json::from_str(
        items[0]["payload"]
            .as_str()
            .expect("joined payload should be string"),
    )
    .expect("joined payload should be valid json");
    assert_eq!(joined_payload["conversationId"], "c_member_realtime");
    assert_eq!(joined_payload["member"]["principalId"], "u_other_demo");
    assert_eq!(joined_payload["member"]["state"], "joined");
    assert_eq!(joined_payload["actor"]["id"], "u_demo");

    let role_changed_payload: serde_json::Value = serde_json::from_str(
        items[1]["payload"]
            .as_str()
            .expect("role changed payload should be string"),
    )
    .expect("role changed payload should be valid json");
    assert_eq!(role_changed_payload["conversationId"], "c_member_realtime");
    assert_eq!(role_changed_payload["previousMember"]["role"], "member");
    assert_eq!(role_changed_payload["updatedMember"]["role"], "admin");
    assert_eq!(role_changed_payload["actor"]["id"], "u_demo");

    let removed_payload: serde_json::Value = serde_json::from_str(
        items[2]["payload"]
            .as_str()
            .expect("removed payload should be string"),
    )
    .expect("removed payload should be valid json");
    assert_eq!(removed_payload["member"]["principalId"], "u_other_demo");
    assert_eq!(removed_payload["member"]["state"], "removed");

    let left_payload: serde_json::Value = serde_json::from_str(
        items[4]["payload"]
            .as_str()
            .expect("left payload should be string"),
    )
    .expect("left payload should be valid json");
    assert_eq!(left_payload["member"]["principalId"], "u_leave_demo");
    assert_eq!(left_payload["member"]["state"], "left");
    assert_eq!(left_payload["actor"]["id"], "u_leave_demo");
    assert_eq!(realtime_events_json["nextAfterSeq"], 5);
    assert_eq!(realtime_events_json["hasMore"], false);
}

#[tokio::test]
async fn test_local_minimal_profile_member_governance_rejects_actor_kind_mismatch_before_side_effects()
 {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
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
                        "conversationId":"c_member_actor_kind_sync",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let sync_subscriptions = app
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
                                "scopeId":"c_member_actor_kind_sync",
                                "eventTypes":[
                                    "conversation.member_joined",
                                    "conversation.member_role_changed",
                                    "conversation.member_removed",
                                    "conversation.member_left"
                                ]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let add_other_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_member_actor_kind_sync/members/add")
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
        .expect("add other member should succeed");
    assert_eq!(add_other_member.status(), StatusCode::OK);

    let add_leaver = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_member_actor_kind_sync/members/add")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_leave_demo",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add leaver should succeed");
    assert_eq!(add_leaver.status(), StatusCode::OK);

    let change_other_role = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_member_actor_kind_sync/members/change-role")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "agent")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_member_actor_kind_sync_u_other_demo",
                        "role":"admin"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("change role should return response");
    assert_eq!(change_other_role.status(), StatusCode::FORBIDDEN);

    let remove_other_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_member_actor_kind_sync/members/remove")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "agent")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_member_actor_kind_sync_u_other_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("remove other member should return response");
    assert_eq!(remove_other_member.status(), StatusCode::FORBIDDEN);

    let leave_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_member_actor_kind_sync/members/leave")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_leave_demo")
                .header("x-actor-kind", "system")
                .header("x-device-id", "d_leave")
                .header("x-session-id", "s_leave")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("leave conversation should return response");
    assert_eq!(leave_conversation.status(), StatusCode::FORBIDDEN);

    let realtime_events = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
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
        .expect("items should be array");
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["eventType"], "conversation.member_joined");
    assert_eq!(items[1]["eventType"], "conversation.member_joined");

    let first_joined_payload: serde_json::Value = serde_json::from_str(
        items[0]["payload"]
            .as_str()
            .expect("first joined payload should be string"),
    )
    .expect("first joined payload should be valid json");
    assert_eq!(first_joined_payload["actor"]["id"], "u_demo");
    assert_eq!(first_joined_payload["actor"]["kind"], "user");
    assert_eq!(
        first_joined_payload["member"]["principalId"],
        "u_other_demo"
    );

    let second_joined_payload: serde_json::Value = serde_json::from_str(
        items[1]["payload"]
            .as_str()
            .expect("second joined payload should be string"),
    )
    .expect("second joined payload should be valid json");
    assert_eq!(second_joined_payload["actor"]["id"], "u_demo");
    assert_eq!(second_joined_payload["actor"]["kind"], "user");
    assert_eq!(
        second_joined_payload["member"]["principalId"],
        "u_leave_demo"
    );
    assert_eq!(realtime_events_json["nextAfterSeq"], 2);
    assert_eq!(realtime_events_json["hasMore"], false);

    let members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_member_actor_kind_sync/members")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("members request should succeed");
    assert_eq!(members.status(), StatusCode::OK);
    let members_body = members
        .into_body()
        .collect()
        .await
        .expect("members body should collect")
        .to_bytes();
    let members_json: serde_json::Value =
        serde_json::from_slice(&members_body).expect("members should be valid json");
    let member_items = members_json["items"]
        .as_array()
        .expect("member items should be array");
    assert_eq!(member_items.len(), 3);
    let other_member = member_items
        .iter()
        .find(|item| item["principalId"] == "u_other_demo")
        .expect("other member should exist");
    assert_eq!(other_member["role"], "member");
    assert_eq!(other_member["state"], "joined");
    let leave_member = member_items
        .iter()
        .find(|item| item["principalId"] == "u_leave_demo")
        .expect("leave member should exist");
    assert_eq!(leave_member["state"], "joined");

    let audit_export = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/export")
                .header("x-permissions", "audit.read")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("audit export should succeed");
    assert_eq!(audit_export.status(), StatusCode::OK);
    let audit_export_body = audit_export
        .into_body()
        .collect()
        .await
        .expect("audit export body should collect")
        .to_bytes();
    let audit_export_json: serde_json::Value =
        serde_json::from_slice(&audit_export_body).expect("audit export should be valid json");
    let audit_items = audit_export_json["items"]
        .as_array()
        .expect("audit items should be array");
    let governance_actions = [
        "conversation.member_joined",
        "conversation.member_role_changed",
        "conversation.member_removed",
        "conversation.member_left",
    ];
    let governance_items: Vec<&serde_json::Value> = audit_items
        .iter()
        .filter(|item| {
            item["action"]
                .as_str()
                .is_some_and(|action| governance_actions.contains(&action))
        })
        .collect();
    assert_eq!(governance_items.len(), 2);
    for item in governance_items {
        assert_eq!(item["actorKind"], "user");
        assert_eq!(item["action"], "conversation.member_joined");
    }
}

#[tokio::test]
async fn test_local_minimal_profile_owner_transfer_rejects_actor_kind_mismatch_before_audit() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
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
                        "conversationId":"c_owner_transfer_actor_kind_sync",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_target_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_owner_transfer_actor_kind_sync/members/add")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_target_demo",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add target member should succeed");
    assert_eq!(add_target_member.status(), StatusCode::OK);

    let transfer_owner = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/api/v1/conversations/c_owner_transfer_actor_kind_sync/members/transfer-owner",
                )
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "agent")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_owner_transfer_actor_kind_sync_u_target_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("transfer owner should return response");
    assert_eq!(transfer_owner.status(), StatusCode::FORBIDDEN);

    let audit_export = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/export")
                .header("x-permissions", "audit.read")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("audit export should succeed");
    assert_eq!(audit_export.status(), StatusCode::OK);
    let audit_export_body = audit_export
        .into_body()
        .collect()
        .await
        .expect("audit export body should collect")
        .to_bytes();
    let audit_export_json: serde_json::Value =
        serde_json::from_slice(&audit_export_body).expect("audit export should be valid json");
    let owner_transfer_item = audit_export_json["items"]
        .as_array()
        .expect("audit items should be array")
        .iter()
        .find(|item| item["action"] == "conversation.owner_transferred");
    assert!(owner_transfer_item.is_none());

    let members = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_owner_transfer_actor_kind_sync/members")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("members request should succeed");
    assert_eq!(members.status(), StatusCode::OK);
    let members_body = members
        .into_body()
        .collect()
        .await
        .expect("members body should collect")
        .to_bytes();
    let members_json: serde_json::Value =
        serde_json::from_slice(&members_body).expect("members should be valid json");
    let member_items = members_json["items"]
        .as_array()
        .expect("member items should be array");
    let owner = member_items
        .iter()
        .find(|item| item["principalId"] == "u_demo")
        .expect("owner should exist");
    assert_eq!(owner["role"], "owner");
    let target = member_items
        .iter()
        .find(|item| item["principalId"] == "u_target_demo")
        .expect("target should exist");
    assert_eq!(target["role"], "member");
}

#[tokio::test]
async fn test_local_minimal_profile_projects_member_governance_sync_feed_deltas() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
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
                        "conversationId":"c_member_sync",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    for (user_id, device_id, session_id) in [
        ("u_demo", "d_owner", "s_owner"),
        ("u_demo", "d_pad", "s_pad"),
        ("u_other_demo", "d_other", "s_other"),
        ("u_leave_demo", "d_leave", "s_leave"),
    ] {
        let register = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/devices/register")
                    .header("x-tenant-id", "t_demo")
                    .header("x-user-id", user_id)
                    .header("x-device-id", device_id)
                    .header("x-session-id", session_id)
                    .header("content-type", "application/json")
                    .body(Body::from(format!(r#"{{"deviceId":"{device_id}"}}"#)))
                    .unwrap(),
            )
            .await
            .expect("device register should succeed");
        assert_eq!(register.status(), StatusCode::OK);
    }

    let add_other_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_member_sync/members/add")
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
        .expect("add other member should succeed");
    assert_eq!(add_other_member.status(), StatusCode::OK);

    let change_other_role = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_member_sync/members/change-role")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_member_sync_u_other_demo",
                        "role":"admin"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("change role should succeed");
    assert_eq!(change_other_role.status(), StatusCode::OK);

    let remove_other_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_member_sync/members/remove")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_member_sync_u_other_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("remove other member should succeed");
    assert_eq!(remove_other_member.status(), StatusCode::OK);

    let add_leaver = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_member_sync/members/add")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_leave_demo",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add leaver should succeed");
    assert_eq!(add_leaver.status(), StatusCode::OK);

    let leave_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_member_sync/members/leave")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_leave_demo")
                .header("x-device-id", "d_leave")
                .header("x-session-id", "s_leave")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("leave conversation should succeed");
    assert_eq!(leave_conversation.status(), StatusCode::OK);

    let owner_sync_feed = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/devices/d_pad/sync-feed?afterSeq=0")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("owner sync feed should succeed");
    assert_eq!(owner_sync_feed.status(), StatusCode::OK);
    let owner_sync_feed_body = owner_sync_feed
        .into_body()
        .collect()
        .await
        .expect("owner sync feed body should collect")
        .to_bytes();
    let owner_sync_feed_json: serde_json::Value = serde_json::from_slice(&owner_sync_feed_body)
        .expect("owner sync feed should be valid json");
    let owner_items = owner_sync_feed_json["items"]
        .as_array()
        .expect("owner sync items should be array");
    assert_eq!(owner_items.len(), 5);
    assert_eq!(
        owner_items[0]["originEventType"],
        "conversation.member_joined"
    );
    assert_eq!(
        owner_items[1]["originEventType"],
        "conversation.member_role_changed"
    );
    assert_eq!(
        owner_items[2]["originEventType"],
        "conversation.member_removed"
    );
    assert_eq!(
        owner_items[3]["originEventType"],
        "conversation.member_joined"
    );
    assert_eq!(
        owner_items[4]["originEventType"],
        "conversation.member_left"
    );

    assert_eq!(owner_items[0]["payloadSchema"], "conversation.member.v1");
    let joined_payload: serde_json::Value = serde_json::from_str(
        owner_items[0]["payload"]
            .as_str()
            .expect("joined sync payload should be string"),
    )
    .expect("joined sync payload should be valid json");
    assert_eq!(joined_payload["principalId"], "u_other_demo");
    assert_eq!(joined_payload["state"], "joined");
    assert_eq!(owner_items[0]["actorId"], "u_demo");
    assert_eq!(owner_items[0]["actorKind"], "user");

    assert_eq!(
        owner_items[1]["payloadSchema"],
        "conversation.member_role_changed.v1"
    );
    let role_changed_payload: serde_json::Value = serde_json::from_str(
        owner_items[1]["payload"]
            .as_str()
            .expect("role change payload should be string"),
    )
    .expect("role change payload should be valid json");
    assert_eq!(role_changed_payload["previousMember"]["role"], "member");
    assert_eq!(role_changed_payload["updatedMember"]["role"], "admin");
    assert_eq!(owner_items[1]["actorId"], "u_demo");
    assert_eq!(owner_items[1]["actorKind"], "user");

    assert_eq!(owner_items[2]["payloadSchema"], "conversation.member.v1");
    let removed_payload: serde_json::Value = serde_json::from_str(
        owner_items[2]["payload"]
            .as_str()
            .expect("removed payload should be string"),
    )
    .expect("removed payload should be valid json");
    assert_eq!(removed_payload["principalId"], "u_other_demo");
    assert_eq!(removed_payload["state"], "removed");
    assert_eq!(owner_items[2]["actorId"], "u_demo");
    assert_eq!(owner_items[2]["actorKind"], "user");

    assert_eq!(owner_items[4]["payloadSchema"], "conversation.member.v1");
    let left_payload: serde_json::Value = serde_json::from_str(
        owner_items[4]["payload"]
            .as_str()
            .expect("left payload should be string"),
    )
    .expect("left payload should be valid json");
    assert_eq!(left_payload["principalId"], "u_leave_demo");
    assert_eq!(left_payload["state"], "left");
    assert_eq!(owner_items[4]["actorId"], "u_leave_demo");
    assert_eq!(owner_items[4]["actorKind"], "user");

    let removed_principal_sync_feed = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/devices/d_other/sync-feed?afterSeq=0")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("x-device-id", "d_other")
                .header("x-session-id", "s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("removed principal sync feed should succeed");
    assert_eq!(removed_principal_sync_feed.status(), StatusCode::OK);
    let removed_principal_sync_feed_body = removed_principal_sync_feed
        .into_body()
        .collect()
        .await
        .expect("removed principal sync feed body should collect")
        .to_bytes();
    let removed_principal_sync_feed_json: serde_json::Value =
        serde_json::from_slice(&removed_principal_sync_feed_body)
            .expect("removed principal sync feed should be valid json");
    let removed_principal_items = removed_principal_sync_feed_json["items"]
        .as_array()
        .expect("removed principal sync items should be array");
    assert_eq!(removed_principal_items.len(), 3);
    assert_eq!(
        removed_principal_items[2]["originEventType"],
        "conversation.member_removed"
    );
    let removed_principal_payload: serde_json::Value = serde_json::from_str(
        removed_principal_items[2]["payload"]
            .as_str()
            .expect("removed principal payload should be string"),
    )
    .expect("removed principal payload should be valid json");
    assert_eq!(removed_principal_payload["principalId"], "u_other_demo");
    assert_eq!(removed_principal_payload["state"], "removed");
}

#[tokio::test]
async fn test_local_minimal_profile_fanouts_agent_handoff_lifecycle_realtime_events_to_other_device()
 {
    let app = local_minimal_node::build_default_app();

    let create_handoff = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/agent-handoffs")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "ag_source")
                .header("x-actor-kind", "agent")
                .header("x-device-id", "d_agent")
                .header("x-session-id", "s_agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_handoff_realtime",
                        "targetId":"u_demo",
                        "targetKind":"user",
                        "handoffSessionId":"hs_realtime",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create agent handoff should succeed");
    assert_eq!(create_handoff.status(), StatusCode::OK);

    let register_phone = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register phone should succeed");
    assert_eq!(register_phone.status(), StatusCode::OK);

    let register_pad = app
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

    let sync_subscriptions = app
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
                                "scopeId":"c_handoff_realtime",
                                "eventTypes":["conversation.agent_handoff_status_changed"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let accept_handoff = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_handoff_realtime/agent-handoff/accept")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("accept handoff should succeed");
    assert_eq!(accept_handoff.status(), StatusCode::OK);

    let realtime_events = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
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
        .expect("items should be array");
    assert_eq!(items.len(), 1);
    assert_eq!(
        items[0]["eventType"],
        "conversation.agent_handoff_status_changed"
    );
    assert_eq!(items[0]["scopeType"], "conversation");
    assert_eq!(items[0]["scopeId"], "c_handoff_realtime");

    let payload: serde_json::Value = serde_json::from_str(
        items[0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["conversationId"], "c_handoff_realtime");
    assert_eq!(payload["currentStatus"], "accepted");
    assert_eq!(payload["changedBy"]["id"], "u_demo");
    assert_eq!(payload["state"]["status"], "accepted");
    assert_eq!(payload["state"]["target"]["id"], "u_demo");
    assert_eq!(realtime_events_json["nextAfterSeq"], 1);
    assert_eq!(realtime_events_json["hasMore"], false);

    let sync_feed = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/devices/d_pad/sync-feed?afterSeq=0")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("sync feed query should succeed");
    assert_eq!(sync_feed.status(), StatusCode::OK);
    let sync_feed_body = sync_feed
        .into_body()
        .collect()
        .await
        .expect("sync feed body should collect")
        .to_bytes();
    let sync_feed_json: serde_json::Value =
        serde_json::from_slice(&sync_feed_body).expect("sync feed should be valid json");
    let sync_items = sync_feed_json["items"]
        .as_array()
        .expect("sync items should be array");
    assert_eq!(sync_items.len(), 1);
    assert_eq!(
        sync_items[0]["originEventType"],
        "conversation.agent_handoff_status_changed"
    );
    assert_eq!(
        sync_items[0]["payloadSchema"],
        "conversation.agent_handoff_status_changed.v1"
    );
    let sync_payload: serde_json::Value = serde_json::from_str(
        sync_items[0]["payload"]
            .as_str()
            .expect("sync payload should be string"),
    )
    .expect("sync payload should be valid json");
    assert_eq!(sync_payload["conversationId"], "c_handoff_realtime");
    assert_eq!(sync_payload["currentStatus"], "accepted");
    assert_eq!(sync_payload["changedBy"]["id"], "u_demo");
    assert_eq!(sync_payload["state"]["status"], "accepted");
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_open_stream_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_demo",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let first_open = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_local_open_idempotent",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first open stream should succeed");
    assert_eq!(first_open.status(), StatusCode::OK);

    let append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_local_open_idempotent/frames")
                .header("authorization", DEMO_BEARER)
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

    let idempotent_open = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_local_open_idempotent",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("idempotent open stream should return response");
    assert_eq!(idempotent_open.status(), StatusCode::OK);
    let idempotent_open_body = idempotent_open
        .into_body()
        .collect()
        .await
        .expect("idempotent open body should collect")
        .to_bytes();
    let idempotent_open_json: serde_json::Value = serde_json::from_slice(&idempotent_open_body)
        .expect("idempotent open should be valid json");
    assert_eq!(idempotent_open_json["state"], "active");
    assert_eq!(idempotent_open_json["lastFrameSeq"], 1);

    let list_frames = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/streams/st_local_open_idempotent/frames?afterFrameSeq=0&limit=10")
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list frames should return response");
    assert_eq!(list_frames.status(), StatusCode::OK);
    let list_frames_body = list_frames
        .into_body()
        .collect()
        .await
        .expect("list frames body should collect")
        .to_bytes();
    let list_frames_json: serde_json::Value =
        serde_json::from_slice(&list_frames_body).expect("list frames should be valid json");
    assert_eq!(list_frames_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(list_frames_json["items"][0]["frameSeq"], 1);

    let conflicting_open = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_local_open_idempotent",
                        "streamType":"custom.delta.binary",
                        "scopeKind":"conversation",
                        "scopeId":"c_other",
                        "durabilityClass":"eventLog",
                        "schemaRef":"custom.delta.binary.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting open stream should return response");
    assert_eq!(conflicting_open.status(), StatusCode::CONFLICT);
    let conflicting_open_body = conflicting_open
        .into_body()
        .collect()
        .await
        .expect("conflicting open body should collect")
        .to_bytes();
    let conflicting_open_json: serde_json::Value = serde_json::from_slice(&conflicting_open_body)
        .expect("conflicting open should be valid json");
    assert_eq!(conflicting_open_json["code"], "stream_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_conflicting_invite_after_accept_without_new_signal() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_rtc_invite_conflict",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let create_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_invite_conflict",
                        "conversationId":"c_rtc_invite_conflict",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session should succeed");
    assert_eq!(create_rtc.status(), StatusCode::OK);

    let first_invite = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_invite_conflict/invite")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalingStreamId":"st_invite_initial"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first invite should succeed");
    assert_eq!(first_invite.status(), StatusCode::OK);

    let accept_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_invite_conflict/accept")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_accept_once"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("accept should succeed");
    assert_eq!(accept_response.status(), StatusCode::OK);

    let conflicting_invite = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_invite_conflict/invite")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalingStreamId":"st_invite_conflict"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting invite should return response");
    assert_eq!(conflicting_invite.status(), StatusCode::CONFLICT);
    let conflicting_invite_body = conflicting_invite
        .into_body()
        .collect()
        .await
        .expect("conflicting invite body should collect")
        .to_bytes();
    let conflicting_invite_json: serde_json::Value =
        serde_json::from_slice(&conflicting_invite_body)
            .expect("conflicting invite should be valid json");
    assert_eq!(
        conflicting_invite_json["code"],
        "rtc_session_state_conflict"
    );

    let timeline = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_rtc_invite_conflict/messages")
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline should succeed");
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
        .expect("timeline items should be array");
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["summary"], "rtc.invite");
    assert_eq!(items[1]["summary"], "rtc.accept");
}
