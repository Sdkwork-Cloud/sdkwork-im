use craw_chat_contract_core::ContractError;
use im_domain_core::message::{MessageAttributes, Sender};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceSubject {
    pub device_id: String,
    pub owner_principal_id: Option<String>,
    pub session_id: Option<String>,
    pub metadata: MessageAttributes,
}

impl DeviceSubject {
    pub fn sender(&self, member_id: Option<String>) -> Sender {
        Sender {
            id: self.device_id.clone(),
            kind: "device".into(),
            member_id,
            device_id: Some(self.device_id.clone()),
            session_id: self.session_id.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceSubjectRecord {
    pub tenant_id: String,
    pub device: DeviceSubject,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceTwinRecord {
    pub tenant_id: String,
    pub device_id: String,
    pub desired_state_json: String,
    pub reported_state_json: String,
    pub updated_at: String,
}

pub trait DeviceSubjectStore: Send + Sync {
    fn load_subject(
        &self,
        tenant_id: &str,
        device_id: &str,
    ) -> Result<Option<DeviceSubjectRecord>, ContractError>;

    fn save_subject(&self, record: DeviceSubjectRecord) -> Result<(), ContractError>;
}

pub trait DeviceTwinStore: Send + Sync {
    fn load_twin(
        &self,
        tenant_id: &str,
        device_id: &str,
    ) -> Result<Option<DeviceTwinRecord>, ContractError>;

    fn save_twin(&self, record: DeviceTwinRecord) -> Result<(), ContractError>;
}
