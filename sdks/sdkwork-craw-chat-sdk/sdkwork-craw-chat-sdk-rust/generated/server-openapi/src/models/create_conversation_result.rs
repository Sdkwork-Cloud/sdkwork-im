use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateConversationResult {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "eventId")]
    pub event_id: String,
}
