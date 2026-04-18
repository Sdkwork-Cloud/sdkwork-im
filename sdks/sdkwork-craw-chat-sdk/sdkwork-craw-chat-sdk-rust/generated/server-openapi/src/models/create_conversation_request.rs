use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateConversationRequest {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "conversationType")]
    pub conversation_type: String,
}
