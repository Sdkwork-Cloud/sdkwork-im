use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use im_adapters_local_memory::MemoryNotificationTaskStore;
use im_auth_context::AuthContext;
use im_domain_events::CommitEnvelope;
use im_platform_contracts::{CommitJournal, CommitPosition, ContractError};

#[derive(Clone, Default)]
struct RecordingJournal {
    events: Arc<Mutex<Vec<CommitEnvelope>>>,
}

impl CommitJournal for RecordingJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let mut events = self.events.lock().expect("journal should lock");
        events.push(envelope);
        Ok(CommitPosition::new("p0", events.len() as u64))
    }
}

#[test]
fn test_runtime_restores_notification_projection_on_rebuild_with_shared_store() {
    let journal = Arc::new(RecordingJournal::default());
    let task_store = Arc::new(MemoryNotificationTaskStore::default());
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: None,
        permissions: BTreeSet::new(),
    };

    let runtime_before = notification_service::NotificationRuntime::with_journal_and_store(
        journal.clone(),
        task_store.clone(),
    );

    runtime_before
        .request_notification(
            &auth,
            notification_service::RequestNotification {
                notification_id: "ntf_rebuild".into(),
                source_event_id: "evt_notification_rebuild".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "u_demo".into(),
                title: Some("hello".into()),
                body: Some("world".into()),
                payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("notification request should succeed");

    let runtime_after =
        notification_service::NotificationRuntime::with_journal_and_store(journal, task_store);

    let items = runtime_after
        .list_notifications(&auth)
        .expect("notifications should restore after rebuild");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].notification_id, "ntf_rebuild");
    assert_eq!(items[0].status.as_str(), "dispatched");
}
