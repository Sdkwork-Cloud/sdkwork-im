use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

use im_domain_events::CommitEnvelope;
use im_platform_contracts::{
    AutomationExecutionRecord, AutomationExecutionStore, CommitJournal, CommitPosition,
    ContractError, MetadataStore, NotificationTaskRecord, NotificationTaskStore,
    PresenceStateRecord, PresenceStateStore, RealtimeCheckpointRecord, RealtimeCheckpointStore,
    RealtimeDisconnectFenceRecord, RealtimeDisconnectFenceStore, RealtimeSubscriptionRecord,
    RealtimeSubscriptionStore, RtcStateRecord, RtcStateStore, StreamStateRecord, StreamStateStore,
    TimelineProjectionStore,
};

#[derive(Clone)]
pub struct MemoryCommitJournal {
    partition: Arc<String>,
    events: Arc<Mutex<Vec<CommitEnvelope>>>,
}

impl Default for MemoryCommitJournal {
    fn default() -> Self {
        Self::with_partition("local-memory")
    }
}

impl MemoryCommitJournal {
    pub fn with_partition(partition: impl Into<String>) -> Self {
        Self {
            partition: Arc::new(partition.into()),
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn recorded(&self) -> Vec<CommitEnvelope> {
        self.events.lock().expect("journal should lock").clone()
    }
}

impl CommitJournal for MemoryCommitJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let mut events = self.events.lock().expect("journal should lock");
        events.push(envelope);
        Ok(CommitPosition::new(
            self.partition.as_str(),
            events.len() as u64,
        ))
    }
}

#[derive(Clone, Default)]
pub struct MemoryMetadataStore {
    snapshots: Arc<Mutex<HashMap<String, String>>>,
}

impl MemoryMetadataStore {
    pub fn snapshot(&self, scope: &str, key: &str) -> Option<String> {
        self.snapshots
            .lock()
            .expect("metadata store should lock")
            .get(snapshot_key(scope, key).as_str())
            .cloned()
    }
}

impl MetadataStore for MemoryMetadataStore {
    fn put_snapshot(&self, scope: &str, key: &str, value: &str) -> Result<(), ContractError> {
        self.snapshots
            .lock()
            .expect("metadata store should lock")
            .insert(snapshot_key(scope, key), value.to_string());
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct MemoryRealtimeCheckpointStore {
    checkpoints: Arc<Mutex<HashMap<String, RealtimeCheckpointRecord>>>,
}

impl MemoryRealtimeCheckpointStore {
    pub fn checkpoint(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Option<RealtimeCheckpointRecord> {
        self.checkpoints
            .lock()
            .expect("realtime checkpoint store should lock")
            .get(device_scope_key(tenant_id, principal_id, device_id).as_str())
            .cloned()
    }
}

impl RealtimeCheckpointStore for MemoryRealtimeCheckpointStore {
    fn load_checkpoint(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        Ok(self.checkpoint(tenant_id, principal_id, device_id))
    }

    fn save_checkpoint(&self, record: RealtimeCheckpointRecord) -> Result<(), ContractError> {
        self.checkpoints
            .lock()
            .expect("realtime checkpoint store should lock")
            .insert(
                device_scope_key(
                    record.tenant_id.as_str(),
                    record.principal_id.as_str(),
                    record.device_id.as_str(),
                ),
                record,
            );
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct MemoryRealtimeDisconnectFenceStore {
    fences: Arc<Mutex<HashMap<String, RealtimeDisconnectFenceRecord>>>,
}

impl MemoryRealtimeDisconnectFenceStore {
    pub fn fence(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Option<RealtimeDisconnectFenceRecord> {
        self.fences
            .lock()
            .expect("realtime disconnect fence store should lock")
            .get(device_scope_key(tenant_id, principal_id, device_id).as_str())
            .cloned()
    }
}

impl RealtimeDisconnectFenceStore for MemoryRealtimeDisconnectFenceStore {
    fn load_fence(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeDisconnectFenceRecord>, ContractError> {
        Ok(self.fence(tenant_id, principal_id, device_id))
    }

    fn save_fence(&self, record: RealtimeDisconnectFenceRecord) -> Result<(), ContractError> {
        self.fences
            .lock()
            .expect("realtime disconnect fence store should lock")
            .insert(
                device_scope_key(
                    record.tenant_id.as_str(),
                    record.principal_id.as_str(),
                    record.device_id.as_str(),
                ),
                record,
            );
        Ok(())
    }

    fn clear_fence(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        Ok(self
            .fences
            .lock()
            .expect("realtime disconnect fence store should lock")
            .remove(device_scope_key(tenant_id, principal_id, device_id).as_str())
            .is_some())
    }
}

#[derive(Clone, Default)]
pub struct MemoryRealtimeSubscriptionStore {
    subscriptions: Arc<Mutex<HashMap<String, RealtimeSubscriptionRecord>>>,
}

impl MemoryRealtimeSubscriptionStore {
    pub fn subscriptions(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Option<RealtimeSubscriptionRecord> {
        self.subscriptions
            .lock()
            .expect("realtime subscription store should lock")
            .get(device_scope_key(tenant_id, principal_id, device_id).as_str())
            .cloned()
    }
}

impl RealtimeSubscriptionStore for MemoryRealtimeSubscriptionStore {
    fn load_subscriptions(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeSubscriptionRecord>, ContractError> {
        Ok(self.subscriptions(tenant_id, principal_id, device_id))
    }

    fn save_subscriptions(&self, record: RealtimeSubscriptionRecord) -> Result<(), ContractError> {
        self.subscriptions
            .lock()
            .expect("realtime subscription store should lock")
            .insert(
                device_scope_key(
                    record.tenant_id.as_str(),
                    record.principal_id.as_str(),
                    record.device_id.as_str(),
                ),
                record,
            );
        Ok(())
    }

    fn clear_subscriptions(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        Ok(self
            .subscriptions
            .lock()
            .expect("realtime subscription store should lock")
            .remove(device_scope_key(tenant_id, principal_id, device_id).as_str())
            .is_some())
    }
}

#[derive(Clone, Default)]
pub struct MemoryStreamStateStore {
    states: Arc<Mutex<HashMap<String, StreamStateRecord>>>,
}

impl MemoryStreamStateStore {
    pub fn state(&self, tenant_id: &str, stream_id: &str) -> Option<StreamStateRecord> {
        self.states
            .lock()
            .expect("stream state store should lock")
            .get(stream_scope_key(tenant_id, stream_id).as_str())
            .cloned()
    }
}

#[derive(Clone, Default)]
pub struct MemoryRtcStateStore {
    states: Arc<Mutex<HashMap<String, RtcStateRecord>>>,
}

impl MemoryRtcStateStore {
    pub fn state(&self, tenant_id: &str, rtc_session_id: &str) -> Option<RtcStateRecord> {
        self.states
            .lock()
            .expect("rtc state store should lock")
            .get(rtc_scope_key(tenant_id, rtc_session_id).as_str())
            .cloned()
    }
}

impl RtcStateStore for MemoryRtcStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<Option<RtcStateRecord>, ContractError> {
        Ok(self.state(tenant_id, rtc_session_id))
    }

    fn save_state(&self, record: RtcStateRecord) -> Result<(), ContractError> {
        self.states
            .lock()
            .expect("rtc state store should lock")
            .insert(
                rtc_scope_key(record.tenant_id.as_str(), record.rtc_session_id.as_str()),
                record,
            );
        Ok(())
    }

    fn clear_state(&self, tenant_id: &str, rtc_session_id: &str) -> Result<bool, ContractError> {
        Ok(self
            .states
            .lock()
            .expect("rtc state store should lock")
            .remove(rtc_scope_key(tenant_id, rtc_session_id).as_str())
            .is_some())
    }
}

impl StreamStateStore for MemoryStreamStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        stream_id: &str,
    ) -> Result<Option<StreamStateRecord>, ContractError> {
        Ok(self.state(tenant_id, stream_id))
    }

    fn save_state(&self, record: StreamStateRecord) -> Result<(), ContractError> {
        self.states
            .lock()
            .expect("stream state store should lock")
            .insert(
                stream_scope_key(record.tenant_id.as_str(), record.stream_id.as_str()),
                record,
            );
        Ok(())
    }

    fn clear_state(&self, tenant_id: &str, stream_id: &str) -> Result<bool, ContractError> {
        Ok(self
            .states
            .lock()
            .expect("stream state store should lock")
            .remove(stream_scope_key(tenant_id, stream_id).as_str())
            .is_some())
    }
}

#[derive(Clone, Default)]
pub struct MemoryNotificationTaskStore {
    tasks: Arc<Mutex<HashMap<String, NotificationTaskRecord>>>,
}

impl MemoryNotificationTaskStore {
    pub fn task(&self, tenant_id: &str, notification_id: &str) -> Option<NotificationTaskRecord> {
        self.tasks
            .lock()
            .expect("notification task store should lock")
            .get(notification_scope_key(tenant_id, notification_id).as_str())
            .cloned()
    }
}

impl NotificationTaskStore for MemoryNotificationTaskStore {
    fn load_task(
        &self,
        tenant_id: &str,
        notification_id: &str,
    ) -> Result<Option<NotificationTaskRecord>, ContractError> {
        Ok(self.task(tenant_id, notification_id))
    }

    fn save_task(&self, record: NotificationTaskRecord) -> Result<(), ContractError> {
        self.tasks
            .lock()
            .expect("notification task store should lock")
            .insert(
                notification_scope_key(record.tenant_id.as_str(), record.notification_id.as_str()),
                record,
            );
        Ok(())
    }

    fn list_tasks_for_recipient(
        &self,
        tenant_id: &str,
        recipient_id: &str,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError> {
        Ok(self
            .tasks
            .lock()
            .expect("notification task store should lock")
            .values()
            .filter(|record| {
                record.tenant_id == tenant_id && record.task.recipient_id == recipient_id
            })
            .cloned()
            .collect())
    }
}

#[derive(Clone, Default)]
pub struct MemoryAutomationExecutionStore {
    executions: Arc<Mutex<HashMap<String, AutomationExecutionRecord>>>,
}

impl MemoryAutomationExecutionStore {
    pub fn execution(
        &self,
        tenant_id: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Option<AutomationExecutionRecord> {
        self.executions
            .lock()
            .expect("automation execution store should lock")
            .get(execution_scope_key(tenant_id, principal_id, execution_id).as_str())
            .cloned()
    }
}

impl AutomationExecutionStore for MemoryAutomationExecutionStore {
    fn load_execution(
        &self,
        tenant_id: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Result<Option<AutomationExecutionRecord>, ContractError> {
        Ok(self.execution(tenant_id, principal_id, execution_id))
    }

    fn save_execution(&self, record: AutomationExecutionRecord) -> Result<(), ContractError> {
        self.executions
            .lock()
            .expect("automation execution store should lock")
            .insert(
                execution_scope_key(
                    record.tenant_id.as_str(),
                    record.principal_id.as_str(),
                    record.execution_id.as_str(),
                ),
                record,
            );
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct MemoryPresenceStateStore {
    states: Arc<Mutex<HashMap<String, PresenceStateRecord>>>,
}

impl MemoryPresenceStateStore {
    pub fn state(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Option<PresenceStateRecord> {
        self.states
            .lock()
            .expect("presence state store should lock")
            .get(device_scope_key(tenant_id, principal_id, device_id).as_str())
            .cloned()
    }
}

impl PresenceStateStore for MemoryPresenceStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError> {
        Ok(self.state(tenant_id, principal_id, device_id))
    }

    fn save_state(&self, record: PresenceStateRecord) -> Result<(), ContractError> {
        self.states
            .lock()
            .expect("presence state store should lock")
            .insert(
                device_scope_key(
                    record.tenant_id.as_str(),
                    record.principal_id.as_str(),
                    record.device_id.as_str(),
                ),
                record,
            );
        Ok(())
    }

    fn list_states_for_principal(
        &self,
        tenant_id: &str,
        principal_id: &str,
    ) -> Result<Vec<PresenceStateRecord>, ContractError> {
        Ok(self
            .states
            .lock()
            .expect("presence state store should lock")
            .values()
            .filter(|record| record.tenant_id == tenant_id && record.principal_id == principal_id)
            .cloned()
            .collect())
    }
}

#[derive(Clone, Default)]
pub struct MemoryTimelineProjectionStore {
    entries: Arc<Mutex<HashMap<String, BTreeMap<u64, String>>>>,
}

impl MemoryTimelineProjectionStore {
    pub fn entries(&self, conversation_id: &str) -> Vec<(u64, String)> {
        self.entries
            .lock()
            .expect("timeline projection store should lock")
            .get(conversation_id)
            .map(|items| {
                items
                    .iter()
                    .map(|(message_seq, payload)| (*message_seq, payload.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl TimelineProjectionStore for MemoryTimelineProjectionStore {
    fn upsert_timeline_entry(
        &self,
        conversation_id: &str,
        message_seq: u64,
        payload: &str,
    ) -> Result<(), ContractError> {
        self.entries
            .lock()
            .expect("timeline projection store should lock")
            .entry(conversation_id.to_string())
            .or_default()
            .insert(message_seq, payload.to_string());
        Ok(())
    }
}

fn snapshot_key(scope: &str, key: &str) -> String {
    format!("{scope}:{key}")
}

fn device_scope_key(tenant_id: &str, principal_id: &str, device_id: &str) -> String {
    format!("{tenant_id}:{principal_id}:{device_id}")
}

fn stream_scope_key(tenant_id: &str, stream_id: &str) -> String {
    format!("{tenant_id}:{stream_id}")
}

fn rtc_scope_key(tenant_id: &str, rtc_session_id: &str) -> String {
    format!("{tenant_id}:{rtc_session_id}")
}

fn notification_scope_key(tenant_id: &str, notification_id: &str) -> String {
    format!("{tenant_id}:{notification_id}")
}

fn execution_scope_key(tenant_id: &str, principal_id: &str, execution_id: &str) -> String {
    format!("{tenant_id}:{principal_id}:{execution_id}")
}
