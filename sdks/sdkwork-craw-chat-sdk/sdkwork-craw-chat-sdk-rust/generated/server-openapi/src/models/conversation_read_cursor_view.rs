use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConversationReadCursorView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "memberId")]
    pub member_id: String,

    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "readSeq")]
    pub read_seq: i64,

    #[serde(rename = "lastReadMessageId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_read_message_id: Option<String>,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    #[serde(rename = "unreadCount")]
    pub unread_count: i64,
}
