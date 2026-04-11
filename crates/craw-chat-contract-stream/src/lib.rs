use craw_chat_contract_core::ContractError;
use im_domain_core::stream::{StreamFrame, StreamSession};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamStateRecord {
    pub tenant_id: String,
    pub stream_id: String,
    pub session: StreamSession,
    pub frames: Vec<StreamFrame>,
    pub updated_at: String,
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
