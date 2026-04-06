use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use im_auth_context::AuthContext;
use im_domain_events::{AggregateType, CommitEnvelope};
use im_platform_contracts::{CommitJournal, CommitPosition, ContractError};

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
