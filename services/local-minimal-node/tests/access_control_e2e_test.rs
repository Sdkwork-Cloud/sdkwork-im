use im_app_context::DualTokenRequestBuilderExt;
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
        self.with_dual_token_tenant("t_demo")
            .with_dual_token_user("u_owner")
            .with_dual_token_actor_kind("user")
            .with_dual_token_session("s_owner")
            .with_dual_token_device("d_owner")
    }

    fn owner_as_agent_app_context(self) -> Self {
        self.with_dual_token_tenant("t_demo")
            .with_dual_token_user("u_owner")
            .with_dual_token_actor("u_owner")
            .with_dual_token_actor_kind("agent")
            .with_dual_token_session("s_owner")
            .with_dual_token_device("d_owner")
    }

    fn admin_app_context(self) -> Self {
        self.with_dual_token_tenant("t_demo")
            .with_dual_token_user("u_admin")
            .with_dual_token_actor_kind("user")
            .with_dual_token_session("s_admin")
            .with_dual_token_device("d_admin")
    }

    fn member_app_context(self) -> Self {
        self.with_dual_token_tenant("t_demo")
            .with_dual_token_user("u_member")
            .with_dual_token_actor_kind("user")
            .with_dual_token_session("s_member")
            .with_dual_token_device("d_member")
    }

    fn intruder_app_context(self) -> Self {
        self.with_dual_token_tenant("t_demo")
            .with_dual_token_user("u_intruder")
            .with_dual_token_actor_kind("user")
            .with_dual_token_session("s_intruder")
            .with_dual_token_device("d_intruder")
    }

    fn agent_app_context(self) -> Self {
        self.with_dual_token_tenant("t_demo")
            .with_dual_token_user("ag_source")
            .with_dual_token_actor_kind("agent")
            .with_dual_token_session("s_agent")
            .with_dual_token_device("d_agent")
    }

    fn system_app_context(self) -> Self {
        self.with_dual_token_tenant("t_demo")
            .with_dual_token_user("svc_ops")
            .with_dual_token_actor_kind("system")
            .with_dual_token_session("s_system")
            .with_dual_token_device("d_system")
    }
}

async fn create_owner_friendship_for_test(app: &axum::Router, target_user_id: &str) {
    let submit_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/social/friend_requests")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "targetUserId":"{target_user_id}",
                        "requestMessage":"hello"
                    }}"#,
                )))
                .unwrap(),
        )
        .await
        .expect("friend request submit should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("friend request submit body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("friend request submit should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("friend request submit should return request id")
        .to_owned();

    let accept_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friend_requests/{request_id}/accept"
                ))
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user(target_user_id)
                .with_dual_token_actor_kind("user")
                .with_dual_token_session(format!("s_{target_user_id}"))
                .with_dual_token_device(format!("d_{target_user_id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request accept should return response");
    assert_eq!(accept_request.status(), StatusCode::OK);
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
async fn test_local_profile_message_reply_reference_is_preserved_in_timeline() {
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
                        "conversationId":"c_reply_reference_timeline",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_root = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_reply_reference_timeline/messages")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_reply_root",
                        "summary":"root message",
                        "text":"root message"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post root message should return response");
    assert_eq!(post_root.status(), StatusCode::OK);
    let post_root_body = post_root
        .into_body()
        .collect()
        .await
        .expect("post root body should collect")
        .to_bytes();
    let post_root_json: serde_json::Value =
        serde_json::from_slice(&post_root_body).expect("post root should be valid json");
    let root_message_id = post_root_json["messageId"]
        .as_str()
        .expect("root post should return message id");

    let post_reply = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_reply_reference_timeline/messages")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "clientMsgId":"client_reply_child",
                        "summary":"reply message",
                        "text":"reply message",
                        "replyTo":{{
                            "messageId":"{root_message_id}",
                            "senderDisplayName":"Owner",
                            "contentPreview":"root message"
                        }}
                    }}"#,
                )))
                .unwrap(),
        )
        .await
        .expect("post reply message should return response");
    assert_eq!(post_reply.status(), StatusCode::OK);

    let timeline = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_reply_reference_timeline/messages")
                .owner_app_context()
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
    assert_eq!(
        timeline_json["items"][1]["body"]["replyTo"]["messageId"],
        root_message_id
    );
    assert_eq!(
        timeline_json["items"][1]["body"]["replyTo"]["senderDisplayName"],
        "Owner"
    );
    assert_eq!(
        timeline_json["items"][1]["body"]["replyTo"]["contentPreview"],
        "root message"
    );
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
                .uri("/im/v3/api/calls/sessions")
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
                .uri("/im/v3/api/calls/sessions")
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
                .uri("/im/v3/api/calls/sessions/rtc_private_signal/signals")
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
                .uri("/im/v3/api/calls/sessions/rtc_private_signal/signals")
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
async fn test_local_profile_message_reaction_and_pin_routes_are_sdk_backed() {
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
                        "conversationId":"c_message_interaction_routes",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_message_interaction_routes/messages")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_message_interaction_routes_1",
                        "summary":"hello",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should return response");
    assert_eq!(post_message.status(), StatusCode::OK);
    let post_body = post_message
        .into_body()
        .collect()
        .await
        .expect("post body should collect")
        .to_bytes();
    let post_value: serde_json::Value =
        serde_json::from_slice(&post_body).expect("post response should be valid json");
    let message_id = post_value["messageId"]
        .as_str()
        .expect("post response should include message id");

    let add_reaction = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/im/v3/api/chat/messages/{message_id}/reactions"))
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(r#"{"reactionKey":"thumbs_up"}"#))
                .unwrap(),
        )
        .await
        .expect("add reaction should return response");
    assert_eq!(add_reaction.status(), StatusCode::OK);
    let add_reaction_body = add_reaction
        .into_body()
        .collect()
        .await
        .expect("add reaction body should collect")
        .to_bytes();
    let add_reaction_value: serde_json::Value =
        serde_json::from_slice(&add_reaction_body).expect("add reaction should be valid json");
    assert_eq!(add_reaction_value["messageId"], message_id);
    assert_eq!(add_reaction_value["reactionKey"], "thumbs_up");
    assert_eq!(add_reaction_value["changed"], true);

    let pin = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/im/v3/api/chat/messages/{message_id}/pin"))
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("pin should return response");
    assert_eq!(pin.status(), StatusCode::OK);
    let pin_body = pin
        .into_body()
        .collect()
        .await
        .expect("pin body should collect")
        .to_bytes();
    let pin_value: serde_json::Value =
        serde_json::from_slice(&pin_body).expect("pin response should be valid json");
    assert_eq!(pin_value["messageId"], message_id);
    assert_eq!(pin_value["changed"], true);

    let interaction_summary = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/c_message_interaction_routes/messages/{message_id}/interaction_summary"
                ))
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("interaction summary should return response");
    assert_eq!(interaction_summary.status(), StatusCode::OK);
    let interaction_summary_body = interaction_summary
        .into_body()
        .collect()
        .await
        .expect("interaction summary body should collect")
        .to_bytes();
    let interaction_summary_value: serde_json::Value =
        serde_json::from_slice(&interaction_summary_body)
            .expect("interaction summary should be valid json");
    assert_eq!(interaction_summary_value["messageId"], message_id);
    assert_eq!(interaction_summary_value["totalReactionCount"], 1);
    assert_eq!(
        interaction_summary_value["reactionCounts"][0]["reactionKey"],
        "thumbs_up"
    );
    assert_eq!(interaction_summary_value["reactionCounts"][0]["count"], 1);
    assert_eq!(
        interaction_summary_value["pin"]["pinnedBy"]["id"],
        "u_owner"
    );

    let pinned_messages = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_message_interaction_routes/pins")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("pinned messages should return response");
    assert_eq!(pinned_messages.status(), StatusCode::OK);
    let pinned_messages_body = pinned_messages
        .into_body()
        .collect()
        .await
        .expect("pinned messages body should collect")
        .to_bytes();
    let pinned_messages_value: serde_json::Value = serde_json::from_slice(&pinned_messages_body)
        .expect("pinned messages should be valid json");
    let pinned_items = pinned_messages_value["items"]
        .as_array()
        .expect("pinned messages should include items");
    assert_eq!(pinned_items.len(), 1);
    assert_eq!(pinned_items[0]["messageId"], message_id);

    let unpin = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/im/v3/api/chat/messages/{message_id}/unpin"))
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("unpin should return response");
    assert_eq!(unpin.status(), StatusCode::OK);
    let unpin_body = unpin
        .into_body()
        .collect()
        .await
        .expect("unpin body should collect")
        .to_bytes();
    let unpin_value: serde_json::Value =
        serde_json::from_slice(&unpin_body).expect("unpin response should be valid json");
    assert_eq!(unpin_value["messageId"], message_id);
    assert_eq!(unpin_value["changed"], true);

    let remove_reaction = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/messages/{message_id}/reactions/remove"
                ))
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(r#"{"reactionKey":"thumbs_up"}"#))
                .unwrap(),
        )
        .await
        .expect("remove reaction should return response");
    assert_eq!(remove_reaction.status(), StatusCode::OK);
    let remove_reaction_body = remove_reaction
        .into_body()
        .collect()
        .await
        .expect("remove reaction body should collect")
        .to_bytes();
    let remove_reaction_value: serde_json::Value = serde_json::from_slice(&remove_reaction_body)
        .expect("remove reaction should be valid json");
    assert_eq!(remove_reaction_value["messageId"], message_id);
    assert_eq!(remove_reaction_value["reactionKey"], "thumbs_up");
    assert_eq!(remove_reaction_value["changed"], true);

    let cleared_summary = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/c_message_interaction_routes/messages/{message_id}/interaction_summary"
                ))
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("cleared interaction summary should return response");
    assert_eq!(cleared_summary.status(), StatusCode::OK);
    let cleared_summary_body = cleared_summary
        .into_body()
        .collect()
        .await
        .expect("cleared interaction summary body should collect")
        .to_bytes();
    let cleared_summary_value: serde_json::Value = serde_json::from_slice(&cleared_summary_body)
        .expect("cleared interaction summary should be valid json");
    assert_eq!(cleared_summary_value["totalReactionCount"], 0);
    assert_eq!(
        cleared_summary_value["reactionCounts"]
            .as_array()
            .expect("cleared reaction counts should be array")
            .len(),
        0
    );
    assert!(cleared_summary_value["pin"].is_null());

    let cleared_pinned_messages = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_message_interaction_routes/pins")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("cleared pinned messages should return response");
    assert_eq!(cleared_pinned_messages.status(), StatusCode::OK);
    let cleared_pinned_messages_body = cleared_pinned_messages
        .into_body()
        .collect()
        .await
        .expect("cleared pinned messages body should collect")
        .to_bytes();
    let cleared_pinned_messages_value: serde_json::Value =
        serde_json::from_slice(&cleared_pinned_messages_body)
            .expect("cleared pinned messages should be valid json");
    assert_eq!(
        cleared_pinned_messages_value["items"]
            .as_array()
            .expect("cleared pinned messages items should be array")
            .len(),
        0
    );
}

#[tokio::test]
async fn test_local_profile_conversation_preferences_are_member_scoped_and_sdk_backed() {
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
                        "conversationId":"c_conversation_preferences_routes",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_conversation_preferences_routes/members/add")
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
        .expect("add member should return response");
    assert_eq!(add_member.status(), StatusCode::OK);

    let owner_update = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/im/v3/api/chat/conversations/c_conversation_preferences_routes/preferences")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"isPinned":true,"isMuted":true,"isMarkedUnread":true,"isHidden":true}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("owner preferences update should return response");
    assert_eq!(owner_update.status(), StatusCode::OK);
    let owner_update_body = owner_update
        .into_body()
        .collect()
        .await
        .expect("owner update body should collect")
        .to_bytes();
    let owner_update_value: serde_json::Value =
        serde_json::from_slice(&owner_update_body).expect("owner update should be valid json");
    assert_eq!(
        owner_update_value["conversationId"],
        "c_conversation_preferences_routes"
    );
    assert_eq!(owner_update_value["principalKind"], "user");
    assert_eq!(owner_update_value["principalId"], "u_owner");
    assert_eq!(owner_update_value["isPinned"], true);
    assert_eq!(owner_update_value["isMuted"], true);
    assert_eq!(owner_update_value["isMarkedUnread"], true);
    assert_eq!(owner_update_value["isHidden"], true);
    assert!(owner_update_value["updatedAt"].as_str().is_some());

    let member_default = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_conversation_preferences_routes/preferences")
                .member_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("member preferences read should return response");
    assert_eq!(member_default.status(), StatusCode::OK);
    let member_default_body = member_default
        .into_body()
        .collect()
        .await
        .expect("member preferences body should collect")
        .to_bytes();
    let member_default_value: serde_json::Value = serde_json::from_slice(&member_default_body)
        .expect("member preferences should be valid json");
    assert_eq!(member_default_value["principalId"], "u_member");
    assert_eq!(member_default_value["isPinned"], false);
    assert_eq!(member_default_value["isMuted"], false);
    assert_eq!(member_default_value["isMarkedUnread"], false);
    assert_eq!(member_default_value["isHidden"], false);

    let member_update = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/im/v3/api/chat/conversations/c_conversation_preferences_routes/preferences")
                .member_app_context()
                .header("content-type", "application/json")
                .body(Body::from(r#"{"isMuted":true,"isMarkedUnread":true}"#))
                .unwrap(),
        )
        .await
        .expect("member preferences update should return response");
    assert_eq!(member_update.status(), StatusCode::OK);

    let owner_read = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_conversation_preferences_routes/preferences")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("owner preferences read should return response");
    assert_eq!(owner_read.status(), StatusCode::OK);
    let owner_read_body = owner_read
        .into_body()
        .collect()
        .await
        .expect("owner preferences body should collect")
        .to_bytes();
    let owner_read_value: serde_json::Value =
        serde_json::from_slice(&owner_read_body).expect("owner preferences should be valid json");
    assert_eq!(owner_read_value["isPinned"], true);
    assert_eq!(owner_read_value["isMuted"], true);
    assert_eq!(owner_read_value["isMarkedUnread"], true);
    assert_eq!(owner_read_value["isHidden"], true);

    let intruder_read = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_conversation_preferences_routes/preferences")
                .intruder_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("intruder preferences read should return response");
    assert_eq!(intruder_read.status(), StatusCode::FORBIDDEN);

    let intruder_update = app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/im/v3/api/chat/conversations/c_conversation_preferences_routes/preferences")
                .intruder_app_context()
                .header("content-type", "application/json")
                .body(Body::from(r#"{"isPinned":false}"#))
                .unwrap(),
        )
        .await
        .expect("intruder preferences update should return response");
    assert_eq!(intruder_update.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_local_profile_contact_preferences_are_friendship_scoped_and_sdk_backed() {
    let app = local_minimal_node::build_default_app();

    let submit_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/social/friend_requests")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "targetUserId":"u_member",
                        "requestMessage":"hello member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request submit should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("friend request submit body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("friend request submit should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("friend request submit should return request id")
        .to_owned();

    let accept_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friend_requests/{request_id}/accept"
                ))
                .member_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request accept should return response");
    assert_eq!(accept_request.status(), StatusCode::OK);

    let owner_update = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/im/v3/api/social/contacts/u_member/preferences")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"isStarred":true,"remark":"Ops partner","isBlocked":true}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("owner contact preferences update should return response");
    assert_eq!(owner_update.status(), StatusCode::OK);
    let owner_update_body = owner_update
        .into_body()
        .collect()
        .await
        .expect("owner contact preferences update body should collect")
        .to_bytes();
    let owner_update_value: serde_json::Value = serde_json::from_slice(&owner_update_body)
        .expect("owner contact preferences update should be valid json");
    assert_eq!(owner_update_value["tenantId"], "t_demo");
    assert_eq!(owner_update_value["ownerUserId"], "u_owner");
    assert_eq!(owner_update_value["targetUserId"], "u_member");
    assert_eq!(
        owner_update_value["isStarred"], false,
        "blocking a contact should clear starred state so PC lists remain mutually consistent"
    );
    assert_eq!(owner_update_value["remark"], "Ops partner");
    assert_eq!(owner_update_value["isBlocked"], true);
    assert!(owner_update_value["updatedAt"].as_str().is_some());

    let owner_read = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/social/contacts/u_member/preferences")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("owner contact preferences read should return response");
    assert_eq!(owner_read.status(), StatusCode::OK);
    let owner_read_body = owner_read
        .into_body()
        .collect()
        .await
        .expect("owner contact preferences read body should collect")
        .to_bytes();
    let owner_read_value: serde_json::Value = serde_json::from_slice(&owner_read_body)
        .expect("owner contact preferences read should be valid json");
    assert_eq!(owner_read_value["isStarred"], false);
    assert_eq!(owner_read_value["remark"], "Ops partner");
    assert_eq!(owner_read_value["isBlocked"], true);

    let owner_contacts = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/contacts")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("owner contacts after block should return response");
    assert_eq!(owner_contacts.status(), StatusCode::OK);
    let owner_contacts_body = owner_contacts
        .into_body()
        .collect()
        .await
        .expect("owner contacts after block body should collect")
        .to_bytes();
    let owner_contacts_value: serde_json::Value = serde_json::from_slice(&owner_contacts_body)
        .expect("owner contacts after block should be valid json");
    assert_eq!(
        owner_contacts_value["items"]
            .as_array()
            .expect("owner contacts after block items should be array")
            .len(),
        0,
        "blocked contact must not remain visible in owner contact list"
    );

    let member_default = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/social/contacts/u_owner/preferences")
                .member_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("member contact preferences read should return response");
    assert_eq!(member_default.status(), StatusCode::OK);
    let member_default_body = member_default
        .into_body()
        .collect()
        .await
        .expect("member contact preferences read body should collect")
        .to_bytes();
    let member_default_value: serde_json::Value = serde_json::from_slice(&member_default_body)
        .expect("member contact preferences read should be valid json");
    assert_eq!(member_default_value["ownerUserId"], "u_member");
    assert_eq!(member_default_value["targetUserId"], "u_owner");
    assert_eq!(member_default_value["isStarred"], false);
    assert_eq!(member_default_value["remark"], "");
    assert_eq!(member_default_value["isBlocked"], false);

    let member_contacts = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/contacts")
                .member_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("member contacts should return response");
    assert_eq!(member_contacts.status(), StatusCode::OK);
    let member_contacts_body = member_contacts
        .into_body()
        .collect()
        .await
        .expect("member contacts body should collect")
        .to_bytes();
    let member_contacts_value: serde_json::Value = serde_json::from_slice(&member_contacts_body)
        .expect("member contacts should be valid json");
    let member_contacts_items = member_contacts_value["items"]
        .as_array()
        .expect("member contacts items should be array");
    assert_eq!(member_contacts_items.len(), 1);
    assert_eq!(member_contacts_items[0]["targetUserId"], "u_owner");

    let intruder_read = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/social/contacts/u_member/preferences")
                .intruder_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("intruder contact preferences read should return response");
    assert_eq!(intruder_read.status(), StatusCode::FORBIDDEN);

    let intruder_update = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/im/v3/api/social/contacts/u_member/preferences")
                .intruder_app_context()
                .header("content-type", "application/json")
                .body(Body::from(r#"{"isBlocked":false}"#))
                .unwrap(),
        )
        .await
        .expect("intruder contact preferences update should return response");
    assert_eq!(intruder_update.status(), StatusCode::FORBIDDEN);

    let agent_read = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/social/contacts/u_member/preferences")
                .owner_as_agent_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("agent contact preferences read should return response");
    assert_eq!(agent_read.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_local_profile_contact_tags_and_recommendations_are_user_scoped_and_sdk_backed() {
    let app = local_minimal_node::build_default_app();
    create_owner_friendship_for_test(&app, "u_member").await;

    let create_family = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/social/contacts/tags")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "name":"Family",
                        "color":"bg-red-500",
                        "count":2,
                        "bg":"bg-red-500/10",
                        "border":"border-red-500/20"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("owner contact tag create should return response");
    assert_eq!(create_family.status(), StatusCode::OK);
    let create_family_body = create_family
        .into_body()
        .collect()
        .await
        .expect("owner contact tag create body should collect")
        .to_bytes();
    let create_family_json: serde_json::Value = serde_json::from_slice(&create_family_body)
        .expect("owner contact tag create should be valid json");
    assert_eq!(create_family_json["tenantId"], "t_demo");
    assert_eq!(create_family_json["ownerUserId"], "u_owner");
    assert_eq!(create_family_json["name"], "Family");
    assert_eq!(create_family_json["color"], "bg-red-500");
    assert_eq!(create_family_json["count"], 2);
    assert!(create_family_json["tagId"].as_str().is_some());
    let family_tag_id = create_family_json["tagId"]
        .as_str()
        .expect("created tag must include tagId")
        .to_owned();

    let create_team = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/social/contacts/tags")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "name":"Team",
                        "color":"bg-indigo-500",
                        "count":0,
                        "bg":"bg-indigo-500/10",
                        "border":"border-indigo-500/20"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("owner second contact tag create should return response");
    assert_eq!(create_team.status(), StatusCode::OK);

    let first_page = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/social/contacts/tags?limit=1")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("owner contact tags first page should return response");
    assert_eq!(first_page.status(), StatusCode::OK);
    let first_page_body = first_page
        .into_body()
        .collect()
        .await
        .expect("owner contact tags first page body should collect")
        .to_bytes();
    let first_page_json: serde_json::Value = serde_json::from_slice(&first_page_body)
        .expect("owner contact tags first page should be valid json");
    assert_eq!(
        first_page_json["items"]
            .as_array()
            .expect("contact tags first page items should be array")
            .len(),
        1
    );
    assert_eq!(first_page_json["hasMore"], true);
    let next_cursor = first_page_json["nextCursor"]
        .as_str()
        .expect("contact tags first page should expose nextCursor")
        .to_owned();

    let second_page = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/social/contacts/tags?limit=1&cursor={next_cursor}"
                ))
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("owner contact tags second page should return response");
    assert_eq!(second_page.status(), StatusCode::OK);
    let second_page_body = second_page
        .into_body()
        .collect()
        .await
        .expect("owner contact tags second page body should collect")
        .to_bytes();
    let second_page_json: serde_json::Value = serde_json::from_slice(&second_page_body)
        .expect("owner contact tags second page should be valid json");
    assert_eq!(
        second_page_json["items"]
            .as_array()
            .expect("contact tags second page items should be array")
            .len(),
        1
    );
    assert_eq!(second_page_json["hasMore"], false);

    let member_tags = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/social/contacts/tags")
                .member_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("member contact tags should return response");
    assert_eq!(member_tags.status(), StatusCode::OK);
    let member_tags_body = member_tags
        .into_body()
        .collect()
        .await
        .expect("member contact tags body should collect")
        .to_bytes();
    let member_tags_json: serde_json::Value = serde_json::from_slice(&member_tags_body)
        .expect("member contact tags should be valid json");
    assert_eq!(
        member_tags_json["items"]
            .as_array()
            .expect("member contact tags items should be array")
            .len(),
        0,
        "contact tags must be scoped to the current user"
    );

    let update_family = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/im/v3/api/social/contacts/tags/{family_tag_id}"))
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"Family Team","count":3}"#))
                .unwrap(),
        )
        .await
        .expect("owner contact tag update should return response");
    assert_eq!(update_family.status(), StatusCode::OK);
    let update_family_body = update_family
        .into_body()
        .collect()
        .await
        .expect("owner contact tag update body should collect")
        .to_bytes();
    let update_family_json: serde_json::Value = serde_json::from_slice(&update_family_body)
        .expect("owner contact tag update should be valid json");
    assert_eq!(update_family_json["name"], "Family Team");
    assert_eq!(update_family_json["count"], 3);

    let recommend = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/social/contacts/u_member/recommendations")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("owner contact recommendation should return response");
    assert_eq!(recommend.status(), StatusCode::OK);
    let recommend_body = recommend
        .into_body()
        .collect()
        .await
        .expect("owner contact recommendation body should collect")
        .to_bytes();
    let recommend_json: serde_json::Value = serde_json::from_slice(&recommend_body)
        .expect("owner contact recommendation should be valid json");
    assert_eq!(recommend_json["tenantId"], "t_demo");
    assert_eq!(recommend_json["ownerUserId"], "u_owner");
    assert_eq!(recommend_json["targetUserId"], "u_member");
    assert!(recommend_json["recommendationId"].as_str().is_some());

    let intruder_recommend = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/social/contacts/u_member/recommendations")
                .intruder_app_context()
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("intruder contact recommendation should return response");
    assert_eq!(intruder_recommend.status(), StatusCode::FORBIDDEN);

    let agent_tag_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/social/contacts/tags")
                .owner_as_agent_app_context()
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"Agent","color":"bg-gray-500"}"#))
                .unwrap(),
        )
        .await
        .expect("agent contact tag create should return response");
    assert_eq!(agent_tag_create.status(), StatusCode::FORBIDDEN);

    let delete_family = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/im/v3/api/social/contacts/tags/{family_tag_id}"))
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("owner contact tag delete should return response");
    assert_eq!(delete_family.status(), StatusCode::OK);
    let delete_family_body = delete_family
        .into_body()
        .collect()
        .await
        .expect("owner contact tag delete body should collect")
        .to_bytes();
    let delete_family_json: serde_json::Value = serde_json::from_slice(&delete_family_body)
        .expect("owner contact tag delete should be valid json");
    assert_eq!(delete_family_json["tagId"], family_tag_id);
    assert_eq!(delete_family_json["deleted"], true);
}

#[tokio::test]
async fn test_local_profile_contacts_list_is_cursor_bounded_for_data_sync() {
    let app = local_minimal_node::build_default_app();

    for target_user_id in ["u_alpha", "u_beta", "u_gamma"] {
        create_owner_friendship_for_test(&app, target_user_id).await;
    }

    let first_page = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/contacts?limit=2")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("first contacts page should return response");
    assert_eq!(first_page.status(), StatusCode::OK);
    let first_body = first_page
        .into_body()
        .collect()
        .await
        .expect("first contacts page body should collect")
        .to_bytes();
    let first_json: serde_json::Value =
        serde_json::from_slice(&first_body).expect("first contacts page should be valid json");
    let first_items = first_json["items"]
        .as_array()
        .expect("first contacts page items should be array");
    assert_eq!(first_items.len(), 2);
    assert_eq!(first_json["hasMore"], true);
    let next_cursor = first_json["nextCursor"]
        .as_str()
        .expect("first contacts page should include nextCursor")
        .to_owned();

    let second_page = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/contacts?limit=2&cursor={next_cursor}"
                ))
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second contacts page should return response");
    assert_eq!(second_page.status(), StatusCode::OK);
    let second_body = second_page
        .into_body()
        .collect()
        .await
        .expect("second contacts page body should collect")
        .to_bytes();
    let second_json: serde_json::Value =
        serde_json::from_slice(&second_body).expect("second contacts page should be valid json");
    let second_items = second_json["items"]
        .as_array()
        .expect("second contacts page items should be array");
    assert_eq!(second_items.len(), 1);
    assert_eq!(second_json["hasMore"], false);

    let mut target_user_ids = first_items
        .iter()
        .chain(second_items.iter())
        .map(|item| {
            item["targetUserId"]
                .as_str()
                .expect("contact item should include targetUserId")
                .to_owned()
        })
        .collect::<Vec<_>>();
    target_user_ids.sort();
    assert_eq!(target_user_ids, ["u_alpha", "u_beta", "u_gamma"]);
}

#[tokio::test]
async fn test_local_profile_conversation_profile_is_member_visible_and_admin_mutable() {
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
                        "conversationId":"c_conversation_profile_routes",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_admin = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_conversation_profile_routes/members/add")
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
        .expect("add admin should return response");
    assert_eq!(add_admin.status(), StatusCode::OK);

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_conversation_profile_routes/members/add")
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
        .expect("add member should return response");
    assert_eq!(add_member.status(), StatusCode::OK);

    let owner_update = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/im/v3/api/chat/conversations/c_conversation_profile_routes/profile")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "displayName":"Backend Group Name",
                        "avatarUrl":"https://cdn.example.test/group.png",
                        "notice":"Backend group notice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("owner profile update should return response");
    assert_eq!(owner_update.status(), StatusCode::OK);
    let owner_update_body = owner_update
        .into_body()
        .collect()
        .await
        .expect("owner profile update body should collect")
        .to_bytes();
    let owner_update_value: serde_json::Value = serde_json::from_slice(&owner_update_body)
        .expect("owner profile update should be valid json");
    assert_eq!(
        owner_update_value["conversationId"],
        "c_conversation_profile_routes"
    );
    assert_eq!(owner_update_value["displayName"], "Backend Group Name");
    assert_eq!(
        owner_update_value["avatarUrl"],
        "https://cdn.example.test/group.png"
    );
    assert_eq!(owner_update_value["notice"], "Backend group notice");
    assert_eq!(owner_update_value["updatedByPrincipalKind"], "user");
    assert_eq!(owner_update_value["updatedByPrincipalId"], "u_owner");
    assert!(owner_update_value["updatedAt"].as_str().is_some());

    let member_read = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_conversation_profile_routes/profile")
                .member_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("member profile read should return response");
    assert_eq!(member_read.status(), StatusCode::OK);
    let member_read_body = member_read
        .into_body()
        .collect()
        .await
        .expect("member profile read body should collect")
        .to_bytes();
    let member_read_value: serde_json::Value = serde_json::from_slice(&member_read_body)
        .expect("member profile read should be valid json");
    assert_eq!(member_read_value["displayName"], "Backend Group Name");
    assert_eq!(member_read_value["notice"], "Backend group notice");

    let admin_update = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/im/v3/api/chat/conversations/c_conversation_profile_routes/profile")
                .admin_app_context()
                .header("content-type", "application/json")
                .body(Body::from(r#"{"notice":"Admin updated notice"}"#))
                .unwrap(),
        )
        .await
        .expect("admin profile update should return response");
    assert_eq!(admin_update.status(), StatusCode::OK);

    let owner_read = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_conversation_profile_routes/profile")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("owner profile read should return response");
    assert_eq!(owner_read.status(), StatusCode::OK);
    let owner_read_body = owner_read
        .into_body()
        .collect()
        .await
        .expect("owner profile read body should collect")
        .to_bytes();
    let owner_read_value: serde_json::Value =
        serde_json::from_slice(&owner_read_body).expect("owner profile read should be valid json");
    assert_eq!(owner_read_value["displayName"], "Backend Group Name");
    assert_eq!(owner_read_value["notice"], "Admin updated notice");
    assert_eq!(owner_read_value["updatedByPrincipalId"], "u_admin");

    let member_update = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/im/v3/api/chat/conversations/c_conversation_profile_routes/profile")
                .member_app_context()
                .header("content-type", "application/json")
                .body(Body::from(r#"{"notice":"member cannot edit"}"#))
                .unwrap(),
        )
        .await
        .expect("member profile update should return response");
    assert_eq!(member_update.status(), StatusCode::FORBIDDEN);
    let member_update_body = member_update
        .into_body()
        .collect()
        .await
        .expect("member profile update body should collect")
        .to_bytes();
    let member_update_value: serde_json::Value = serde_json::from_slice(&member_update_body)
        .expect("member profile update should be valid json");
    assert_eq!(
        member_update_value["code"],
        "conversation_profile_permission_denied"
    );

    let intruder_read = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_conversation_profile_routes/profile")
                .intruder_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("intruder profile read should return response");
    assert_eq!(intruder_read.status(), StatusCode::FORBIDDEN);

    let intruder_update = app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/im/v3/api/chat/conversations/c_conversation_profile_routes/profile")
                .intruder_app_context()
                .header("content-type", "application/json")
                .body(Body::from(r#"{"displayName":"intruder"}"#))
                .unwrap(),
        )
        .await
        .expect("intruder profile update should return response");
    assert_eq!(intruder_update.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_local_profile_message_visibility_delete_is_principal_scoped_and_sdk_backed() {
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
                        "conversationId":"c_message_visibility_delete_routes",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_message_visibility_delete_routes/members/add")
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
        .expect("add member should return response");
    assert_eq!(add_member.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_message_visibility_delete_routes/messages")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_message_visibility_delete",
                        "summary":"delete for me",
                        "text":"delete for me"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should return response");
    assert_eq!(post_message.status(), StatusCode::OK);
    let post_message_body = post_message
        .into_body()
        .collect()
        .await
        .expect("post message body should collect")
        .to_bytes();
    let post_message_value: serde_json::Value =
        serde_json::from_slice(&post_message_body).expect("post message should be valid json");
    let message_id = post_message_value["messageId"]
        .as_str()
        .expect("post message must return message id");

    let owner_delete = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/im/v3/api/chat/messages/{message_id}/visibility"))
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("owner message visibility delete should return response");
    assert_eq!(owner_delete.status(), StatusCode::OK);
    let owner_delete_body = owner_delete
        .into_body()
        .collect()
        .await
        .expect("owner message visibility delete body should collect")
        .to_bytes();
    let owner_delete_value: serde_json::Value = serde_json::from_slice(&owner_delete_body)
        .expect("owner message visibility delete should be valid json");
    assert_eq!(
        owner_delete_value["conversationId"],
        "c_message_visibility_delete_routes"
    );
    assert_eq!(owner_delete_value["messageId"], message_id);
    assert_eq!(owner_delete_value["principalKind"], "user");
    assert_eq!(owner_delete_value["principalId"], "u_owner");
    assert_eq!(owner_delete_value["isDeleted"], true);
    assert!(owner_delete_value["updatedAt"].as_str().is_some());

    let owner_timeline = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_message_visibility_delete_routes/messages")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("owner timeline should return response");
    assert_eq!(owner_timeline.status(), StatusCode::OK);
    let owner_timeline_body = owner_timeline
        .into_body()
        .collect()
        .await
        .expect("owner timeline body should collect")
        .to_bytes();
    let owner_timeline_value: serde_json::Value =
        serde_json::from_slice(&owner_timeline_body).expect("owner timeline should be valid json");
    let owner_items = owner_timeline_value["items"]
        .as_array()
        .expect("owner timeline items should be an array");
    assert!(
        owner_items.is_empty(),
        "message visibility delete must hide the message only from the current principal timeline"
    );

    let member_timeline = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_message_visibility_delete_routes/messages")
                .member_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("member timeline should return response");
    assert_eq!(member_timeline.status(), StatusCode::OK);
    let member_timeline_body = member_timeline
        .into_body()
        .collect()
        .await
        .expect("member timeline body should collect")
        .to_bytes();
    let member_timeline_value: serde_json::Value = serde_json::from_slice(&member_timeline_body)
        .expect("member timeline should be valid json");
    assert_eq!(member_timeline_value["items"][0]["messageId"], message_id);
    assert_eq!(
        member_timeline_value["items"][0]["summary"],
        "delete for me"
    );

    let intruder_delete = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/im/v3/api/chat/messages/{message_id}/visibility"))
                .intruder_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("intruder message visibility delete should return response");
    assert_eq!(intruder_delete.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_local_profile_message_favorites_are_principal_scoped_and_sdk_backed() {
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
                        "conversationId":"c_message_favorites_routes",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_message_favorites_routes/members/add")
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
        .expect("add member should return response");
    assert_eq!(add_member.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_message_favorites_routes/messages")
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_message_favorites_routes_1",
                        "summary":"favorite me",
                        "text":"favorite me"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should return response");
    assert_eq!(post_message.status(), StatusCode::OK);
    let post_message_body = post_message
        .into_body()
        .collect()
        .await
        .expect("post message body should collect")
        .to_bytes();
    let post_message_value: serde_json::Value =
        serde_json::from_slice(&post_message_body).expect("post message should be valid json");
    let message_id = post_message_value["messageId"]
        .as_str()
        .expect("post message must return message id");

    let owner_favorite = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/im/v3/api/chat/messages/{message_id}/favorites"))
                .owner_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_message_favorites_routes",
                        "favoriteType":"chat",
                        "title":"Important chat",
                        "contentPreview":"favorite me",
                        "sourceDisplayName":"Owner"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("owner favorite should return response");
    assert_eq!(owner_favorite.status(), StatusCode::OK);
    let owner_favorite_body = owner_favorite
        .into_body()
        .collect()
        .await
        .expect("owner favorite body should collect")
        .to_bytes();
    let owner_favorite_value: serde_json::Value =
        serde_json::from_slice(&owner_favorite_body).expect("owner favorite should be valid json");
    let favorite_id = owner_favorite_value["favoriteId"]
        .as_str()
        .expect("favorite create must return favorite id")
        .to_owned();
    assert_eq!(
        owner_favorite_value["conversationId"],
        "c_message_favorites_routes"
    );
    assert_eq!(owner_favorite_value["messageId"], message_id);
    assert_eq!(owner_favorite_value["principalKind"], "user");
    assert_eq!(owner_favorite_value["principalId"], "u_owner");
    assert_eq!(owner_favorite_value["favoriteType"], "chat");
    assert_eq!(owner_favorite_value["title"], "Important chat");
    assert_eq!(owner_favorite_value["contentPreview"], "favorite me");
    assert_eq!(owner_favorite_value["sourceDisplayName"], "Owner");
    assert!(owner_favorite_value["favoritedAt"].as_str().is_some());

    let owner_favorites = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/messages/favorites?favoriteType=chat&limit=10&q=important")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("owner favorite list should return response");
    assert_eq!(owner_favorites.status(), StatusCode::OK);
    let owner_favorites_body = owner_favorites
        .into_body()
        .collect()
        .await
        .expect("owner favorite list body should collect")
        .to_bytes();
    let owner_favorites_value: serde_json::Value = serde_json::from_slice(&owner_favorites_body)
        .expect("owner favorite list should be valid json");
    let owner_items = owner_favorites_value["items"]
        .as_array()
        .expect("owner favorite list items should be an array");
    assert_eq!(owner_items.len(), 1);
    assert_eq!(owner_items[0]["favoriteId"], favorite_id);
    assert_eq!(owner_favorites_value["hasMore"], false);

    let member_favorites = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/messages/favorites?favoriteType=chat&limit=10")
                .member_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("member favorite list should return response");
    assert_eq!(member_favorites.status(), StatusCode::OK);
    let member_favorites_body = member_favorites
        .into_body()
        .collect()
        .await
        .expect("member favorite list body should collect")
        .to_bytes();
    let member_favorites_value: serde_json::Value = serde_json::from_slice(&member_favorites_body)
        .expect("member favorite list should be valid json");
    assert!(
        member_favorites_value["items"]
            .as_array()
            .expect("member favorite list items should be an array")
            .is_empty(),
        "message favorites must be scoped to the current principal"
    );

    let intruder_favorite = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/im/v3/api/chat/messages/{message_id}/favorites"))
                .intruder_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_message_favorites_routes",
                        "favoriteType":"chat",
                        "title":"Intruder",
                        "contentPreview":"no access",
                        "sourceDisplayName":"Intruder"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("intruder favorite should return response");
    assert_eq!(intruder_favorite.status(), StatusCode::FORBIDDEN);

    let delete_favorite = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/im/v3/api/chat/messages/favorites/{favorite_id}"))
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("favorite delete should return response");
    assert_eq!(delete_favorite.status(), StatusCode::OK);
    let delete_favorite_body = delete_favorite
        .into_body()
        .collect()
        .await
        .expect("favorite delete body should collect")
        .to_bytes();
    let delete_favorite_value: serde_json::Value = serde_json::from_slice(&delete_favorite_body)
        .expect("favorite delete should be valid json");
    assert_eq!(delete_favorite_value["favoriteId"], favorite_id);
    assert_eq!(delete_favorite_value["deleted"], true);

    let cleared_favorites = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/messages/favorites?favoriteType=chat&limit=10")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("cleared favorite list should return response");
    assert_eq!(cleared_favorites.status(), StatusCode::OK);
    let cleared_favorites_body = cleared_favorites
        .into_body()
        .collect()
        .await
        .expect("cleared favorite list body should collect")
        .to_bytes();
    let cleared_favorites_value: serde_json::Value =
        serde_json::from_slice(&cleared_favorites_body)
            .expect("cleared favorite list should be valid json");
    assert!(
        cleared_favorites_value["items"]
            .as_array()
            .expect("cleared favorite list items should be an array")
            .is_empty(),
        "favorite delete must remove only the current principal favorite"
    );
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
                .uri("/im/v3/api/calls/sessions")
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
                .uri("/im/v3/api/calls/sessions")
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
                .uri("/im/v3/api/calls/sessions/rtc_actor_kind_guard_http/signals")
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
                        "agentId":"agent.demo"
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
            .any(|item| item["principalId"] == "agent.demo" && item["principalKind"] == "agent")
    );
}

#[tokio::test]
async fn test_local_profile_list_members_returns_bounded_cursor_window() {
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
                        "conversationId":"c_local_members_window",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    for principal_id in ["u_alpha", "u_beta", "u_gamma"] {
        let add_member = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/chat/conversations/c_local_members_window/members/add")
                    .owner_app_context()
                    .header("content-type", "application/json")
                    .body(Body::from(format!(
                        r#"{{
                            "principalId":"{principal_id}",
                            "principalKind":"user",
                            "role":"member"
                        }}"#,
                    )))
                    .unwrap(),
            )
            .await
            .expect("add member should return response");
        assert_eq!(add_member.status(), StatusCode::OK);
    }

    let first_page = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_local_members_window/members?limit=2")
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("first member page should return response");
    assert_eq!(first_page.status(), StatusCode::OK);
    let first_body = first_page
        .into_body()
        .collect()
        .await
        .expect("first member page body should collect")
        .to_bytes();
    let first_json: serde_json::Value =
        serde_json::from_slice(&first_body).expect("first member page should be valid json");
    assert_eq!(first_json["items"].as_array().unwrap().len(), 2);
    assert_eq!(first_json["hasMore"], true);
    let next_cursor = first_json["nextCursor"]
        .as_str()
        .expect("first member page should include nextCursor")
        .to_owned();

    let second_page = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/c_local_members_window/members?limit=2&cursor={next_cursor}"
                ))
                .owner_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second member page should return response");
    assert_eq!(second_page.status(), StatusCode::OK);
    let second_body = second_page
        .into_body()
        .collect()
        .await
        .expect("second member page body should collect")
        .to_bytes();
    let second_json: serde_json::Value =
        serde_json::from_slice(&second_body).expect("second member page should be valid json");
    assert_eq!(second_json["items"].as_array().unwrap().len(), 2);
    assert_eq!(second_json["hasMore"], false);

    let mut principal_ids = first_json["items"]
        .as_array()
        .unwrap()
        .iter()
        .chain(second_json["items"].as_array().unwrap().iter())
        .map(|item| item["principalId"].as_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    principal_ids.sort();
    assert_eq!(principal_ids, ["u_alpha", "u_beta", "u_gamma", "u_owner"]);
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
                        "agentId":"agent.demo"
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
                .uri("/im/v3/api/calls/sessions")
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
                .uri("/im/v3/api/calls/sessions/rtc_handoff_closed_local/signals")
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
                .uri("/im/v3/api/calls/sessions")
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
                .uri("/im/v3/api/calls/sessions")
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
                .uri("/im/v3/api/calls/sessions/rtc_system_channel_local/signals")
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
                .uri("/im/v3/api/calls/sessions")
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
