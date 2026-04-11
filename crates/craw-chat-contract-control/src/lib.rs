use craw_chat_contract_core::ContractError;
use im_domain_core::realtime::RealtimeSubscription;
use im_domain_core::session::DevicePresenceView;
use serde::{Deserialize, Serialize};

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
pub struct PresenceStateRecord {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
    pub presence: DevicePresenceView,
    pub resume_required: bool,
    pub updated_at: String,
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
