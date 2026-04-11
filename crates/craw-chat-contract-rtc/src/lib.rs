use craw_chat_contract_core::ContractError;
use im_domain_core::rtc::{RtcSession, RtcSignalEvent};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtcStateRecord {
    pub tenant_id: String,
    pub rtc_session_id: String,
    pub session: RtcSession,
    pub signals: Vec<RtcSignalEvent>,
    pub updated_at: String,
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
