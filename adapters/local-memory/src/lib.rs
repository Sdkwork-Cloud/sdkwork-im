use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::{Arc, Mutex};

use im_domain_events::CommitEnvelope;
use im_platform_contracts::{
    AutomationExecutionRecord, AutomationExecutionStore, CommitJournal, CommitPosition,
    ContractError, DeviceTwinRecord, DeviceTwinStore, MetadataSnapshotRecord, MetadataStore,
    NotificationTaskRecord, NotificationTaskStore, PresenceStateRecord, PresenceStateStore,
    RealtimeCheckpointRecord, RealtimeCheckpointStore, RealtimeDisconnectFenceRecord,
    RealtimeDisconnectFenceStore, RealtimeEventWindowDiagnosticsSnapshot,
    RealtimeEventWindowRecord, RealtimeEventWindowStore, RealtimeMatchingSubscriptionQuery,
    RealtimeSubscriptionRecord, RealtimeSubscriptionStore, RtcStateRecord, RtcStateStore,
    StreamStateRecord, StreamStateStore, TimelineProjectionBatch, TimelineProjectionRecord,
    TimelineProjectionStore,
};
use im_storage_contracts::{StorageDomainSnapshot, StorageDomainSnapshotStore};

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

    fn append_batch(
        &self,
        envelopes: Vec<CommitEnvelope>,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        let mut events = self.events.lock().expect("journal should lock");
        let start_offset = events.len() as u64 + 1;
        let batch_len = envelopes.len() as u64;
        events.extend(envelopes);
        Ok((0..batch_len)
            .map(|index| CommitPosition::new(self.partition.as_str(), start_offset + index))
            .collect())
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

    fn load_snapshot(&self, scope: &str, key: &str) -> Result<Option<String>, ContractError> {
        Ok(self.snapshot(scope, key))
    }

    fn put_snapshots(&self, snapshots: &[MetadataSnapshotRecord]) -> Result<(), ContractError> {
        let mut stored = self.snapshots.lock().expect("metadata store should lock");
        for snapshot in snapshots {
            stored.insert(
                snapshot_key(snapshot.scope.as_str(), snapshot.key.as_str()),
                snapshot.value.clone(),
            );
        }
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct MemoryStorageDomainSnapshotStore {
    snapshots: Arc<Mutex<HashMap<String, StorageDomainSnapshot>>>,
}

impl MemoryStorageDomainSnapshotStore {
    pub fn snapshot(&self, domain: &str) -> Option<StorageDomainSnapshot> {
        self.snapshots
            .lock()
            .expect("storage snapshot store should lock")
            .get(domain)
            .cloned()
    }
}

impl StorageDomainSnapshotStore for MemoryStorageDomainSnapshotStore {
    fn load_snapshot(&self, domain: &str) -> Result<Option<StorageDomainSnapshot>, ContractError> {
        Ok(self.snapshot(domain))
    }

    fn save_snapshot(&self, snapshot: StorageDomainSnapshot) -> Result<(), ContractError> {
        self.snapshots
            .lock()
            .expect("storage snapshot store should lock")
            .insert(snapshot.catalog.domain.clone(), snapshot);
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
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Option<RealtimeCheckpointRecord> {
        self.checkpoints
            .lock()
            .expect("realtime checkpoint store should lock")
            .get(device_scope_key(tenant_id, principal_kind, principal_id, device_id).as_str())
            .cloned()
    }
}

impl RealtimeCheckpointStore for MemoryRealtimeCheckpointStore {
    fn load_checkpoint(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        Ok(self.checkpoint(tenant_id, principal_kind, principal_id, device_id))
    }

    fn save_checkpoints(
        &self,
        records: Vec<RealtimeCheckpointRecord>,
    ) -> Result<(), ContractError> {
        let mut checkpoints = self
            .checkpoints
            .lock()
            .expect("realtime checkpoint store should lock");
        for record in records {
            let key = device_scope_key(
                record.tenant_id.as_str(),
                record.principal_kind.as_str(),
                record.principal_id.as_str(),
                record.device_id.as_str(),
            );
            let next = checkpoints
                .remove(key.as_str())
                .map(|previous| previous.merge_monotonic(record.clone()))
                .unwrap_or_else(|| record.normalized());
            checkpoints.insert(key, next);
        }
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct MemoryRealtimeEventWindowStore {
    windows: Arc<Mutex<HashMap<String, RealtimeEventWindowRecord>>>,
}

impl MemoryRealtimeEventWindowStore {
    pub fn window(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Option<RealtimeEventWindowRecord> {
        self.windows
            .lock()
            .expect("realtime event window store should lock")
            .get(device_scope_key(tenant_id, principal_kind, principal_id, device_id).as_str())
            .cloned()
    }
}

impl RealtimeEventWindowStore for MemoryRealtimeEventWindowStore {
    fn load_window(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeEventWindowRecord>, ContractError> {
        Ok(self.window(tenant_id, principal_kind, principal_id, device_id))
    }

    fn save_windows(&self, records: Vec<RealtimeEventWindowRecord>) -> Result<(), ContractError> {
        let mut windows = self
            .windows
            .lock()
            .expect("realtime event window store should lock");
        for record in records {
            windows.insert(
                device_scope_key(
                    record.tenant_id.as_str(),
                    record.principal_kind.as_str(),
                    record.principal_id.as_str(),
                    record.device_id.as_str(),
                ),
                record.normalized(),
            );
        }
        Ok(())
    }

    fn clear_window(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        Ok(self
            .windows
            .lock()
            .expect("realtime event window store should lock")
            .remove(device_scope_key(tenant_id, principal_kind, principal_id, device_id).as_str())
            .is_some())
    }

    fn diagnostics_snapshot(
        &self,
    ) -> Result<RealtimeEventWindowDiagnosticsSnapshot, ContractError> {
        let windows = self
            .windows
            .lock()
            .expect("realtime event window store should lock");
        Ok(RealtimeEventWindowDiagnosticsSnapshot::from_records(
            windows.values().cloned(),
        ))
    }

    fn trim_window(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        acked_through_seq: u64,
    ) -> Result<(), ContractError> {
        let key = device_scope_key(tenant_id, principal_kind, principal_id, device_id);
        if let Some(record) = self
            .windows
            .lock()
            .expect("realtime event window store should lock")
            .get_mut(key.as_str())
        {
            record.trimmed_through_seq = record.trimmed_through_seq.max(acked_through_seq);
            record
                .events
                .retain(|event| event.realtime_seq > record.trimmed_through_seq);
        }
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct MemoryDeviceTwinStore {
    twins: Arc<Mutex<HashMap<String, DeviceTwinRecord>>>,
}

impl MemoryDeviceTwinStore {
    pub fn twin(&self, tenant_id: &str, device_id: &str) -> Option<DeviceTwinRecord> {
        self.twins
            .lock()
            .expect("device twin store should lock")
            .get(device_twin_scope_key(tenant_id, device_id).as_str())
            .cloned()
    }
}

impl DeviceTwinStore for MemoryDeviceTwinStore {
    fn load_twin(
        &self,
        tenant_id: &str,
        device_id: &str,
    ) -> Result<Option<DeviceTwinRecord>, ContractError> {
        Ok(self.twin(tenant_id, device_id))
    }

    fn save_twin(&self, record: DeviceTwinRecord) -> Result<(), ContractError> {
        self.twins
            .lock()
            .expect("device twin store should lock")
            .insert(
                device_twin_scope_key(record.tenant_id.as_str(), record.device_id.as_str()),
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
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Option<RealtimeDisconnectFenceRecord> {
        self.fences
            .lock()
            .expect("realtime disconnect fence store should lock")
            .get(device_scope_key(tenant_id, principal_kind, principal_id, device_id).as_str())
            .cloned()
    }
}

impl RealtimeDisconnectFenceStore for MemoryRealtimeDisconnectFenceStore {
    fn load_fence(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeDisconnectFenceRecord>, ContractError> {
        Ok(self.fence(tenant_id, principal_kind, principal_id, device_id))
    }

    fn save_fence(&self, record: RealtimeDisconnectFenceRecord) -> Result<(), ContractError> {
        let key = device_scope_key(
            record.tenant_id.as_str(),
            record.principal_kind.as_str(),
            record.principal_id.as_str(),
            record.device_id.as_str(),
        );
        let mut fences = self
            .fences
            .lock()
            .expect("realtime disconnect fence store should lock");
        let next = fences
            .remove(key.as_str())
            .map(|previous| previous.merge_latest(record.clone()))
            .unwrap_or(record);
        fences.insert(key, next);
        Ok(())
    }

    fn clear_fence(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        Ok(self
            .fences
            .lock()
            .expect("realtime disconnect fence store should lock")
            .remove(device_scope_key(tenant_id, principal_kind, principal_id, device_id).as_str())
            .is_some())
    }

    fn clear_fence_disconnected_at_or_before(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        cutoff_disconnected_at: &str,
    ) -> Result<bool, ContractError> {
        let key = device_scope_key(tenant_id, principal_kind, principal_id, device_id);
        let mut fences = self
            .fences
            .lock()
            .expect("realtime disconnect fence store should lock");
        let should_clear = fences
            .get(key.as_str())
            .map(|record| record.disconnected_at.as_str() <= cutoff_disconnected_at)
            .unwrap_or(false);
        if !should_clear {
            return Ok(false);
        }
        Ok(fences.remove(key.as_str()).is_some())
    }

    fn clear_fence_if_matches(
        &self,
        expected: &RealtimeDisconnectFenceRecord,
    ) -> Result<bool, ContractError> {
        let key = device_scope_key(
            expected.tenant_id.as_str(),
            expected.principal_kind.as_str(),
            expected.principal_id.as_str(),
            expected.device_id.as_str(),
        );
        let mut fences = self
            .fences
            .lock()
            .expect("realtime disconnect fence store should lock");
        let should_clear = fences
            .get(key.as_str())
            .map(|record| record == expected)
            .unwrap_or(false);
        if !should_clear {
            return Ok(false);
        }
        Ok(fences.remove(key.as_str()).is_some())
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
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Option<RealtimeSubscriptionRecord> {
        self.subscriptions
            .lock()
            .expect("realtime subscription store should lock")
            .get(device_scope_key(tenant_id, principal_kind, principal_id, device_id).as_str())
            .cloned()
    }
}

impl RealtimeSubscriptionStore for MemoryRealtimeSubscriptionStore {
    fn load_subscriptions(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeSubscriptionRecord>, ContractError> {
        Ok(self.subscriptions(tenant_id, principal_kind, principal_id, device_id))
    }

    fn load_matching_subscriptions(
        &self,
        query: RealtimeMatchingSubscriptionQuery<'_>,
    ) -> Result<Vec<RealtimeSubscriptionRecord>, ContractError> {
        let subscriptions = self
            .subscriptions
            .lock()
            .expect("realtime subscription store should lock");
        Ok(query
            .candidate_device_ids
            .iter()
            .filter_map(|device_id| {
                subscriptions
                    .get(
                        device_scope_key(
                            query.tenant_id,
                            query.principal_kind,
                            query.principal_id,
                            device_id,
                        )
                        .as_str(),
                    )
                    .filter(|record| {
                        record.matches_scope_event(
                            query.scope_type,
                            query.scope_id,
                            query.event_type,
                        )
                    })
                    .cloned()
            })
            .collect())
    }

    fn save_subscriptions(&self, record: RealtimeSubscriptionRecord) -> Result<(), ContractError> {
        self.subscriptions
            .lock()
            .expect("realtime subscription store should lock")
            .insert(
                device_scope_key(
                    record.tenant_id.as_str(),
                    record.principal_kind.as_str(),
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
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        Ok(self
            .subscriptions
            .lock()
            .expect("realtime subscription store should lock")
            .remove(device_scope_key(tenant_id, principal_kind, principal_id, device_id).as_str())
            .is_some())
    }

    fn clear_subscriptions_synced_at_or_before(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        cutoff_synced_at: &str,
    ) -> Result<bool, ContractError> {
        let key = device_scope_key(tenant_id, principal_kind, principal_id, device_id);
        let mut subscriptions = self
            .subscriptions
            .lock()
            .expect("realtime subscription store should lock");
        let should_clear = subscriptions
            .get(key.as_str())
            .map(|record| record.synced_at.as_str() <= cutoff_synced_at)
            .unwrap_or(false);
        if !should_clear {
            return Ok(false);
        }
        Ok(subscriptions.remove(key.as_str()).is_some())
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
        let key = rtc_scope_key(record.tenant_id.as_str(), record.rtc_session_id.as_str());
        let mut states = self.states.lock().expect("rtc state store should lock");
        let next = states
            .remove(key.as_str())
            .map(|previous| previous.merge_monotonic(record.clone()))
            .unwrap_or(record);
        states.insert(key, next);
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
        let key = stream_scope_key(record.tenant_id.as_str(), record.stream_id.as_str());
        let mut states = self.states.lock().expect("stream state store should lock");
        let next = states
            .remove(key.as_str())
            .map(|previous| previous.merge_monotonic(record.clone()))
            .unwrap_or(record);
        states.insert(key, next);
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
    state: Arc<Mutex<MemoryNotificationTaskState>>,
}

#[derive(Default)]
struct MemoryNotificationTaskState {
    tasks: HashMap<String, NotificationTaskRecord>,
    tasks_by_recipient: HashMap<String, BTreeSet<String>>,
}

impl MemoryNotificationTaskStore {
    pub fn task(&self, tenant_id: &str, notification_id: &str) -> Option<NotificationTaskRecord> {
        self.state
            .lock()
            .expect("notification task store should lock")
            .tasks
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
        let notification_key =
            notification_scope_key(record.tenant_id.as_str(), record.notification_id.as_str());
        let mut state = self
            .state
            .lock()
            .expect("notification task store should lock");
        if let Some(previous) = state.tasks.get(notification_key.as_str()).cloned() {
            remove_notification_recipient_index(
                &mut state.tasks_by_recipient,
                notification_key.as_str(),
                &previous,
            );
            let merged = previous.merge_monotonic(record);
            insert_notification_recipient_index(
                &mut state.tasks_by_recipient,
                notification_key.as_str(),
                &merged,
            );
            state.tasks.insert(notification_key, merged);
            return Ok(());
        }
        insert_notification_recipient_index(
            &mut state.tasks_by_recipient,
            notification_key.as_str(),
            &record,
        );
        state.tasks.insert(notification_key, record);
        Ok(())
    }

    fn list_tasks_for_recipient(
        &self,
        tenant_id: &str,
        recipient_kind: &str,
        recipient_id: &str,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError> {
        let state = self
            .state
            .lock()
            .expect("notification task store should lock");
        let task_keys = state
            .tasks_by_recipient
            .get(notification_recipient_scope_key(tenant_id, recipient_kind, recipient_id).as_str())
            .cloned()
            .unwrap_or_default();
        Ok(task_keys
            .into_iter()
            .filter_map(|task_key| state.tasks.get(task_key.as_str()).cloned())
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
        principal_kind: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Option<AutomationExecutionRecord> {
        self.executions
            .lock()
            .expect("automation execution store should lock")
            .get(
                execution_scope_key(tenant_id, principal_kind, principal_id, execution_id).as_str(),
            )
            .cloned()
    }
}

impl AutomationExecutionStore for MemoryAutomationExecutionStore {
    fn load_execution(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Result<Option<AutomationExecutionRecord>, ContractError> {
        Ok(self.execution(tenant_id, principal_kind, principal_id, execution_id))
    }

    fn save_execution(&self, record: AutomationExecutionRecord) -> Result<(), ContractError> {
        let key = execution_scope_key(
            record.tenant_id.as_str(),
            record.execution.principal_kind.as_str(),
            record.principal_id.as_str(),
            record.execution_id.as_str(),
        );
        let mut executions = self
            .executions
            .lock()
            .expect("automation execution store should lock");
        let next = executions
            .remove(key.as_str())
            .map(|previous| previous.merge_monotonic(record.clone()))
            .unwrap_or(record);
        executions.insert(key, next);
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct MemoryPresenceStateStore {
    state: Arc<Mutex<MemoryPresenceState>>,
}

#[derive(Default)]
struct MemoryPresenceState {
    by_device: HashMap<String, PresenceStateRecord>,
    presence_by_principal: HashMap<String, BTreeSet<String>>,
    online_by_seen_at: BTreeSet<PresenceOnlineSeenAtKey>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PresenceOnlineSeenAtKey {
    last_seen_at: String,
    device_key: String,
}

impl MemoryPresenceStateStore {
    pub fn state(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Option<PresenceStateRecord> {
        self.state
            .lock()
            .expect("presence state store should lock")
            .by_device
            .get(device_scope_key(tenant_id, principal_kind, principal_id, device_id).as_str())
            .cloned()
    }
}

impl PresenceStateStore for MemoryPresenceStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError> {
        Ok(self.state(tenant_id, principal_kind, principal_id, device_id))
    }

    fn save_state(&self, record: PresenceStateRecord) -> Result<(), ContractError> {
        let device_key = device_scope_key(
            record.tenant_id.as_str(),
            record.principal_kind.as_str(),
            record.principal_id.as_str(),
            record.device_id.as_str(),
        );
        let principal_key = principal_scope_key(
            record.tenant_id.as_str(),
            record.principal_kind.as_str(),
            record.principal_id.as_str(),
        );
        let mut state = self.state.lock().expect("presence state store should lock");
        if let Some(previous) = state.by_device.get(device_key.as_str()).cloned() {
            remove_presence_online_seen_at_index(&mut state.online_by_seen_at, &previous);
        }
        insert_presence_online_seen_at_index(
            &mut state.online_by_seen_at,
            device_key.as_str(),
            &record,
        );
        state.by_device.insert(device_key.clone(), record);
        state
            .presence_by_principal
            .entry(principal_key)
            .or_default()
            .insert(device_key);
        Ok(())
    }

    fn list_states_for_principal(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Result<Vec<PresenceStateRecord>, ContractError> {
        let state = self.state.lock().expect("presence state store should lock");
        let device_keys = state
            .presence_by_principal
            .get(principal_scope_key(tenant_id, principal_kind, principal_id).as_str())
            .cloned()
            .unwrap_or_default();
        Ok(device_keys
            .into_iter()
            .filter_map(|device_key| state.by_device.get(device_key.as_str()).cloned())
            .collect())
    }

    fn list_online_states_seen_at_or_before(
        &self,
        cutoff_seen_at: &str,
        limit: usize,
    ) -> Result<Vec<PresenceStateRecord>, ContractError> {
        if limit == 0 {
            return Ok(Vec::new());
        }
        let state = self.state.lock().expect("presence state store should lock");
        Ok(state
            .online_by_seen_at
            .iter()
            .take_while(|key| key.last_seen_at.as_str() <= cutoff_seen_at)
            .take(limit)
            .filter_map(|key| state.by_device.get(key.device_key.as_str()).cloned())
            .collect())
    }

    fn expire_online_state_if_seen_at_or_before(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        cutoff_seen_at: &str,
        expired_at: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError> {
        let device_key = device_scope_key(tenant_id, principal_kind, principal_id, device_id);
        let mut state = self.state.lock().expect("presence state store should lock");
        let Some(current) = state.by_device.get(device_key.as_str()).cloned() else {
            return Ok(None);
        };
        if !current.is_online_seen_at_or_before(cutoff_seen_at) {
            return Ok(None);
        }
        remove_presence_online_seen_at_index(&mut state.online_by_seen_at, &current);
        let expired = current.into_expired_offline(expired_at);
        insert_presence_online_seen_at_index(
            &mut state.online_by_seen_at,
            device_key.as_str(),
            &expired,
        );
        state.by_device.insert(device_key, expired.clone());
        Ok(Some(expired))
    }
}

#[derive(Clone, Default)]
pub struct MemoryTimelineProjectionStore {
    entries: Arc<Mutex<HashMap<String, BTreeMap<u64, String>>>>,
}

impl MemoryTimelineProjectionStore {
    pub fn entries(&self, tenant_id: &str, timeline_scope: &str) -> Vec<(u64, String)> {
        self.entries
            .lock()
            .expect("timeline projection store should lock")
            .get(timeline_projection_scope_key(tenant_id, timeline_scope).as_str())
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
        tenant_id: &str,
        timeline_scope: &str,
        message_seq: u64,
        payload: &str,
    ) -> Result<(), ContractError> {
        self.entries
            .lock()
            .expect("timeline projection store should lock")
            .entry(timeline_projection_scope_key(tenant_id, timeline_scope))
            .or_default()
            .insert(message_seq, payload.to_string());
        Ok(())
    }

    fn load_timeline(
        &self,
        tenant_id: &str,
        timeline_scope: &str,
    ) -> Result<Vec<(u64, String)>, ContractError> {
        Ok(self.entries(tenant_id, timeline_scope))
    }

    fn upsert_timeline_entries(
        &self,
        tenant_id: &str,
        timeline_scope: &str,
        records: &[TimelineProjectionRecord],
    ) -> Result<(), ContractError> {
        let mut entries = self
            .entries
            .lock()
            .expect("timeline projection store should lock");
        let scope_entries = entries
            .entry(timeline_projection_scope_key(tenant_id, timeline_scope))
            .or_default();
        for record in records {
            scope_entries.insert(record.message_seq, record.payload.clone());
        }
        Ok(())
    }

    fn upsert_timeline_batches(
        &self,
        batches: &[TimelineProjectionBatch],
    ) -> Result<(), ContractError> {
        let mut entries = self
            .entries
            .lock()
            .expect("timeline projection store should lock");
        for batch in batches {
            let scope_entries = entries
                .entry(timeline_projection_scope_key(
                    batch.tenant_id.as_str(),
                    batch.timeline_scope.as_str(),
                ))
                .or_default();
            for record in &batch.records {
                scope_entries.insert(record.message_seq, record.payload.clone());
            }
        }
        Ok(())
    }
}

fn scope_key_parts(parts: &[&str]) -> String {
    parts
        .iter()
        .map(|part| format!("{}:{part}", part.len()))
        .collect::<Vec<_>>()
        .join("|")
}

fn snapshot_key(scope: &str, key: &str) -> String {
    scope_key_parts(&[scope, key])
}

fn device_twin_scope_key(tenant_id: &str, device_id: &str) -> String {
    scope_key_parts(&[tenant_id, device_id])
}

fn device_scope_key(
    tenant_id: &str,
    principal_kind: &str,
    principal_id: &str,
    device_id: &str,
) -> String {
    scope_key_parts(&[tenant_id, principal_kind, principal_id, device_id])
}

fn principal_scope_key(tenant_id: &str, principal_kind: &str, principal_id: &str) -> String {
    scope_key_parts(&[tenant_id, principal_kind, principal_id])
}

fn presence_online_seen_at_key(
    device_key: &str,
    record: &PresenceStateRecord,
) -> Option<PresenceOnlineSeenAtKey> {
    Some(PresenceOnlineSeenAtKey {
        last_seen_at: record.online_seen_at()?.to_owned(),
        device_key: device_key.to_owned(),
    })
}

fn insert_presence_online_seen_at_index(
    index: &mut BTreeSet<PresenceOnlineSeenAtKey>,
    device_key: &str,
    record: &PresenceStateRecord,
) {
    if let Some(key) = presence_online_seen_at_key(device_key, record) {
        index.insert(key);
    }
}

fn remove_presence_online_seen_at_index(
    index: &mut BTreeSet<PresenceOnlineSeenAtKey>,
    record: &PresenceStateRecord,
) {
    let device_key = device_scope_key(
        record.tenant_id.as_str(),
        record.principal_kind.as_str(),
        record.principal_id.as_str(),
        record.device_id.as_str(),
    );
    if let Some(key) = presence_online_seen_at_key(device_key.as_str(), record) {
        index.remove(&key);
    }
}

fn stream_scope_key(tenant_id: &str, stream_id: &str) -> String {
    scope_key_parts(&[tenant_id, stream_id])
}

fn rtc_scope_key(tenant_id: &str, rtc_session_id: &str) -> String {
    scope_key_parts(&[tenant_id, rtc_session_id])
}

fn notification_scope_key(tenant_id: &str, notification_id: &str) -> String {
    scope_key_parts(&[tenant_id, notification_id])
}

fn notification_recipient_scope_key(
    tenant_id: &str,
    recipient_kind: &str,
    recipient_id: &str,
) -> String {
    scope_key_parts(&[tenant_id, recipient_kind, recipient_id])
}

fn timeline_projection_scope_key(tenant_id: &str, timeline_scope: &str) -> String {
    scope_key_parts(&[tenant_id, timeline_scope])
}

fn record_notification_recipient_scope_key(record: &NotificationTaskRecord) -> String {
    notification_recipient_scope_key(
        record.tenant_id.as_str(),
        record.task.recipient_kind.as_str(),
        record.task.recipient_id.as_str(),
    )
}

fn insert_notification_recipient_index(
    index: &mut HashMap<String, BTreeSet<String>>,
    notification_key: &str,
    record: &NotificationTaskRecord,
) {
    index
        .entry(record_notification_recipient_scope_key(record))
        .or_default()
        .insert(notification_key.to_owned());
}

fn remove_notification_recipient_index(
    index: &mut HashMap<String, BTreeSet<String>>,
    notification_key: &str,
    record: &NotificationTaskRecord,
) {
    let recipient_key = record_notification_recipient_scope_key(record);
    if let Some(task_keys) = index.get_mut(recipient_key.as_str()) {
        task_keys.remove(notification_key);
        if task_keys.is_empty() {
            index.remove(recipient_key.as_str());
        }
    }
}

fn execution_scope_key(
    tenant_id: &str,
    principal_kind: &str,
    principal_id: &str,
    execution_id: &str,
) -> String {
    scope_key_parts(&[tenant_id, principal_kind, principal_id, execution_id])
}
