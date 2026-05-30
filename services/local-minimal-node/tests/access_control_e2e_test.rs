use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

trait AppContextRequestBuilderExt {
    fn owner_app_context(self) -> Self;
    fn owner_as_agent_app_context(self) -> Self;
    fn admin_app_context(self) -> Self;
    fn member_app_context(self) -> Self;
    fn intruder_app_context(self) -> Self;
    fn agent_app_context(self) -> Self;
    fn system_app_context(self) -> Self;
}

impl AppContextRequestBuilderExt for axum::http::request::Builder {
    fn owner_app_context(self) -> Self {
        self.header("x-sdkwork-tenant-id", "t_demo")
            .header("x-sdkwork-user-id", "u_owner")
            .header("x-sdkwork-actor-kind", "user")
            .header("x-sdkwork-session-id", "s_owner")
            .header("x-sdkwork-device-id", "d_owner")
    }

    fn owner_as_agent_app_context(self) -> Self {
        self.header("x-sdkwork-tenant-id", "t_demo")
            .header("x-sdkwork-user-id", "u_owner")
            .header("x-sdkwork-actor-id", "u_owner")
            .header("x-sdkwork-actor-kind", "agent")
            .header("x-sdkwork-session-id", "s_owner")
            .header("x-sdkwork-device-id", "d_owner")
    }

    fn admin_app_context(self) -> Self {
        self.header("x-sdkwork-tenant-id", "t_demo")
            .header("x-sdkwork-user-id", "u_admin")
            .header("x-sdkwork-actor-kind", "user")
            .header("x-sdkwork-session-id", "s_admin")
            .header("x-sdkwork-device-id", "d_admin")
    }

    fn member_app_context(self) -> Self {
        self.header("x-sdkwork-tenant-id", "t_demo")
            .header("x-sdkwork-user-id", "u_member")
            .header("x-sdkwork-actor-kind", "user")
            .header("x-sdkwork-session-id", "s_member")
            .header("x-sdkwork-device-id", "d_member")
    }

    fn intruder_app_context(self) -> Self {
        self.header("x-sdkwork-tenant-id", "t_demo")
            .header("x-sdkwork-user-id", "u_intruder")
            .header("x-sdkwork-actor-kind", "user")
            .header("x-sdkwork-session-id", "s_intruder")
            .header("x-sdkwork-device-id", "d_intruder")
    }

    fn agent_app_context(self) -> Self {
        self.header("x-sdkwork-tenant-id", "t_demo")
            .header("x-sdkwork-user-id", "ag_source")
            .header("x-sdkwork-actor-kind", "agent")
            .header("x-sdkwork-session-id", "s_agent")
            .header("x-sdkwork-device-id", "d_agent")
    }

    fn system_app_context(self) -> Self {
        self.header("x-sdkwork-tenant-id", "t_demo")
            .header("x-sdkwork-user-id", "svc_ops")
            .header("x-sdkwork-actor-kind", "system")
            .header("x-sdkwork-session-id", "s_system")
            .header("x-sdkwork-device-id", "d_system")
    }
}

#[tokio::test]
async fn test_non_member_cannot_read_private_conversation_views() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_private_view",
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
                .uri("/im/v3/api/chat/conversations/c_private_view/messages")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_private_view",
                        "summary":"secret",
                        "text":"secret"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_private_view/members")
                .intruder_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list members should return response");
    assert_eq!(members.status(), StatusCode::FORBIDDEN);

    let timeline = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_private_view/messages")
                .intruder_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline should return response");
    assert_eq!(timeline.status(), StatusCode::FORBIDDEN);

    let summary = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_private_view")
                .intruder_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("summary should return response");
    assert_eq!(summary.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_non_member_cannot_create_conversation_bound_rtc_session() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_private_rtc_create",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let create_rtc = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/rtc/sessions")
                .intruder_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_private_create",
                        "conversationId":"c_private_rtc_create",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc should return response");
    assert_eq!(create_rtc.status(), StatusCode::FORBIDDEN);
    let create_rtc_body = create_rtc
        .into_body()
        .collect()
        .await
        .expect("create rtc body should collect")
        .to_bytes();
    let create_rtc_json: serde_json::Value =
        serde_json::from_slice(&create_rtc_body).expect("create rtc should be valid json");
    assert_eq!(create_rtc_json["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_non_member_cannot_mutate_conversation_bound_rtc_signal_state() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_private_rtc_signal",
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
                .uri("/im/v3/api/rtc/sessions")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_private_signal",
                        "conversationId":"c_private_rtc_signal",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc should succeed");
    assert_eq!(create_rtc.status(), StatusCode::OK);

    let intruder_signal = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/rtc/sessions/rtc_private_signal/signals")
                .intruder_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalType":"rtc.offer",
                        "schemaRef":"webrtc.offer.v1",
                        "payload":"{\"sdp\":\"intruder\"}",
                        "signalingStreamId":"st_intruder"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("intruder signal should return response");
    assert_eq!(intruder_signal.status(), StatusCode::FORBIDDEN);
    let intruder_signal_body = intruder_signal
        .into_body()
        .collect()
        .await
        .expect("intruder signal body should collect")
        .to_bytes();
    let intruder_signal_json: serde_json::Value = serde_json::from_slice(&intruder_signal_body)
        .expect("intruder signal should be valid json");
    assert_eq!(
        intruder_signal_json["code"],
        "conversation_permission_denied"
    );

    let owner_signal = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/rtc/sessions/rtc_private_signal/signals")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalType":"rtc.answer",
                        "schemaRef":"webrtc.answer.v1",
                        "payload":"{\"sdp\":\"owner\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("owner signal should succeed");
    assert_eq!(owner_signal.status(), StatusCode::OK);
    let owner_signal_body = owner_signal
        .into_body()
        .collect()
        .await
        .expect("owner signal body should collect")
        .to_bytes();
    let owner_signal_json: serde_json::Value =
        serde_json::from_slice(&owner_signal_body).expect("owner signal should be valid json");
    assert_eq!(
        owner_signal_json["signalingStreamId"],
        serde_json::Value::Null
    );
}

#[tokio::test]
async fn test_non_member_cannot_open_conversation_bound_stream() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_private_stream_open",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let open_stream = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .intruder_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_private_open",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_private_stream_open",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should return response");
    assert_eq!(open_stream.status(), StatusCode::FORBIDDEN);
    let open_stream_body = open_stream
        .into_body()
        .collect()
        .await
        .expect("open stream body should collect")
        .to_bytes();
    let open_stream_json: serde_json::Value =
        serde_json::from_slice(&open_stream_body).expect("open stream should be valid json");
    assert_eq!(open_stream_json["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_non_member_cannot_mutate_or_read_conversation_bound_stream_state() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_private_stream_state",
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
                .uri("/im/v3/api/streams")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_private_state",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_private_stream_state",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let intruder_append = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_private_state/frames")
                .intruder_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq":1,
                        "frameType":"delta",
                        "schemaRef":"custom.delta.text.v1",
                        "encoding":"json",
                        "payload":"{\"delta\":\"intruder\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("intruder append should return response");
    assert_eq!(intruder_append.status(), StatusCode::FORBIDDEN);

    let intruder_checkpoint = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_private_state/checkpoint")
                .intruder_app_context()
                .header("content-type", "application/json")
                .body(Body::from(r#"{"frameSeq":9}"#))
                .unwrap(),
        )
        .await
        .expect("intruder checkpoint should return response");
    assert_eq!(intruder_checkpoint.status(), StatusCode::FORBIDDEN);

    let intruder_complete = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_private_state/complete")
                .intruder_app_context()
                .header("content-type", "application/json")
                .body(Body::from(r#"{"frameSeq":9}"#))
                .unwrap(),
        )
        .await
        .expect("intruder complete should return response");
    assert_eq!(intruder_complete.status(), StatusCode::FORBIDDEN);

    let intruder_abort = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_private_state/abort")
                .intruder_app_context()
                .header("content-type", "application/json")
                .body(Body::from(r#"{"frameSeq":9,"reason":"intruder"}"#))
                .unwrap(),
        )
        .await
        .expect("intruder abort should return response");
    assert_eq!(intruder_abort.status(), StatusCode::FORBIDDEN);

    let intruder_list = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/streams/st_private_state/frames?afterFrameSeq=0&limit=10")
                .intruder_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("intruder list should return response");
    assert_eq!(intruder_list.status(), StatusCode::FORBIDDEN);
    let intruder_list_body = intruder_list
        .into_body()
        .collect()
        .await
        .expect("intruder list body should collect")
        .to_bytes();
    let intruder_list_json: serde_json::Value =
        serde_json::from_slice(&intruder_list_body).expect("intruder list should be valid json");
    assert_eq!(intruder_list_json["code"], "conversation_permission_denied");

    let owner_append = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_private_state/frames")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq":1,
                        "frameType":"delta",
                        "schemaRef":"custom.delta.text.v1",
                        "encoding":"json",
                        "payload":"{\"delta\":\"owner\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("owner append should succeed");
    assert_eq!(owner_append.status(), StatusCode::OK);

    let owner_list = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/streams/st_private_state/frames?afterFrameSeq=0&limit=10")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("owner list should succeed");
    assert_eq!(owner_list.status(), StatusCode::OK);
    let owner_list_body = owner_list
        .into_body()
        .collect()
        .await
        .expect("owner list body should collect")
        .to_bytes();
    let owner_list_json: serde_json::Value =
        serde_json::from_slice(&owner_list_body).expect("owner list should be valid json");
    let items = owner_list_json["items"]
        .as_array()
        .expect("items should be an array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["frameSeq"], 1);
    assert_eq!(items[0]["payload"], "{\"delta\":\"owner\"}");
}

#[tokio::test]
async fn test_group_member_governance_requires_owner_or_admin() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_group_member_governance_http",
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
                .uri("/im/v3/api/chat/conversations/c_group_member_governance_http/members/add")
                .owner_app_context()
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
        .expect("owner add member should succeed");
    assert_eq!(add_member.status(), StatusCode::OK);

    let member_add = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_group_member_governance_http/members/add")
                .member_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_extra",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("member add should return response");
    assert_eq!(member_add.status(), StatusCode::FORBIDDEN);
    let member_add_body = member_add
        .into_body()
        .collect()
        .await
        .expect("member add body should collect")
        .to_bytes();
    let member_add_json: serde_json::Value =
        serde_json::from_slice(&member_add_body).expect("member add should be valid json");
    assert_eq!(member_add_json["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_group_member_governance_rejects_bearer_actor_kind_mismatch() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_group_member_kind_guard_http",
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
                .uri("/im/v3/api/chat/conversations/c_group_member_kind_guard_http/members/add")
                .owner_as_agent_app_context()
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
        .expect("add member request should return response");
    assert_eq!(add_member.status(), StatusCode::FORBIDDEN);
    let add_member_body = add_member
        .into_body()
        .collect()
        .await
        .expect("add member body should collect")
        .to_bytes();
    let add_member_json: serde_json::Value =
        serde_json::from_slice(&add_member_body).expect("add member should be valid json");
    assert_eq!(add_member_json["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_read_cursor_rejects_bearer_actor_kind_mismatch() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_cursor_kind_guard_http",
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
                .uri("/im/v3/api/chat/conversations/c_cursor_kind_guard_http/messages")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_cursor_kind_guard_http_1",
                        "summary":"hello",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("owner post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let update_cursor = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_cursor_kind_guard_http/read_cursor")
                .owner_as_agent_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "readSeq":1,
                        "lastReadMessageId":"msg_c_cursor_kind_guard_http_1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("update cursor should return response");
    assert_eq!(update_cursor.status(), StatusCode::FORBIDDEN);
    let update_cursor_body = update_cursor
        .into_body()
        .collect()
        .await
        .expect("update cursor body should collect")
        .to_bytes();
    let update_cursor_json: serde_json::Value =
        serde_json::from_slice(&update_cursor_body).expect("update cursor should be valid json");
    assert_eq!(update_cursor_json["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_conversation_bound_stream_writes_reject_bearer_actor_kind_mismatch() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_stream_actor_kind_guard_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let mismatched_open = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .owner_as_agent_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_stream_actor_kind_guard_http",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_actor_kind_guard_http",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("mismatched open stream should return response");
    assert_eq!(mismatched_open.status(), StatusCode::FORBIDDEN);
    let mismatched_open_body = mismatched_open
        .into_body()
        .collect()
        .await
        .expect("mismatched open body should collect")
        .to_bytes();
    let mismatched_open_json: serde_json::Value = serde_json::from_slice(&mismatched_open_body)
        .expect("mismatched open stream should be valid json");
    assert_eq!(
        mismatched_open_json["code"],
        "conversation_permission_denied"
    );

    let owner_open = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_stream_actor_kind_guard_http",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_actor_kind_guard_http",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("owner open stream should succeed");
    assert_eq!(owner_open.status(), StatusCode::OK);

    let mismatched_append = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_stream_actor_kind_guard_http/frames")
                .owner_as_agent_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq":1,
                        "frameType":"delta",
                        "schemaRef":"custom.delta.text.v1",
                        "encoding":"json",
                        "payload":"{\"delta\":\"forged\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("mismatched append stream should return response");
    assert_eq!(mismatched_append.status(), StatusCode::FORBIDDEN);
    let mismatched_append_body = mismatched_append
        .into_body()
        .collect()
        .await
        .expect("mismatched append body should collect")
        .to_bytes();
    let mismatched_append_json: serde_json::Value = serde_json::from_slice(&mismatched_append_body)
        .expect("mismatched append should be valid json");
    assert_eq!(
        mismatched_append_json["code"],
        "conversation_permission_denied"
    );

    let owner_frames = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/streams/st_stream_actor_kind_guard_http/frames?afterFrameSeq=0&limit=10")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("owner frames should succeed");
    assert_eq!(owner_frames.status(), StatusCode::OK);
    let owner_frames_body = owner_frames
        .into_body()
        .collect()
        .await
        .expect("owner frames body should collect")
        .to_bytes();
    let owner_frames_json: serde_json::Value =
        serde_json::from_slice(&owner_frames_body).expect("owner frames should be valid json");
    let items = owner_frames_json["items"]
        .as_array()
        .expect("owner frames items should be an array");
    assert!(items.is_empty());
}

#[tokio::test]
async fn test_conversation_bound_rtc_writes_reject_bearer_actor_kind_mismatch() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_rtc_actor_kind_guard_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let mismatched_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/rtc/sessions")
                .owner_as_agent_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_actor_kind_guard_http",
                        "conversationId":"c_rtc_actor_kind_guard_http",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("mismatched create rtc should return response");
    assert_eq!(mismatched_create.status(), StatusCode::FORBIDDEN);
    let mismatched_create_body = mismatched_create
        .into_body()
        .collect()
        .await
        .expect("mismatched create rtc body should collect")
        .to_bytes();
    let mismatched_create_json: serde_json::Value = serde_json::from_slice(&mismatched_create_body)
        .expect("mismatched create rtc should be valid json");
    assert_eq!(
        mismatched_create_json["code"],
        "conversation_permission_denied"
    );

    let owner_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/rtc/sessions")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_actor_kind_guard_http",
                        "conversationId":"c_rtc_actor_kind_guard_http",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("owner create rtc should succeed");
    assert_eq!(owner_create.status(), StatusCode::OK);

    let mismatched_signal = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/rtc/sessions/rtc_actor_kind_guard_http/signals")
                .owner_as_agent_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalType":"rtc.offer",
                        "schemaRef":"webrtc.offer.v1",
                        "payload":"{\"sdp\":\"forged\"}",
                        "signalingStreamId":"st_rtc_actor_kind_guard_http"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("mismatched rtc signal should return response");
    assert_eq!(mismatched_signal.status(), StatusCode::FORBIDDEN);
    let mismatched_signal_body = mismatched_signal
        .into_body()
        .collect()
        .await
        .expect("mismatched rtc signal body should collect")
        .to_bytes();
    let mismatched_signal_json: serde_json::Value = serde_json::from_slice(&mismatched_signal_body)
        .expect("mismatched rtc signal should be valid json");
    assert_eq!(
        mismatched_signal_json["code"],
        "conversation_permission_denied"
    );

    let owner_messages = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_rtc_actor_kind_guard_http/messages")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("owner messages should succeed");
    assert_eq!(owner_messages.status(), StatusCode::OK);
    let owner_messages_body = owner_messages
        .into_body()
        .collect()
        .await
        .expect("owner messages body should collect")
        .to_bytes();
    let owner_messages_json: serde_json::Value =
        serde_json::from_slice(&owner_messages_body).expect("owner messages should be valid json");
    let items = owner_messages_json["items"]
        .as_array()
        .expect("owner messages items should be an array");
    assert!(items.is_empty());
}

#[tokio::test]
async fn test_direct_conversation_member_management_is_restricted() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_direct_member_governance_http",
                        "conversationType":"direct"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_peer = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_direct_member_governance_http/members/add")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_admin",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("owner add peer should succeed");
    assert_eq!(add_peer.status(), StatusCode::OK);

    let add_third = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_direct_member_governance_http/members/add")
                .owner_app_context()
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
        .expect("owner add third should return response");
    assert_eq!(add_third.status(), StatusCode::FORBIDDEN);
    let add_third_body = add_third
        .into_body()
        .collect()
        .await
        .expect("add third body should collect")
        .to_bytes();
    let add_third_json: serde_json::Value =
        serde_json::from_slice(&add_third_body).expect("add third should be valid json");
    assert_eq!(add_third_json["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_group_member_can_leave_and_then_loses_conversation_access() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_group_leave_http",
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
                .uri("/im/v3/api/chat/conversations/c_group_leave_http/members/add")
                .owner_app_context()
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
        .expect("owner add member should succeed");
    assert_eq!(add_member.status(), StatusCode::OK);

    let leave = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_group_leave_http/members/leave")
                .member_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("leave request should return response");
    assert_eq!(leave.status(), StatusCode::OK);
    let leave_body = leave
        .into_body()
        .collect()
        .await
        .expect("leave body should collect")
        .to_bytes();
    let leave_json: serde_json::Value =
        serde_json::from_slice(&leave_body).expect("leave response should be valid json");
    assert_eq!(leave_json["state"], "left");

    let member_read = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_group_leave_http")
                .member_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("member read after leave should return response");
    assert_eq!(member_read.status(), StatusCode::FORBIDDEN);

    let owner_members = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_group_leave_http/members")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("owner members should return response");
    assert_eq!(owner_members.status(), StatusCode::OK);
    let owner_members_body = owner_members
        .into_body()
        .collect()
        .await
        .expect("owner members body should collect")
        .to_bytes();
    let owner_members_json: serde_json::Value =
        serde_json::from_slice(&owner_members_body).expect("owner members should be valid json");
    assert_eq!(owner_members_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(owner_members_json["items"][0]["principalId"], "u_owner");
}

#[tokio::test]
async fn test_left_member_rejoin_gets_new_member_identity_and_fresh_cursor() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_group_rejoin_http",
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
                .uri("/im/v3/api/chat/conversations/c_group_rejoin_http/members/add")
                .owner_app_context()
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
        .expect("owner add member should succeed");
    assert_eq!(add_member.status(), StatusCode::OK);
    let add_member_body = add_member
        .into_body()
        .collect()
        .await
        .expect("add member body should collect")
        .to_bytes();
    let add_member_json: serde_json::Value =
        serde_json::from_slice(&add_member_body).expect("add member should be valid json");
    let first_member_id = add_member_json["memberId"]
        .as_str()
        .expect("add member response should include member id")
        .to_owned();

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_group_rejoin_http/messages")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_group_rejoin_http_1",
                        "summary":"hello",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("owner post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let update_cursor = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_group_rejoin_http/read_cursor")
                .member_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "readSeq":1,
                        "lastReadMessageId":"msg_c_group_rejoin_http_1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("member update cursor should succeed");
    assert_eq!(update_cursor.status(), StatusCode::OK);

    let leave = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_group_rejoin_http/members/leave")
                .member_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("leave request should return response");
    assert_eq!(leave.status(), StatusCode::OK);

    let rejoin = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_group_rejoin_http/members/add")
                .owner_app_context()
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
        .expect("owner re-add member should succeed");
    assert_eq!(rejoin.status(), StatusCode::OK);
    let rejoin_body = rejoin
        .into_body()
        .collect()
        .await
        .expect("rejoin body should collect")
        .to_bytes();
    let rejoin_json: serde_json::Value =
        serde_json::from_slice(&rejoin_body).expect("rejoin response should be valid json");
    let rejoin_member_id = rejoin_json["memberId"]
        .as_str()
        .expect("rejoin response should include member id")
        .to_owned();
    assert_ne!(rejoin_member_id, first_member_id);
    assert_eq!(rejoin_json["state"], "joined");
    assert_eq!(rejoin_json["removedAt"], serde_json::Value::Null);

    let read_cursor = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_group_rejoin_http/read_cursor")
                .member_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("rejoined member read cursor should return response");
    assert_eq!(read_cursor.status(), StatusCode::OK);
    let read_cursor_body = read_cursor
        .into_body()
        .collect()
        .await
        .expect("read cursor body should collect")
        .to_bytes();
    let read_cursor_json: serde_json::Value =
        serde_json::from_slice(&read_cursor_body).expect("read cursor should be valid json");
    assert_eq!(read_cursor_json["memberId"], rejoin_member_id);
    assert_eq!(read_cursor_json["readSeq"], 0);
    assert_eq!(
        read_cursor_json["lastReadMessageId"],
        serde_json::Value::Null
    );
}

#[tokio::test]
async fn test_group_role_change_requires_owner_and_updates_target_role() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_group_role_change_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_admin = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_group_role_change_http/members/add")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_admin",
                        "principalKind":"user",
                        "role":"admin"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("owner add admin should succeed");
    assert_eq!(add_admin.status(), StatusCode::OK);

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_group_role_change_http/members/add")
                .owner_app_context()
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
        .expect("owner add member should succeed");
    assert_eq!(add_member.status(), StatusCode::OK);

    let admin_change = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_group_role_change_http/members/change_role")
                .admin_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_group_role_change_http_user_u_member",
                        "role":"guest"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("admin change role should return response");
    assert_eq!(admin_change.status(), StatusCode::FORBIDDEN);
    let admin_change_body = admin_change
        .into_body()
        .collect()
        .await
        .expect("admin change body should collect")
        .to_bytes();
    let admin_change_json: serde_json::Value =
        serde_json::from_slice(&admin_change_body).expect("admin change should be valid json");
    assert_eq!(admin_change_json["code"], "conversation_permission_denied");

    let owner_change = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_group_role_change_http/members/change_role")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_group_role_change_http_user_u_member",
                        "role":"guest"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("owner change role should return response");
    assert_eq!(owner_change.status(), StatusCode::OK);
    let owner_change_body = owner_change
        .into_body()
        .collect()
        .await
        .expect("owner change body should collect")
        .to_bytes();
    let owner_change_json: serde_json::Value =
        serde_json::from_slice(&owner_change_body).expect("owner change should be valid json");
    assert_eq!(owner_change_json["previousMember"]["role"], "member");
    assert_eq!(owner_change_json["updatedMember"]["role"], "guest");

    let owner_members = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_group_role_change_http/members")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("owner members should return response");
    assert_eq!(owner_members.status(), StatusCode::OK);
    let owner_members_body = owner_members
        .into_body()
        .collect()
        .await
        .expect("owner members body should collect")
        .to_bytes();
    let owner_members_json: serde_json::Value =
        serde_json::from_slice(&owner_members_body).expect("owner members should be valid json");
    let target = owner_members_json["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["principalId"] == "u_member")
        .expect("member should exist");
    assert_eq!(target["role"], "guest");
}

#[tokio::test]
async fn test_group_owner_transfer_allows_safe_handoff_and_leave() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_group_transfer_http",
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
                .uri("/im/v3/api/chat/conversations/c_group_transfer_http/members/add")
                .owner_app_context()
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
        .expect("owner add member should succeed");
    assert_eq!(add_member.status(), StatusCode::OK);

    let transfer = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_group_transfer_http/members/transfer_owner")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_group_transfer_http_user_u_member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("owner transfer should return response");
    assert_eq!(transfer.status(), StatusCode::OK);
    let transfer_body = transfer
        .into_body()
        .collect()
        .await
        .expect("transfer body should collect")
        .to_bytes();
    let transfer_json: serde_json::Value =
        serde_json::from_slice(&transfer_body).expect("transfer response should be valid json");
    assert_eq!(transfer_json["previousOwner"]["role"], "admin");
    assert_eq!(transfer_json["newOwner"]["role"], "owner");

    let leave = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_group_transfer_http/members/leave")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("leave after transfer should return response");
    assert_eq!(leave.status(), StatusCode::OK);

    let old_owner_read = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_group_transfer_http")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("old owner read after leave should return response");
    assert_eq!(old_owner_read.status(), StatusCode::FORBIDDEN);

    let new_owner_members = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_group_transfer_http/members")
                .member_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("new owner members should return response");
    assert_eq!(new_owner_members.status(), StatusCode::OK);
    let new_owner_members_body = new_owner_members
        .into_body()
        .collect()
        .await
        .expect("new owner members body should collect")
        .to_bytes();
    let new_owner_members_json: serde_json::Value =
        serde_json::from_slice(&new_owner_members_body).expect("members should be valid json");
    assert_eq!(new_owner_members_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(
        new_owner_members_json["items"][0]["principalId"],
        "u_member"
    );
    assert_eq!(new_owner_members_json["items"][0]["role"], "owner");
}

#[tokio::test]
async fn test_generic_create_rejects_reserved_special_types_in_local_profile() {
    let app = local_minimal_node::build_default_app();

    for (conversation_id, conversation_type) in [
        ("c_agent_dialog_local", "agent_dialog"),
        ("c_agent_handoff_local", "agent_handoff"),
        ("c_system_channel_local", "system_channel"),
    ] {
        let create_conversation = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/chat/conversations")
                    .system_app_context()
                    .header("content-type", "application/json")
                    .body(Body::from(format!(
                        r#"{{
                            "conversationId":"{conversation_id}",
                            "conversationType":"{conversation_type}"
                        }}"#
                    )))
                    .unwrap(),
            )
            .await
            .expect("reserved special create should return response");
        assert_eq!(
            create_conversation.status(),
            StatusCode::BAD_REQUEST,
            "reserved type should be rejected: {conversation_type}"
        );
        let body = create_conversation
            .into_body()
            .collect()
            .await
            .expect("body should collect")
            .to_bytes();
        let value: serde_json::Value =
            serde_json::from_slice(&body).expect("response should be valid json");
        assert_eq!(value["code"], "conversation_type_invalid");
    }
}

#[tokio::test]
async fn test_agent_dialog_create_in_local_profile_creates_user_and_agent_members() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_dialogs")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_dialog_local",
                        "agentId":"ag_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent dialog create should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let list_members = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_agent_dialog_local/members")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("members request should return response");
    assert_eq!(list_members.status(), StatusCode::OK);
    let body = list_members
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["items"].as_array().unwrap().len(), 2);
    assert!(
        value["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["principalId"] == "u_owner" && item["principalKind"] == "user")
    );
    assert!(
        value["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["principalId"] == "ag_demo" && item["principalKind"] == "agent")
    );
}

#[tokio::test]
async fn test_agent_dialog_create_rejects_non_user_creator_in_local_profile() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_dialogs")
                .system_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_dialog_system_local",
                        "agentId":"ag_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent dialog create should return response");
    assert_eq!(create_conversation.status(), StatusCode::FORBIDDEN);
    let body = create_conversation
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_agent_handoff_create_in_local_profile_creates_agent_and_target_members() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .agent_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_local",
                        "targetId":"u_owner",
                        "targetKind":"user",
                        "handoffSessionId":"hs_local",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent handoff create should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let list_members = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_local/members")
                .agent_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("members request should return response");
    assert_eq!(list_members.status(), StatusCode::OK);
    let body = list_members
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["items"].as_array().unwrap().len(), 2);
    assert!(
        value["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["principalId"] == "ag_source" && item["principalKind"] == "agent")
    );
    assert!(
        value["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["principalId"] == "u_owner" && item["principalKind"] == "user")
    );
}

#[tokio::test]
async fn test_agent_handoff_create_rejects_non_agent_creator_in_local_profile() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_invalid_local",
                        "targetId":"u_member",
                        "targetKind":"user",
                        "handoffSessionId":"hs_invalid_local",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent handoff create should return response");
    assert_eq!(create_conversation.status(), StatusCode::FORBIDDEN);
    let body = create_conversation
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_agent_handoff_target_can_post_in_local_profile() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .agent_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_post_local",
                        "targetId":"u_owner",
                        "targetKind":"user",
                        "handoffSessionId":"hs_post_local",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent handoff create should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_message = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_post_local/messages")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_agent_handoff_target_post",
                        "text":"accepted"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("target post should return response");
    assert_eq!(post_message.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_agent_handoff_accept_resolve_close_in_local_profile() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .agent_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_lifecycle_local",
                        "targetId":"u_owner",
                        "targetKind":"user",
                        "handoffSessionId":"hs_local",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent handoff create should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let accept_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_lifecycle_local/agent_handoff/accept")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("accept request should return response");
    assert_eq!(accept_response.status(), StatusCode::OK);
    let accept_body = accept_response
        .into_body()
        .collect()
        .await
        .expect("accept body should collect")
        .to_bytes();
    let accept_json: serde_json::Value =
        serde_json::from_slice(&accept_body).expect("accept response should be valid json");
    assert_eq!(accept_json["status"], "accepted");

    let resolve_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_lifecycle_local/agent_handoff/resolve")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("resolve request should return response");
    assert_eq!(resolve_response.status(), StatusCode::OK);

    let close_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_lifecycle_local/agent_handoff/close")
                .agent_app_context()
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("close request should return response");
    assert_eq!(close_response.status(), StatusCode::OK);

    let post_after_close = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_lifecycle_local/messages")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_agent_handoff_closed_local",
                        "summary":"should fail",
                        "text":"should fail"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("closed post request should return response");
    assert_eq!(post_after_close.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_agent_handoff_accept_rejects_source_actor_in_local_profile() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .agent_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_accept_invalid_local",
                        "targetId":"u_owner",
                        "targetKind":"user",
                        "handoffSessionId":"hs_invalid_local",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent handoff create should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let accept_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_accept_invalid_local/agent_handoff/accept")
                .agent_app_context()
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("invalid accept request should return response");
    assert_eq!(accept_response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_closed_agent_handoff_blocks_conversation_bound_stream_writes_in_local_profile() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .agent_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_stream_closed_local",
                        "targetId":"u_owner",
                        "targetKind":"user",
                        "handoffSessionId":"hs_stream_closed_local",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent handoff create should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_handoff_closed_local",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_agent_handoff_stream_closed_local",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should return response");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let close_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/im/v3/api/chat/conversations/c_agent_handoff_stream_closed_local/agent_handoff/close",
                )
                .agent_app_context()
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("close request should return response");
    assert_eq!(close_response.status(), StatusCode::OK);

    let append_after_close = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_handoff_closed_local/frames")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq":1,
                        "frameType":"delta",
                        "schemaRef":"custom.delta.text.v1",
                        "encoding":"json",
                        "payload":"{\"delta\":\"blocked\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append after close should return response");
    assert_eq!(append_after_close.status(), StatusCode::CONFLICT);
    let append_after_close_body = append_after_close
        .into_body()
        .collect()
        .await
        .expect("append after close body should collect")
        .to_bytes();
    let append_after_close_json: serde_json::Value =
        serde_json::from_slice(&append_after_close_body)
            .expect("append after close response should be valid json");
    assert_eq!(append_after_close_json["code"], "conversation_conflict");

    let open_after_close = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_handoff_closed_local_reopen",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_agent_handoff_stream_closed_local",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open after close should return response");
    assert_eq!(open_after_close.status(), StatusCode::CONFLICT);
    let open_after_close_body = open_after_close
        .into_body()
        .collect()
        .await
        .expect("open after close body should collect")
        .to_bytes();
    let open_after_close_json: serde_json::Value = serde_json::from_slice(&open_after_close_body)
        .expect("open after close response should be valid json");
    assert_eq!(open_after_close_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_closed_agent_handoff_blocks_conversation_bound_rtc_writes_in_local_profile() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .agent_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_rtc_closed_local",
                        "targetId":"u_owner",
                        "targetKind":"user",
                        "handoffSessionId":"hs_rtc_closed_local",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent handoff create should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let create_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/rtc/sessions")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_handoff_closed_local",
                        "conversationId":"c_agent_handoff_rtc_closed_local",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc should return response");
    assert_eq!(create_rtc.status(), StatusCode::OK);

    let close_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_rtc_closed_local/agent_handoff/close")
                .agent_app_context()
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("close request should return response");
    assert_eq!(close_response.status(), StatusCode::OK);

    let signal_after_close = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/rtc/sessions/rtc_handoff_closed_local/signals")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalType":"rtc.offer",
                        "schemaRef":"webrtc.offer.v1",
                        "payload":"{\"sdp\":\"blocked\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("signal after close should return response");
    assert_eq!(signal_after_close.status(), StatusCode::CONFLICT);
    let signal_after_close_body = signal_after_close
        .into_body()
        .collect()
        .await
        .expect("signal after close body should collect")
        .to_bytes();
    let signal_after_close_json: serde_json::Value =
        serde_json::from_slice(&signal_after_close_body)
            .expect("signal after close response should be valid json");
    assert_eq!(signal_after_close_json["code"], "conversation_conflict");

    let create_after_close = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/rtc/sessions")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_handoff_closed_local_reopen",
                        "conversationId":"c_agent_handoff_rtc_closed_local",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create after close should return response");
    assert_eq!(create_after_close.status(), StatusCode::CONFLICT);
    let create_after_close_body = create_after_close
        .into_body()
        .collect()
        .await
        .expect("create after close body should collect")
        .to_bytes();
    let create_after_close_json: serde_json::Value =
        serde_json::from_slice(&create_after_close_body)
            .expect("create after close response should be valid json");
    assert_eq!(create_after_close_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_agent_handoff_summary_and_inbox_projection_in_local_profile() {
    let app = local_minimal_node::build_default_app();

    let create_handoff = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .agent_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_projection_local",
                        "targetId":"u_member",
                        "targetKind":"user",
                        "handoffSessionId":"hs_projection_local",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create agent handoff should succeed");
    assert_eq!(create_handoff.status(), StatusCode::OK);

    let initial_summary = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_projection_local")
                .agent_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get initial handoff summary should succeed");
    assert_eq!(initial_summary.status(), StatusCode::OK);
    let initial_summary_body = initial_summary
        .into_body()
        .collect()
        .await
        .expect("initial summary body should collect")
        .to_bytes();
    let initial_summary_json: serde_json::Value = serde_json::from_slice(&initial_summary_body)
        .expect("initial summary should be valid json");
    assert_eq!(initial_summary_json["messageCount"], 0);
    assert_eq!(
        initial_summary_json["lastMessageId"],
        serde_json::Value::Null
    );
    assert_eq!(initial_summary_json["agentHandoff"]["status"], "open");
    assert_eq!(
        initial_summary_json["agentHandoff"]["source"]["id"],
        "ag_source"
    );
    assert_eq!(
        initial_summary_json["agentHandoff"]["target"]["id"],
        "u_member"
    );

    let initial_inbox = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/inbox")
                .member_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get initial handoff inbox should succeed");
    assert_eq!(initial_inbox.status(), StatusCode::OK);
    let initial_inbox_body = initial_inbox
        .into_body()
        .collect()
        .await
        .expect("initial inbox body should collect")
        .to_bytes();
    let initial_inbox_json: serde_json::Value =
        serde_json::from_slice(&initial_inbox_body).expect("initial inbox should be valid json");
    assert_eq!(
        initial_inbox_json["items"][0]["conversationId"],
        "c_agent_handoff_projection_local"
    );
    assert_eq!(
        initial_inbox_json["items"][0]["agentHandoff"]["status"],
        "open"
    );
    assert_eq!(initial_inbox_json["items"][0]["messageCount"], 0);

    let accept_handoff = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_projection_local/agent_handoff/accept")
                .member_app_context()
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("accept agent handoff should succeed");
    assert_eq!(accept_handoff.status(), StatusCode::OK);

    let accepted_summary = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_projection_local")
                .member_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get accepted handoff summary should succeed");
    assert_eq!(accepted_summary.status(), StatusCode::OK);
    let accepted_summary_body = accepted_summary
        .into_body()
        .collect()
        .await
        .expect("accepted summary body should collect")
        .to_bytes();
    let accepted_summary_json: serde_json::Value = serde_json::from_slice(&accepted_summary_body)
        .expect("accepted summary should be valid json");
    assert_eq!(accepted_summary_json["agentHandoff"]["status"], "accepted");
    assert_eq!(
        accepted_summary_json["agentHandoff"]["acceptedBy"]["id"],
        "u_member"
    );

    let accepted_inbox = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/inbox")
                .member_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get accepted handoff inbox should succeed");
    assert_eq!(accepted_inbox.status(), StatusCode::OK);
    let accepted_inbox_body = accepted_inbox
        .into_body()
        .collect()
        .await
        .expect("accepted inbox body should collect")
        .to_bytes();
    let accepted_inbox_json: serde_json::Value =
        serde_json::from_slice(&accepted_inbox_body).expect("accepted inbox should be valid json");
    assert_eq!(
        accepted_inbox_json["items"][0]["agentHandoff"]["status"],
        "accepted"
    );
}

#[tokio::test]
async fn test_system_channel_create_in_local_profile_creates_system_and_subscriber_members() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .system_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_local",
                        "subscriberId":"u_owner"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("system channel create should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let list_members = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_system_channel_local/members")
                .system_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("members request should return response");
    assert_eq!(list_members.status(), StatusCode::OK);
    let body = list_members
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["items"].as_array().unwrap().len(), 2);
    assert!(
        value["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["principalId"] == "svc_ops" && item["principalKind"] == "system")
    );
    assert!(
        value["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["principalId"] == "u_owner" && item["principalKind"] == "user")
    );
}

#[tokio::test]
async fn test_system_channel_create_rejects_non_system_creator_in_local_profile() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_invalid_local",
                        "subscriberId":"u_member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("system channel create should return response");
    assert_eq!(create_conversation.status(), StatusCode::FORBIDDEN);
    let body = create_conversation
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_system_channel_subscriber_cannot_post_in_local_profile() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .system_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_post_local",
                        "subscriberId":"u_owner"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("system channel create should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_message = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_system_channel_post_local/messages")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_system_channel_subscriber_post",
                        "text":"should fail"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscriber post should return response");
    assert_eq!(post_message.status(), StatusCode::FORBIDDEN);
    let body = post_message
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_system_channel_publisher_must_use_dedicated_publish_in_local_profile() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .system_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_publish_local",
                        "subscriberId":"u_owner"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("system channel create should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_message = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_system_channel_publish_local/messages")
                .system_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_system_channel_generic_publish_local",
                        "text":"must use dedicated route"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("generic publisher post should return response");
    assert_eq!(post_message.status(), StatusCode::FORBIDDEN);
    let body = post_message
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_system_channel_dedicated_publish_allows_only_publisher_in_local_profile() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .system_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_publish_local_dedicated",
                        "subscriberId":"u_owner"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("system channel create should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let publish_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_system_channel_publish_local_dedicated/system_channel/publish")
                .system_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_system_channel_publish_local_dedicated",
                        "text":"system notice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("dedicated publish should return response");
    assert_eq!(publish_message.status(), StatusCode::OK);
    let publish_body = publish_message
        .into_body()
        .collect()
        .await
        .expect("publish body should collect")
        .to_bytes();
    let publish_value: serde_json::Value =
        serde_json::from_slice(&publish_body).expect("response should be valid json");
    assert_eq!(publish_value["messageSeq"], 1);

    let subscriber_publish = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_system_channel_publish_local_dedicated/system_channel/publish")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_system_channel_publish_local_subscriber",
                        "text":"should fail"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscriber dedicated publish should return response");
    assert_eq!(subscriber_publish.status(), StatusCode::FORBIDDEN);
    let subscriber_body = subscriber_publish
        .into_body()
        .collect()
        .await
        .expect("subscriber body should collect")
        .to_bytes();
    let subscriber_value: serde_json::Value =
        serde_json::from_slice(&subscriber_body).expect("response should be valid json");
    assert_eq!(subscriber_value["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_system_channel_subscriber_cannot_write_conversation_bound_streams_in_local_profile() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .system_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_stream_local",
                        "subscriberId":"u_owner"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("system channel create should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .system_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_system_channel_local",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_system_channel_stream_local",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("system stream open should return response");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_system_channel_local/frames")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq":1,
                        "frameType":"delta",
                        "schemaRef":"custom.delta.text.v1",
                        "encoding":"json",
                        "payload":"{\"delta\":\"blocked\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscriber append should return response");
    assert_eq!(append_frame.status(), StatusCode::FORBIDDEN);
    let append_body = append_frame
        .into_body()
        .collect()
        .await
        .expect("subscriber append body should collect")
        .to_bytes();
    let append_json: serde_json::Value =
        serde_json::from_slice(&append_body).expect("append response should be valid json");
    assert_eq!(append_json["code"], "conversation_permission_denied");

    let open_stream_as_subscriber = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_system_channel_local_subscriber",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_system_channel_stream_local",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscriber open should return response");
    assert_eq!(open_stream_as_subscriber.status(), StatusCode::FORBIDDEN);
    let open_body = open_stream_as_subscriber
        .into_body()
        .collect()
        .await
        .expect("subscriber open body should collect")
        .to_bytes();
    let open_json: serde_json::Value =
        serde_json::from_slice(&open_body).expect("open response should be valid json");
    assert_eq!(open_json["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_system_channel_subscriber_cannot_write_conversation_bound_rtc_in_local_profile() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .system_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_rtc_local",
                        "subscriberId":"u_owner"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("system channel create should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let create_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/rtc/sessions")
                .system_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_system_channel_local",
                        "conversationId":"c_system_channel_rtc_local",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("system rtc create should return response");
    assert_eq!(create_rtc.status(), StatusCode::OK);

    let post_signal = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/rtc/sessions/rtc_system_channel_local/signals")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalType":"rtc.offer",
                        "schemaRef":"webrtc.offer.v1",
                        "payload":"{\"sdp\":\"blocked\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscriber signal should return response");
    assert_eq!(post_signal.status(), StatusCode::FORBIDDEN);
    let signal_body = post_signal
        .into_body()
        .collect()
        .await
        .expect("subscriber signal body should collect")
        .to_bytes();
    let signal_json: serde_json::Value =
        serde_json::from_slice(&signal_body).expect("signal response should be valid json");
    assert_eq!(signal_json["code"], "conversation_permission_denied");

    let create_rtc_as_subscriber = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/rtc/sessions")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_system_channel_local_subscriber",
                        "conversationId":"c_system_channel_rtc_local",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscriber rtc create should return response");
    assert_eq!(create_rtc_as_subscriber.status(), StatusCode::FORBIDDEN);
    let create_body = create_rtc_as_subscriber
        .into_body()
        .collect()
        .await
        .expect("subscriber rtc create body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(&create_body).expect("rtc create response should be valid json");
    assert_eq!(create_json["code"], "conversation_permission_denied");
}
