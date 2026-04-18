use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateAgentDialogRequest {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "agentId")]
    pub agent_id: String,
}
