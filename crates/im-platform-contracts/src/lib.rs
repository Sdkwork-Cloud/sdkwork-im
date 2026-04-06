use im_domain_core::automation::AutomationExecution;
use im_domain_core::notification::NotificationTask;
use im_domain_core::session::DevicePresenceView;
use serde::{Deserialize, Serialize};

use im_domain_core::realtime::RealtimeSubscription;
use im_domain_core::rtc::{RtcSession, RtcSignalEvent};
use im_domain_core::stream::{StreamFrame, StreamSession};

pub use im_domain_events::CommitEnvelope;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitPosition {
    pub partition: String,
    pub offset: u64,
}

impl CommitPosition {
    pub fn new(partition: impl Into<String>, offset: u64) -> Self {
        Self {
            partition: partition.into(),
            offset,
        }
    }

    pub fn cursor(&self) -> String {
        format!("{}:{}", self.partition, self.offset)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeaseGrant {
    pub scope_id: String,
    pub owner_node_id: String,
    pub epoch: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectPutRequest {
    pub object_key: String,
    pub content_length: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectDescriptor {
    pub object_key: String,
    pub content_length: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeCheckpointRecord {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
    pub latest_realtime_seq: u64,
    pub acked_through_seq: u64,
    pub trimmed_through_seq: u64,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeDisconnectFenceRecord {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
    pub session_id: Option<String>,
    pub owner_node_id: String,
    pub disconnected_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeSubscriptionRecord {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
    pub items: Vec<RealtimeSubscription>,
    pub synced_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamStateRecord {
    pub tenant_id: String,
    pub stream_id: String,
    pub session: StreamSession,
    pub frames: Vec<StreamFrame>,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtcStateRecord {
    pub tenant_id: String,
    pub rtc_session_id: String,
    pub session: RtcSession,
    pub signals: Vec<RtcSignalEvent>,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationTaskRecord {
    pub tenant_id: String,
    pub notification_id: String,
    pub task: NotificationTask,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationExecutionRecord {
    pub tenant_id: String,
    pub principal_id: String,
    pub execution_id: String,
    pub execution: AutomationExecution,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresenceStateRecord {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
    pub presence: DevicePresenceView,
    pub resume_required: bool,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContractError {
    UnsupportedCapability(String),
    Conflict(String),
    Unavailable(String),
}

pub trait CommitJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError>;
}

pub trait MetadataStore {
    fn put_snapshot(&self, scope: &str, key: &str, value: &str) -> Result<(), ContractError>;
}

pub trait RealtimeCheckpointStore: Send + Sync {
    fn load_checkpoint(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError>;

    fn save_checkpoint(&self, record: RealtimeCheckpointRecord) -> Result<(), ContractError>;
}

pub trait RealtimeDisconnectFenceStore: Send + Sync {
    fn load_fence(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeDisconnectFenceRecord>, ContractError>;

    fn save_fence(&self, record: RealtimeDisconnectFenceRecord) -> Result<(), ContractError>;

    fn clear_fence(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError>;
}

pub trait RealtimeSubscriptionStore: Send + Sync {
    fn load_subscriptions(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeSubscriptionRecord>, ContractError>;

    fn save_subscriptions(&self, record: RealtimeSubscriptionRecord) -> Result<(), ContractError>;

    fn clear_subscriptions(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError>;
}

pub trait StreamStateStore: Send + Sync {
    fn load_state(
        &self,
        tenant_id: &str,
        stream_id: &str,
    ) -> Result<Option<StreamStateRecord>, ContractError>;

    fn save_state(&self, record: StreamStateRecord) -> Result<(), ContractError>;

    fn clear_state(&self, tenant_id: &str, stream_id: &str) -> Result<bool, ContractError>;
}

pub trait RtcStateStore: Send + Sync {
    fn load_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<Option<RtcStateRecord>, ContractError>;

    fn save_state(&self, record: RtcStateRecord) -> Result<(), ContractError>;

    fn clear_state(&self, tenant_id: &str, rtc_session_id: &str) -> Result<bool, ContractError>;
}

pub trait NotificationTaskStore: Send + Sync {
    fn load_task(
        &self,
        tenant_id: &str,
        notification_id: &str,
    ) -> Result<Option<NotificationTaskRecord>, ContractError>;

    fn save_task(&self, record: NotificationTaskRecord) -> Result<(), ContractError>;

    fn list_tasks_for_recipient(
        &self,
        tenant_id: &str,
        recipient_id: &str,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError>;
}

pub trait AutomationExecutionStore: Send + Sync {
    fn load_execution(
        &self,
        tenant_id: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Result<Option<AutomationExecutionRecord>, ContractError>;

    fn save_execution(&self, record: AutomationExecutionRecord) -> Result<(), ContractError>;
}

pub trait PresenceStateStore: Send + Sync {
    fn load_state(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError>;

    fn save_state(&self, record: PresenceStateRecord) -> Result<(), ContractError>;

    fn list_states_for_principal(
        &self,
        tenant_id: &str,
        principal_id: &str,
    ) -> Result<Vec<PresenceStateRecord>, ContractError>;
}

pub trait TimelineProjectionStore {
    fn upsert_timeline_entry(
        &self,
        conversation_id: &str,
        message_seq: u64,
        payload: &str,
    ) -> Result<(), ContractError>;
}

pub trait LeaseStore {
    fn acquire(&self, grant: LeaseGrant) -> Result<LeaseGrant, ContractError>;
}

pub trait ObjectStore {
    fn put(&self, request: ObjectPutRequest) -> Result<ObjectDescriptor, ContractError>;
}
