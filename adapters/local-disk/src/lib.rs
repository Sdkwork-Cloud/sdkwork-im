use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use im_platform_contracts::{
    AutomationExecutionRecord, AutomationExecutionStore, CommitEnvelope, CommitJournal,
    CommitPosition, ContractError, NotificationTaskRecord, NotificationTaskStore,
    PresenceStateRecord, PresenceStateStore, RealtimeCheckpointRecord, RealtimeCheckpointStore,
    RealtimeDisconnectFenceRecord, RealtimeDisconnectFenceStore, RealtimeSubscriptionRecord,
    RealtimeSubscriptionStore, RtcStateRecord, RtcStateStore, StreamStateRecord, StreamStateStore,
};
use serde::de::DeserializeOwned;

#[derive(Clone, Debug)]
pub struct FileCommitJournal {
    partition: Arc<String>,
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileCommitJournal {
    pub fn new(partition: impl Into<String>, file_path: impl Into<PathBuf>) -> Self {
        Self {
            partition: Arc::new(partition.into()),
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    pub fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("commit journal file store lock should lock");
        self.read_events()
    }

    fn read_events(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }

        let bytes = fs::read(self.file_path.as_path()).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to read commit journal {}: {error}",
                self.file_path.display()
            ))
        })?;
        if bytes.is_empty() {
            return Ok(Vec::new());
        }

        serde_json::from_slice(&bytes).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to parse commit journal {}: {error}",
                self.file_path.display()
            ))
        })
    }

    fn write_events(&self, events: &[CommitEnvelope]) -> Result<(), ContractError> {
        write_json_records(self.file_path.as_path(), events, "commit journal")
    }
}

impl CommitJournal for FileCommitJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("commit journal file store lock should lock");
        let mut events = self.read_events()?;
        events.push(envelope);
        self.write_events(&events)?;
        Ok(CommitPosition::new(
            self.partition.as_str(),
            events.len() as u64,
        ))
    }
}

#[derive(Clone, Debug)]
pub struct FileRealtimeCheckpointStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileRealtimeCheckpointStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, RealtimeCheckpointRecord>, ContractError> {
        if !self.file_path.exists() {
            return Ok(BTreeMap::new());
        }

        let bytes = fs::read(self.file_path.as_path()).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to read realtime checkpoint store {}: {error}",
                self.file_path.display()
            ))
        })?;
        if bytes.is_empty() {
            return Ok(BTreeMap::new());
        }

        serde_json::from_slice(&bytes).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to parse realtime checkpoint store {}: {error}",
                self.file_path.display()
            ))
        })
    }

    fn write_records(
        &self,
        records: &BTreeMap<String, RealtimeCheckpointRecord>,
    ) -> Result<(), ContractError> {
        write_json_records(
            self.file_path.as_path(),
            records,
            "realtime checkpoint store",
        )
    }
}

#[derive(Clone, Debug)]
pub struct FileRealtimeDisconnectFenceStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileRealtimeDisconnectFenceStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(
        &self,
    ) -> Result<BTreeMap<String, RealtimeDisconnectFenceRecord>, ContractError> {
        if !self.file_path.exists() {
            return Ok(BTreeMap::new());
        }

        let bytes = fs::read(self.file_path.as_path()).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to read disconnect fence store {}: {error}",
                self.file_path.display()
            ))
        })?;
        if bytes.is_empty() {
            return Ok(BTreeMap::new());
        }

        serde_json::from_slice(&bytes).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to parse disconnect fence store {}: {error}",
                self.file_path.display()
            ))
        })
    }

    fn write_records(
        &self,
        records: &BTreeMap<String, RealtimeDisconnectFenceRecord>,
    ) -> Result<(), ContractError> {
        let parent = self.file_path.parent().ok_or_else(|| {
            ContractError::Unavailable(format!(
                "disconnect fence store path has no parent: {}",
                self.file_path.display()
            ))
        })?;
        fs::create_dir_all(parent).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to create disconnect fence store dir {}: {error}",
                parent.display()
            ))
        })?;

        let payload = serde_json::to_vec_pretty(records).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to serialize disconnect fence store {}: {error}",
                self.file_path.display()
            ))
        })?;

        let temp_path = self.file_path.with_extension("json.tmp");
        fs::write(&temp_path, payload).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to write disconnect fence temp file {}: {error}",
                temp_path.display()
            ))
        })?;

        if self.file_path.exists() {
            fs::remove_file(self.file_path.as_path()).map_err(|error| {
                ContractError::Unavailable(format!(
                    "failed to replace disconnect fence store {}: {error}",
                    self.file_path.display()
                ))
            })?;
        }

        fs::rename(&temp_path, self.file_path.as_path()).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to finalize disconnect fence store {}: {error}",
                self.file_path.display()
            ))
        })?;

        Ok(())
    }
}

impl RealtimeDisconnectFenceStore for FileRealtimeDisconnectFenceStore {
    fn load_fence(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeDisconnectFenceRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("disconnect fence file store lock should lock");
        Ok(self
            .read_records()?
            .remove(scope_key(tenant_id, principal_id, device_id).as_str()))
    }

    fn save_fence(&self, record: RealtimeDisconnectFenceRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("disconnect fence file store lock should lock");
        let mut records = self.read_records()?;
        records.insert(
            scope_key(
                record.tenant_id.as_str(),
                record.principal_id.as_str(),
                record.device_id.as_str(),
            ),
            record,
        );
        self.write_records(&records)
    }

    fn clear_fence(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("disconnect fence file store lock should lock");
        let mut records = self.read_records()?;
        let removed = records
            .remove(scope_key(tenant_id, principal_id, device_id).as_str())
            .is_some();
        self.write_records(&records)?;
        Ok(removed)
    }
}

impl RealtimeCheckpointStore for FileRealtimeCheckpointStore {
    fn load_checkpoint(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("checkpoint file store lock should lock");
        Ok(self
            .read_records()?
            .remove(scope_key(tenant_id, principal_id, device_id).as_str()))
    }

    fn save_checkpoint(&self, record: RealtimeCheckpointRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("checkpoint file store lock should lock");
        let mut records = self.read_records()?;
        records.insert(
            scope_key(
                record.tenant_id.as_str(),
                record.principal_id.as_str(),
                record.device_id.as_str(),
            ),
            record,
        );
        self.write_records(&records)
    }
}

#[derive(Clone, Debug)]
pub struct FileRealtimeSubscriptionStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileRealtimeSubscriptionStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, RealtimeSubscriptionRecord>, ContractError> {
        if !self.file_path.exists() {
            return Ok(BTreeMap::new());
        }

        let bytes = fs::read(self.file_path.as_path()).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to read realtime subscription store {}: {error}",
                self.file_path.display()
            ))
        })?;
        if bytes.is_empty() {
            return Ok(BTreeMap::new());
        }

        serde_json::from_slice(&bytes).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to parse realtime subscription store {}: {error}",
                self.file_path.display()
            ))
        })
    }

    fn write_records(
        &self,
        records: &BTreeMap<String, RealtimeSubscriptionRecord>,
    ) -> Result<(), ContractError> {
        write_json_records(
            self.file_path.as_path(),
            records,
            "realtime subscription store",
        )
    }
}

impl RealtimeSubscriptionStore for FileRealtimeSubscriptionStore {
    fn load_subscriptions(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeSubscriptionRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("subscription file store lock should lock");
        Ok(self
            .read_records()?
            .remove(scope_key(tenant_id, principal_id, device_id).as_str()))
    }

    fn save_subscriptions(&self, record: RealtimeSubscriptionRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("subscription file store lock should lock");
        let mut records = self.read_records()?;
        records.insert(
            scope_key(
                record.tenant_id.as_str(),
                record.principal_id.as_str(),
                record.device_id.as_str(),
            ),
            record,
        );
        self.write_records(&records)
    }

    fn clear_subscriptions(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("subscription file store lock should lock");
        let mut records = self.read_records()?;
        let removed = records
            .remove(scope_key(tenant_id, principal_id, device_id).as_str())
            .is_some();
        self.write_records(&records)?;
        Ok(removed)
    }
}

#[derive(Clone, Debug)]
pub struct FileStreamStateStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileStreamStateStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, StreamStateRecord>, ContractError> {
        if !self.file_path.exists() {
            return Ok(BTreeMap::new());
        }

        let bytes = fs::read(self.file_path.as_path()).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to read stream state store {}: {error}",
                self.file_path.display()
            ))
        })?;
        if bytes.is_empty() {
            return Ok(BTreeMap::new());
        }

        serde_json::from_slice(&bytes).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to parse stream state store {}: {error}",
                self.file_path.display()
            ))
        })
    }

    fn write_records(
        &self,
        records: &BTreeMap<String, StreamStateRecord>,
    ) -> Result<(), ContractError> {
        write_json_records(self.file_path.as_path(), records, "stream state store")
    }
}

impl StreamStateStore for FileStreamStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        stream_id: &str,
    ) -> Result<Option<StreamStateRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("stream state file store lock should lock");
        Ok(self
            .read_records()?
            .remove(stream_scope_key(tenant_id, stream_id).as_str()))
    }

    fn save_state(&self, record: StreamStateRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("stream state file store lock should lock");
        let mut records = self.read_records()?;
        records.insert(
            stream_scope_key(record.tenant_id.as_str(), record.stream_id.as_str()),
            record,
        );
        self.write_records(&records)
    }

    fn clear_state(&self, tenant_id: &str, stream_id: &str) -> Result<bool, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("stream state file store lock should lock");
        let mut records = self.read_records()?;
        let removed = records
            .remove(stream_scope_key(tenant_id, stream_id).as_str())
            .is_some();
        self.write_records(&records)?;
        Ok(removed)
    }
}

#[derive(Clone, Debug)]
pub struct FileRtcStateStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileRtcStateStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, RtcStateRecord>, ContractError> {
        if !self.file_path.exists() {
            return Ok(BTreeMap::new());
        }

        let bytes = fs::read(self.file_path.as_path()).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to read rtc state store {}: {error}",
                self.file_path.display()
            ))
        })?;
        if bytes.is_empty() {
            return Ok(BTreeMap::new());
        }

        serde_json::from_slice(&bytes).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to parse rtc state store {}: {error}",
                self.file_path.display()
            ))
        })
    }

    fn write_records(
        &self,
        records: &BTreeMap<String, RtcStateRecord>,
    ) -> Result<(), ContractError> {
        write_json_records(self.file_path.as_path(), records, "rtc state store")
    }
}

impl RtcStateStore for FileRtcStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<Option<RtcStateRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("rtc state file store lock should lock");
        Ok(self
            .read_records()?
            .remove(rtc_scope_key(tenant_id, rtc_session_id).as_str()))
    }

    fn save_state(&self, record: RtcStateRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("rtc state file store lock should lock");
        let mut records = self.read_records()?;
        records.insert(
            rtc_scope_key(record.tenant_id.as_str(), record.rtc_session_id.as_str()),
            record,
        );
        self.write_records(&records)
    }

    fn clear_state(&self, tenant_id: &str, rtc_session_id: &str) -> Result<bool, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("rtc state file store lock should lock");
        let mut records = self.read_records()?;
        let removed = records
            .remove(rtc_scope_key(tenant_id, rtc_session_id).as_str())
            .is_some();
        self.write_records(&records)?;
        Ok(removed)
    }
}

#[derive(Clone, Debug)]
pub struct FilePresenceStateStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FilePresenceStateStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, PresenceStateRecord>, ContractError> {
        if !self.file_path.exists() {
            return Ok(BTreeMap::new());
        }

        let bytes = fs::read(self.file_path.as_path()).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to read presence state store {}: {error}",
                self.file_path.display()
            ))
        })?;
        if bytes.is_empty() {
            return Ok(BTreeMap::new());
        }

        serde_json::from_slice(&bytes).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to parse presence state store {}: {error}",
                self.file_path.display()
            ))
        })
    }

    fn write_records(
        &self,
        records: &BTreeMap<String, PresenceStateRecord>,
    ) -> Result<(), ContractError> {
        write_json_records(self.file_path.as_path(), records, "presence state store")
    }
}

impl PresenceStateStore for FilePresenceStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("presence state file store lock should lock");
        Ok(self
            .read_records()?
            .remove(scope_key(tenant_id, principal_id, device_id).as_str()))
    }

    fn save_state(&self, record: PresenceStateRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("presence state file store lock should lock");
        let mut records = self.read_records()?;
        records.insert(
            scope_key(
                record.tenant_id.as_str(),
                record.principal_id.as_str(),
                record.device_id.as_str(),
            ),
            record,
        );
        self.write_records(&records)
    }

    fn list_states_for_principal(
        &self,
        tenant_id: &str,
        principal_id: &str,
    ) -> Result<Vec<PresenceStateRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("presence state file store lock should lock");
        Ok(self
            .read_records()?
            .into_values()
            .filter(|record| record.tenant_id == tenant_id && record.principal_id == principal_id)
            .collect())
    }
}

#[derive(Clone, Debug)]
pub struct FileNotificationTaskStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileNotificationTaskStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, NotificationTaskRecord>, ContractError> {
        if !self.file_path.exists() {
            return Ok(BTreeMap::new());
        }

        let bytes = fs::read(self.file_path.as_path()).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to read notification task store {}: {error}",
                self.file_path.display()
            ))
        })?;
        if bytes.is_empty() {
            return Ok(BTreeMap::new());
        }

        serde_json::from_slice(&bytes).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to parse notification task store {}: {error}",
                self.file_path.display()
            ))
        })
    }

    fn write_records(
        &self,
        records: &BTreeMap<String, NotificationTaskRecord>,
    ) -> Result<(), ContractError> {
        write_json_records(self.file_path.as_path(), records, "notification task store")
    }
}

impl NotificationTaskStore for FileNotificationTaskStore {
    fn load_task(
        &self,
        tenant_id: &str,
        notification_id: &str,
    ) -> Result<Option<NotificationTaskRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("notification task file store lock should lock");
        Ok(self
            .read_records()?
            .remove(notification_scope_key(tenant_id, notification_id).as_str()))
    }

    fn save_task(&self, record: NotificationTaskRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("notification task file store lock should lock");
        let mut records = self.read_records()?;
        records.insert(
            notification_scope_key(record.tenant_id.as_str(), record.notification_id.as_str()),
            record,
        );
        self.write_records(&records)
    }

    fn list_tasks_for_recipient(
        &self,
        tenant_id: &str,
        recipient_id: &str,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("notification task file store lock should lock");
        Ok(self
            .read_records()?
            .into_values()
            .filter(|record| {
                record.tenant_id == tenant_id && record.task.recipient_id == recipient_id
            })
            .collect())
    }
}

#[derive(Clone, Debug)]
pub struct FileAutomationExecutionStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileAutomationExecutionStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, AutomationExecutionRecord>, ContractError> {
        if !self.file_path.exists() {
            return Ok(BTreeMap::new());
        }

        let bytes = fs::read(self.file_path.as_path()).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to read automation execution store {}: {error}",
                self.file_path.display()
            ))
        })?;
        if bytes.is_empty() {
            return Ok(BTreeMap::new());
        }

        serde_json::from_slice(&bytes).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to parse automation execution store {}: {error}",
                self.file_path.display()
            ))
        })
    }

    fn write_records(
        &self,
        records: &BTreeMap<String, AutomationExecutionRecord>,
    ) -> Result<(), ContractError> {
        write_json_records(
            self.file_path.as_path(),
            records,
            "automation execution store",
        )
    }
}

impl AutomationExecutionStore for FileAutomationExecutionStore {
    fn load_execution(
        &self,
        tenant_id: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Result<Option<AutomationExecutionRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("automation execution file store lock should lock");
        Ok(self
            .read_records()?
            .remove(execution_scope_key(tenant_id, principal_id, execution_id).as_str()))
    }

    fn save_execution(&self, record: AutomationExecutionRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("automation execution file store lock should lock");
        let mut records = self.read_records()?;
        records.insert(
            execution_scope_key(
                record.tenant_id.as_str(),
                record.principal_id.as_str(),
                record.execution_id.as_str(),
            ),
            record,
        );
        self.write_records(&records)
    }
}

pub fn read_commit_journal_file(
    file_path: impl AsRef<Path>,
) -> Result<Vec<CommitEnvelope>, ContractError> {
    read_json_records_or_default(file_path.as_ref(), "commit journal")
}

pub fn validate_commit_journal_file(file_path: impl AsRef<Path>) -> Result<(), ContractError> {
    let _: Vec<CommitEnvelope> = read_commit_journal_file(file_path)?;
    Ok(())
}

pub fn validate_realtime_checkpoint_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, RealtimeCheckpointRecord> =
        read_json_records_or_default(file_path.as_ref(), "realtime checkpoint store")?;
    Ok(())
}

pub fn validate_realtime_disconnect_fence_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, RealtimeDisconnectFenceRecord> =
        read_json_records_or_default(file_path.as_ref(), "disconnect fence store")?;
    Ok(())
}

pub fn validate_realtime_subscription_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, RealtimeSubscriptionRecord> =
        read_json_records_or_default(file_path.as_ref(), "realtime subscription store")?;
    Ok(())
}

pub fn validate_stream_state_store_file(file_path: impl AsRef<Path>) -> Result<(), ContractError> {
    let _: BTreeMap<String, StreamStateRecord> =
        read_json_records_or_default(file_path.as_ref(), "stream state store")?;
    Ok(())
}

pub fn validate_rtc_state_store_file(file_path: impl AsRef<Path>) -> Result<(), ContractError> {
    let _: BTreeMap<String, RtcStateRecord> =
        read_json_records_or_default(file_path.as_ref(), "rtc state store")?;
    Ok(())
}

pub fn validate_presence_state_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, PresenceStateRecord> =
        read_json_records_or_default(file_path.as_ref(), "presence state store")?;
    Ok(())
}

pub fn validate_notification_task_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, NotificationTaskRecord> =
        read_json_records_or_default(file_path.as_ref(), "notification task store")?;
    Ok(())
}

pub fn validate_automation_execution_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, AutomationExecutionRecord> =
        read_json_records_or_default(file_path.as_ref(), "automation execution store")?;
    Ok(())
}

fn read_json_records_or_default<T>(file_path: &Path, store_name: &str) -> Result<T, ContractError>
where
    T: DeserializeOwned + Default,
{
    if !file_path.exists() {
        return Ok(T::default());
    }

    let bytes = fs::read(file_path).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to read {store_name} {}: {error}",
            file_path.display()
        ))
    })?;
    if bytes.is_empty() {
        return Ok(T::default());
    }

    serde_json::from_slice(&bytes).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to parse {store_name} {}: {error}",
            file_path.display()
        ))
    })
}

fn write_json_records<T: serde::Serialize + ?Sized>(
    file_path: &Path,
    records: &T,
    store_name: &str,
) -> Result<(), ContractError> {
    let parent = file_path.parent().ok_or_else(|| {
        ContractError::Unavailable(format!(
            "{store_name} path has no parent: {}",
            file_path.display()
        ))
    })?;
    fs::create_dir_all(parent).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to create {store_name} dir {}: {error}",
            parent.display()
        ))
    })?;

    let payload = serde_json::to_vec_pretty(records).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to serialize {store_name} {}: {error}",
            file_path.display()
        ))
    })?;

    let temp_path = file_path.with_extension("json.tmp");
    fs::write(&temp_path, payload).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to write {store_name} temp file {}: {error}",
            temp_path.display()
        ))
    })?;

    if file_path.exists() {
        fs::remove_file(file_path).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to replace {store_name} {}: {error}",
                file_path.display()
            ))
        })?;
    }

    fs::rename(&temp_path, file_path).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to finalize {store_name} {}: {error}",
            file_path.display()
        ))
    })?;

    Ok(())
}

fn scope_key(tenant_id: &str, principal_id: &str, device_id: &str) -> String {
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

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;
    use im_domain_core::realtime::RealtimeSubscription;
    use im_domain_core::session::{DevicePresenceStatus, DevicePresenceView};
    use im_platform_contracts::CommitEnvelope;

    fn unique_store_file() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("craw_chat_disconnect_fence_store_{unique}.json"))
    }

    fn unique_checkpoint_store_file() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("craw_chat_realtime_checkpoint_store_{unique}.json"))
    }

    fn unique_subscription_store_file() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "craw_chat_realtime_subscription_store_{unique}.json"
        ))
    }

    fn unique_commit_journal_file() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("craw_chat_commit_journal_{unique}.json"))
    }

    fn unique_stream_state_store_file() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("craw_chat_stream_state_store_{unique}.json"))
    }

    fn unique_rtc_state_store_file() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("craw_chat_rtc_state_store_{unique}.json"))
    }

    fn unique_notification_task_store_file() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("craw_chat_notification_task_store_{unique}.json"))
    }

    fn unique_automation_execution_store_file() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "craw_chat_automation_execution_store_{unique}.json"
        ))
    }

    fn unique_presence_state_store_file() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("craw_chat_presence_state_store_{unique}.json"))
    }

    #[test]
    fn test_file_commit_journal_persists_across_reopen() {
        let file_path = unique_commit_journal_file();
        let journal = FileCommitJournal::new("local-minimal", &file_path);
        journal
            .append(CommitEnvelope::minimal(
                "evt_demo_1",
                "t_demo",
                "conversation.created",
                "conversation",
                "c_demo",
                0,
            ))
            .expect("append should succeed");
        journal
            .append(CommitEnvelope::minimal(
                "evt_demo_2",
                "t_demo",
                "message.posted",
                "conversation",
                "c_demo",
                1,
            ))
            .expect("append should succeed");

        let reopened = FileCommitJournal::new("local-minimal", &file_path);
        let recorded = reopened.recorded().expect("recorded should succeed");
        assert_eq!(recorded.len(), 2);
        assert_eq!(recorded[0].event_id, "evt_demo_1");
        assert_eq!(recorded[1].event_id, "evt_demo_2");

        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_read_commit_journal_file_restores_minimal_events() {
        let file_path = unique_commit_journal_file();
        fs::write(
            &file_path,
            serde_json::to_vec_pretty(&vec![CommitEnvelope::minimal(
                "evt_demo_1",
                "t_demo",
                "conversation.created",
                "conversation",
                "c_demo",
                0,
            )])
            .expect("journal payload should serialize"),
        )
        .expect("journal file should be written");

        let restored = read_commit_journal_file(&file_path).expect("journal should parse");
        assert_eq!(restored.len(), 1);
        assert_eq!(restored[0].event_id, "evt_demo_1");

        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_validate_checkpoint_store_file_rejects_array_shape() {
        let file_path = unique_checkpoint_store_file();
        fs::write(&file_path, b"[]").expect("checkpoint file should be written");

        let error = validate_realtime_checkpoint_store_file(&file_path)
            .expect_err("array-shaped checkpoint store should be rejected");
        assert!(matches!(error, ContractError::Unavailable(_)));
        let message = match error {
            ContractError::Unavailable(message) => message,
            other => panic!("unexpected error variant: {other:?}"),
        };
        assert!(message.contains("failed to parse realtime checkpoint store"));

        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_file_checkpoint_store_persists_across_reopen() {
        let file_path = unique_checkpoint_store_file();
        let store = FileRealtimeCheckpointStore::new(&file_path);
        store
            .save_checkpoint(RealtimeCheckpointRecord {
                tenant_id: "t_demo".into(),
                principal_id: "u_demo".into(),
                device_id: "d_pad".into(),
                latest_realtime_seq: 7,
                acked_through_seq: 5,
                trimmed_through_seq: 5,
                updated_at: "2026-04-06T00:00:00.000Z".into(),
            })
            .expect("save should succeed");

        let reopened = FileRealtimeCheckpointStore::new(&file_path);
        let restored = reopened
            .load_checkpoint("t_demo", "u_demo", "d_pad")
            .expect("load should succeed")
            .expect("checkpoint should exist");
        assert_eq!(restored.latest_realtime_seq, 7);
        assert_eq!(restored.acked_through_seq, 5);
        assert_eq!(restored.trimmed_through_seq, 5);

        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_file_disconnect_fence_store_persists_and_clears_across_reopen() {
        let file_path = unique_store_file();
        let store = FileRealtimeDisconnectFenceStore::new(&file_path);
        store
            .save_fence(RealtimeDisconnectFenceRecord {
                tenant_id: "t_demo".into(),
                principal_id: "u_demo".into(),
                device_id: "d_pad".into(),
                session_id: Some("s_old".into()),
                owner_node_id: "node_a".into(),
                disconnected_at: "2026-04-06T00:00:00.000Z".into(),
            })
            .expect("save should succeed");

        let reopened = FileRealtimeDisconnectFenceStore::new(&file_path);
        let restored = reopened
            .load_fence("t_demo", "u_demo", "d_pad")
            .expect("load should succeed")
            .expect("fence should exist");
        assert_eq!(restored.session_id.as_deref(), Some("s_old"));
        assert_eq!(restored.owner_node_id, "node_a");

        assert!(
            reopened
                .clear_fence("t_demo", "u_demo", "d_pad")
                .expect("clear should succeed")
        );

        let reopened_after_clear = FileRealtimeDisconnectFenceStore::new(&file_path);
        assert!(
            reopened_after_clear
                .load_fence("t_demo", "u_demo", "d_pad")
                .expect("load after clear should succeed")
                .is_none()
        );

        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_file_subscription_store_persists_across_reopen() {
        let file_path = unique_subscription_store_file();
        let store = FileRealtimeSubscriptionStore::new(&file_path);
        store
            .save_subscriptions(RealtimeSubscriptionRecord {
                tenant_id: "t_demo".into(),
                principal_id: "u_demo".into(),
                device_id: "d_pad".into(),
                items: vec![RealtimeSubscription {
                    scope_type: "conversation".into(),
                    scope_id: "c_demo".into(),
                    event_types: vec!["message.posted".into()],
                    subscribed_at: "2026-04-06T00:00:00.000Z".into(),
                }],
                synced_at: "2026-04-06T00:00:00.000Z".into(),
            })
            .expect("save should succeed");

        let reopened = FileRealtimeSubscriptionStore::new(&file_path);
        let restored = reopened
            .load_subscriptions("t_demo", "u_demo", "d_pad")
            .expect("load should succeed")
            .expect("subscriptions should exist");
        assert_eq!(restored.items.len(), 1);
        assert_eq!(restored.items[0].scope_id, "c_demo");
        assert_eq!(restored.items[0].event_types, vec!["message.posted"]);
        assert_eq!(restored.synced_at, "2026-04-06T00:00:00.000Z");

        assert!(
            reopened
                .clear_subscriptions("t_demo", "u_demo", "d_pad")
                .expect("clear should succeed")
        );

        let reopened_after_clear = FileRealtimeSubscriptionStore::new(&file_path);
        assert!(
            reopened_after_clear
                .load_subscriptions("t_demo", "u_demo", "d_pad")
                .expect("load after clear should succeed")
                .is_none()
        );

        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_file_stream_state_store_persists_across_reopen() {
        let file_path = unique_stream_state_store_file();
        let store = FileStreamStateStore::new(&file_path);
        store
            .save_state(StreamStateRecord {
                tenant_id: "t_demo".into(),
                stream_id: "st_demo".into(),
                session: im_domain_core::stream::StreamSession {
                    tenant_id: "t_demo".into(),
                    stream_id: "st_demo".into(),
                    stream_type: "custom.delta.text".into(),
                    scope_kind: "request".into(),
                    scope_id: "req_demo".into(),
                    durability_class: im_domain_core::stream::StreamDurabilityClass::DurableSession,
                    ordering_scope: "stream".into(),
                    schema_ref: Some("custom.delta.text.v1".into()),
                    state: im_domain_core::stream::StreamSessionState::Active,
                    last_frame_seq: 1,
                    last_checkpoint_seq: Some(1),
                    result_message_id: None,
                    opened_at: "2026-04-06T00:00:00.000Z".into(),
                    closed_at: None,
                    expires_at: None,
                },
                frames: vec![im_domain_core::stream::StreamFrame {
                    tenant_id: "t_demo".into(),
                    stream_id: "st_demo".into(),
                    stream_type: "custom.delta.text".into(),
                    scope_kind: "request".into(),
                    scope_id: "req_demo".into(),
                    frame_seq: 1,
                    frame_type: "delta".into(),
                    schema_ref: Some("custom.delta.text.v1".into()),
                    encoding: "json".into(),
                    payload: "{\"delta\":\"hello\"}".into(),
                    sender: im_domain_core::message::Sender {
                        id: "u_demo".into(),
                        kind: "user".into(),
                        member_id: None,
                        device_id: Some("d_demo".into()),
                        session_id: Some("s_demo".into()),
                        metadata: BTreeMap::new(),
                    },
                    attributes: BTreeMap::new(),
                    occurred_at: "2026-04-06T00:00:00.000Z".into(),
                }],
                updated_at: "2026-04-06T00:00:00.000Z".into(),
            })
            .expect("save should succeed");

        let reopened = FileStreamStateStore::new(&file_path);
        let restored = reopened
            .load_state("t_demo", "st_demo")
            .expect("load should succeed")
            .expect("stream state should exist");
        assert_eq!(restored.session.last_frame_seq, 1);
        assert_eq!(restored.frames.len(), 1);
        assert_eq!(restored.frames[0].frame_seq, 1);

        assert!(
            reopened
                .clear_state("t_demo", "st_demo")
                .expect("clear should succeed")
        );

        let reopened_after_clear = FileStreamStateStore::new(&file_path);
        assert!(
            reopened_after_clear
                .load_state("t_demo", "st_demo")
                .expect("load after clear should succeed")
                .is_none()
        );

        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_file_rtc_state_store_persists_across_reopen() {
        let file_path = unique_rtc_state_store_file();
        let store = FileRtcStateStore::new(&file_path);
        store
            .save_state(RtcStateRecord {
                tenant_id: "t_demo".into(),
                rtc_session_id: "rtc_demo".into(),
                session: im_domain_core::rtc::RtcSession {
                    tenant_id: "t_demo".into(),
                    rtc_session_id: "rtc_demo".into(),
                    conversation_id: Some("c_demo".into()),
                    rtc_mode: "voice".into(),
                    initiator_id: "u_demo".into(),
                    state: im_domain_core::rtc::RtcSessionState::Accepted,
                    signaling_stream_id: Some("st_demo".into()),
                    artifact_message_id: Some("msg_accept".into()),
                    started_at: "2026-04-06T00:00:00.000Z".into(),
                    ended_at: None,
                },
                signals: vec![im_domain_core::rtc::RtcSignalEvent {
                    tenant_id: "t_demo".into(),
                    rtc_session_id: "rtc_demo".into(),
                    conversation_id: Some("c_demo".into()),
                    rtc_mode: "voice".into(),
                    signal_type: "rtc.offer".into(),
                    schema_ref: Some("webrtc.offer.v1".into()),
                    payload: "{\"sdp\":\"offer\"}".into(),
                    sender: im_domain_core::message::Sender {
                        id: "u_demo".into(),
                        kind: "user".into(),
                        member_id: None,
                        device_id: Some("d_demo".into()),
                        session_id: Some("s_demo".into()),
                        metadata: BTreeMap::new(),
                    },
                    signaling_stream_id: Some("st_demo".into()),
                    occurred_at: "2026-04-06T00:00:01.000Z".into(),
                }],
                updated_at: "2026-04-06T00:00:02.000Z".into(),
            })
            .expect("save should succeed");

        let reopened = FileRtcStateStore::new(&file_path);
        let restored = reopened
            .load_state("t_demo", "rtc_demo")
            .expect("load should succeed")
            .expect("rtc state should exist");
        assert_eq!(
            restored.session.state,
            im_domain_core::rtc::RtcSessionState::Accepted
        );
        assert_eq!(
            restored.session.signaling_stream_id.as_deref(),
            Some("st_demo")
        );
        assert_eq!(restored.signals.len(), 1);
        assert_eq!(restored.signals[0].signal_type, "rtc.offer");

        assert!(
            reopened
                .clear_state("t_demo", "rtc_demo")
                .expect("clear should succeed")
        );

        let reopened_after_clear = FileRtcStateStore::new(&file_path);
        assert!(
            reopened_after_clear
                .load_state("t_demo", "rtc_demo")
                .expect("load after clear should succeed")
                .is_none()
        );

        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_file_notification_task_store_persists_across_reopen() {
        let file_path = unique_notification_task_store_file();
        let store = FileNotificationTaskStore::new(&file_path);
        store
            .save_task(NotificationTaskRecord {
                tenant_id: "t_demo".into(),
                notification_id: "ntf_demo".into(),
                task: im_domain_core::notification::NotificationTask {
                    tenant_id: "t_demo".into(),
                    notification_id: "ntf_demo".into(),
                    source_event_id: "evt_demo".into(),
                    source_event_type: "message.posted".into(),
                    category: "message.new".into(),
                    channel: "inapp".into(),
                    recipient_id: "u_demo".into(),
                    status: im_domain_core::notification::NotificationStatus::Dispatched,
                    title: Some("hello".into()),
                    body: Some("world".into()),
                    payload: Some("{\"conversationId\":\"c_demo\"}".into()),
                    requested_at: "2026-04-06T00:00:00.000Z".into(),
                    dispatched_at: Some("2026-04-06T00:00:01.000Z".into()),
                    failure_reason: None,
                },
                updated_at: "2026-04-06T00:00:01.000Z".into(),
            })
            .expect("save should succeed");

        let reopened = FileNotificationTaskStore::new(&file_path);
        let restored = reopened
            .load_task("t_demo", "ntf_demo")
            .expect("load should succeed")
            .expect("notification task should exist");
        assert_eq!(restored.task.notification_id, "ntf_demo");
        assert_eq!(restored.task.recipient_id, "u_demo");

        let listed = reopened
            .list_tasks_for_recipient("t_demo", "u_demo")
            .expect("list should succeed");
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].notification_id, "ntf_demo");

        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_file_automation_execution_store_persists_across_reopen() {
        let file_path = unique_automation_execution_store_file();
        let store = FileAutomationExecutionStore::new(&file_path);
        store
            .save_execution(AutomationExecutionRecord {
                tenant_id: "t_demo".into(),
                principal_id: "u_demo".into(),
                execution_id: "ae_demo".into(),
                execution: im_domain_core::automation::AutomationExecution {
                    tenant_id: "t_demo".into(),
                    principal_id: "u_demo".into(),
                    principal_kind: "user".into(),
                    execution_id: "ae_demo".into(),
                    trigger_type: "webhook.manual".into(),
                    target_kind: "workflow".into(),
                    target_ref: "wf_demo".into(),
                    input_payload: Some("{\"conversationId\":\"c_demo\"}".into()),
                    output_payload: Some("{\"accepted\":true}".into()),
                    state: im_domain_core::automation::AutomationExecutionState::Succeeded,
                    retry_count: 0,
                    requested_at: "2026-04-06T00:00:00.000Z".into(),
                    completed_at: Some("2026-04-06T00:00:01.000Z".into()),
                    failure_reason: None,
                },
                updated_at: "2026-04-06T00:00:01.000Z".into(),
            })
            .expect("save should succeed");

        let reopened = FileAutomationExecutionStore::new(&file_path);
        let restored = reopened
            .load_execution("t_demo", "u_demo", "ae_demo")
            .expect("load should succeed")
            .expect("automation execution should exist");
        assert_eq!(restored.execution.execution_id, "ae_demo");
        assert_eq!(restored.execution.principal_id, "u_demo");
        assert_eq!(
            restored.execution.state,
            im_domain_core::automation::AutomationExecutionState::Succeeded
        );

        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_file_presence_state_store_persists_across_reopen() {
        let file_path = unique_presence_state_store_file();
        let store = FilePresenceStateStore::new(&file_path);
        store
            .save_state(PresenceStateRecord {
                tenant_id: "t_demo".into(),
                principal_id: "u_demo".into(),
                device_id: "d_pad".into(),
                presence: DevicePresenceView {
                    tenant_id: "t_demo".into(),
                    principal_id: "u_demo".into(),
                    device_id: "d_pad".into(),
                    platform: None,
                    session_id: None,
                    status: DevicePresenceStatus::Offline,
                    last_sync_seq: 7,
                    last_resume_at: Some("2026-04-06T00:00:00.000Z".into()),
                    last_seen_at: Some("2026-04-06T00:00:01.000Z".into()),
                },
                resume_required: true,
                updated_at: "2026-04-06T00:00:01.000Z".into(),
            })
            .expect("save should succeed");
        store
            .save_state(PresenceStateRecord {
                tenant_id: "t_demo".into(),
                principal_id: "u_demo".into(),
                device_id: "d_phone".into(),
                presence: DevicePresenceView {
                    tenant_id: "t_demo".into(),
                    principal_id: "u_demo".into(),
                    device_id: "d_phone".into(),
                    platform: None,
                    session_id: None,
                    status: DevicePresenceStatus::Offline,
                    last_sync_seq: 0,
                    last_resume_at: None,
                    last_seen_at: None,
                },
                resume_required: false,
                updated_at: "2026-04-06T00:00:02.000Z".into(),
            })
            .expect("save should succeed");

        let reopened = FilePresenceStateStore::new(&file_path);
        let restored = reopened
            .load_state("t_demo", "u_demo", "d_pad")
            .expect("load should succeed")
            .expect("presence state should exist");
        assert_eq!(restored.device_id, "d_pad");
        assert!(restored.resume_required);
        assert_eq!(restored.presence.last_sync_seq, 7);

        let listed = reopened
            .list_states_for_principal("t_demo", "u_demo")
            .expect("list should succeed");
        assert_eq!(listed.len(), 2);
        assert!(listed.iter().any(|record| record.device_id == "d_pad"));
        assert!(listed.iter().any(|record| record.device_id == "d_phone"));

        let _ = fs::remove_file(file_path);
    }
}
