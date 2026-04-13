use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_adapters_local_memory::MemoryRealtimeDisconnectFenceStore;
use projection_service::TimelineProjectionService;
use session_gateway::RealtimeClusterBridge;
use std::sync::Arc;
use tower::ServiceExt;

const DEMO_BEARER: &str = "Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0ZW5hbnRfaWQiOiJ0X2RlbW8iLCJzdWIiOiJ1X2RlbW8iLCJzaWQiOiJzX2RlbW8ifQ.";
const OTHER_BEARER: &str = "Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0ZW5hbnRfaWQiOiJ0X290aGVyIiwic3ViIjoidV9vdGhlciIsInNpZCI6InNfb3RoZXIifQ.";

fn friendship_activated_event(
    tenant_id: &str,
    friendship_id: &str,
    user_low_id: &str,
    user_high_id: &str,
    direct_chat_id: Option<&str>,
    established_at: &str,
) -> im_domain_events::CommitEnvelope {
    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{friendship_id}_friendship"),
        tenant_id,
        "friendship.activated",
        "friendship",
        friendship_id,
        1,
    )
    .with_payload(
        "social.friendship.activated.v1",
        &serde_json::json!({
            "friendshipId": friendship_id,
            "userLowId": user_low_id,
            "userHighId": user_high_id,
            "initiatorUserId": user_low_id,
            "directChatId": direct_chat_id,
            "establishedAt": established_at,
        })
        .to_string(),
    )
}

fn direct_chat_bound_event(
    tenant_id: &str,
    direct_chat_id: &str,
    conversation_id: &str,
    bound_at: &str,
) -> im_domain_events::CommitEnvelope {
    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{direct_chat_id}_bound"),
        tenant_id,
        "direct_chat.bound",
        "direct_chat",
        direct_chat_id,
        1,
    )
    .with_payload(
        "social.direct_chat.bound.v1",
        &serde_json::json!({
            "directChatId": direct_chat_id,
            "conversationId": conversation_id,
            "leftActorId": "actor_alice",
            "rightActorId": "actor_bob",
            "pairHash": "actor_alice:actor_bob",
            "boundAt": bound_at,
        })
        .to_string(),
    )
}

fn message_reaction_added_event(
    tenant_id: &str,
    conversation_id: &str,
    message_id: &str,
    message_seq: u64,
    reaction_key: &str,
    actor_id: &str,
    reacted_at: &str,
) -> im_domain_events::CommitEnvelope {
    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{message_id}_{reaction_key}_{actor_id}_reaction_added"),
        tenant_id,
        "message.reaction_added",
        "conversation",
        conversation_id,
        message_seq + 1,
    )
    .with_payload(
        "message.reaction_added.v1",
        &serde_json::json!({
            "tenantId": tenant_id,
            "conversationId": conversation_id,
            "messageId": message_id,
            "messageSeq": message_seq,
            "reactionKey": reaction_key,
            "reactedBy": {
                "id": actor_id,
                "kind": "user",
                "memberId": format!("cm_{actor_id}"),
                "deviceId": format!("d_{actor_id}"),
                "sessionId": format!("s_{actor_id}"),
                "metadata": {}
            },
            "reactedAt": reacted_at
        })
        .to_string(),
    )
}

fn message_pinned_event(
    tenant_id: &str,
    conversation_id: &str,
    message_id: &str,
    message_seq: u64,
    actor_id: &str,
    pinned_at: &str,
) -> im_domain_events::CommitEnvelope {
    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{message_id}_{actor_id}_pin_added"),
        tenant_id,
        "message.pin_added",
        "conversation",
        conversation_id,
        message_seq + 2,
    )
    .with_payload(
        "message.pin_added.v1",
        &serde_json::json!({
            "tenantId": tenant_id,
            "conversationId": conversation_id,
            "messageId": message_id,
            "messageSeq": message_seq,
            "pinnedBy": {
                "id": actor_id,
                "kind": "user",
                "memberId": format!("cm_{actor_id}"),
                "deviceId": format!("d_{actor_id}"),
                "sessionId": format!("s_{actor_id}"),
                "metadata": {}
            },
            "pinnedAt": pinned_at
        })
        .to_string(),
    )
}

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
async fn test_local_minimal_profile_treats_duplicate_create_conversation_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_create_retry_local",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first create should return response");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value =
        serde_json::from_slice(&first_create_body).expect("first create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert_eq!(
        first_create_json["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );

    let duplicate_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_create_retry_local",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::OK);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate create should be valid json");
    assert_eq!(duplicate_create_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        duplicate_create_json["eventId"],
        first_create_json["eventId"]
    );

    let members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_create_retry_local/members")
                .header("authorization", DEMO_BEARER)
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
    assert_eq!(members_json["items"].as_array().unwrap().len(), 1);

    let conflicting_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_create_retry_local",
                        "conversationType":"direct"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting create should return response");
    assert_eq!(conflicting_create.status(), StatusCode::CONFLICT);
    let conflicting_create_body = conflicting_create
        .into_body()
        .collect()
        .await
        .expect("conflicting create body should collect")
        .to_bytes();
    let conflicting_create_json: serde_json::Value =
        serde_json::from_slice(&conflicting_create_body)
            .expect("conflicting create should be valid json");
    assert_eq!(conflicting_create_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_agent_dialog_create_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/agent-dialogs")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_dialog_retry_local",
                        "agentId":"ag_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first agent dialog create should return response");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first agent dialog create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value = serde_json::from_slice(&first_create_body)
        .expect("first agent dialog create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert_eq!(
        first_create_json["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_create_json["requestKey"],
        "t_demo:user:u_demo:create-agent-dialog:c_agent_dialog_retry_local"
    );

    let duplicate_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/agent-dialogs")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_dialog_retry_local",
                        "agentId":"ag_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate agent dialog create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::OK);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate agent dialog create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate agent dialog create should be valid json");
    assert_eq!(duplicate_create_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        duplicate_create_json["eventId"],
        first_create_json["eventId"]
    );

    let members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_agent_dialog_retry_local/members")
                .header("authorization", DEMO_BEARER)
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
    assert_eq!(members_json["items"].as_array().unwrap().len(), 2);

    let conflicting_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/agent-dialogs")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_dialog_retry_local",
                        "agentId":"ag_other"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting agent dialog create should return response");
    assert_eq!(conflicting_create.status(), StatusCode::CONFLICT);
    let conflicting_create_body = conflicting_create
        .into_body()
        .collect()
        .await
        .expect("conflicting agent dialog create body should collect")
        .to_bytes();
    let conflicting_create_json: serde_json::Value =
        serde_json::from_slice(&conflicting_create_body)
            .expect("conflicting agent dialog create should be valid json");
    assert_eq!(conflicting_create_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_system_channel_create_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/system-channels")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "svc_ops")
                .header("x-actor-kind", "system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_retry_local",
                        "subscriberId":"u_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first system channel create should return response");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first system channel create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value = serde_json::from_slice(&first_create_body)
        .expect("first system channel create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert_eq!(
        first_create_json["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_create_json["requestKey"],
        "t_demo:system:svc_ops:create-system-channel:c_system_channel_retry_local"
    );

    let duplicate_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/system-channels")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "svc_ops")
                .header("x-actor-kind", "system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_retry_local",
                        "subscriberId":"u_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate system channel create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::OK);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate system channel create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate system channel create should be valid json");
    assert_eq!(duplicate_create_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        duplicate_create_json["eventId"],
        first_create_json["eventId"]
    );

    let members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_system_channel_retry_local/members")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "svc_ops")
                .header("x-actor-kind", "system")
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
    assert_eq!(members_json["items"].as_array().unwrap().len(), 2);

    let conflicting_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/system-channels")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "svc_ops")
                .header("x-actor-kind", "system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_retry_local",
                        "subscriberId":"u_other"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting system channel create should return response");
    assert_eq!(conflicting_create.status(), StatusCode::CONFLICT);
    let conflicting_create_body = conflicting_create
        .into_body()
        .collect()
        .await
        .expect("conflicting system channel create body should collect")
        .to_bytes();
    let conflicting_create_json: serde_json::Value =
        serde_json::from_slice(&conflicting_create_body)
            .expect("conflicting system channel create should be valid json");
    assert_eq!(conflicting_create_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_agent_handoff_create_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/agent-handoffs")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "ag_source")
                .header("x-actor-kind", "agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_retry_local",
                        "targetId":"u_demo",
                        "targetKind":"user",
                        "handoffSessionId":"hs_retry_local",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first agent handoff create should return response");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first agent handoff create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value = serde_json::from_slice(&first_create_body)
        .expect("first agent handoff create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert_eq!(
        first_create_json["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_create_json["requestKey"],
        "t_demo:agent:ag_source:create-agent-handoff:c_agent_handoff_retry_local"
    );

    let duplicate_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/agent-handoffs")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "ag_source")
                .header("x-actor-kind", "agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_retry_local",
                        "targetId":"u_demo",
                        "targetKind":"user",
                        "handoffSessionId":"hs_retry_local",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate agent handoff create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::OK);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate agent handoff create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate agent handoff create should be valid json");
    assert_eq!(duplicate_create_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        duplicate_create_json["eventId"],
        first_create_json["eventId"]
    );

    let members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_agent_handoff_retry_local/members")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "ag_source")
                .header("x-actor-kind", "agent")
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
    assert_eq!(members_json["items"].as_array().unwrap().len(), 2);

    let conflicting_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/agent-handoffs")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "ag_source")
                .header("x-actor-kind", "agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_retry_local",
                        "targetId":"u_other",
                        "targetKind":"user",
                        "handoffSessionId":"hs_retry_local",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting agent handoff create should return response");
    assert_eq!(conflicting_create.status(), StatusCode::CONFLICT);
    let conflicting_create_body = conflicting_create
        .into_body()
        .collect()
        .await
        .expect("conflicting agent handoff create body should collect")
        .to_bytes();
    let conflicting_create_json: serde_json::Value =
        serde_json::from_slice(&conflicting_create_body)
            .expect("conflicting agent handoff create should be valid json");
    assert_eq!(conflicting_create_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_thread_create_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let create_parent = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_parent_thread_retry_local",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create parent conversation should return response");
    assert_eq!(create_parent.status(), StatusCode::OK);

    let first_root = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_parent_thread_retry_local/messages")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_thread_retry_local_root_1",
                        "summary":"root-1",
                        "text":"root-1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first root message should return response");
    assert_eq!(first_root.status(), StatusCode::OK);
    let first_root_body = first_root
        .into_body()
        .collect()
        .await
        .expect("first root body should collect")
        .to_bytes();
    let first_root_json: serde_json::Value =
        serde_json::from_slice(&first_root_body).expect("first root should be valid json");

    let second_root = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_parent_thread_retry_local/messages")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_thread_retry_local_root_2",
                        "summary":"root-2",
                        "text":"root-2"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second root message should return response");
    assert_eq!(second_root.status(), StatusCode::OK);
    let second_root_body = second_root
        .into_body()
        .collect()
        .await
        .expect("second root body should collect")
        .to_bytes();
    let second_root_json: serde_json::Value =
        serde_json::from_slice(&second_root_body).expect("second root should be valid json");

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/threads")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "conversationId":"c_thread_retry_local",
                        "parentConversationId":"c_parent_thread_retry_local",
                        "rootMessageId":"{}"
                    }}"#,
                    first_root_json["messageId"].as_str().unwrap()
                )))
                .unwrap(),
        )
        .await
        .expect("first thread create should return response");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first thread create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value = serde_json::from_slice(&first_create_body)
        .expect("first thread create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert_eq!(
        first_create_json["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_create_json["requestKey"],
        "t_demo:user:u_demo:create-thread:c_thread_retry_local"
    );

    let duplicate_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/threads")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "conversationId":"c_thread_retry_local",
                        "parentConversationId":"c_parent_thread_retry_local",
                        "rootMessageId":"{}"
                    }}"#,
                    first_root_json["messageId"].as_str().unwrap()
                )))
                .unwrap(),
        )
        .await
        .expect("duplicate thread create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::OK);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate thread create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate thread create should be valid json");
    assert_eq!(duplicate_create_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        duplicate_create_json["eventId"],
        first_create_json["eventId"]
    );

    let members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_thread_retry_local/members")
                .header("authorization", DEMO_BEARER)
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
    assert_eq!(members_json["items"].as_array().unwrap().len(), 1);

    let conflicting_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/threads")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "conversationId":"c_thread_retry_local",
                        "parentConversationId":"c_parent_thread_retry_local",
                        "rootMessageId":"{}"
                    }}"#,
                    second_root_json["messageId"].as_str().unwrap()
                )))
                .unwrap(),
        )
        .await
        .expect("conflicting thread create should return response");
    assert_eq!(conflicting_create.status(), StatusCode::CONFLICT);
    let conflicting_create_body = conflicting_create
        .into_body()
        .collect()
        .await
        .expect("conflicting thread create body should collect")
        .to_bytes();
    let conflicting_create_json: serde_json::Value =
        serde_json::from_slice(&conflicting_create_body)
            .expect("conflicting thread create should be valid json");
    assert_eq!(conflicting_create_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_direct_chat_binding_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let first_bind = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "svc_control")
                .header("x-actor-kind", "system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_direct_retry_local",
                        "directChatId":"dc_retry_local",
                        "leftActorId":"actor_a",
                        "leftActorKind":"user",
                        "rightActorId":"actor_b",
                        "rightActorKind":"user"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first direct chat binding should return response");
    assert_eq!(first_bind.status(), StatusCode::OK);
    let first_bind_body = first_bind
        .into_body()
        .collect()
        .await
        .expect("first direct chat bind body should collect")
        .to_bytes();
    let first_bind_json: serde_json::Value = serde_json::from_slice(&first_bind_body)
        .expect("first direct chat bind should be valid json");
    assert_eq!(first_bind_json["deliveryStatus"], "applied");
    assert_eq!(
        first_bind_json["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_bind_json["requestKey"],
        "t_demo:system:svc_control:bind-direct-chat:c_direct_retry_local"
    );

    let duplicate_bind = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "svc_control")
                .header("x-actor-kind", "system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_direct_retry_local",
                        "directChatId":"dc_retry_local",
                        "leftActorId":"actor_a",
                        "leftActorKind":"user",
                        "rightActorId":"actor_b",
                        "rightActorKind":"user"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate direct chat binding should return response");
    assert_eq!(duplicate_bind.status(), StatusCode::OK);
    let duplicate_bind_body = duplicate_bind
        .into_body()
        .collect()
        .await
        .expect("duplicate direct chat bind body should collect")
        .to_bytes();
    let duplicate_bind_json: serde_json::Value = serde_json::from_slice(&duplicate_bind_body)
        .expect("duplicate direct chat bind should be valid json");
    assert_eq!(duplicate_bind_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_bind_json["requestKey"],
        first_bind_json["requestKey"]
    );
    assert_eq!(duplicate_bind_json["eventId"], first_bind_json["eventId"]);

    let members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_direct_retry_local/members")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "actor_a")
                .header("x-actor-kind", "user")
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
    assert_eq!(members_json["items"].as_array().unwrap().len(), 2);

    let conflicting_bind = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "svc_control")
                .header("x-actor-kind", "system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_direct_retry_local",
                        "directChatId":"dc_other_local",
                        "leftActorId":"actor_a",
                        "leftActorKind":"user",
                        "rightActorId":"actor_b",
                        "rightActorKind":"user"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting direct chat binding should return response");
    assert_eq!(conflicting_bind.status(), StatusCode::CONFLICT);
    let conflicting_bind_body = conflicting_bind
        .into_body()
        .collect()
        .await
        .expect("conflicting direct chat bind body should collect")
        .to_bytes();
    let conflicting_bind_json: serde_json::Value = serde_json::from_slice(&conflicting_bind_body)
        .expect("conflicting direct chat bind should be valid json");
    assert_eq!(conflicting_bind_json["code"], "conversation_conflict");
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
async fn test_local_minimal_profile_treats_duplicate_media_upload_requests_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let create_request = r#"{
        "mediaAssetId":"ma_local_media_idempotent",
        "resource":{
            "uuid":"res_local_media_idempotent",
            "type":"image",
            "mimeType":"image/png",
            "size":42,
            "name":"local-proof.png",
            "extension":"png"
        }
    }"#;

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(create_request))
                .unwrap(),
        )
        .await
        .expect("first media create should return response");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first media create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value = serde_json::from_slice(&first_create_body)
        .expect("first media create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert_eq!(
        first_create_json["proofVersion"],
        "media.upload.delivery-proof.v1"
    );

    let duplicate_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(create_request))
                .unwrap(),
        )
        .await
        .expect("duplicate media create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::OK);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate media create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate media create should be valid json");
    assert_eq!(duplicate_create_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_json["requestKey"],
        first_create_json["requestKey"]
    );

    let complete_request = r#"{
        "bucket":"local-media",
        "objectKey":"tenant/t_demo/ma_local_media_idempotent/local-proof.png",
        "storageProvider":"local",
        "url":"https://cdn.example.com/ma_local_media_idempotent/local-proof.png",
        "checksum":"sha256:local-proof"
    }"#;

    let first_complete = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads/ma_local_media_idempotent/complete")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(complete_request))
                .unwrap(),
        )
        .await
        .expect("first media complete should return response");
    assert_eq!(first_complete.status(), StatusCode::OK);
    let first_complete_body = first_complete
        .into_body()
        .collect()
        .await
        .expect("first media complete body should collect")
        .to_bytes();
    let first_complete_json: serde_json::Value = serde_json::from_slice(&first_complete_body)
        .expect("first media complete should be valid json");
    assert_eq!(first_complete_json["deliveryStatus"], "applied");
    assert_eq!(
        first_complete_json["proofVersion"],
        "media.upload.delivery-proof.v1"
    );

    let duplicate_complete = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/uploads/ma_local_media_idempotent/complete")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(complete_request))
                .unwrap(),
        )
        .await
        .expect("duplicate media complete should return response");
    assert_eq!(duplicate_complete.status(), StatusCode::OK);
    let duplicate_complete_body = duplicate_complete
        .into_body()
        .collect()
        .await
        .expect("duplicate media complete body should collect")
        .to_bytes();
    let duplicate_complete_json: serde_json::Value =
        serde_json::from_slice(&duplicate_complete_body)
            .expect("duplicate media complete should be valid json");
    assert_eq!(duplicate_complete_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_complete_json["requestKey"],
        first_complete_json["requestKey"]
    );
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
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first rtc create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value =
        serde_json::from_slice(&first_create_body).expect("first rtc create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert!(
        !first_create_json["requestKey"]
            .as_str()
            .expect("first rtc create requestKey should be present")
            .is_empty()
    );
    assert_eq!(
        first_create_json["proofVersion"],
        "rtc.session.delivery-proof.v1"
    );

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
    assert_eq!(idempotent_json["deliveryStatus"], "replayed");
    assert_eq!(
        idempotent_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        idempotent_json["proofVersion"],
        first_create_json["proofVersion"]
    );

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
async fn test_local_minimal_profile_rejects_duplicate_rtc_create_from_different_actor_kind() {
    let app = local_minimal_node::build_default_app();

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "shared_actor")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_local_kind_scope",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first rtc create should succeed");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first rtc create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value =
        serde_json::from_slice(&first_create_body).expect("first rtc create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert!(
        first_create_json["requestKey"]
            .as_str()
            .expect("first rtc create requestKey should be present")
            .contains(":user:shared_actor:create:rtc_local_kind_scope")
    );

    let conflicting_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "shared_actor")
                .header("x-actor-kind", "system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_local_kind_scope",
                        "rtcMode":"voice"
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
    let duplicate_accept_body = duplicate_accept
        .into_body()
        .collect()
        .await
        .expect("duplicate accept body should collect")
        .to_bytes();
    let duplicate_accept_json: serde_json::Value = serde_json::from_slice(&duplicate_accept_body)
        .expect("duplicate accept should be valid json");
    assert_eq!(duplicate_accept_json["deliveryStatus"], "replayed");
    assert!(
        !duplicate_accept_json["requestKey"]
            .as_str()
            .expect("duplicate accept requestKey should be present")
            .is_empty()
    );
    assert_eq!(
        duplicate_accept_json["proofVersion"],
        "rtc.session.delivery-proof.v1"
    );

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
async fn test_local_minimal_profile_preserves_add_member_request_attributes_for_non_user_principal()
{
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
                        "conversationId":"c_member_request_attributes",
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
                .uri("/api/v1/conversations/c_member_request_attributes/members/add")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"ag_attr_demo",
                        "principalKind":"agent",
                        "role":"member",
                        "attributes":{
                            "serviceTier":"gold",
                            "region":"cn-sh"
                        }
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
    assert_eq!(add_member_json["attributes"]["serviceTier"], "gold");
    assert_eq!(add_member_json["attributes"]["region"], "cn-sh");
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
async fn test_local_minimal_profile_preserves_message_post_audit_for_max_length_conversation_ids() {
    let app = local_minimal_node::build_default_app();
    let conversation_id = "c".repeat(256);

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
                    serde_json::json!({
                        "conversationId": conversation_id,
                        "conversationType": "group",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let register_device = app
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
                .body(Body::from(r#"{"deviceId":"d_phone"}"#))
                .unwrap(),
        )
        .await
        .expect("device register should succeed");
    assert_eq!(register_device.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/conversations/{conversation_id}/messages"))
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_local_long_message_id",
                        "summary":"hello",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);
    let post_message_body = post_message
        .into_body()
        .collect()
        .await
        .expect("post message body should collect")
        .to_bytes();
    let post_message_json: serde_json::Value =
        serde_json::from_slice(&post_message_body).expect("post message should be valid json");
    let message_id = post_message_json["messageId"]
        .as_str()
        .expect("message id should be present")
        .to_owned();

    let audit_export = app
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
    let posted_audit = audit_export_json["items"]
        .as_array()
        .expect("audit items should be array")
        .iter()
        .find(|item| {
            item["action"] == "message.posted"
                && item["aggregateId"]
                    .as_str()
                    .is_some_and(|aggregate_id| aggregate_id == conversation_id)
        })
        .expect("message.posted audit should be recorded for legal long conversation ids");
    let posted_payload: serde_json::Value = serde_json::from_str(
        posted_audit["payload"]
            .as_str()
            .expect("payload should be present"),
    )
    .expect("audit payload should be valid json");
    assert_eq!(posted_payload["messageId"], message_id);
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
async fn test_local_minimal_profile_rejects_notification_queries_from_different_actor_kind() {
    let app = local_minimal_node::build_default_app();

    let create_notification = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/notifications/requests")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_sender")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_local_actor_kind_guard",
                        "sourceEventId":"evt_local_actor_kind_guard",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"u_demo",
                        "title":"New message",
                        "body":"hello",
                        "payload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create notification should succeed");
    assert_eq!(create_notification.status(), StatusCode::OK);

    let recipient_notifications = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("recipient notifications should succeed");
    assert_eq!(recipient_notifications.status(), StatusCode::OK);
    let recipient_notifications_body = recipient_notifications
        .into_body()
        .collect()
        .await
        .expect("recipient notifications body should collect")
        .to_bytes();
    let recipient_notifications_json: serde_json::Value =
        serde_json::from_slice(&recipient_notifications_body)
            .expect("recipient notifications should be valid json");
    let recipient_items = recipient_notifications_json["items"]
        .as_array()
        .expect("recipient items should be array");
    assert_eq!(recipient_items.len(), 1);
    assert_eq!(
        recipient_items[0]["notificationId"],
        "ntf_local_actor_kind_guard"
    );
    assert_eq!(recipient_items[0]["recipientKind"], "user");

    let cross_kind_notifications = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "system")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("cross-kind notifications should succeed");
    assert_eq!(cross_kind_notifications.status(), StatusCode::OK);
    let cross_kind_notifications_body = cross_kind_notifications
        .into_body()
        .collect()
        .await
        .expect("cross-kind notifications body should collect")
        .to_bytes();
    let cross_kind_notifications_json: serde_json::Value =
        serde_json::from_slice(&cross_kind_notifications_body)
            .expect("cross-kind notifications should be valid json");
    assert_eq!(
        cross_kind_notifications_json["items"]
            .as_array()
            .expect("cross-kind items should be array")
            .len(),
        0
    );

    let cross_kind_get = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications/ntf_local_actor_kind_guard")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "system")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("cross-kind get should succeed");
    assert_eq!(cross_kind_get.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_local_minimal_profile_preserves_notification_request_audit_for_max_length_notification_ids()
 {
    let app = local_minimal_node::build_default_app();
    let notification_id = format!("ntf_{}", "n".repeat(508));

    let create_notification = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/notifications/requests")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_sender")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "notificationId": notification_id,
                        "sourceEventId": "evt_local_notification_audit_long_id",
                        "sourceEventType": "message.posted",
                        "category": "message.new",
                        "channel": "inapp",
                        "recipientId": "u_demo",
                        "title": "New message",
                        "body": "hello",
                        "payload": "{\"conversationId\":\"c_demo\"}",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("create notification should succeed");
    assert_eq!(create_notification.status(), StatusCode::OK);
    let create_notification_body = create_notification
        .into_body()
        .collect()
        .await
        .expect("create notification body should collect")
        .to_bytes();
    let create_notification_json: serde_json::Value =
        serde_json::from_slice(&create_notification_body)
            .expect("create notification should be valid json");
    assert_eq!(create_notification_json["notificationId"], notification_id);

    let audit_export = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/export")
                .header("x-permissions", "audit.read")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_sender")
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
    let notification_audit = audit_export_json["items"]
        .as_array()
        .expect("audit items should be array")
        .iter()
        .find(|item| {
            item["action"] == "notification.requested"
                && item["payload"].as_str().is_some_and(|payload| {
                    serde_json::from_str::<serde_json::Value>(payload)
                        .ok()
                        .and_then(|value| {
                            value["notificationId"]
                                .as_str()
                                .map(|notification_id_in_payload| {
                                    notification_id_in_payload == notification_id
                                })
                        })
                        .unwrap_or(false)
                })
        })
        .expect("notification.requested audit should be recorded for legal long notification ids");
    let notification_payload: serde_json::Value = serde_json::from_str(
        notification_audit["payload"]
            .as_str()
            .expect("payload should be present"),
    )
    .expect("notification audit payload should be valid json");
    assert_eq!(notification_payload["recipientId"], "u_demo");
    assert_eq!(notification_payload["sourceEventType"], "message.posted");
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
async fn test_local_minimal_profile_rejects_realtime_limit_above_guardrail_over_http() {
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=5000")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("x-session-id", "s_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime limit request should return response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("realtime limit rejection body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("realtime limit rejection body should be valid json");
    assert_eq!(json["code"], "limit_invalid");
}

#[tokio::test]
async fn test_local_minimal_profile_does_not_fan_out_conversation_realtime_to_non_member_same_actor_id_different_actor_kind()
 {
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
                        "conversationId":"c_realtime_kind_guard",
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
                .uri("/api/v1/conversations/c_realtime_kind_guard/members/add")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_dual",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member should succeed");
    assert_eq!(add_member.status(), StatusCode::OK);

    let register_user_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_dual")
                .header("x-device-id", "d_dual_user")
                .header("x-session-id", "s_dual_user")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register user device should succeed");
    assert_eq!(register_user_device.status(), StatusCode::OK);

    let register_agent_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_dual")
                .header("x-actor-kind", "agent")
                .header("x-device-id", "d_dual_agent")
                .header("x-session-id", "s_dual_agent")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register agent device should return response");
    assert_eq!(register_agent_device.status(), StatusCode::OK);

    let sync_user_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/realtime/subscriptions/sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_dual")
                .header("x-device-id", "d_dual_user")
                .header("x-session-id", "s_dual_user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_realtime_kind_guard",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("user subscription sync should succeed");
    assert_eq!(sync_user_subscriptions.status(), StatusCode::OK);

    let sync_agent_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/realtime/subscriptions/sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_dual")
                .header("x-actor-kind", "agent")
                .header("x-device-id", "d_dual_agent")
                .header("x-session-id", "s_dual_agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_realtime_kind_guard",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent subscription sync should succeed");
    assert_eq!(sync_agent_subscriptions.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_realtime_kind_guard/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_realtime_kind_guard_1",
                        "summary":"typed fanout",
                        "text":"typed fanout"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let user_events = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_dual")
                .header("x-device-id", "d_dual_user")
                .header("x-session-id", "s_dual_user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("user realtime events should succeed");
    assert_eq!(user_events.status(), StatusCode::OK);
    let user_events_body = user_events
        .into_body()
        .collect()
        .await
        .expect("user realtime events body should collect")
        .to_bytes();
    let user_events_json: serde_json::Value = serde_json::from_slice(&user_events_body)
        .expect("user realtime events should be valid json");
    assert_eq!(user_events_json["deviceId"], "d_dual_user");
    assert_eq!(user_events_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(user_events_json["items"][0]["eventType"], "message.posted");

    let agent_events = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_dual")
                .header("x-actor-kind", "agent")
                .header("x-device-id", "d_dual_agent")
                .header("x-session-id", "s_dual_agent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("agent realtime events should succeed");
    assert_eq!(agent_events.status(), StatusCode::OK);
    let agent_events_body = agent_events
        .into_body()
        .collect()
        .await
        .expect("agent realtime events body should collect")
        .to_bytes();
    let agent_events_json: serde_json::Value = serde_json::from_slice(&agent_events_body)
        .expect("agent realtime events should be valid json");
    assert_eq!(agent_events_json["deviceId"], "d_dual_agent");
    assert_eq!(
        agent_events_json["items"].as_array().unwrap().len(),
        0,
        "non-member agent principal sharing the same actor id must not receive conversation realtime events"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_does_not_refanout_duplicate_message_post_retry() {
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
                        "conversationId":"c_post_retry_fanout",
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
                                "scopeId":"c_post_retry_fanout",
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

    let first_post = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_post_retry_fanout/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_post_retry_fanout",
                        "summary":"hello retry",
                        "text":"hello retry"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first post should succeed");
    assert_eq!(first_post.status(), StatusCode::OK);
    let first_post_body = first_post
        .into_body()
        .collect()
        .await
        .expect("first post body should collect")
        .to_bytes();
    let first_post_json: serde_json::Value =
        serde_json::from_slice(&first_post_body).expect("first post should be valid json");
    assert_eq!(first_post_json["deliveryStatus"], "applied");
    assert_eq!(
        first_post_json["proofVersion"],
        "conversation.message.delivery-proof.v1"
    );

    let duplicate_post = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_post_retry_fanout/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_post_retry_fanout",
                        "summary":"hello retry",
                        "text":"hello retry"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate post should return response");
    assert_eq!(duplicate_post.status(), StatusCode::OK);
    let duplicate_post_body = duplicate_post
        .into_body()
        .collect()
        .await
        .expect("duplicate post body should collect")
        .to_bytes();
    let duplicate_post_json: serde_json::Value =
        serde_json::from_slice(&duplicate_post_body).expect("duplicate post should be valid json");
    assert_eq!(duplicate_post_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_post_json["requestKey"],
        first_post_json["requestKey"]
    );
    assert_eq!(
        duplicate_post_json["messageId"],
        first_post_json["messageId"]
    );

    let history = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_post_retry_fanout/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s_phone")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("history should succeed");
    assert_eq!(history.status(), StatusCode::OK);
    let history_body = history
        .into_body()
        .collect()
        .await
        .expect("history body should collect")
        .to_bytes();
    let history_json: serde_json::Value =
        serde_json::from_slice(&history_body).expect("history should be valid json");
    assert_eq!(history_json["items"].as_array().unwrap().len(), 1);

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
    assert_eq!(
        items.len(),
        1,
        "duplicate idempotent message post must not emit a second realtime fanout event"
    );
    assert_eq!(items[0]["eventType"], "message.posted");
    assert_eq!(items[0]["scopeType"], "conversation");
    assert_eq!(items[0]["scopeId"], "c_post_retry_fanout");
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
async fn test_local_minimal_profile_does_not_refanout_duplicate_stream_frame_retry() {
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
                        "conversationId":"c_stream_retry_fanout",
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
                .uri("/api/v1/conversations/c_stream_retry_fanout/members/add")
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
                        "streamId":"st_stream_retry_fanout",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_retry_fanout",
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
                                "scopeId":"st_stream_retry_fanout",
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

    let first_append = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_stream_retry_fanout/frames")
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
        .expect("first append should succeed");
    assert_eq!(first_append.status(), StatusCode::OK);
    let first_append_body = first_append
        .into_body()
        .collect()
        .await
        .expect("first append body should collect")
        .to_bytes();
    let first_append_json: serde_json::Value =
        serde_json::from_slice(&first_append_body).expect("first append should be valid json");
    assert_eq!(first_append_json["deliveryStatus"], "applied");
    assert_eq!(
        first_append_json["proofVersion"],
        "stream.frame.delivery-proof.v1"
    );

    let duplicate_append = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_stream_retry_fanout/frames")
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
        .expect("duplicate append should return response");
    assert_eq!(duplicate_append.status(), StatusCode::OK);
    let duplicate_append_body = duplicate_append
        .into_body()
        .collect()
        .await
        .expect("duplicate append body should collect")
        .to_bytes();
    let duplicate_append_json: serde_json::Value = serde_json::from_slice(&duplicate_append_body)
        .expect("duplicate append should be valid json");
    assert_eq!(duplicate_append_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_append_json["requestKey"],
        first_append_json["requestKey"]
    );

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
    assert_eq!(
        items.len(),
        1,
        "duplicate idempotent stream append must not emit a second realtime fanout event"
    );
    assert_eq!(items[0]["eventType"], "stream.frame.appended");
    assert_eq!(items[0]["scopeId"], "st_stream_retry_fanout");
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
async fn test_local_minimal_profile_does_not_refanout_duplicate_stream_abort_retry() {
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
                        "conversationId":"c_stream_abort_idempotent",
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
                .uri("/api/v1/conversations/c_stream_abort_idempotent/members/add")
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
                        "streamId":"st_abort_retry_fanout",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_abort_idempotent",
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
                                "scopeId":"st_abort_retry_fanout",
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
                .uri("/api/v1/streams/st_abort_retry_fanout/frames")
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

    let first_abort = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_abort_retry_fanout/abort")
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
        .expect("first abort should return response");
    assert_eq!(first_abort.status(), StatusCode::OK);
    let first_abort_body = first_abort
        .into_body()
        .collect()
        .await
        .expect("first abort body should collect")
        .to_bytes();
    let first_abort_json: serde_json::Value =
        serde_json::from_slice(&first_abort_body).expect("first abort should be valid json");
    assert_eq!(first_abort_json["deliveryStatus"], "applied");
    assert_eq!(
        first_abort_json["proofVersion"],
        "stream.session.delivery-proof.v1"
    );

    let duplicate_abort = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_abort_retry_fanout/abort")
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
        .expect("duplicate abort should return response");
    assert_eq!(duplicate_abort.status(), StatusCode::OK);
    let duplicate_abort_body = duplicate_abort
        .into_body()
        .collect()
        .await
        .expect("duplicate abort body should collect")
        .to_bytes();
    let duplicate_abort_json: serde_json::Value = serde_json::from_slice(&duplicate_abort_body)
        .expect("duplicate abort should be valid json");
    assert_eq!(duplicate_abort_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_abort_json["requestKey"],
        first_abort_json["requestKey"]
    );

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

    let payload: serde_json::Value = serde_json::from_str(
        items[0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["streamId"], "st_abort_retry_fanout");
    assert_eq!(payload["reason"], "user_cancelled");
}

#[tokio::test]
async fn test_local_minimal_profile_replays_duplicate_checkpoint_retry_after_complete() {
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
                        "conversationId":"c_stream_checkpoint_idempotent",
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
                        "streamId":"st_local_checkpoint_idempotent",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_checkpoint_idempotent",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let first_checkpoint = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_local_checkpoint_idempotent/checkpoint")
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
        .expect("first checkpoint should return response");
    assert_eq!(first_checkpoint.status(), StatusCode::OK);
    let first_checkpoint_body = first_checkpoint
        .into_body()
        .collect()
        .await
        .expect("first checkpoint body should collect")
        .to_bytes();
    let first_checkpoint_json: serde_json::Value = serde_json::from_slice(&first_checkpoint_body)
        .expect("first checkpoint should be valid json");
    assert_eq!(first_checkpoint_json["deliveryStatus"], "applied");
    assert_eq!(
        first_checkpoint_json["proofVersion"],
        "stream.session.delivery-proof.v1"
    );

    let complete_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_local_checkpoint_idempotent/complete")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 5,
                        "resultMessageId": "msg_checkpoint_complete"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete stream should return response");
    assert_eq!(complete_stream.status(), StatusCode::OK);

    let duplicate_checkpoint = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_local_checkpoint_idempotent/checkpoint")
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
        .expect("duplicate checkpoint should return response");
    assert_eq!(duplicate_checkpoint.status(), StatusCode::OK);
    let duplicate_checkpoint_body = duplicate_checkpoint
        .into_body()
        .collect()
        .await
        .expect("duplicate checkpoint body should collect")
        .to_bytes();
    let duplicate_checkpoint_json: serde_json::Value =
        serde_json::from_slice(&duplicate_checkpoint_body)
            .expect("duplicate checkpoint should be valid json");
    assert_eq!(duplicate_checkpoint_json["state"], "completed");
    assert_eq!(duplicate_checkpoint_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_checkpoint_json["requestKey"],
        first_checkpoint_json["requestKey"]
    );
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
    let first_open_body = first_open
        .into_body()
        .collect()
        .await
        .expect("first open body should collect")
        .to_bytes();
    let first_open_json: serde_json::Value =
        serde_json::from_slice(&first_open_body).expect("first open should be valid json");
    assert_eq!(first_open_json["deliveryStatus"], "applied");
    assert_eq!(
        first_open_json["proofVersion"],
        "stream.session.delivery-proof.v1"
    );

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
    assert_eq!(idempotent_open_json["deliveryStatus"], "replayed");
    assert_eq!(
        idempotent_open_json["requestKey"],
        first_open_json["requestKey"]
    );
    assert_eq!(
        idempotent_open_json["proofVersion"],
        first_open_json["proofVersion"]
    );

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
async fn test_local_minimal_profile_rejects_duplicate_open_stream_from_different_actor() {
    let app = local_minimal_node::build_default_app();

    let first_open = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_local_actor_scope_open",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first open stream should succeed");
    assert_eq!(first_open.status(), StatusCode::OK);
    let first_open_body = first_open
        .into_body()
        .collect()
        .await
        .expect("first open body should collect")
        .to_bytes();
    let first_open_json: serde_json::Value =
        serde_json::from_slice(&first_open_body).expect("first open should be valid json");
    assert_eq!(first_open_json["deliveryStatus"], "applied");
    assert!(
        first_open_json["requestKey"]
            .as_str()
            .expect("first open requestKey should be present")
            .contains(":u_demo:open:st_local_actor_scope_open")
    );

    let conflicting_open = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_local_actor_scope_open",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("different actor open stream should return response");
    assert_eq!(conflicting_open.status(), StatusCode::CONFLICT);
    let conflicting_open_body = conflicting_open
        .into_body()
        .collect()
        .await
        .expect("different actor open body should collect")
        .to_bytes();
    let conflicting_open_json: serde_json::Value = serde_json::from_slice(&conflicting_open_body)
        .expect("different actor open should be valid json");
    assert_eq!(conflicting_open_json["code"], "stream_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_request_stream_list_from_different_actor() {
    let app = local_minimal_node::build_default_app();

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_local_request_scope_owner_only_list",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
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
                .uri("/api/v1/streams/st_local_request_scope_owner_only_list/frames")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
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
        .expect("owner append should succeed");
    assert_eq!(append_frame.status(), StatusCode::OK);

    let list_frames = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/streams/st_local_request_scope_owner_only_list/frames?afterFrameSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_other_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("different actor list should return response");
    assert_eq!(list_frames.status(), StatusCode::NOT_FOUND);
    let list_frames_body = list_frames
        .into_body()
        .collect()
        .await
        .expect("different actor list body should collect")
        .to_bytes();
    let list_frames_json: serde_json::Value = serde_json::from_slice(&list_frames_body)
        .expect("different actor list should be valid json");
    assert_eq!(list_frames_json["code"], "stream_not_found");
}

#[tokio::test]
async fn test_local_minimal_profile_does_not_refanout_duplicate_stream_complete_retry() {
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
                        "conversationId":"c_stream_complete_idempotent",
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
                .uri("/api/v1/conversations/c_stream_complete_idempotent/members/add")
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
                        "streamId":"st_complete_retry_fanout",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_complete_idempotent",
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
                                "scopeId":"st_complete_retry_fanout",
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
                .uri("/api/v1/streams/st_complete_retry_fanout/frames")
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

    let first_complete = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_complete_retry_fanout/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "resultMessageId": "msg_complete_retry_fanout"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first complete should return response");
    assert_eq!(first_complete.status(), StatusCode::OK);
    let first_complete_body = first_complete
        .into_body()
        .collect()
        .await
        .expect("first complete body should collect")
        .to_bytes();
    let first_complete_json: serde_json::Value =
        serde_json::from_slice(&first_complete_body).expect("first complete should be valid json");
    assert_eq!(first_complete_json["deliveryStatus"], "applied");
    assert_eq!(
        first_complete_json["proofVersion"],
        "stream.session.delivery-proof.v1"
    );

    let duplicate_complete = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_complete_retry_fanout/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "resultMessageId": "msg_complete_retry_fanout"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate complete should return response");
    assert_eq!(duplicate_complete.status(), StatusCode::OK);
    let duplicate_complete_body = duplicate_complete
        .into_body()
        .collect()
        .await
        .expect("duplicate complete body should collect")
        .to_bytes();
    let duplicate_complete_json: serde_json::Value =
        serde_json::from_slice(&duplicate_complete_body)
            .expect("duplicate complete should be valid json");
    assert_eq!(duplicate_complete_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_complete_json["requestKey"],
        first_complete_json["requestKey"]
    );

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
    assert_eq!(items[0]["eventType"], "stream.completed");
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

#[tokio::test]
async fn test_local_minimal_profile_issues_rtc_participant_credential_over_http() {
    let app = local_minimal_node::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_local_provider_http",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let credential_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_local_provider_http/credentials")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "participantId":"u_peer"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("issue rtc credential request should return response");

    assert_eq!(credential_response.status(), StatusCode::OK);
    let credential_body = credential_response
        .into_body()
        .collect()
        .await
        .expect("credential body should collect")
        .to_bytes();
    let credential_json: serde_json::Value =
        serde_json::from_slice(&credential_body).expect("credential response should be valid json");

    assert_eq!(credential_json["tenantId"], "t_demo");
    assert_eq!(credential_json["rtcSessionId"], "rtc_local_provider_http");
    assert_eq!(credential_json["participantId"], "u_peer");
    assert_eq!(
        credential_json["credential"],
        "volcengine-token:t_demo:rtc_local_provider_http:u_peer"
    );
    assert!(credential_json["expiresAt"].as_str().is_some());
}

#[tokio::test]
async fn test_local_minimal_profile_gets_rtc_provider_health_over_http() {
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/rtc/provider-health")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("provider health request should return response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("provider health body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("provider health response should be valid json");

    assert_eq!(json["pluginId"], "rtc-volcengine");
    assert_eq!(json["status"], "healthy");
    assert_eq!(json["details"]["providerKind"], "volcengine");
    assert_eq!(
        json["details"]["accessEndpoint"],
        "wss://rtc.volcengine.local/session"
    );
    assert!(json["checkedAt"].as_str().is_some());
}

#[tokio::test]
async fn test_local_minimal_profile_maps_rtc_provider_callback_over_http() {
    let app = local_minimal_node::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_local_callback_http",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let callback_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/provider-callbacks")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_local_callback_http",
                        "callbackType":"room-ended",
                        "payloadJson":"{\"reason\":\"host_left\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("rtc provider callback request should return response");

    assert_eq!(callback_response.status(), StatusCode::OK);
    let callback_body = callback_response
        .into_body()
        .collect()
        .await
        .expect("callback body should collect")
        .to_bytes();
    let callback_json: serde_json::Value =
        serde_json::from_slice(&callback_body).expect("callback response should be valid json");

    assert_eq!(callback_json["rtcSessionId"], "rtc_local_callback_http");
    assert_eq!(callback_json["eventType"], "room-ended");
    assert_eq!(callback_json["participantId"], serde_json::Value::Null);
    assert_eq!(callback_json["payloadJson"], "{\"reason\":\"host_left\"}");
}

#[tokio::test]
async fn test_local_minimal_profile_gets_rtc_recording_artifact_over_http() {
    let app = local_minimal_node::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_local_recording_http",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let artifact_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/rtc/sessions/rtc_local_recording_http/artifacts/recording")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("recording artifact request should return response");

    assert_eq!(artifact_response.status(), StatusCode::OK);
    let artifact_body = artifact_response
        .into_body()
        .collect()
        .await
        .expect("recording artifact body should collect")
        .to_bytes();
    let artifact_json: serde_json::Value = serde_json::from_slice(&artifact_body)
        .expect("recording artifact response should be valid json");

    assert_eq!(artifact_json["tenantId"], "t_demo");
    assert_eq!(artifact_json["rtcSessionId"], "rtc_local_recording_http");
    assert_eq!(artifact_json["bucket"], "rtc-artifacts");
    assert_eq!(
        artifact_json["objectKey"],
        "recordings/t_demo/rtc_local_recording_http.mp4"
    );
    assert_eq!(
        artifact_json["storageProvider"],
        "object-storage-volcengine"
    );
    assert_eq!(
        artifact_json["playbackUrl"],
        "https://tos.volcengine.local/rtc-artifacts/recordings/t_demo/rtc_local_recording_http.mp4?provider=object-storage-volcengine&expires=3600"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_audit_payload_over_http() {
    let app = local_minimal_node::build_default_app();
    let request_body = serde_json::json!({
        "recordId": "audit_local_oversized_payload",
        "aggregateType": "notification",
        "aggregateId": "ntf_local_oversized_payload",
        "action": "notification.requested",
        "payload": "x".repeat(200_000)
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/audit/records")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "audit.write,audit.read")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized local audit payload should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("payload")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_audit_anchor_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let first_record = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/audit/records")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "audit.write,audit.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "recordId":"audit_local_idempotent",
                        "aggregateType":"notification",
                        "aggregateId":"ntf_local_idempotent",
                        "action":"notification.requested",
                        "payload":"{\"recipientId\":\"u_target\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first local audit record should succeed");
    assert_eq!(first_record.status(), StatusCode::OK);
    let first_record_body = first_record
        .into_body()
        .collect()
        .await
        .expect("first local audit body should collect")
        .to_bytes();
    let first_record_json: serde_json::Value =
        serde_json::from_slice(&first_record_body).expect("first local audit should be valid json");
    assert_eq!(first_record_json["deliveryStatus"], "applied");
    assert_eq!(
        first_record_json["proofVersion"],
        "audit.record.delivery-proof.v1"
    );

    let duplicate_record = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/audit/records")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "audit.write,audit.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "recordId":"audit_local_idempotent",
                        "aggregateType":"notification",
                        "aggregateId":"ntf_local_idempotent",
                        "action":"notification.requested",
                        "payload":"{\"recipientId\":\"u_target\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate local audit record should return response");
    assert_eq!(duplicate_record.status(), StatusCode::OK);
    let duplicate_record_body = duplicate_record
        .into_body()
        .collect()
        .await
        .expect("duplicate local audit body should collect")
        .to_bytes();
    let duplicate_record_json: serde_json::Value = serde_json::from_slice(&duplicate_record_body)
        .expect("duplicate local audit should be valid json");
    assert_eq!(duplicate_record_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_record_json["requestKey"],
        first_record_json["requestKey"]
    );

    let list_records = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/records")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "audit.write,audit.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list local audit records should succeed");
    assert_eq!(list_records.status(), StatusCode::OK);
    let list_records_body = list_records
        .into_body()
        .collect()
        .await
        .expect("list local audit body should collect")
        .to_bytes();
    let list_records_json: serde_json::Value =
        serde_json::from_slice(&list_records_body).expect("list local audit should be valid json");
    assert_eq!(list_records_json["items"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn test_local_minimal_profile_replays_duplicate_audit_anchor_after_session_rotation() {
    let app = local_minimal_node::build_default_app();

    let first_record = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/audit/records")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_before")
                .header("x-permissions", "audit.write,audit.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "recordId":"audit_local_session_rotation",
                        "aggregateType":"notification",
                        "aggregateId":"ntf_local_session_rotation",
                        "action":"notification.requested",
                        "payload":"{\"recipientId\":\"u_target\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first local audit record should succeed");
    assert_eq!(first_record.status(), StatusCode::OK);
    let first_record_body = first_record
        .into_body()
        .collect()
        .await
        .expect("first local audit body should collect")
        .to_bytes();
    let first_record_json: serde_json::Value =
        serde_json::from_slice(&first_record_body).expect("first local audit should be valid json");
    assert_eq!(first_record_json["deliveryStatus"], "applied");

    let duplicate_record = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/audit/records")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_after")
                .header("x-permissions", "audit.write,audit.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "recordId":"audit_local_session_rotation",
                        "aggregateType":"notification",
                        "aggregateId":"ntf_local_session_rotation",
                        "action":"notification.requested",
                        "payload":"{\"recipientId\":\"u_target\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate local audit record should return response after session rotation");
    assert_eq!(duplicate_record.status(), StatusCode::OK);
    let duplicate_record_body = duplicate_record
        .into_body()
        .collect()
        .await
        .expect("duplicate local audit body should collect")
        .to_bytes();
    let duplicate_record_json: serde_json::Value = serde_json::from_slice(&duplicate_record_body)
        .expect("duplicate local audit should be valid json");
    assert_eq!(duplicate_record_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_record_json["requestKey"],
        first_record_json["requestKey"]
    );

    let list_records = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/records")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "audit.write,audit.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list local audit records should succeed");
    assert_eq!(list_records.status(), StatusCode::OK);
    let list_records_body = list_records
        .into_body()
        .collect()
        .await
        .expect("list local audit body should collect")
        .to_bytes();
    let list_records_json: serde_json::Value =
        serde_json::from_slice(&list_records_body).expect("list local audit should be valid json");
    assert_eq!(list_records_json["items"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_device_id_on_register_over_http() {
    let app = local_minimal_node::build_default_app();
    let request_body = serde_json::json!({
        "deviceId": "d".repeat(2048)
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized device register should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("deviceId")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_conversation_id_on_timeline_query_over_http()
{
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/api/v1/conversations/{}/messages",
                    "c".repeat(2048)
                ))
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("oversized local timeline query should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("conversationId")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_stream_id_on_list_frames_over_http() {
    let app = local_minimal_node::build_default_app();
    let oversized_stream_id = "s".repeat(2048);

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/api/v1/streams/{oversized_stream_id}/frames?afterFrameSeq=0&limit=10"
                ))
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("oversized local list frames request should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("oversized local list frames body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("oversized local list frames should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("streamId")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_exposes_projection_read_routes_for_contacts_directory_and_interactions()
 {
    let service = Arc::new(TimelineProjectionService::default());
    service
        .apply(&friendship_activated_event(
            "t_demo",
            "fs_local_001",
            "u_alice",
            "u_bob",
            Some("dc_local_001"),
            "2026-04-10T12:00:00Z",
        ))
        .expect("friendship projection should succeed");
    service
        .apply(&friendship_activated_event(
            "t_demo",
            "fs_local_002",
            "u_alice",
            "u_cathy",
            None,
            "2026-04-10T11:00:00Z",
        ))
        .expect("second friendship projection should succeed");
    service
        .apply(&direct_chat_bound_event(
            "t_demo",
            "dc_local_001",
            "c_direct_local_001",
            "2026-04-10T12:05:00Z",
        ))
        .expect("direct chat bind projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_local_projection_owner_joined",
                "t_demo",
                "conversation.member_joined",
                "conversation",
                "c_projection_local",
                1,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_projection_local",
                    "memberId":"cm_u_owner",
                    "principalId":"u_owner",
                    "principalKind":"user",
                    "role":"owner",
                    "state":"joined",
                    "invitedBy":null,
                    "joinedAt":"2026-04-10T12:00:00Z",
                    "removedAt":null,
                    "attributes":{"displayName":"Owner"}
                }"#,
            ),
        )
        .expect("owner projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_local_projection_member_joined",
                "t_demo",
                "conversation.member_joined",
                "conversation",
                "c_projection_local",
                2,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_projection_local",
                    "memberId":"cm_u_member",
                    "principalId":"u_member",
                    "principalKind":"user",
                    "role":"member",
                    "state":"joined",
                    "invitedBy":"u_owner",
                    "joinedAt":"2026-04-10T12:00:01Z",
                    "removedAt":null,
                    "attributes":{"displayName":"Member"}
                }"#,
            ),
        )
        .expect("member projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_local_projection_message_posted",
                "t_demo",
                "message.posted",
                "conversation",
                "c_projection_local",
                3,
            )
            .with_payload(
                "message.posted.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_projection_local",
                    "messageId":"msg_c_projection_local_1",
                    "messageSeq":1,
                    "sender":{"id":"u_owner","kind":"user","memberId":"cm_u_owner","deviceId":"d_u_owner","sessionId":"s_u_owner","metadata":{}},
                    "messageType":"standard",
                    "deliveryMode":"discrete",
                    "clientMsgId":"client_projection_local_1",
                    "streamSessionId":null,
                    "rtcSessionId":null,
                    "body":{"summary":"projection local","parts":[{"kind":"text","text":"projection local"}],"renderHints":{}},
                    "attributes":{},
                    "metadata":{},
                    "occurredAt":"2026-04-10T12:00:02Z",
                    "committedAt":"2026-04-10T12:00:02Z"
                }"#,
            ),
        )
        .expect("message projection should succeed");
    service
        .apply(&message_reaction_added_event(
            "t_demo",
            "c_projection_local",
            "msg_c_projection_local_1",
            1,
            "thumbs_up",
            "u_owner",
            "2026-04-10T12:00:10Z",
        ))
        .expect("first reaction projection should succeed");
    service
        .apply(&message_reaction_added_event(
            "t_demo",
            "c_projection_local",
            "msg_c_projection_local_1",
            1,
            "thumbs_up",
            "u_member",
            "2026-04-10T12:00:11Z",
        ))
        .expect("second reaction projection should succeed");
    service
        .apply(&message_pinned_event(
            "t_demo",
            "c_projection_local",
            "msg_c_projection_local_1",
            1,
            "u_owner",
            "2026-04-10T12:00:20Z",
        ))
        .expect("pin projection should succeed");

    let app = local_minimal_node::build_app_with_dependencies(
        "local_projection_routes",
        "127.0.0.1:18124",
        service,
        Arc::new(RealtimeClusterBridge::default()),
    );

    let contacts_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/contacts")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_alice")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("contacts request should return response");
    assert_eq!(contacts_response.status(), StatusCode::OK);
    let contacts_body = contacts_response
        .into_body()
        .collect()
        .await
        .expect("contacts body should collect")
        .to_bytes();
    let contacts_value: serde_json::Value =
        serde_json::from_slice(&contacts_body).expect("contacts body should be valid json");
    let contacts_items = contacts_value["items"]
        .as_array()
        .expect("contacts items should be an array");
    assert_eq!(contacts_items.len(), 2);
    assert_eq!(contacts_items[0]["targetUserId"], "u_bob");
    assert_eq!(contacts_items[0]["conversationId"], "c_direct_local_001");
    assert_eq!(contacts_items[1]["targetUserId"], "u_cathy");

    let member_directory_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_projection_local/member-directory")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("member directory request should return response");
    assert_eq!(member_directory_response.status(), StatusCode::OK);
    let member_directory_body = member_directory_response
        .into_body()
        .collect()
        .await
        .expect("member directory body should collect")
        .to_bytes();
    let member_directory_value: serde_json::Value = serde_json::from_slice(&member_directory_body)
        .expect("member directory body should be valid json");
    let directory_items = member_directory_value["items"]
        .as_array()
        .expect("member directory items should be an array");
    assert_eq!(directory_items.len(), 2);
    assert_eq!(directory_items[0]["principalId"], "u_owner");
    assert_eq!(directory_items[1]["attributes"]["displayName"], "Member");

    let interaction_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_projection_local/messages/msg_c_projection_local_1/interaction-summary")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("interaction summary request should return response");
    assert_eq!(interaction_response.status(), StatusCode::OK);
    let interaction_body = interaction_response
        .into_body()
        .collect()
        .await
        .expect("interaction summary body should collect")
        .to_bytes();
    let interaction_value: serde_json::Value = serde_json::from_slice(&interaction_body)
        .expect("interaction summary body should be valid json");
    assert_eq!(interaction_value["messageId"], "msg_c_projection_local_1");
    assert_eq!(interaction_value["totalReactionCount"], 2);
    assert_eq!(
        interaction_value["reactionCounts"][0]["reactionKey"],
        "thumbs_up"
    );
    assert_eq!(interaction_value["pin"]["pinnedBy"]["id"], "u_owner");

    let pins_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_projection_local/pins")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_member")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("pins request should return response");
    assert_eq!(pins_response.status(), StatusCode::OK);
    let pins_body = pins_response
        .into_body()
        .collect()
        .await
        .expect("pins body should collect")
        .to_bytes();
    let pins_value: serde_json::Value =
        serde_json::from_slice(&pins_body).expect("pins body should be valid json");
    let pin_items = pins_value["items"]
        .as_array()
        .expect("pins items should be an array");
    assert_eq!(pin_items.len(), 1);
    assert_eq!(pin_items[0]["messageId"], "msg_c_projection_local_1");
    assert_eq!(pin_items[0]["pin"]["pinnedAt"], "2026-04-10T12:00:20Z");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_same_actor_id_with_different_actor_kind_on_contacts_query()
 {
    let service = Arc::new(TimelineProjectionService::default());
    service
        .apply(&friendship_activated_event(
            "t_demo",
            "fs_local_actor_kind_contacts",
            "u_alice",
            "u_bob",
            Some("dc_local_actor_kind_contacts"),
            "2026-04-13T12:00:00Z",
        ))
        .expect("friendship projection should succeed");
    service
        .apply(&direct_chat_bound_event(
            "t_demo",
            "dc_local_actor_kind_contacts",
            "c_local_actor_kind_contacts",
            "2026-04-13T12:05:00Z",
        ))
        .expect("direct chat bind projection should succeed");

    let app = local_minimal_node::build_app_with_dependencies(
        "local_projection_actor_kind_contacts",
        "127.0.0.1:18125",
        service,
        Arc::new(RealtimeClusterBridge::default()),
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/contacts")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_alice")
                .header("x-actor-kind", "system")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("actor-kind mismatch contacts request should return response");

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "contact_scope_forbidden");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_sender_session_id_on_post_message() {
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
                        "conversationId":"c_local_oversized_sender_session",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let register_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"deviceId":"d_phone"}"#))
                .unwrap(),
        )
        .await
        .expect("register device should succeed");
    assert_eq!(register_device.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_local_oversized_sender_session/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("x-session-id", "s".repeat(257))
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_local_oversized_sender_session",
                        "summary":"oversized sender session",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("oversized sender session post should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("senderSessionId")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_render_hints_on_post_message() {
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
                        "conversationId":"c_local_oversized_render_hints",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let register_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"deviceId":"d_phone"}"#))
                .unwrap(),
        )
        .await
        .expect("register device should succeed");
    assert_eq!(register_device.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_local_oversized_render_hints/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "clientMsgId": "client_local_oversized_render_hints",
                        "summary": "oversized render hints",
                        "text": "hello",
                        "renderHints": {
                            "preview": "x".repeat(70 * 1024)
                        }
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("oversized render hints post should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("renderHints")
    );
}
