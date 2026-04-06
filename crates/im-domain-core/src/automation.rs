use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AutomationExecutionState {
    Requested,
    Running,
    Succeeded,
    Failed,
}

impl AutomationExecutionState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::Running => "running",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomationExecution {
    pub tenant_id: String,
    pub principal_id: String,
    pub principal_kind: String,
    pub execution_id: String,
    pub trigger_type: String,
    pub target_kind: String,
    pub target_ref: String,
    pub input_payload: Option<String>,
    pub output_payload: Option<String>,
    pub state: AutomationExecutionState,
    pub retry_count: u32,
    pub requested_at: String,
    pub completed_at: Option<String>,
    pub failure_reason: Option<String>,
}
