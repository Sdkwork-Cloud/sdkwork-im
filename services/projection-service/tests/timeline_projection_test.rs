use im_auth_context::AuthContext;
use im_domain_core::conversation::MembershipRole;
use projection_service::{
    MessageReactionCountView, NotificationRecipientView, RealtimeFanoutTarget,
    TimelineProjectionService, TimelineViewEntry,
};
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_message_posted_event_projects_into_timeline_view() {
    let service = TimelineProjectionService::default();

    let event = im_domain_events::CommitEnvelope::minimal(
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
    );

    service.apply(&event).expect("projection should succeed");

    assert_eq!(
        service.timeline("t_demo", "c_demo"),
        vec![TimelineViewEntry {
            tenant_id: "t_demo".into(),
            conversation_id: "c_demo".into(),
            message_id: "m_demo".into(),
            message_seq: 1,
            summary: Some("hello".into()),
        }]
    );
}

#[test]
fn test_same_conversation_id_is_isolated_per_tenant_in_projection() {
    let service = TimelineProjectionService::default();

    let alpha_event = im_domain_events::CommitEnvelope::minimal(
        "evt_alpha",
        "t_alpha",
        "message.posted",
        "conversation",
        "c_shared",
        1,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"t_alpha",
            "conversationId":"c_shared",
            "messageId":"m_alpha",
            "messageSeq":1,
            "sender":{"id":"u_alpha","kind":"user","memberId":"cm_alpha","deviceId":"d_alpha","sessionId":"s_alpha","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_alpha",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"alpha","parts":[{"kind":"text","text":"alpha"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:01Z",
            "committedAt":"2026-04-05T10:00:01Z"
        }"#,
    );
    let beta_event = im_domain_events::CommitEnvelope::minimal(
        "evt_beta",
        "t_beta",
        "message.posted",
        "conversation",
        "c_shared",
        1,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"t_beta",
            "conversationId":"c_shared",
            "messageId":"m_beta",
            "messageSeq":1,
            "sender":{"id":"agent_beta","kind":"agent","memberId":"cm_beta","deviceId":null,"sessionId":"s_beta","metadata":{"agentMode":"handoff"}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_beta",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"beta","parts":[{"kind":"text","text":"beta"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:01Z",
            "committedAt":"2026-04-05T10:00:01Z"
        }"#,
    );

    service
        .apply(&alpha_event)
        .expect("alpha projection should succeed");
    service
        .apply(&beta_event)
        .expect("beta projection should succeed");

    assert_eq!(service.timeline("t_alpha", "c_shared").len(), 1);
    assert_eq!(
        service.timeline("t_alpha", "c_shared")[0]
            .summary
            .as_deref(),
        Some("alpha")
    );
    assert_eq!(service.timeline("t_beta", "c_shared").len(), 1);
    assert_eq!(
        service.timeline("t_beta", "c_shared")[0].summary.as_deref(),
        Some("beta")
    );
}

#[test]
fn test_message_posted_event_projects_into_conversation_summary_view() {
    let service = TimelineProjectionService::default();

    let first_event = im_domain_events::CommitEnvelope::minimal(
        "evt_first",
        "t_demo",
        "message.posted",
        "conversation",
        "c_summary",
        1,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_summary",
            "messageId":"m_first",
            "messageSeq":1,
            "sender":{"id":"u_demo","kind":"user","memberId":"cm_demo","deviceId":"d_demo","sessionId":"s_demo","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_first",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"first","parts":[{"kind":"text","text":"hello"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:01Z",
            "committedAt":"2026-04-05T10:00:01Z"
        }"#,
    );
    let second_event = im_domain_events::CommitEnvelope::minimal(
        "evt_second",
        "t_demo",
        "message.posted",
        "conversation",
        "c_summary",
        2,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_summary",
            "messageId":"m_second",
            "messageSeq":2,
            "sender":{"id":"agent_demo","kind":"agent","memberId":null,"deviceId":null,"sessionId":"s_agent","metadata":{"agentId":"ag_demo"}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_second",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"second","parts":[{"kind":"text","text":"world"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:02Z",
            "committedAt":"2026-04-05T10:00:02Z"
        }"#,
    );

    service
        .apply(&first_event)
        .expect("first projection should succeed");
    service
        .apply(&second_event)
        .expect("second projection should succeed");

    let summary = service
        .conversation_summary("t_demo", "c_summary")
        .expect("summary should exist");

    assert_eq!(summary.message_count, 2);
    assert_eq!(summary.last_message_id.as_deref(), Some("m_second"));
    assert_eq!(summary.last_message_seq, 2);
    assert_eq!(summary.last_sender_id.as_deref(), Some("agent_demo"));
    assert_eq!(summary.last_sender_kind.as_deref(), Some("agent"));
    assert_eq!(summary.last_summary.as_deref(), Some("second"));
}

#[test]
fn test_read_cursor_event_projects_into_cursor_view_with_unread_count() {
    let service = TimelineProjectionService::default();

    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_member",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_cursor",
        1,
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
    );
    let message_posted = im_domain_events::CommitEnvelope::minimal(
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
            "messageId":"m_cursor_2",
            "messageSeq":2,
            "sender":{"id":"u_demo","kind":"user","memberId":"cm_demo","deviceId":null,"sessionId":"s_demo","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_2",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"second","parts":[{"kind":"text","text":"second"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:02Z",
            "committedAt":"2026-04-05T10:00:02Z"
        }"#,
    );
    let read_cursor_updated = im_domain_events::CommitEnvelope::minimal(
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
            "lastReadMessageId":"m_cursor_1",
            "updatedAt":"2026-04-05T10:00:10Z"
        }"#,
    );

    service
        .apply(&member_joined)
        .expect("member projection should succeed");
    service
        .apply(&message_posted)
        .expect("message projection should succeed");
    service
        .apply(&read_cursor_updated)
        .expect("read cursor projection should succeed");

    let cursor = service
        .read_cursor("t_demo", "c_cursor", "u_demo")
        .expect("cursor should exist");
    assert_eq!(cursor.member_id, "cm_demo");
    assert_eq!(cursor.read_seq, 1);
    assert_eq!(cursor.last_read_message_id.as_deref(), Some("m_cursor_1"));
    assert_eq!(cursor.unread_count, 1);
}

#[test]
fn test_member_role_changed_event_updates_member_snapshot() {
    let service = TimelineProjectionService::default();

    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_member_joined",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_role_projection",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_role_projection",
            "memberId":"cm_c_role_projection_u_member",
            "principalId":"u_member",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"u_owner",
            "joinedAt":"2026-04-06T10:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let role_changed = im_domain_events::CommitEnvelope::minimal(
        "evt_member_role_changed",
        "t_demo",
        "conversation.member_role_changed",
        "conversation",
        "c_role_projection",
        2,
    )
    .with_payload(
        "conversation.member_role_changed.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_role_projection",
            "previousMember":{
                "tenantId":"t_demo",
                "conversationId":"c_role_projection",
                "memberId":"cm_c_role_projection_u_member",
                "principalId":"u_member",
                "principalKind":"user",
                "role":"member",
                "state":"joined",
                "invitedBy":"u_owner",
                "joinedAt":"2026-04-06T10:00:00Z",
                "removedAt":null,
                "attributes":{}
            },
            "updatedMember":{
                "tenantId":"t_demo",
                "conversationId":"c_role_projection",
                "memberId":"cm_c_role_projection_u_member",
                "principalId":"u_member",
                "principalKind":"user",
                "role":"admin",
                "state":"joined",
                "invitedBy":"u_owner",
                "joinedAt":"2026-04-06T10:00:00Z",
                "removedAt":null,
                "attributes":{}
            },
            "changedAt":"2026-04-06T10:01:00Z"
        }"#,
    );

    service
        .apply(&member_joined)
        .expect("member joined projection should succeed");
    service
        .apply(&role_changed)
        .expect("role changed projection should succeed");

    let member = service
        .member_snapshot("t_demo", "c_role_projection", "u_member")
        .expect("member snapshot should exist");
    assert_eq!(member.role, MembershipRole::Admin);
}

#[test]
fn test_inbox_view_projects_member_summary_and_unread_count() {
    let service = TimelineProjectionService::default();

    let conversation_created = im_domain_events::CommitEnvelope::minimal(
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
    );
    let member_joined = im_domain_events::CommitEnvelope::minimal(
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
            "memberId":"cm_inbox_demo",
            "principalId":"u_demo",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-05T10:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let message_posted = im_domain_events::CommitEnvelope::minimal(
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
            "messageId":"m_inbox_2",
            "messageSeq":2,
            "sender":{"id":"u_other","kind":"user","memberId":"cm_other","deviceId":null,"sessionId":"s_other","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_2",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"second","parts":[{"kind":"text","text":"second"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:02Z",
            "committedAt":"2026-04-05T10:00:02Z"
        }"#,
    );
    let cursor_updated = im_domain_events::CommitEnvelope::minimal(
        "evt_cursor",
        "t_demo",
        "conversation.read_cursor_updated",
        "conversation",
        "c_inbox",
        1,
    )
    .with_payload(
        "conversation.read_cursor.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_inbox",
            "memberId":"cm_inbox_demo",
            "principalId":"u_demo",
            "readSeq":1,
            "lastReadMessageId":"m_inbox_1",
            "updatedAt":"2026-04-05T10:00:10Z"
        }"#,
    );

    service
        .apply(&conversation_created)
        .expect("conversation projection should succeed");
    service
        .apply(&member_joined)
        .expect("member projection should succeed");
    service
        .apply(&message_posted)
        .expect("message projection should succeed");
    service
        .apply(&cursor_updated)
        .expect("cursor projection should succeed");

    let inbox = service.inbox("t_demo", "u_demo");
    assert_eq!(inbox.len(), 1);
    assert_eq!(inbox[0].conversation_id, "c_inbox");
    assert_eq!(inbox[0].conversation_type, "group");
    assert_eq!(inbox[0].message_count, 2);
    assert_eq!(inbox[0].last_message_id.as_deref(), Some("m_inbox_2"));
    assert_eq!(inbox[0].last_sender_id.as_deref(), Some("u_other"));
    assert_eq!(inbox[0].unread_count, 1);
}

#[test]
fn test_device_sync_feed_projects_registered_devices_for_message_and_read_cursor_events() {
    let service = TimelineProjectionService::default();

    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_member",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_sync",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_sync",
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
    );
    let message_posted = im_domain_events::CommitEnvelope::minimal(
        "evt_message",
        "t_demo",
        "message.posted",
        "conversation",
        "c_sync",
        2,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_sync",
            "messageId":"msg_c_sync_1",
            "messageSeq":1,
            "sender":{"id":"u_demo","kind":"user","memberId":"cm_demo","deviceId":"d_phone","sessionId":"s_demo","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_sync_1",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"sync hello","parts":[{"kind":"text","text":"sync hello"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:02Z",
            "committedAt":"2026-04-05T10:00:02Z"
        }"#,
    );
    let cursor_updated = im_domain_events::CommitEnvelope::minimal(
        "evt_cursor",
        "t_demo",
        "conversation.read_cursor_updated",
        "conversation",
        "c_sync",
        1,
    )
    .with_payload(
        "conversation.read_cursor.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_sync",
            "memberId":"cm_demo",
            "principalId":"u_demo",
            "readSeq":1,
            "lastReadMessageId":"msg_c_sync_1",
            "updatedAt":"2026-04-05T10:00:10Z"
        }"#,
    );

    service
        .apply(&member_joined)
        .expect("member projection should succeed");
    service.register_device("t_demo", "u_demo", "d_phone");
    service.register_device("t_demo", "u_demo", "d_pad");
    service
        .apply(&message_posted)
        .expect("message projection should succeed");
    service
        .apply(&cursor_updated)
        .expect("cursor projection should succeed");

    let feed = service.device_sync_feed("t_demo", "u_demo", "d_pad", Some(0));
    assert_eq!(feed.len(), 2);
    assert_eq!(feed[0].sync_seq, 1);
    assert_eq!(feed[0].origin_event_type, "message.posted");
    assert_eq!(feed[0].message_id.as_deref(), Some("msg_c_sync_1"));
    assert_eq!(feed[0].actor_device_id.as_deref(), Some("d_phone"));
    assert_eq!(feed[1].sync_seq, 2);
    assert_eq!(
        feed[1].origin_event_type,
        "conversation.read_cursor_updated"
    );
    assert_eq!(feed[1].read_seq, Some(1));
    assert_eq!(
        feed[1].last_read_message_id.as_deref(),
        Some("msg_c_sync_1")
    );
}

#[test]
fn test_member_governance_events_project_typed_sync_feed_deltas() {
    let service = TimelineProjectionService::default();

    let mut owner_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_owner_joined",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_member_sync",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_member_sync",
            "memberId":"cm_c_member_sync_u_owner",
            "principalId":"u_owner",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-06T12:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    owner_joined.actor.actor_id = "u_owner".into();
    owner_joined.actor.actor_kind = "user".into();
    service
        .apply(&owner_joined)
        .expect("owner joined projection should succeed");

    service.register_device("t_demo", "u_owner", "d_owner");
    service.register_device("t_demo", "u_other", "d_other");
    service.register_device("t_demo", "u_leave", "d_leave");

    let mut other_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_other_joined",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_member_sync",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_member_sync",
            "memberId":"cm_c_member_sync_u_other",
            "principalId":"u_other",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"u_owner",
            "joinedAt":"2026-04-06T12:01:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    other_joined.actor.actor_id = "u_owner".into();
    other_joined.actor.actor_kind = "user".into();
    let mut other_role_changed = im_domain_events::CommitEnvelope::minimal(
        "evt_other_role_changed",
        "t_demo",
        "conversation.member_role_changed",
        "conversation",
        "c_member_sync",
        3,
    )
    .with_payload(
        "conversation.member_role_changed.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_member_sync",
            "previousMember":{
                "tenantId":"t_demo",
                "conversationId":"c_member_sync",
                "memberId":"cm_c_member_sync_u_other",
                "principalId":"u_other",
                "principalKind":"user",
                "role":"member",
                "state":"joined",
                "invitedBy":"u_owner",
                "joinedAt":"2026-04-06T12:01:00Z",
                "removedAt":null,
                "attributes":{}
            },
            "updatedMember":{
                "tenantId":"t_demo",
                "conversationId":"c_member_sync",
                "memberId":"cm_c_member_sync_u_other",
                "principalId":"u_other",
                "principalKind":"user",
                "role":"admin",
                "state":"joined",
                "invitedBy":"u_owner",
                "joinedAt":"2026-04-06T12:01:00Z",
                "removedAt":null,
                "attributes":{}
            },
            "changedAt":"2026-04-06T12:02:00Z"
        }"#,
    );
    other_role_changed.actor.actor_id = "u_owner".into();
    other_role_changed.actor.actor_kind = "user".into();
    let mut other_removed = im_domain_events::CommitEnvelope::minimal(
        "evt_other_removed",
        "t_demo",
        "conversation.member_removed",
        "conversation",
        "c_member_sync",
        4,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_member_sync",
            "memberId":"cm_c_member_sync_u_other",
            "principalId":"u_other",
            "principalKind":"user",
            "role":"admin",
            "state":"removed",
            "invitedBy":"u_owner",
            "joinedAt":"2026-04-06T12:01:00Z",
            "removedAt":"2026-04-06T12:03:00Z",
            "attributes":{}
        }"#,
    );
    other_removed.actor.actor_id = "u_owner".into();
    other_removed.actor.actor_kind = "user".into();
    let mut leave_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_leave_joined",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_member_sync",
        5,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_member_sync",
            "memberId":"cm_c_member_sync_u_leave",
            "principalId":"u_leave",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"u_owner",
            "joinedAt":"2026-04-06T12:04:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    leave_joined.actor.actor_id = "u_owner".into();
    leave_joined.actor.actor_kind = "user".into();
    let mut leave_left = im_domain_events::CommitEnvelope::minimal(
        "evt_leave_left",
        "t_demo",
        "conversation.member_left",
        "conversation",
        "c_member_sync",
        6,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_member_sync",
            "memberId":"cm_c_member_sync_u_leave",
            "principalId":"u_leave",
            "principalKind":"user",
            "role":"member",
            "state":"left",
            "invitedBy":"u_owner",
            "joinedAt":"2026-04-06T12:04:00Z",
            "removedAt":"2026-04-06T12:05:00Z",
            "attributes":{}
        }"#,
    );
    leave_left.actor.actor_id = "u_leave".into();
    leave_left.actor.actor_kind = "user".into();

    for event in [
        other_joined,
        other_role_changed,
        other_removed,
        leave_joined,
        leave_left,
    ] {
        service
            .apply(&event)
            .expect("member governance projection should succeed");
    }

    let owner_feed = service.device_sync_feed("t_demo", "u_owner", "d_owner", Some(0));
    assert_eq!(owner_feed.len(), 5);
    assert_eq!(
        owner_feed[0].origin_event_type,
        "conversation.member_joined"
    );
    assert_eq!(
        owner_feed[1].origin_event_type,
        "conversation.member_role_changed"
    );
    assert_eq!(
        owner_feed[2].origin_event_type,
        "conversation.member_removed"
    );
    assert_eq!(
        owner_feed[3].origin_event_type,
        "conversation.member_joined"
    );
    assert_eq!(owner_feed[4].origin_event_type, "conversation.member_left");

    let joined_value =
        serde_json::to_value(&owner_feed[0]).expect("joined sync entry should serialize");
    assert_eq!(
        joined_value["payloadSchema"],
        serde_json::Value::String("conversation.member.v1".into())
    );
    let joined_payload: serde_json::Value = serde_json::from_str(
        joined_value["payload"]
            .as_str()
            .expect("joined sync payload should be present"),
    )
    .expect("joined sync payload should be valid json");
    assert_eq!(joined_payload["principalId"], "u_other");
    assert_eq!(joined_payload["state"], "joined");
    assert_eq!(joined_value["actorId"], "u_owner");
    assert_eq!(joined_value["actorKind"], "user");

    let role_changed_value =
        serde_json::to_value(&owner_feed[1]).expect("role change sync entry should serialize");
    assert_eq!(
        role_changed_value["payloadSchema"],
        serde_json::Value::String("conversation.member_role_changed.v1".into())
    );
    let role_changed_payload: serde_json::Value = serde_json::from_str(
        role_changed_value["payload"]
            .as_str()
            .expect("role change payload should be present"),
    )
    .expect("role change payload should be valid json");
    assert_eq!(role_changed_payload["previousMember"]["role"], "member");
    assert_eq!(role_changed_payload["updatedMember"]["role"], "admin");
    assert_eq!(role_changed_value["actorId"], "u_owner");
    assert_eq!(role_changed_value["actorKind"], "user");

    let removed_value =
        serde_json::to_value(&owner_feed[2]).expect("removed sync entry should serialize");
    assert_eq!(
        removed_value["payloadSchema"],
        serde_json::Value::String("conversation.member.v1".into())
    );
    let removed_payload: serde_json::Value = serde_json::from_str(
        removed_value["payload"]
            .as_str()
            .expect("removed payload should be present"),
    )
    .expect("removed payload should be valid json");
    assert_eq!(removed_payload["principalId"], "u_other");
    assert_eq!(removed_payload["state"], "removed");
    assert_eq!(removed_value["actorId"], "u_owner");
    assert_eq!(removed_value["actorKind"], "user");

    let removed_principal_feed = service.device_sync_feed("t_demo", "u_other", "d_other", Some(0));
    assert_eq!(removed_principal_feed.len(), 3);
    assert_eq!(
        removed_principal_feed[2].origin_event_type,
        "conversation.member_removed"
    );
    let removed_principal_value = serde_json::to_value(&removed_principal_feed[2])
        .expect("removed principal sync entry should serialize");
    let removed_principal_payload: serde_json::Value = serde_json::from_str(
        removed_principal_value["payload"]
            .as_str()
            .expect("removed principal payload should be present"),
    )
    .expect("removed principal payload should be valid json");
    assert_eq!(removed_principal_payload["principalId"], "u_other");
    assert_eq!(removed_principal_payload["state"], "removed");

    let leave_feed = service.device_sync_feed("t_demo", "u_leave", "d_leave", Some(0));
    assert_eq!(leave_feed.len(), 2);
    assert_eq!(leave_feed[1].origin_event_type, "conversation.member_left");
    let leave_value =
        serde_json::to_value(&leave_feed[1]).expect("leave sync entry should serialize");
    assert_eq!(
        leave_value["payloadSchema"],
        serde_json::Value::String("conversation.member.v1".into())
    );
    let leave_payload: serde_json::Value = serde_json::from_str(
        leave_value["payload"]
            .as_str()
            .expect("leave payload should be present"),
    )
    .expect("leave payload should be valid json");
    assert_eq!(leave_payload["principalId"], "u_leave");
    assert_eq!(leave_payload["state"], "left");
    assert_eq!(leave_value["actorId"], "u_leave");
    assert_eq!(leave_value["actorKind"], "user");
}

#[test]
fn test_registered_devices_and_latest_sync_seq_are_queryable() {
    let service = TimelineProjectionService::default();

    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_member",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_resume",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_resume",
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
    );
    let message_posted = im_domain_events::CommitEnvelope::minimal(
        "evt_message",
        "t_demo",
        "message.posted",
        "conversation",
        "c_resume",
        2,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_resume",
            "messageId":"msg_c_resume_1",
            "messageSeq":1,
            "sender":{"id":"u_demo","kind":"user","memberId":"cm_demo","deviceId":"d_phone","sessionId":"s_demo","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_resume_1",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"resume hello","parts":[{"kind":"text","text":"resume hello"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:02Z",
            "committedAt":"2026-04-05T10:00:02Z"
        }"#,
    );

    service
        .apply(&member_joined)
        .expect("member projection should succeed");
    service.register_device("t_demo", "u_demo", "d_phone");
    service.register_device("t_demo", "u_demo", "d_pad");
    service
        .apply(&message_posted)
        .expect("message projection should succeed");

    let devices = service.registered_devices("t_demo", "u_demo");
    assert_eq!(devices.len(), 2);
    assert!(devices.iter().any(|item| item.device_id == "d_phone"));
    assert!(devices.iter().any(|item| item.device_id == "d_pad"));

    assert_eq!(
        service.latest_device_sync_seq("t_demo", "u_demo", "d_phone"),
        1
    );
    assert_eq!(
        service.latest_device_sync_seq("t_demo", "u_demo", "d_pad"),
        1
    );
    assert_eq!(
        service.latest_device_sync_seq("t_demo", "u_demo", "d_missing"),
        0
    );
}

#[test]
fn test_realtime_fanout_targets_for_principals_return_registered_principal_device_pairs() {
    let service = TimelineProjectionService::default();

    service.register_device("t_demo", "u_b", "d_phone");
    service.register_device("t_demo", "u_a", "d_watch");
    service.register_device("t_demo", "u_a", "d_pad");

    let targets = service.realtime_fanout_targets_for_principals(
        "t_demo",
        vec![
            "u_b".to_string(),
            "u_missing".to_string(),
            "u_a".to_string(),
        ],
    );

    assert_eq!(
        targets,
        vec![
            RealtimeFanoutTarget {
                principal_id: "u_a".into(),
                principal_kind: None,
                device_id: "d_pad".into(),
            },
            RealtimeFanoutTarget {
                principal_id: "u_a".into(),
                principal_kind: None,
                device_id: "d_watch".into(),
            },
            RealtimeFanoutTarget {
                principal_id: "u_b".into(),
                principal_kind: None,
                device_id: "d_phone".into(),
            },
        ]
    );
}

#[test]
fn test_device_sync_fanout_targets_for_conversation_include_active_members_and_fallback_devices() {
    let service = TimelineProjectionService::default();

    let owner_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_sync_targets_owner",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_sync_targets",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_sync_targets",
            "memberId":"cm_sync_targets_owner",
            "principalId":"u_owner",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-07T09:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_sync_targets_member",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_sync_targets",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_sync_targets",
            "memberId":"cm_sync_targets_member",
            "principalId":"u_member",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"u_owner",
            "joinedAt":"2026-04-07T09:01:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );

    service
        .apply(&owner_joined)
        .expect("owner projection should succeed");
    service
        .apply(&member_joined)
        .expect("member projection should succeed");

    service.register_device("t_demo", "u_owner", "d_phone");
    service.register_device("t_demo", "u_owner", "d_pad");
    service.register_device("t_demo", "u_member", "d_watch");
    service.register_device("t_demo", "u_removed", "d_removed");

    let targets = service.device_sync_fanout_targets_for_conversation(
        "t_demo",
        "c_sync_targets",
        vec![NotificationRecipientView {
            principal_id: "u_removed".into(),
            principal_kind: "user".into(),
        }],
    );

    assert_eq!(
        targets,
        vec![
            RealtimeFanoutTarget {
                principal_id: "u_member".into(),
                principal_kind: Some("user".into()),
                device_id: "d_watch".into(),
            },
            RealtimeFanoutTarget {
                principal_id: "u_owner".into(),
                principal_kind: Some("user".into()),
                device_id: "d_pad".into(),
            },
            RealtimeFanoutTarget {
                principal_id: "u_owner".into(),
                principal_kind: Some("user".into()),
                device_id: "d_phone".into(),
            },
            RealtimeFanoutTarget {
                principal_id: "u_removed".into(),
                principal_kind: Some("user".into()),
                device_id: "d_removed".into(),
            },
        ]
    );
}

#[test]
fn test_active_conversation_principal_ids_from_auth_context_returns_current_active_members() {
    let service = TimelineProjectionService::default();

    let owner_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_active_principals_owner",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_active_principals",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_active_principals",
            "memberId":"cm_active_principals_owner",
            "principalId":"u_owner",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-07T10:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_active_principals_member",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_active_principals",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_active_principals",
            "memberId":"cm_active_principals_member",
            "principalId":"u_member",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"u_owner",
            "joinedAt":"2026-04-07T10:01:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let member_removed = im_domain_events::CommitEnvelope::minimal(
        "evt_active_principals_removed",
        "t_demo",
        "conversation.member_removed",
        "conversation",
        "c_active_principals",
        3,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_active_principals",
            "memberId":"cm_active_principals_member",
            "principalId":"u_member",
            "principalKind":"user",
            "role":"member",
            "state":"removed",
            "invitedBy":"u_owner",
            "joinedAt":"2026-04-07T10:01:00Z",
            "removedAt":"2026-04-07T10:02:00Z",
            "attributes":{}
        }"#,
    );

    for event in [owner_joined, member_joined, member_removed] {
        service
            .apply(&event)
            .expect("member projection should succeed");
    }

    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_owner".into(),
        actor_kind: "user".into(),
        session_id: Some("s_owner".into()),
        device_id: Some("d_owner".into()),
        permissions: Default::default(),
    };

    assert_eq!(
        service
            .active_conversation_principal_ids_from_auth_context(&auth, "c_active_principals")
            .expect("active member should read active principal ids"),
        vec!["u_owner".to_string()]
    );
}

#[test]
fn test_message_posted_notification_recipients_from_auth_context_include_shared_linked_members() {
    let service = TimelineProjectionService::default();

    let owner_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_notification_targets_owner",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_notification_targets",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_notification_targets",
            "memberId":"cm_notification_targets_owner",
            "principalId":"u_owner",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-07T10:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_notification_targets_member",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_notification_targets",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_notification_targets",
            "memberId":"cm_notification_targets_member",
            "principalId":"u_member",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"u_owner",
            "joinedAt":"2026-04-07T10:01:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let shared_linked = im_domain_events::CommitEnvelope::minimal(
        "evt_notification_targets_shared_linked",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_notification_targets",
        3,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_notification_targets",
            "memberId":"cm_notification_targets_shared",
            "principalId":"u_shared_external",
            "principalKind":"external_user",
            "role":"member",
            "state":"linked",
            "invitedBy":"u_owner",
            "joinedAt":"2026-04-07T10:02:00Z",
            "removedAt":null,
            "attributes":{
                "sharedChannelPolicyId":"scp_demo",
                "externalConnectionId":"conn_demo",
                "externalMemberId":"ext_demo"
            }
        }"#,
    );

    for event in [owner_joined, member_joined, shared_linked] {
        service
            .apply(&event)
            .expect("member projection should accept notification target events");
    }

    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_owner".into(),
        actor_kind: "user".into(),
        session_id: Some("s_owner".into()),
        device_id: Some("d_owner".into()),
        permissions: Default::default(),
    };

    assert_eq!(
        service
            .active_conversation_principal_ids_from_auth_context(&auth, "c_notification_targets")
            .expect("active member should still resolve active principal ids"),
        vec!["u_member".to_string(), "u_owner".to_string()]
    );
    assert_eq!(
        service
            .message_posted_notification_recipients_from_auth_context(
                &auth,
                "c_notification_targets"
            )
            .expect("active member should resolve shared notification principal ids"),
        vec![
            projection_service::NotificationRecipientView {
                principal_id: "u_member".into(),
                principal_kind: "user".into(),
            },
            projection_service::NotificationRecipientView {
                principal_id: "u_owner".into(),
                principal_kind: "user".into(),
            },
            projection_service::NotificationRecipientView {
                principal_id: "u_shared_external".into(),
                principal_kind: "external_user".into(),
            }
        ]
    );
}

#[test]
fn test_member_directory_and_notification_recipients_preserve_same_actor_id_across_principal_kinds()
{
    let service = TimelineProjectionService::default();

    let owner_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_typed_member_directory_owner",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_typed_member_directory",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_typed_member_directory",
            "memberId":"cm_typed_member_directory_owner",
            "principalId":"u_dual",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-07T11:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let agent_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_typed_member_directory_agent",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_typed_member_directory",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_typed_member_directory",
            "memberId":"cm_typed_member_directory_agent",
            "principalId":"u_dual",
            "principalKind":"agent",
            "role":"member",
            "state":"joined",
            "invitedBy":"u_dual",
            "joinedAt":"2026-04-07T11:01:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );

    for event in [owner_joined, agent_joined] {
        service
            .apply(&event)
            .expect("typed member projection should succeed");
    }

    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_dual".into(),
        actor_kind: "user".into(),
        session_id: Some("s_dual_user".into()),
        device_id: Some("d_dual_user".into()),
        permissions: Default::default(),
    };

    let directory = service
        .member_directory_from_auth_context(&auth, "c_typed_member_directory")
        .expect("typed user member should still access directory");
    assert_eq!(directory.len(), 2);
    assert!(directory.iter().any(|member| {
        member.principal_id == "u_dual"
            && member.principal_kind == "user"
            && member.role == MembershipRole::Owner
    }));
    assert!(directory.iter().any(|member| {
        member.principal_id == "u_dual"
            && member.principal_kind == "agent"
            && member.role == MembershipRole::Member
    }));

    assert_eq!(
        service
            .message_posted_notification_recipients_from_auth_context(
                &auth,
                "c_typed_member_directory",
            )
            .expect("typed user member should still resolve typed recipients"),
        vec![
            NotificationRecipientView {
                principal_id: "u_dual".into(),
                principal_kind: "agent".into(),
            },
            NotificationRecipientView {
                principal_id: "u_dual".into(),
                principal_kind: "user".into(),
            }
        ]
    );
}

#[test]
fn test_typed_realtime_recipients_exclude_non_member_devices_sharing_same_actor_id() {
    let service = TimelineProjectionService::default();

    let owner_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_typed_realtime_targets_owner",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_typed_realtime_targets",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_typed_realtime_targets",
            "memberId":"cm_typed_realtime_targets_owner",
            "principalId":"u_owner",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-07T12:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_typed_realtime_targets_member",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_typed_realtime_targets",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_typed_realtime_targets",
            "memberId":"cm_typed_realtime_targets_member",
            "principalId":"u_dual",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"u_owner",
            "joinedAt":"2026-04-07T12:01:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );

    for event in [owner_joined, member_joined] {
        service
            .apply(&event)
            .expect("typed realtime target projection should succeed");
    }

    service.register_device_for_principal_kind("t_demo", "u_owner", "user", "d_owner");
    service.register_device_for_principal_kind("t_demo", "u_dual", "user", "d_dual_user");
    service.register_device_for_principal_kind("t_demo", "u_dual", "agent", "d_dual_agent");

    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_owner".into(),
        actor_kind: "user".into(),
        session_id: Some("s_owner".into()),
        device_id: Some("d_owner".into()),
        permissions: Default::default(),
    };

    let recipients = service
        .active_conversation_principal_recipients_from_auth_context(
            &auth,
            "c_typed_realtime_targets",
        )
        .expect("owner should resolve typed realtime recipients");
    assert_eq!(
        recipients,
        vec![
            NotificationRecipientView {
                principal_id: "u_dual".into(),
                principal_kind: "user".into(),
            },
            NotificationRecipientView {
                principal_id: "u_owner".into(),
                principal_kind: "user".into(),
            }
        ]
    );

    assert_eq!(
        service.realtime_fanout_targets_for_recipients_from_auth_context(&auth, recipients),
        vec![
            RealtimeFanoutTarget {
                principal_id: "u_dual".into(),
                principal_kind: Some("user".into()),
                device_id: "d_dual_user".into(),
            },
            RealtimeFanoutTarget {
                principal_id: "u_owner".into(),
                principal_kind: Some("user".into()),
                device_id: "d_owner".into(),
            }
        ]
    );
}

#[test]
fn test_device_sync_state_isolated_for_same_actor_and_device_across_principal_kinds() {
    let service = TimelineProjectionService::default();

    let owner_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_typed_device_scope_owner",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_typed_device_scope",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_typed_device_scope",
            "memberId":"cm_typed_device_scope_owner",
            "principalId":"u_owner",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-07T13:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let user_member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_typed_device_scope_user_member",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_typed_device_scope",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_typed_device_scope",
            "memberId":"cm_typed_device_scope_user_member",
            "principalId":"u_dual",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"u_owner",
            "joinedAt":"2026-04-07T13:01:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );

    for event in [owner_joined, user_member_joined] {
        service
            .apply(&event)
            .expect("typed device scope projection should succeed");
    }

    service.register_device_for_principal_kind("t_demo", "u_owner", "user", "d_owner");
    service.register_device_for_principal_kind("t_demo", "u_dual", "user", "d_shared");
    service.register_device_for_principal_kind("t_demo", "u_dual", "agent", "d_shared");

    let user_auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_dual".into(),
        actor_kind: "user".into(),
        session_id: Some("s_typed_device_scope_user".into()),
        device_id: Some("d_shared".into()),
        permissions: Default::default(),
    };
    let agent_auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_dual".into(),
        actor_kind: "agent".into(),
        session_id: Some("s_typed_device_scope_agent".into()),
        device_id: Some("d_shared".into()),
        permissions: Default::default(),
    };

    let user_devices = service.registered_devices_from_auth_context(&user_auth);
    assert_eq!(user_devices.len(), 1);
    assert_eq!(user_devices[0].device_id, "d_shared");
    assert_eq!(user_devices[0].principal_kind.as_deref(), Some("user"));

    let agent_devices = service.registered_devices_from_auth_context(&agent_auth);
    assert_eq!(agent_devices.len(), 1);
    assert_eq!(agent_devices[0].device_id, "d_shared");
    assert_eq!(agent_devices[0].principal_kind.as_deref(), Some("agent"));

    assert_eq!(
        service.device_sync_fanout_targets_for_conversation(
            "t_demo",
            "c_typed_device_scope",
            vec![],
        ),
        vec![
            RealtimeFanoutTarget {
                principal_id: "u_dual".into(),
                principal_kind: Some("user".into()),
                device_id: "d_shared".into(),
            },
            RealtimeFanoutTarget {
                principal_id: "u_owner".into(),
                principal_kind: Some("user".into()),
                device_id: "d_owner".into(),
            },
        ]
    );

    let message_posted = im_domain_events::CommitEnvelope::minimal(
        "evt_typed_device_scope_message",
        "t_demo",
        "message.posted",
        "conversation",
        "c_typed_device_scope",
        3,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_typed_device_scope",
            "messageId":"msg_typed_device_scope_1",
            "messageSeq":1,
            "sender":{"id":"u_owner","kind":"user","memberId":"cm_typed_device_scope_owner","deviceId":"d_owner","sessionId":"s_typed_device_scope_owner","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_typed_device_scope_1",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"typed-device-scope","parts":[{"kind":"text","text":"typed-device-scope"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-07T13:02:00Z",
            "committedAt":"2026-04-07T13:02:00Z"
        }"#,
    );

    service
        .apply(&message_posted)
        .expect("typed device scope message projection should succeed");

    let user_feed = service
        .device_sync_feed_from_auth_context(&user_auth, "d_shared", Some(0))
        .expect("user feed should remain accessible");
    assert_eq!(user_feed.len(), 1);
    assert_eq!(
        user_feed[0].message_id.as_deref(),
        Some("msg_typed_device_scope_1")
    );
    assert_eq!(
        service
            .latest_device_sync_seq_from_auth_context(&user_auth, "d_shared")
            .expect("user seq should remain accessible"),
        1
    );

    let agent_feed = service
        .device_sync_feed_from_auth_context(&agent_auth, "d_shared", Some(0))
        .expect("agent feed should remain accessible");
    assert!(agent_feed.is_empty());
    assert_eq!(
        service
            .latest_device_sync_seq_from_auth_context(&agent_auth, "d_shared")
            .expect("agent seq should remain accessible"),
        0
    );
}

#[test]
fn test_registered_device_timestamps_advance_between_distinct_registrations() {
    let service = TimelineProjectionService::default();

    let first = service.register_device("t_demo", "u_demo", "d_phone");
    sleep(Duration::from_millis(20));
    let second = service.register_device("t_demo", "u_demo", "d_pad");

    assert!(first.registered_at < second.registered_at);
}

#[test]
fn test_message_edit_and_recall_events_update_timeline_and_summary() {
    let service = TimelineProjectionService::default();

    let posted = im_domain_events::CommitEnvelope::minimal(
        "evt_message_posted",
        "t_demo",
        "message.posted",
        "conversation",
        "c_mutation",
        1,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_mutation",
            "messageId":"msg_c_mutation_1",
            "messageSeq":1,
            "sender":{"id":"u_demo","kind":"user","memberId":"cm_demo","deviceId":"d_demo","sessionId":"s_demo","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_mutation_1",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"hello","parts":[{"kind":"text","text":"hello"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:01Z",
            "committedAt":"2026-04-05T10:00:01Z"
        }"#,
    );
    let edited = im_domain_events::CommitEnvelope::minimal(
        "evt_message_edited",
        "t_demo",
        "message.edited",
        "conversation",
        "c_mutation",
        1,
    )
    .with_payload(
        "message.edited.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_mutation",
            "messageId":"msg_c_mutation_1",
            "messageSeq":1,
            "body":{"summary":"edited","parts":[{"kind":"text","text":"edited"}],"renderHints":{}},
            "editor":{"id":"u_demo","kind":"user","memberId":"cm_demo","deviceId":"d_demo","sessionId":"s_demo","metadata":{}},
            "editedAt":"2026-04-05T10:00:30Z"
        }"#,
    );
    let recalled = im_domain_events::CommitEnvelope::minimal(
        "evt_message_recalled",
        "t_demo",
        "message.recalled",
        "conversation",
        "c_mutation",
        1,
    )
    .with_payload(
        "message.recalled.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_mutation",
            "messageId":"msg_c_mutation_1",
            "messageSeq":1,
            "recalledBy":{"id":"u_demo","kind":"user","memberId":"cm_demo","deviceId":"d_demo","sessionId":"s_demo","metadata":{}},
            "recalledAt":"2026-04-05T10:00:40Z"
        }"#,
    );

    service
        .apply(&posted)
        .expect("post projection should succeed");
    service
        .apply(&edited)
        .expect("edit projection should succeed");
    service
        .apply(&recalled)
        .expect("recall projection should succeed");

    let timeline = service.timeline("t_demo", "c_mutation");
    assert_eq!(timeline.len(), 1);
    assert_eq!(timeline[0].message_id, "msg_c_mutation_1");
    assert_eq!(timeline[0].summary.as_deref(), Some("[recalled]"));

    let summary = service
        .conversation_summary("t_demo", "c_mutation")
        .expect("summary should exist");
    assert_eq!(summary.last_message_id.as_deref(), Some("msg_c_mutation_1"));
    assert_eq!(summary.last_summary.as_deref(), Some("[recalled]"));
}

#[test]
fn test_reaction_and_pin_events_project_into_interaction_summary_views() {
    let service = TimelineProjectionService::default();

    let posted = im_domain_events::CommitEnvelope::minimal(
        "evt_interaction_posted",
        "t_demo",
        "message.posted",
        "conversation",
        "c_interaction",
        1,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_interaction",
            "messageId":"msg_c_interaction_1",
            "messageSeq":1,
            "sender":{"id":"u_owner","kind":"user","memberId":"cm_owner","deviceId":"d_owner","sessionId":"s_owner","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_interaction_1",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"interaction target","parts":[{"kind":"text","text":"interaction target"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-10T12:00:00Z",
            "committedAt":"2026-04-10T12:00:00Z"
        }"#,
    );
    let reaction_added_owner = im_domain_events::CommitEnvelope::minimal(
        "evt_interaction_reaction_owner",
        "t_demo",
        "message.reaction_added",
        "conversation",
        "c_interaction",
        2,
    )
    .with_payload(
        "message.reaction_added.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_interaction",
            "messageId":"msg_c_interaction_1",
            "messageSeq":1,
            "reactionKey":"thumbs_up",
            "reactedBy":{"id":"u_owner","kind":"user","memberId":"cm_owner","deviceId":"d_owner","sessionId":"s_owner","metadata":{}},
            "reactedAt":"2026-04-10T12:00:10Z"
        }"#,
    );
    let reaction_added_member = im_domain_events::CommitEnvelope::minimal(
        "evt_interaction_reaction_member",
        "t_demo",
        "message.reaction_added",
        "conversation",
        "c_interaction",
        3,
    )
    .with_payload(
        "message.reaction_added.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_interaction",
            "messageId":"msg_c_interaction_1",
            "messageSeq":1,
            "reactionKey":"thumbs_up",
            "reactedBy":{"id":"u_member","kind":"user","memberId":"cm_member","deviceId":"d_member","sessionId":"s_member","metadata":{}},
            "reactedAt":"2026-04-10T12:00:11Z"
        }"#,
    );
    let pinned = im_domain_events::CommitEnvelope::minimal(
        "evt_interaction_pin",
        "t_demo",
        "message.pin_added",
        "conversation",
        "c_interaction",
        4,
    )
    .with_payload(
        "message.pin_added.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_interaction",
            "messageId":"msg_c_interaction_1",
            "messageSeq":1,
            "pinnedBy":{"id":"u_owner","kind":"user","memberId":"cm_owner","deviceId":"d_owner","sessionId":"s_owner","metadata":{}},
            "pinnedAt":"2026-04-10T12:00:20Z"
        }"#,
    );
    let reaction_removed_owner = im_domain_events::CommitEnvelope::minimal(
        "evt_interaction_reaction_removed_owner",
        "t_demo",
        "message.reaction_removed",
        "conversation",
        "c_interaction",
        5,
    )
    .with_payload(
        "message.reaction_removed.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_interaction",
            "messageId":"msg_c_interaction_1",
            "messageSeq":1,
            "reactionKey":"thumbs_up",
            "removedBy":{"id":"u_owner","kind":"user","memberId":"cm_owner","deviceId":"d_owner","sessionId":"s_owner","metadata":{}},
            "removedAt":"2026-04-10T12:00:30Z"
        }"#,
    );
    let unpinned = im_domain_events::CommitEnvelope::minimal(
        "evt_interaction_unpin",
        "t_demo",
        "message.pin_removed",
        "conversation",
        "c_interaction",
        6,
    )
    .with_payload(
        "message.pin_removed.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_interaction",
            "messageId":"msg_c_interaction_1",
            "messageSeq":1,
            "unpinnedBy":{"id":"u_owner","kind":"user","memberId":"cm_owner","deviceId":"d_owner","sessionId":"s_owner","metadata":{}},
            "unpinnedAt":"2026-04-10T12:00:40Z"
        }"#,
    );

    for event in [
        posted,
        reaction_added_owner.clone(),
        reaction_added_owner,
        reaction_added_member,
        pinned.clone(),
        pinned,
        reaction_removed_owner.clone(),
        reaction_removed_owner,
        unpinned.clone(),
        unpinned,
    ] {
        service
            .apply(&event)
            .expect("interaction projection should succeed");
    }

    let summary = service
        .message_interaction_summary("t_demo", "c_interaction", "msg_c_interaction_1")
        .expect("interaction summary should exist");
    assert_eq!(summary.message_seq, 1);
    assert_eq!(summary.total_reaction_count, 1);
    assert_eq!(
        summary.reaction_counts,
        vec![MessageReactionCountView {
            reaction_key: "thumbs_up".into(),
            count: 1,
        }]
    );
    assert_eq!(summary.pin, None);

    assert!(
        service
            .pinned_messages("t_demo", "c_interaction")
            .is_empty(),
        "unpinned message should not stay in pinned-message summary view"
    );
}

#[test]
fn test_agent_handoff_lifecycle_projects_into_summary_and_inbox_views() {
    let service = TimelineProjectionService::default();

    let conversation_created = im_domain_events::CommitEnvelope::minimal(
        "evt_handoff_created",
        "t_demo",
        "conversation.created",
        "conversation",
        "c_handoff_projection",
        0,
    )
    .with_payload(
        "conversation.created.v1",
        r#"{
            "conversationId":"c_handoff_projection",
            "conversationType":"agent_handoff",
            "source":{"id":"ag_source","kind":"agent"},
            "target":{"id":"u_member","kind":"user"},
            "handoff":{"sessionId":"hs_projection","reason":"manual_escalation","status":"open"}
        }"#,
    );
    let source_member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_handoff_source_member",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_handoff_projection",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_handoff_projection",
            "memberId":"cm_handoff_source",
            "principalId":"ag_source",
            "principalKind":"agent",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-06T10:00:00Z",
            "removedAt":null,
            "attributes":{"handoffRole":"source"}
        }"#,
    );
    let target_member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_handoff_target_member",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_handoff_projection",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_handoff_projection",
            "memberId":"cm_handoff_target",
            "principalId":"u_member",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"ag_source",
            "joinedAt":"2026-04-06T10:00:00Z",
            "removedAt":null,
            "attributes":{"handoffRole":"target"}
        }"#,
    );
    let handoff_accepted = im_domain_events::CommitEnvelope::minimal(
        "evt_handoff_accepted",
        "t_demo",
        "conversation.agent_handoff_status_changed",
        "conversation",
        "c_handoff_projection",
        3,
    )
    .with_payload(
        "conversation.agent_handoff_status_changed.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_handoff_projection",
            "previousStatus":"open",
            "currentStatus":"accepted",
            "changedBy":{"id":"u_member","kind":"user"},
            "changedAt":"2026-04-06T10:01:00Z",
            "state":{
                "tenantId":"t_demo",
                "conversationId":"c_handoff_projection",
                "status":"accepted",
                "source":{"id":"ag_source","kind":"agent"},
                "target":{"id":"u_member","kind":"user"},
                "handoffSessionId":"hs_projection",
                "handoffReason":"manual_escalation",
                "acceptedAt":"2026-04-06T10:01:00Z",
                "acceptedBy":{"id":"u_member","kind":"user"},
                "resolvedAt":null,
                "resolvedBy":null,
                "closedAt":null,
                "closedBy":null
            }
        }"#,
    );

    service
        .apply(&conversation_created)
        .expect("conversation created projection should succeed");
    service
        .apply(&source_member_joined)
        .expect("source member projection should succeed");
    service
        .apply(&target_member_joined)
        .expect("target member projection should succeed");

    let initial_summary = service
        .conversation_summary("t_demo", "c_handoff_projection")
        .expect("handoff summary should exist immediately after create");
    let initial_handoff = initial_summary
        .agent_handoff
        .as_ref()
        .expect("handoff summary should expose handoff state");
    assert_eq!(initial_summary.message_count, 0);
    assert_eq!(initial_summary.last_message_id, None);
    assert_eq!(initial_handoff.status, "open");
    assert_eq!(initial_handoff.source.id, "ag_source");
    assert_eq!(initial_handoff.target.id, "u_member");

    let initial_inbox = service.inbox("t_demo", "u_member");
    assert_eq!(initial_inbox.len(), 1);
    let initial_inbox_handoff = initial_inbox[0]
        .agent_handoff
        .as_ref()
        .expect("inbox should expose handoff state");
    assert_eq!(initial_inbox_handoff.status, "open");
    assert_eq!(initial_inbox[0].message_count, 0);
    assert_eq!(initial_inbox[0].unread_count, 0);

    service
        .apply(&handoff_accepted)
        .expect("handoff accepted projection should succeed");

    let accepted_summary = service
        .conversation_summary("t_demo", "c_handoff_projection")
        .expect("handoff summary should still exist after accept");
    let accepted_handoff = accepted_summary
        .agent_handoff
        .as_ref()
        .expect("accepted summary should expose handoff state");
    assert_eq!(accepted_handoff.status, "accepted");
    assert_eq!(
        accepted_handoff
            .accepted_by
            .as_ref()
            .map(|actor| actor.id.as_str()),
        Some("u_member")
    );

    let accepted_inbox = service.inbox("t_demo", "u_member");
    let accepted_inbox_handoff = accepted_inbox[0]
        .agent_handoff
        .as_ref()
        .expect("accepted inbox should expose handoff state");
    assert_eq!(accepted_inbox_handoff.status, "accepted");
}

#[test]
fn test_agent_handoff_status_change_projects_device_sync_entries_for_active_members() {
    let service = TimelineProjectionService::default();

    let conversation_created = im_domain_events::CommitEnvelope::minimal(
        "evt_handoff_sync_created",
        "t_demo",
        "conversation.created",
        "conversation",
        "c_handoff_sync",
        1,
    )
    .with_payload(
        "conversation.created.v1",
        r#"{
            "conversationId":"c_handoff_sync",
            "conversationType":"agent_handoff",
            "source":{"id":"ag_source","kind":"agent"},
            "target":{"id":"u_member","kind":"user"},
            "handoff":{"sessionId":"hs_sync","reason":"manual_escalation","status":"open"}
        }"#,
    );
    let source_member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_handoff_sync_source_member",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_handoff_sync",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_handoff_sync",
            "memberId":"cm_handoff_sync_source",
            "principalId":"ag_source",
            "principalKind":"agent",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-06T11:00:00Z",
            "removedAt":null,
            "attributes":{"handoffRole":"source"}
        }"#,
    );
    let target_member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_handoff_sync_target_member",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_handoff_sync",
        3,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_handoff_sync",
            "memberId":"cm_handoff_sync_target",
            "principalId":"u_member",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"ag_source",
            "joinedAt":"2026-04-06T11:00:00Z",
            "removedAt":null,
            "attributes":{"handoffRole":"target"}
        }"#,
    );
    let handoff_accepted = im_domain_events::CommitEnvelope::minimal(
        "evt_handoff_sync_accepted",
        "t_demo",
        "conversation.agent_handoff_status_changed",
        "conversation",
        "c_handoff_sync",
        4,
    )
    .with_payload(
        "conversation.agent_handoff_status_changed.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_handoff_sync",
            "previousStatus":"open",
            "currentStatus":"accepted",
            "changedBy":{"id":"u_member","kind":"user"},
            "changedAt":"2026-04-06T11:01:00Z",
            "state":{
                "tenantId":"t_demo",
                "conversationId":"c_handoff_sync",
                "status":"accepted",
                "source":{"id":"ag_source","kind":"agent"},
                "target":{"id":"u_member","kind":"user"},
                "handoffSessionId":"hs_sync",
                "handoffReason":"manual_escalation",
                "acceptedAt":"2026-04-06T11:01:00Z",
                "acceptedBy":{"id":"u_member","kind":"user"},
                "resolvedAt":null,
                "resolvedBy":null,
                "closedAt":null,
                "closedBy":null
            }
        }"#,
    );

    service
        .apply(&conversation_created)
        .expect("conversation created projection should succeed");
    service
        .apply(&source_member_joined)
        .expect("source member projection should succeed");
    service
        .apply(&target_member_joined)
        .expect("target member projection should succeed");
    service.register_device("t_demo", "u_member", "d_pad");
    service.register_device("t_demo", "ag_source", "d_agent");
    service
        .apply(&handoff_accepted)
        .expect("handoff accepted projection should succeed");

    let target_feed = service.device_sync_feed("t_demo", "u_member", "d_pad", Some(0));
    assert_eq!(target_feed.len(), 1);
    assert_eq!(
        target_feed[0].origin_event_type,
        "conversation.agent_handoff_status_changed"
    );
    assert_eq!(
        target_feed[0].conversation_id.as_deref(),
        Some("c_handoff_sync")
    );
    assert_eq!(target_feed[0].actor_id.as_deref(), Some("u_member"));
    assert_eq!(target_feed[0].summary.as_deref(), Some("accepted"));
    assert_eq!(
        target_feed[0].payload_schema.as_deref(),
        Some("conversation.agent_handoff_status_changed.v1")
    );
    let target_payload: serde_json::Value = serde_json::from_str(
        target_feed[0]
            .payload
            .as_deref()
            .expect("target payload should be present"),
    )
    .expect("target payload should be valid json");
    assert_eq!(target_payload["conversationId"], "c_handoff_sync");
    assert_eq!(target_payload["currentStatus"], "accepted");
    assert_eq!(target_payload["changedBy"]["id"], "u_member");
    assert_eq!(target_payload["state"]["status"], "accepted");
    assert_eq!(target_feed[0].message_id, None);
    assert_eq!(target_feed[0].read_seq, None);
    assert_eq!(target_feed[0].occurred_at, "2026-04-06T11:01:00Z");

    let source_feed = service.device_sync_feed("t_demo", "ag_source", "d_agent", Some(0));
    assert_eq!(source_feed.len(), 1);
    assert_eq!(source_feed[0].actor_id.as_deref(), Some("u_member"));
    assert_eq!(source_feed[0].summary.as_deref(), Some("accepted"));
    assert_eq!(
        source_feed[0].payload_schema.as_deref(),
        Some("conversation.agent_handoff_status_changed.v1")
    );
    let source_payload: serde_json::Value = serde_json::from_str(
        source_feed[0]
            .payload
            .as_deref()
            .expect("source payload should be present"),
    )
    .expect("source payload should be valid json");
    assert_eq!(source_payload["conversationId"], "c_handoff_sync");
    assert_eq!(source_payload["currentStatus"], "accepted");
    assert_eq!(source_payload["changedBy"]["id"], "u_member");
}
