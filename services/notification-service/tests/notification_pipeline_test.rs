use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use im_auth_context::AuthContext;
use im_domain_events::{AggregateType, CommitEnvelope};
use im_platform_contracts::{CommitJournal, CommitPosition, ContractError};
use projection_service::TimelineProjectionService;

#[derive(Clone, Default)]
struct RecordingJournal {
    events: Arc<Mutex<Vec<CommitEnvelope>>>,
}

impl RecordingJournal {
    fn recorded(&self) -> Vec<CommitEnvelope> {
        self.events.lock().expect("journal should lock").clone()
    }
}

impl CommitJournal for RecordingJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let mut events = self.events.lock().expect("journal should lock");
        events.push(envelope);
        Ok(CommitPosition::new("p0", events.len() as u64))
    }
}

#[test]
fn test_request_notification_appends_requested_and_dispatched_events() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = notification_service::NotificationRuntime::with_journal(journal.clone());
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: None,
        permissions: BTreeSet::new(),
    };

    let task = runtime
        .request_notification(
            &auth,
            notification_service::RequestNotification {
                notification_id: "ntf_demo".into(),
                source_event_id: "evt_message_1".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "u_target".into(),
                title: Some("New message".into()),
                body: Some("hello".into()),
                payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("notification request should succeed");

    assert_eq!(task.notification_id, "ntf_demo");
    assert_eq!(task.status.as_str(), "dispatched");

    let events = journal.recorded();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].event_type, "notification.requested");
    assert_eq!(events[1].event_type, "notification.dispatched");
    assert_eq!(events[0].aggregate_type, AggregateType::Notification);
    assert_eq!(events[0].aggregate_id, "ntf_demo");
    assert_eq!(events[0].actor.actor_id, "u_demo");
    assert_eq!(events[0].actor.actor_session_id.as_deref(), Some("s_demo"));

    let payload: serde_json::Value =
        serde_json::from_str(&events[1].payload).expect("payload should be valid json");
    assert_eq!(payload["notificationId"], "ntf_demo");
    assert_eq!(payload["recipientId"], "u_target");
    assert_eq!(payload["status"], "dispatched");
}

#[test]
fn test_duplicate_request_notification_is_idempotent_when_payload_matches() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = notification_service::NotificationRuntime::with_journal(journal.clone());
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: None,
        permissions: BTreeSet::new(),
    };

    let first = runtime
        .request_notification(
            &auth,
            notification_service::RequestNotification {
                notification_id: "ntf_idempotent".into(),
                source_event_id: "evt_notification_1".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "u_target".into(),
                title: Some("New message".into()),
                body: Some("hello".into()),
                payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("first notification request should succeed");
    let second = runtime
        .request_notification(
            &auth,
            notification_service::RequestNotification {
                notification_id: "ntf_idempotent".into(),
                source_event_id: "evt_notification_1".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "u_target".into(),
                title: Some("New message".into()),
                body: Some("hello".into()),
                payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("duplicate notification request should be idempotent");

    assert_eq!(second, first);

    let events = journal.recorded();
    assert_eq!(events.len(), 2);
}

#[test]
fn test_duplicate_request_notification_rejects_conflicting_payload() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = notification_service::NotificationRuntime::with_journal(journal.clone());
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: None,
        permissions: BTreeSet::new(),
    };

    runtime
        .request_notification(
            &auth,
            notification_service::RequestNotification {
                notification_id: "ntf_conflict".into(),
                source_event_id: "evt_notification_1".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "u_target".into(),
                title: Some("New message".into()),
                body: Some("hello".into()),
                payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("first notification request should succeed");

    let error = runtime
        .request_notification(
            &auth,
            notification_service::RequestNotification {
                notification_id: "ntf_conflict".into(),
                source_event_id: "evt_notification_2".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "u_other".into(),
                title: Some("Changed message".into()),
                body: Some("different".into()),
                payload: Some(r#"{"conversationId":"c_other"}"#.into()),
            },
        )
        .expect_err("conflicting duplicate should be rejected");
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::CONFLICT);

    let events = journal.recorded();
    assert_eq!(events.len(), 2);
}

#[test]
fn test_request_notification_fanout_skips_actor_and_creates_notifications_for_other_recipients() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = notification_service::NotificationRuntime::with_journal(journal.clone());
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_owner".into(),
        actor_kind: "user".into(),
        session_id: Some("s_owner".into()),
        device_id: None,
        permissions: BTreeSet::new(),
    };

    let tasks = runtime
        .request_notification_fanout(
            &auth,
            notification_service::RequestNotificationFanout {
                notification_id_seed: "msg_c_demo_1".into(),
                source_event_id: "evt_message_1".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_ids: BTreeSet::from([
                    "u_owner".to_string(),
                    "u_member_a".to_string(),
                    "u_member_b".to_string(),
                ]),
                title: Some("hello".into()),
                body: Some("hello".into()),
                payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("notification fanout should succeed");

    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].recipient_id, "u_member_a");
    assert_eq!(tasks[1].recipient_id, "u_member_b");

    let owner_notifications = runtime
        .list_notifications(&auth)
        .expect("owner notifications should list");
    assert!(owner_notifications.is_empty());

    let member_a_auth = AuthContext {
        actor_id: "u_member_a".into(),
        ..auth.clone()
    };
    let member_a_notifications = runtime
        .list_notifications(&member_a_auth)
        .expect("member a notifications should list");
    assert_eq!(member_a_notifications.len(), 1);
    assert_eq!(
        member_a_notifications[0].notification_id,
        "ntf_msg_c_demo_1_u_member_a"
    );

    let member_b_auth = AuthContext {
        actor_id: "u_member_b".into(),
        ..auth
    };
    let member_b_notifications = runtime
        .list_notifications(&member_b_auth)
        .expect("member b notifications should list");
    assert_eq!(member_b_notifications.len(), 1);
    assert_eq!(
        member_b_notifications[0].notification_id,
        "ntf_msg_c_demo_1_u_member_b"
    );

    let events = journal.recorded();
    assert_eq!(events.len(), 4);
    assert_eq!(events[0].event_type, "notification.requested");
    assert_eq!(events[1].event_type, "notification.dispatched");
    assert_eq!(events[2].event_type, "notification.requested");
    assert_eq!(events[3].event_type, "notification.dispatched");
}

#[test]
fn test_request_message_posted_notifications_resolves_current_active_recipients_from_projection_auth_context()
 {
    let journal = Arc::new(RecordingJournal::default());
    let projection_service = Arc::new(TimelineProjectionService::default());
    let runtime = notification_service::NotificationRuntime::with_journal_and_projection(
        journal.clone(),
        projection_service.clone(),
    );
    let owner_joined = CommitEnvelope::minimal(
        "evt_notification_owner_joined",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_demo",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_demo",
            "memberId":"cm_demo_owner",
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
    let member_joined = CommitEnvelope::minimal(
        "evt_notification_member_joined",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_demo",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_demo",
            "memberId":"cm_demo_member",
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
    let removed_joined = CommitEnvelope::minimal(
        "evt_notification_removed_joined",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_demo",
        3,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_demo",
            "memberId":"cm_demo_removed",
            "principalId":"u_removed",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"u_owner",
            "joinedAt":"2026-04-07T10:02:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let removed_member = CommitEnvelope::minimal(
        "evt_notification_member_removed",
        "t_demo",
        "conversation.member_removed",
        "conversation",
        "c_demo",
        4,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_demo",
            "memberId":"cm_demo_removed",
            "principalId":"u_removed",
            "principalKind":"user",
            "role":"member",
            "state":"removed",
            "invitedBy":"u_owner",
            "joinedAt":"2026-04-07T10:02:00Z",
            "removedAt":"2026-04-07T10:03:00Z",
            "attributes":{}
        }"#,
    );
    for event in [owner_joined, member_joined, removed_joined, removed_member] {
        projection_service
            .apply(&event)
            .expect("projection should accept conversation membership event");
    }
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_owner".into(),
        actor_kind: "user".into(),
        session_id: Some("s_owner".into()),
        device_id: None,
        permissions: BTreeSet::new(),
    };

    let tasks = runtime
        .request_message_posted_notifications(
            &auth,
            notification_service::RequestMessagePostedNotifications {
                source_event_id: "evt_message_1".into(),
                conversation_id: "c_demo".into(),
                message_id: "msg_c_demo_1".into(),
                message_seq: 1,
                message_type: "text".into(),
                summary: Some("hello member".into()),
            },
        )
        .expect("message-posted notifications should succeed");

    assert_eq!(tasks.len(), 1);
    let task = &tasks[0];
    assert_eq!(task.notification_id, "ntf_msg_c_demo_1_u_member");
    assert_eq!(task.source_event_id, "evt_message_1");
    assert_eq!(task.source_event_type, "message.posted");
    assert_eq!(task.category, "message.new");
    assert_eq!(task.channel, "inapp");
    assert_eq!(task.recipient_id, "u_member");
    assert_eq!(task.title.as_deref(), Some("hello member"));
    assert_eq!(task.body.as_deref(), Some("hello member"));
    assert_eq!(
        task.payload.as_deref(),
        Some(
            r#"{"conversationId":"c_demo","messageId":"msg_c_demo_1","messageSeq":1,"messageType":"text"}"#
        )
    );

    let events = journal.recorded();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].event_type, "notification.requested");
    assert_eq!(events[1].event_type, "notification.dispatched");
}

#[test]
fn test_request_message_posted_notifications_includes_shared_linked_recipients_from_projection() {
    let journal = Arc::new(RecordingJournal::default());
    let projection_service = Arc::new(TimelineProjectionService::default());
    let runtime = notification_service::NotificationRuntime::with_journal_and_projection(
        journal.clone(),
        projection_service.clone(),
    );
    let owner_joined = CommitEnvelope::minimal(
        "evt_notification_shared_owner_joined",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_shared_notification",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_shared_notification",
            "memberId":"cm_shared_notification_owner",
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
    let member_joined = CommitEnvelope::minimal(
        "evt_notification_shared_member_joined",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_shared_notification",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_shared_notification",
            "memberId":"cm_shared_notification_member",
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
    let shared_linked = CommitEnvelope::minimal(
        "evt_notification_shared_linked",
        "t_demo",
        "conversation.member_joined",
        "conversation",
        "c_shared_notification",
        3,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"t_demo",
            "conversationId":"c_shared_notification",
            "memberId":"cm_shared_notification_external",
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
        projection_service
            .apply(&event)
            .expect("projection should accept shared notification membership event");
    }
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_owner".into(),
        actor_kind: "user".into(),
        session_id: Some("s_owner".into()),
        device_id: None,
        permissions: BTreeSet::new(),
    };

    let tasks = runtime
        .request_message_posted_notifications(
            &auth,
            notification_service::RequestMessagePostedNotifications {
                source_event_id: "evt_shared_message_1".into(),
                conversation_id: "c_shared_notification".into(),
                message_id: "msg_c_shared_notification_1".into(),
                message_seq: 1,
                message_type: "text".into(),
                summary: Some("hello shared member".into()),
            },
        )
        .expect("message-posted notifications should include shared linked members");

    assert_eq!(tasks.len(), 2);
    let recipient_ids = tasks
        .iter()
        .map(|task| task.recipient_id.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        recipient_ids,
        BTreeSet::from(["u_member", "u_shared_external"])
    );

    let events = journal.recorded();
    assert_eq!(events.len(), 4);
    assert_eq!(events[0].event_type, "notification.requested");
    assert_eq!(events[1].event_type, "notification.dispatched");
    assert_eq!(events[2].event_type, "notification.requested");
    assert_eq!(events[3].event_type, "notification.dispatched");
}

#[test]
fn test_request_automation_result_notification_targets_requesting_actor_idempotently() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = notification_service::NotificationRuntime::with_journal(journal.clone());
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: None,
        permissions: BTreeSet::new(),
    };

    let first = runtime
        .request_automation_result_notification(
            &auth,
            notification_service::RequestAutomationResultNotification {
                execution_id: "ae_demo".into(),
                target_ref: "wf_demo".into(),
                output_payload: Some(r#"{"status":"ok"}"#.into()),
            },
        )
        .expect("automation result notification should succeed");
    let second = runtime
        .request_automation_result_notification(
            &auth,
            notification_service::RequestAutomationResultNotification {
                execution_id: "ae_demo".into(),
                target_ref: "wf_demo".into(),
                output_payload: Some(r#"{"status":"ok"}"#.into()),
            },
        )
        .expect("duplicate automation result notification should be idempotent");

    assert_eq!(second, first);
    assert_eq!(first.notification_id, "ntf_automation_ae_demo");
    assert_eq!(first.source_event_type, "automation.execution_completed");
    assert_eq!(first.category, "automation.result");
    assert_eq!(first.channel, "inapp");
    assert_eq!(first.recipient_id, "u_demo");
    assert_eq!(first.title.as_deref(), Some("Automation completed"));
    assert_eq!(first.body.as_deref(), Some("wf_demo"));
    assert_eq!(first.payload.as_deref(), Some(r#"{"status":"ok"}"#));

    let notifications = runtime
        .list_notifications(&auth)
        .expect("automation notifications should list");
    assert_eq!(notifications.len(), 1);
    assert_eq!(notifications[0], first);

    let events = journal.recorded();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].event_type, "notification.requested");
    assert_eq!(events[1].event_type, "notification.dispatched");
}
