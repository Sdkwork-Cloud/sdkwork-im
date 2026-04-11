use craw_chat_contract_core::ContractError;
use im_domain_core::automation::AutomationExecution;
use im_domain_core::message::{MessageAttributes, Sender};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentSubject {
    pub agent_id: String,
    pub session_id: Option<String>,
    pub metadata: MessageAttributes,
}

impl AgentSubject {
    pub fn sender(&self, member_id: Option<String>) -> Sender {
        Sender {
            id: self.agent_id.clone(),
            kind: "agent".into(),
            member_id,
            device_id: None,
            session_id: self.session_id.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentSubjectRecord {
    pub tenant_id: String,
    pub agent: AgentSubject,
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

pub trait AgentSubjectStore: Send + Sync {
    fn load_subject(
        &self,
        tenant_id: &str,
        agent_id: &str,
    ) -> Result<Option<AgentSubjectRecord>, ContractError>;

    fn save_subject(&self, record: AgentSubjectRecord) -> Result<(), ContractError>;
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
