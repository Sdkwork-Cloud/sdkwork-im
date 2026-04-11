use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use std::thread::sleep;
use std::time::Duration;
use tower::ServiceExt;

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
async fn test_timeline_query_returns_projected_messages() {
    let service = projection_service::TimelineProjectionService::default();
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_member",
                "t_demo",
                "conversation.member_joined",
                "conversation",
                "c_demo",
                0,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_demo",
                    "memberId":"cm_demo",
                    "principalId":"u_demo",
                    "principalKind":"user",
                    "role":"owner",
                    "state":"joined",
                    "invitedBy":null,
                    "joinedAt":"2026-04-05T10:00:00Z",
                    "removedAt":null,
                    "attributes":{}
                }"#,
            ),
        )
        .expect("member projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_demo",
                "t_demo",
                "message.posted",
                "conversation",
                "c_demo",
                1,
            )
            .with_payload(
                "message.posted.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_demo",
                    "messageId":"m_demo",
                    "messageSeq":1,
                    "sender":{"id":"u_demo","kind":"user","memberId":"cm_demo","deviceId":"d_demo","sessionId":"s_demo","metadata":{}},
                    "messageType":"standard",
                    "deliveryMode":"discrete",
                    "clientMsgId":"client_demo",
                    "streamSessionId":null,
                    "rtcSessionId":null,
                    "body":{"summary":"hello","parts":[{"kind":"text","text":"hello"}],"renderHints":{}},
                    "attributes":{},
                    "metadata":{},
                    "occurredAt":"2026-04-05T10:00:01Z",
                    "committedAt":"2026-04-05T10:00:01Z"
                }"#,
            ),
        )
        .expect("projection should succeed");

    let app = projection_service::build_app(std::sync::Arc::new(service));

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_demo/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");

    assert_eq!(value["items"][0]["messageId"], "m_demo");
    assert_eq!(value["items"][0]["summary"], "hello");

    let summary_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_demo")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("summary request should succeed");

    assert_eq!(summary_response.status(), StatusCode::OK);
    let summary_body = summary_response
        .into_body()
        .collect()
        .await
        .expect("summary body should collect")
        .to_bytes();
    let summary_value: serde_json::Value =
        serde_json::from_slice(&summary_body).expect("summary should be valid json");

    assert_eq!(summary_value["messageCount"], 1);
    assert_eq!(summary_value["lastMessageId"], "m_demo");
    assert_eq!(summary_value["lastSender"]["id"], "u_demo");

    let forbidden_timeline_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_demo/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_intruder")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("forbidden timeline request should succeed");
    assert_eq!(forbidden_timeline_response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_read_cursor_query_returns_projected_cursor_view() {
    let service = projection_service::TimelineProjectionService::default();
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_member",
                "t_demo",
                "conversation.member_joined",
                "conversation",
                "c_cursor",
                0,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_cursor",
                    "memberId":"cm_demo",
                    "principalId":"u_demo",
                    "principalKind":"user",
                    "role":"owner",
                    "state":"joined",
                    "invitedBy":null,
                    "joinedAt":"2026-04-05T10:00:00Z",
                    "removedAt":null,
                    "attributes":{}
                }"#,
            ),
        )
        .expect("member projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_message",
                "t_demo",
                "message.posted",
                "conversation",
                "c_cursor",
                2,
            )
            .with_payload(
                "message.posted.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_cursor",
                    "messageId":"m_demo_2",
                    "messageSeq":2,
                    "sender":{"id":"u_demo","kind":"user","memberId":"cm_demo","deviceId":null,"sessionId":"s_demo","metadata":{}},
                    "messageType":"standard",
                    "deliveryMode":"discrete",
                    "clientMsgId":"client_demo_2",
                    "streamSessionId":null,
                    "rtcSessionId":null,
                    "body":{"summary":"hello","parts":[{"kind":"text","text":"hello"}],"renderHints":{}},
                    "attributes":{},
                    "metadata":{},
                    "occurredAt":"2026-04-05T10:00:02Z",
                    "committedAt":"2026-04-05T10:00:02Z"
                }"#,
            ),
        )
        .expect("message projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_cursor",
                "t_demo",
                "conversation.read_cursor_updated",
                "conversation",
                "c_cursor",
                1,
            )
            .with_payload(
                "conversation.read_cursor.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_cursor",
                    "memberId":"cm_demo",
                    "principalId":"u_demo",
                    "readSeq":1,
                    "lastReadMessageId":"m_demo_1",
                    "updatedAt":"2026-04-05T10:00:10Z"
                }"#,
            ),
        )
        .expect("read cursor projection should succeed");

    let app = projection_service::build_app(std::sync::Arc::new(service));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_cursor/read-cursor")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("read cursor request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");

    assert_eq!(value["readSeq"], 1);
    assert_eq!(value["unreadCount"], 1);
    assert_eq!(value["memberId"], "cm_demo");
}

#[tokio::test]
async fn test_inbox_query_returns_projected_entries() {
    let service = projection_service::TimelineProjectionService::default();
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_conversation",
                "t_demo",
                "conversation.created",
                "conversation",
                "c_inbox",
                0,
            )
            .with_payload(
                "conversation.created.v1",
                r#"{
                    "conversationId":"c_inbox",
                    "conversationType":"group"
                }"#,
            ),
        )
        .expect("conversation projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_member",
                "t_demo",
                "conversation.member_joined",
                "conversation",
                "c_inbox",
                1,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_inbox",
                    "memberId":"cm_demo",
                    "principalId":"u_demo",
                    "principalKind":"user",
                    "role":"owner",
                    "state":"joined",
                    "invitedBy":null,
                    "joinedAt":"2026-04-05T10:00:00Z",
                    "removedAt":null,
                    "attributes":{}
                }"#,
            ),
        )
        .expect("member projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_message",
                "t_demo",
                "message.posted",
                "conversation",
                "c_inbox",
                2,
            )
            .with_payload(
                "message.posted.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_inbox",
                    "messageId":"m_demo_2",
                    "messageSeq":2,
                    "sender":{"id":"u_other","kind":"user","memberId":"cm_other","deviceId":null,"sessionId":"s_other","metadata":{}},
                    "messageType":"standard",
                    "deliveryMode":"discrete",
                    "clientMsgId":"client_demo_2",
                    "streamSessionId":null,
                    "rtcSessionId":null,
                    "body":{"summary":"hello","parts":[{"kind":"text","text":"hello"}],"renderHints":{}},
                    "attributes":{},
                    "metadata":{},
                    "occurredAt":"2026-04-05T10:00:02Z",
                    "committedAt":"2026-04-05T10:00:02Z"
                }"#,
            ),
        )
        .expect("message projection should succeed");

    let app = projection_service::build_app(std::sync::Arc::new(service));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/inbox")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("inbox request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");

    assert_eq!(value["items"][0]["conversationId"], "c_inbox");
    assert_eq!(value["items"][0]["conversationType"], "group");
    assert_eq!(value["items"][0]["messageCount"], 2);
}

#[tokio::test]
async fn test_device_sync_feed_query_returns_registered_device_entries() {
    let service = std::sync::Arc::new(projection_service::TimelineProjectionService::default());
    let app = projection_service::build_app(service.clone());

    let register_phone = app
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
        .expect("phone registration request should succeed");
    assert_eq!(register_phone.status(), StatusCode::OK);

    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_member",
                "t_demo",
                "conversation.member_joined",
                "conversation",
                "c_sync_http",
                1,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_sync_http",
                    "memberId":"cm_demo",
                    "principalId":"u_demo",
                    "principalKind":"user",
                    "role":"owner",
                    "state":"joined",
                    "invitedBy":null,
                    "joinedAt":"2026-04-05T10:00:00Z",
                    "removedAt":null,
                    "attributes":{}
                }"#,
            ),
        )
        .expect("member projection should succeed");

    let register_pad = app
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
        .expect("pad registration request should succeed");
    assert_eq!(register_pad.status(), StatusCode::OK);

    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_message",
                "t_demo",
                "message.posted",
                "conversation",
                "c_sync_http",
                2,
            )
            .with_payload(
                "message.posted.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_sync_http",
                    "messageId":"msg_c_sync_http_1",
                    "messageSeq":1,
                    "sender":{"id":"u_demo","kind":"user","memberId":"cm_demo","deviceId":"d_phone","sessionId":"s_demo","metadata":{}},
                    "messageType":"standard",
                    "deliveryMode":"discrete",
                    "clientMsgId":"client_sync_http_1",
                    "streamSessionId":null,
                    "rtcSessionId":null,
                    "body":{"summary":"sync http","parts":[{"kind":"text","text":"sync http"}],"renderHints":{}},
                    "attributes":{},
                    "metadata":{},
                    "occurredAt":"2026-04-05T10:00:02Z",
                    "committedAt":"2026-04-05T10:00:02Z"
                }"#,
            ),
        )
        .expect("message projection should succeed");

    let response = app
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
        .expect("sync feed request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");

    assert_eq!(value["items"][0]["originEventType"], "message.posted");
    assert_eq!(value["items"][0]["actorDeviceId"], "d_phone");
    assert_eq!(value["items"][0]["messageId"], "msg_c_sync_http_1");
}

#[tokio::test]
async fn test_device_registration_returns_advancing_registered_at() {
    let service = std::sync::Arc::new(projection_service::TimelineProjectionService::default());
    let app = projection_service::build_app(service);

    let register_phone = app
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
        .expect("phone registration should succeed");
    assert_eq!(register_phone.status(), StatusCode::OK);
    let register_phone_body = register_phone
        .into_body()
        .collect()
        .await
        .expect("phone registration body should collect")
        .to_bytes();
    let register_phone_json: serde_json::Value = serde_json::from_slice(&register_phone_body)
        .expect("phone registration should be valid json");
    let first_registered_at = register_phone_json["registeredAt"]
        .as_str()
        .expect("registeredAt should be present")
        .to_owned();

    sleep(Duration::from_millis(20));

    let register_pad = app
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
        .expect("pad registration should succeed");
    assert_eq!(register_pad.status(), StatusCode::OK);
    let register_pad_body = register_pad
        .into_body()
        .collect()
        .await
        .expect("pad registration body should collect")
        .to_bytes();
    let register_pad_json: serde_json::Value =
        serde_json::from_slice(&register_pad_body).expect("pad registration should be valid json");
    let second_registered_at = register_pad_json["registeredAt"]
        .as_str()
        .expect("registeredAt should be present")
        .to_owned();

    assert!(first_registered_at < second_registered_at);
}

#[tokio::test]
async fn test_member_directory_query_returns_projected_members() {
    let service = projection_service::TimelineProjectionService::default();
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_directory_owner",
                "t_demo",
                "conversation.member_joined",
                "conversation",
                "c_directory",
                1,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_directory",
                    "memberId":"cm_directory_owner",
                    "principalId":"u_owner",
                    "principalKind":"user",
                    "role":"owner",
                    "state":"joined",
                    "invitedBy":null,
                    "joinedAt":"2026-04-05T10:00:00Z",
                    "removedAt":null,
                    "attributes":{"displayName":"Owner"}
                }"#,
            ),
        )
        .expect("owner projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_directory_member",
                "t_demo",
                "conversation.member_joined",
                "conversation",
                "c_directory",
                2,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_directory",
                    "memberId":"cm_directory_member",
                    "principalId":"u_member",
                    "principalKind":"user",
                    "role":"member",
                    "state":"joined",
                    "invitedBy":"u_owner",
                    "joinedAt":"2026-04-05T10:01:00Z",
                    "removedAt":null,
                    "attributes":{"displayName":"Member"}
                }"#,
            ),
        )
        .expect("member projection should succeed");

    let app = projection_service::build_app(std::sync::Arc::new(service));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_directory/member-directory")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("member directory request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");

    assert_eq!(value["items"].as_array().unwrap().len(), 2);
    assert_eq!(value["items"][0]["principalId"], "u_owner");
    assert_eq!(value["items"][0]["role"], "owner");
    assert_eq!(value["items"][1]["principalId"], "u_member");
    assert_eq!(value["items"][1]["attributes"]["displayName"], "Member");
}

#[tokio::test]
async fn test_contacts_query_returns_friendship_projection_with_direct_chat_enrich() {
    let service = projection_service::TimelineProjectionService::default();
    service
        .apply(&friendship_activated_event(
            "t_demo",
            "fs_001",
            "u_alice",
            "u_bob",
            Some("dc_001"),
            "2026-04-10T12:00:00Z",
        ))
        .expect("friendship projection should succeed");
    service
        .apply(&friendship_activated_event(
            "t_demo",
            "fs_002",
            "u_alice",
            "u_cathy",
            None,
            "2026-04-10T11:00:00Z",
        ))
        .expect("second friendship projection should succeed");
    service
        .apply(&direct_chat_bound_event(
            "t_demo",
            "dc_001",
            "c_direct_001",
            "2026-04-10T12:05:00Z",
        ))
        .expect("direct chat enrich should succeed");

    let app = projection_service::build_app(std::sync::Arc::new(service));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/contacts")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_alice")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("contacts request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("contacts body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("contacts body should be valid json");

    let items = value["items"]
        .as_array()
        .expect("contacts items should be array");
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["ownerUserId"], "u_alice");
    assert_eq!(items[0]["targetUserId"], "u_bob");
    assert_eq!(items[0]["contactType"], "friendship");
    assert_eq!(items[0]["relationshipState"], "active");
    assert_eq!(items[0]["friendshipId"], "fs_001");
    assert_eq!(items[0]["directChatId"], "dc_001");
    assert_eq!(items[0]["conversationId"], "c_direct_001");
    assert_eq!(items[0]["lastInteractionAt"], "2026-04-10T12:05:00Z");
    assert_eq!(items[1]["targetUserId"], "u_cathy");
    assert_eq!(items[1]["conversationId"], serde_json::Value::Null);
}

#[tokio::test]
async fn test_interaction_summary_and_pins_query_return_projected_reaction_and_pin_views() {
    let service = projection_service::TimelineProjectionService::default();
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_interaction_owner_joined",
                "t_demo",
                "conversation.member_joined",
                "conversation",
                "c_interaction_http",
                1,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_interaction_http",
                    "memberId":"cm_u_owner",
                    "principalId":"u_owner",
                    "principalKind":"user",
                    "role":"owner",
                    "state":"joined",
                    "invitedBy":null,
                    "joinedAt":"2026-04-10T12:00:00Z",
                    "removedAt":null,
                    "attributes":{}
                }"#,
            ),
        )
        .expect("owner projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_interaction_member_joined",
                "t_demo",
                "conversation.member_joined",
                "conversation",
                "c_interaction_http",
                2,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_interaction_http",
                    "memberId":"cm_u_member",
                    "principalId":"u_member",
                    "principalKind":"user",
                    "role":"member",
                    "state":"joined",
                    "invitedBy":"u_owner",
                    "joinedAt":"2026-04-10T12:00:01Z",
                    "removedAt":null,
                    "attributes":{}
                }"#,
            ),
        )
        .expect("member projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_interaction_posted",
                "t_demo",
                "message.posted",
                "conversation",
                "c_interaction_http",
                3,
            )
            .with_payload(
                "message.posted.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_interaction_http",
                    "messageId":"msg_c_interaction_http_1",
                    "messageSeq":1,
                    "sender":{"id":"u_owner","kind":"user","memberId":"cm_u_owner","deviceId":"d_owner","sessionId":"s_owner","metadata":{}},
                    "messageType":"standard",
                    "deliveryMode":"discrete",
                    "clientMsgId":"client_interaction_http_1",
                    "streamSessionId":null,
                    "rtcSessionId":null,
                    "body":{"summary":"interaction http","parts":[{"kind":"text","text":"interaction http"}],"renderHints":{}},
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
            "c_interaction_http",
            "msg_c_interaction_http_1",
            1,
            "thumbs_up",
            "u_owner",
            "2026-04-10T12:00:10Z",
        ))
        .expect("reaction projection should succeed");
    service
        .apply(&message_reaction_added_event(
            "t_demo",
            "c_interaction_http",
            "msg_c_interaction_http_1",
            1,
            "thumbs_up",
            "u_member",
            "2026-04-10T12:00:11Z",
        ))
        .expect("second reaction projection should succeed");
    service
        .apply(&message_pinned_event(
            "t_demo",
            "c_interaction_http",
            "msg_c_interaction_http_1",
            1,
            "u_owner",
            "2026-04-10T12:00:20Z",
        ))
        .expect("pin projection should succeed");

    let app = projection_service::build_app(std::sync::Arc::new(service));
    let summary_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_interaction_http/messages/msg_c_interaction_http_1/interaction-summary")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("interaction summary request should succeed");

    assert_eq!(summary_response.status(), StatusCode::OK);
    let summary_body = summary_response
        .into_body()
        .collect()
        .await
        .expect("interaction summary body should collect")
        .to_bytes();
    let summary_value: serde_json::Value = serde_json::from_slice(&summary_body)
        .expect("interaction summary body should be valid json");

    assert_eq!(summary_value["messageId"], "msg_c_interaction_http_1");
    assert_eq!(summary_value["messageSeq"], 1);
    assert_eq!(summary_value["totalReactionCount"], 2);
    assert_eq!(
        summary_value["reactionCounts"][0]["reactionKey"],
        "thumbs_up"
    );
    assert_eq!(summary_value["reactionCounts"][0]["count"], 2);
    assert_eq!(summary_value["pin"]["pinnedBy"]["id"], "u_owner");
    assert_eq!(summary_value["pin"]["pinnedAt"], "2026-04-10T12:00:20Z");

    let pins_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/conversations/c_interaction_http/pins")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_member")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("pins request should succeed");

    assert_eq!(pins_response.status(), StatusCode::OK);
    let pins_body = pins_response
        .into_body()
        .collect()
        .await
        .expect("pins body should collect")
        .to_bytes();
    let pins_value: serde_json::Value =
        serde_json::from_slice(&pins_body).expect("pins response should be valid json");

    let items = pins_value["items"]
        .as_array()
        .expect("pins items should be array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["messageId"], "msg_c_interaction_http_1");
    assert_eq!(items[0]["pin"]["pinnedBy"]["id"], "u_owner");
}
