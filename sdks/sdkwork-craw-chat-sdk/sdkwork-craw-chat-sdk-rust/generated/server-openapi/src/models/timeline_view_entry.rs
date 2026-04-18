use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TimelineViewEntry {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "messageId")]
    pub message_id: String,

    #[serde(rename = "messageSeq")]
    pub message_seq: i64,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}
