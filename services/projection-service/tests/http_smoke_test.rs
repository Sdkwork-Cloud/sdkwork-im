use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use std::thread::sleep;
use std::time::Duration;
use tower::ServiceExt;

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
