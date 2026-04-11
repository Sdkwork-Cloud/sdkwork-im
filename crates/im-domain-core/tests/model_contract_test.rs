use std::collections::BTreeMap;

use im_domain_core::conversation::{
    ConversationActorView, ConversationAgentHandoffView, ConversationInboxEntry,
    ConversationMember, ConversationReadCursor, DeviceSyncFeedEntry, MembershipRole,
    MembershipState,
};
use im_domain_core::media::{MediaResource, MediaResourceType};
use im_domain_core::message::{
    ContentPart, MediaPart, Message, MessageBody, MessageEdited, MessageRecalled, MessageType,
    Sender,
};
use im_domain_core::realtime::{
    RealtimeAckState, RealtimeEvent, RealtimeEventWindow, RealtimeSubscription,
    RealtimeSubscriptionSnapshot,
};
use im_domain_core::rtc::{RtcSession, RtcSessionState, RtcSignalEvent};
use im_domain_core::session::{
    DevicePresenceStatus, DevicePresenceView, PresenceSnapshotView, SessionResumeView,
};
use im_domain_core::stream::{
    StreamDurabilityClass, StreamFrame, StreamSession, StreamSessionState,
};
use serde_json::{Value, json};

#[test]
fn test_message_body_serializes_content_parts_with_expected_shape() {
    let message = Message {
        tenant_id: "t_demo".into(),
        conversation_id: "c_demo".into(),
        message_id: "m_demo".into(),
        message_seq: 1,
        sender: Sender {
            id: "u_demo".into(),
            kind: "user".into(),
            member_id: Some("cm_demo".into()),
            device_id: Some("d_demo".into()),
            session_id: Some("s_demo".into()),
            metadata: BTreeMap::from([("role".into(), "owner".into())]),
        },
        message_type: MessageType::Standard,
        delivery_mode: "discrete".into(),
        client_msg_id: Some("client_demo".into()),
        stream_session_id: None,
        rtc_session_id: None,
        body: MessageBody {
            summary: Some("hello".into()),
            parts: vec![
                ContentPart::text("hello"),
                ContentPart::media(MediaPart {
                    media_asset_id: "asset_demo".into(),
                    resource: Some(MediaResource {
                        id: Some(1),
                        uuid: Some("media_demo".into()),
                        url: Some("https://example.com/demo.png".into()),
                        bytes: None,
                        local_file: None,
                        base64: None,
                        resource_type: Some(MediaResourceType::Image),
                        mime_type: Some("image/png".into()),
                        size: Some(42),
                        name: Some("demo.png".into()),
                        extension: Some("png".into()),
                        tags: None,
                        metadata: Some(BTreeMap::from([("origin".into(), "test".into())])),
                        prompt: Some("poster".into()),
                    }),
                }),
            ],
            render_hints: BTreeMap::new(),
        },
        attributes: BTreeMap::new(),
        metadata: BTreeMap::new(),
        occurred_at: "2026-04-05T10:00:00Z".into(),
        committed_at: Some("2026-04-05T10:00:01Z".into()),
    };

    let value = serde_json::to_value(message).expect("message should serialize");

    assert_eq!(value["messageType"], Value::String("standard".into()));
    assert_eq!(value["sender"]["id"], Value::String("u_demo".into()));
    assert_eq!(value["sender"]["kind"], Value::String("user".into()));
    assert_eq!(value["sender"]["memberId"], Value::String("cm_demo".into()));
    assert_eq!(
        value["sender"]["metadata"]["role"],
        Value::String("owner".into())
    );
    assert_eq!(
        value["body"]["parts"][0]["kind"],
        Value::String("text".into())
    );
    assert_eq!(
        value["body"]["parts"][1]["kind"],
        Value::String("media".into())
    );
    assert_eq!(
        value["body"]["parts"][1]["resource"]["type"],
        Value::String("image".into())
    );
    assert_eq!(
        value["body"]["parts"][1]["resource"]["mimeType"],
        Value::String("image/png".into())
    );
    assert_eq!(
        value["body"]["parts"][1]["resource"]["metadata"]["origin"],
        json!("test")
    );
}

#[test]
fn test_stream_session_serializes_lifecycle_fields() {
    let session = StreamSession {
        tenant_id: "t_demo".into(),
        stream_id: "st_demo".into(),
        stream_type: "custom.delta.text".into(),
        scope_kind: "conversation".into(),
        scope_id: "c_demo".into(),
        durability_class: StreamDurabilityClass::DurableSession,
        ordering_scope: "stream".into(),
        schema_ref: Some("custom.delta.text.v1".into()),
        state: StreamSessionState::Opened,
        last_frame_seq: 3,
        last_checkpoint_seq: Some(2),
        result_message_id: None,
        opened_at: "2026-04-05T10:00:00Z".into(),
        closed_at: None,
        expires_at: None,
    };

    let value = serde_json::to_value(session).expect("stream session should serialize");

    assert_eq!(
        value["durabilityClass"],
        Value::String("durableSession".into())
    );
    assert_eq!(value["state"], Value::String("opened".into()));
}

#[test]
fn test_stream_frame_serializes_transport_shape() {
    let frame = StreamFrame {
        tenant_id: "t_demo".into(),
        stream_id: "st_demo".into(),
        stream_type: "custom.delta.text".into(),
        scope_kind: "conversation".into(),
        scope_id: "c_demo".into(),
        frame_seq: 1,
        frame_type: "delta".into(),
        schema_ref: Some("custom.delta.text.v1".into()),
        encoding: "json".into(),
        payload: r#"{"delta":"hello"}"#.into(),
        sender: Sender {
            id: "u_demo".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_demo".into()),
            session_id: Some("s_demo".into()),
            metadata: BTreeMap::new(),
        },
        attributes: BTreeMap::from([("topic".into(), "llm".into())]),
        occurred_at: "2026-04-05T10:00:05Z".into(),
    };

    let value = serde_json::to_value(frame).expect("stream frame should serialize");

    assert_eq!(value["streamId"], Value::String("st_demo".into()));
    assert_eq!(value["frameSeq"], Value::Number(1.into()));
    assert_eq!(value["frameType"], Value::String("delta".into()));
    assert_eq!(value["encoding"], Value::String("json".into()));
    assert_eq!(value["sender"]["id"], Value::String("u_demo".into()));
    assert_eq!(value["attributes"]["topic"], Value::String("llm".into()));
}

#[test]
fn test_rtc_session_serializes_signal_binding_fields() {
    let session = RtcSession {
        tenant_id: "t_demo".into(),
        rtc_session_id: "rtc_demo".into(),
        conversation_id: Some("c_demo".into()),
        rtc_mode: "voice".into(),
        initiator_id: "u_demo".into(),
        provider_plugin_id: Some("rtc-volcengine".into()),
        provider_session_id: Some("volcengine:rtc_demo".into()),
        access_endpoint: Some("wss://rtc.volcengine.local/session".into()),
        provider_region: Some("cn-beijing".into()),
        state: RtcSessionState::Started,
        signaling_stream_id: Some("st_demo".into()),
        artifact_message_id: None,
        started_at: "2026-04-05T10:00:00Z".into(),
        ended_at: None,
    };

    let value = serde_json::to_value(session).expect("rtc session should serialize");

    assert_eq!(value["rtcMode"], Value::String("voice".into()));
    assert_eq!(value["state"], Value::String("started".into()));
}

#[test]
fn test_rtc_signal_event_serializes_signal_transport_shape() {
    let signal = RtcSignalEvent {
        tenant_id: "t_demo".into(),
        rtc_session_id: "rtc_demo".into(),
        conversation_id: Some("c_demo".into()),
        rtc_mode: "voice".into(),
        signal_type: "rtc.offer".into(),
        schema_ref: Some("webrtc.offer.v1".into()),
        payload: r#"{"sdp":"demo"}"#.into(),
        sender: Sender {
            id: "u_demo".into(),
            kind: "user".into(),
            member_id: Some("cm_demo".into()),
            device_id: Some("d_demo".into()),
            session_id: Some("s_demo".into()),
            metadata: BTreeMap::new(),
        },
        signaling_stream_id: Some("st_demo".into()),
        occurred_at: "2026-04-05T10:00:10Z".into(),
    };

    let value = serde_json::to_value(signal).expect("rtc signal event should serialize");

    assert_eq!(value["rtcSessionId"], Value::String("rtc_demo".into()));
    assert_eq!(value["signalType"], Value::String("rtc.offer".into()));
    assert_eq!(value["schemaRef"], Value::String("webrtc.offer.v1".into()));
    assert_eq!(value["sender"]["id"], Value::String("u_demo".into()));
    assert_eq!(value["signalingStreamId"], Value::String("st_demo".into()));
}

#[test]
fn test_realtime_subscription_snapshot_serializes_shape() {
    let snapshot = RealtimeSubscriptionSnapshot {
        tenant_id: "t_demo".into(),
        principal_id: "u_demo".into(),
        device_id: "d_pad".into(),
        items: vec![RealtimeSubscription {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into()],
            subscribed_at: "2026-04-05T10:10:00Z".into(),
        }],
        synced_at: "2026-04-05T10:10:00Z".into(),
    };

    let value = serde_json::to_value(snapshot).expect("realtime snapshot should serialize");

    assert_eq!(value["deviceId"], Value::String("d_pad".into()));
    assert_eq!(
        value["items"][0]["scopeType"],
        Value::String("conversation".into())
    );
    assert_eq!(
        value["items"][0]["eventTypes"][0],
        Value::String("message.posted".into())
    );
}

#[test]
fn test_realtime_event_window_serializes_shape() {
    let window = RealtimeEventWindow {
        device_id: "d_pad".into(),
        items: vec![RealtimeEvent {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            device_id: "d_pad".into(),
            realtime_seq: 1,
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_type: "message.posted".into(),
            delivery_class: "ephemeral".into(),
            payload: r#"{"messageId":"msg_c_demo_1"}"#.into(),
            occurred_at: "2026-04-05T10:10:01Z".into(),
        }],
        next_after_seq: Some(1),
        has_more: false,
        acked_through_seq: 0,
        trimmed_through_seq: 0,
    };

    let value = serde_json::to_value(window).expect("realtime event window should serialize");

    assert_eq!(value["deviceId"], Value::String("d_pad".into()));
    assert_eq!(value["items"][0]["realtimeSeq"], Value::Number(1.into()));
    assert_eq!(
        value["items"][0]["eventType"],
        Value::String("message.posted".into())
    );
    assert_eq!(value["nextAfterSeq"], Value::Number(1.into()));
    assert_eq!(value["ackedThroughSeq"], Value::Number(0.into()));
    assert_eq!(value["trimmedThroughSeq"], Value::Number(0.into()));
}

#[test]
fn test_realtime_ack_state_serializes_checkpoint_shape() {
    let ack = RealtimeAckState {
        tenant_id: "t_demo".into(),
        principal_id: "u_demo".into(),
        device_id: "d_pad".into(),
        acked_through_seq: 12,
        trimmed_through_seq: 12,
        retained_event_count: 3,
        acked_at: "2026-04-05T10:10:02Z".into(),
    };

    let value = serde_json::to_value(ack).expect("realtime ack state should serialize");

    assert_eq!(value["deviceId"], Value::String("d_pad".into()));
    assert_eq!(value["ackedThroughSeq"], Value::Number(12.into()));
    assert_eq!(value["trimmedThroughSeq"], Value::Number(12.into()));
    assert_eq!(value["retainedEventCount"], Value::Number(3.into()));
}

#[test]
fn test_conversation_member_serializes_membership_shape() {
    let member = ConversationMember {
        tenant_id: "t_demo".into(),
        conversation_id: "c_demo".into(),
        member_id: "cm_demo".into(),
        principal_id: "u_demo".into(),
        principal_kind: "user".into(),
        role: MembershipRole::Owner,
        state: MembershipState::Joined,
        invited_by: Some("u_admin".into()),
        joined_at: "2026-04-05T10:00:00Z".into(),
        removed_at: None,
        attributes: BTreeMap::from([("source".into(), "bootstrap".into())]),
    };

    let value = serde_json::to_value(member).expect("conversation member should serialize");

    assert_eq!(value["memberId"], Value::String("cm_demo".into()));
    assert_eq!(value["principalId"], Value::String("u_demo".into()));
    assert_eq!(value["principalKind"], Value::String("user".into()));
    assert_eq!(value["role"], Value::String("owner".into()));
    assert_eq!(value["state"], Value::String("joined".into()));
    assert_eq!(value["invitedBy"], Value::String("u_admin".into()));
    assert_eq!(
        value["attributes"]["source"],
        Value::String("bootstrap".into())
    );
}

#[test]
fn test_conversation_read_cursor_serializes_cursor_shape() {
    let cursor = ConversationReadCursor {
        tenant_id: "t_demo".into(),
        conversation_id: "c_demo".into(),
        member_id: "cm_demo".into(),
        principal_id: "u_demo".into(),
        read_seq: 12,
        last_read_message_id: Some("msg_c_demo_12".into()),
        updated_at: "2026-04-05T10:00:10Z".into(),
    };

    let value = serde_json::to_value(cursor).expect("read cursor should serialize");

    assert_eq!(value["memberId"], Value::String("cm_demo".into()));
    assert_eq!(value["principalId"], Value::String("u_demo".into()));
    assert_eq!(value["readSeq"], Value::Number(12.into()));
    assert_eq!(
        value["lastReadMessageId"],
        Value::String("msg_c_demo_12".into())
    );
    assert_eq!(
        value["updatedAt"],
        Value::String("2026-04-05T10:00:10Z".into())
    );
}

#[test]
fn test_conversation_inbox_entry_serializes_inbox_shape() {
    let entry = ConversationInboxEntry {
        tenant_id: "t_demo".into(),
        principal_id: "u_demo".into(),
        member_id: "cm_demo".into(),
        conversation_id: "c_demo".into(),
        conversation_type: "group".into(),
        message_count: 12,
        last_message_id: Some("msg_c_demo_12".into()),
        last_message_seq: 12,
        last_sender_id: Some("u_other".into()),
        last_sender_kind: Some("user".into()),
        last_summary: Some("hello".into()),
        unread_count: 3,
        last_activity_at: "2026-04-05T10:00:10Z".into(),
        agent_handoff: Some(ConversationAgentHandoffView {
            status: "accepted".into(),
            source: ConversationActorView {
                id: "ag_source".into(),
                kind: "agent".into(),
            },
            target: ConversationActorView {
                id: "u_demo".into(),
                kind: "user".into(),
            },
            handoff_session_id: "hs_demo".into(),
            handoff_reason: Some("manual_escalation".into()),
            accepted_at: Some("2026-04-05T10:00:09Z".into()),
            accepted_by: Some(ConversationActorView {
                id: "u_demo".into(),
                kind: "user".into(),
            }),
            resolved_at: None,
            resolved_by: None,
            closed_at: None,
            closed_by: None,
        }),
    };

    let value = serde_json::to_value(entry).expect("inbox entry should serialize");

    assert_eq!(value["conversationId"], Value::String("c_demo".into()));
    assert_eq!(value["conversationType"], Value::String("group".into()));
    assert_eq!(value["messageCount"], Value::Number(12.into()));
    assert_eq!(
        value["lastMessageId"],
        Value::String("msg_c_demo_12".into())
    );
    assert_eq!(value["lastSenderId"], Value::String("u_other".into()));
    assert_eq!(value["unreadCount"], Value::Number(3.into()));
    assert_eq!(
        value["agentHandoff"]["status"],
        Value::String("accepted".into())
    );
    assert_eq!(
        value["agentHandoff"]["source"]["id"],
        Value::String("ag_source".into())
    );
    assert_eq!(
        value["lastActivityAt"],
        Value::String("2026-04-05T10:00:10Z".into())
    );
}

#[test]
fn test_device_sync_feed_entry_serializes_sync_shape() {
    let entry = DeviceSyncFeedEntry {
        tenant_id: "t_demo".into(),
        principal_id: "u_demo".into(),
        device_id: "d_demo".into(),
        sync_seq: 2,
        origin_event_id: "evt_demo".into(),
        origin_event_type: "message.posted".into(),
        conversation_id: Some("c_demo".into()),
        message_id: Some("msg_c_demo_2".into()),
        message_seq: Some(2),
        member_id: None,
        read_seq: None,
        last_read_message_id: None,
        actor_id: Some("u_other".into()),
        actor_kind: Some("user".into()),
        actor_device_id: Some("d_other".into()),
        summary: Some("hello".into()),
        payload_schema: Some("message.posted.v1".into()),
        payload: Some(r#"{"messageId":"msg_c_demo_2"}"#.into()),
        occurred_at: "2026-04-05T10:00:10Z".into(),
    };

    let value = serde_json::to_value(entry).expect("device sync feed entry should serialize");

    assert_eq!(value["principalId"], Value::String("u_demo".into()));
    assert_eq!(value["deviceId"], Value::String("d_demo".into()));
    assert_eq!(value["syncSeq"], Value::Number(2.into()));
    assert_eq!(
        value["originEventType"],
        Value::String("message.posted".into())
    );
    assert_eq!(value["messageId"], Value::String("msg_c_demo_2".into()));
    assert_eq!(value["actorKind"], Value::String("user".into()));
    assert_eq!(value["actorDeviceId"], Value::String("d_other".into()));
    assert_eq!(value["summary"], Value::String("hello".into()));
    assert_eq!(
        value["payloadSchema"],
        Value::String("message.posted.v1".into())
    );
    assert_eq!(
        value["payload"],
        Value::String(r#"{"messageId":"msg_c_demo_2"}"#.into())
    );
}

#[test]
fn test_session_resume_view_serializes_presence_snapshot_shape() {
    let view = SessionResumeView {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: "d_demo".into(),
        resume_required: true,
        resume_from_sync_seq: 3,
        latest_sync_seq: 5,
        resumed_at: "2026-04-05T10:00:20Z".into(),
        presence: PresenceSnapshotView {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            current_device_id: Some("d_demo".into()),
            devices: vec![
                DevicePresenceView {
                    tenant_id: "t_demo".into(),
                    principal_id: "u_demo".into(),
                    device_id: "d_demo".into(),
                    platform: None,
                    session_id: Some("s_demo".into()),
                    status: DevicePresenceStatus::Online,
                    last_sync_seq: 5,
                    last_resume_at: Some("2026-04-05T10:00:20Z".into()),
                    last_seen_at: Some("2026-04-05T10:00:20Z".into()),
                },
                DevicePresenceView {
                    tenant_id: "t_demo".into(),
                    principal_id: "u_demo".into(),
                    device_id: "d_pad".into(),
                    platform: None,
                    session_id: Some("s_pad".into()),
                    status: DevicePresenceStatus::Offline,
                    last_sync_seq: 2,
                    last_resume_at: Some("2026-04-05T09:50:00Z".into()),
                    last_seen_at: Some("2026-04-05T09:51:00Z".into()),
                },
            ],
        },
    };

    let value = serde_json::to_value(view).expect("session resume view should serialize");

    assert_eq!(value["deviceId"], Value::String("d_demo".into()));
    assert_eq!(value["resumeRequired"], Value::Bool(true));
    assert_eq!(value["resumeFromSyncSeq"], Value::Number(3.into()));
    assert_eq!(value["latestSyncSeq"], Value::Number(5.into()));
    assert_eq!(
        value["presence"]["currentDeviceId"],
        Value::String("d_demo".into())
    );
    assert_eq!(
        value["presence"]["devices"][0]["status"],
        Value::String("online".into())
    );
    assert_eq!(
        value["presence"]["devices"][1]["status"],
        Value::String("offline".into())
    );
}

#[test]
fn test_message_mutation_payloads_serialize_stable_shape() {
    let edited = MessageEdited {
        tenant_id: "t_demo".into(),
        conversation_id: "c_demo".into(),
        message_id: "msg_c_demo_1".into(),
        message_seq: 1,
        body: MessageBody {
            summary: Some("edited".into()),
            parts: vec![ContentPart::text("edited")],
            render_hints: BTreeMap::new(),
        },
        editor: Sender {
            id: "u_demo".into(),
            kind: "user".into(),
            member_id: Some("cm_demo".into()),
            device_id: Some("d_demo".into()),
            session_id: Some("s_demo".into()),
            metadata: BTreeMap::new(),
        },
        edited_at: "2026-04-05T10:00:30Z".into(),
    };
    let recalled = MessageRecalled {
        tenant_id: "t_demo".into(),
        conversation_id: "c_demo".into(),
        message_id: "msg_c_demo_1".into(),
        message_seq: 1,
        recalled_by: Sender {
            id: "u_demo".into(),
            kind: "user".into(),
            member_id: Some("cm_demo".into()),
            device_id: Some("d_demo".into()),
            session_id: Some("s_demo".into()),
            metadata: BTreeMap::new(),
        },
        recalled_at: "2026-04-05T10:00:40Z".into(),
    };

    let edited_value = serde_json::to_value(edited).expect("edited payload should serialize");
    let recalled_value = serde_json::to_value(recalled).expect("recalled payload should serialize");

    assert_eq!(
        edited_value["messageId"],
        Value::String("msg_c_demo_1".into())
    );
    assert_eq!(edited_value["messageSeq"], Value::Number(1.into()));
    assert_eq!(
        edited_value["editor"]["deviceId"],
        Value::String("d_demo".into())
    );
    assert_eq!(
        edited_value["body"]["summary"],
        Value::String("edited".into())
    );

    assert_eq!(
        recalled_value["recalledBy"]["sessionId"],
        Value::String("s_demo".into())
    );
    assert_eq!(
        recalled_value["recalledAt"],
        Value::String("2026-04-05T10:00:40Z".into())
    );
}
