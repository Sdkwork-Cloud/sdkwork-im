use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConversationMember {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "memberId")]
    pub member_id: String,

    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "principalKind")]
    pub principal_kind: String,

    pub role: String,

    pub state: String,

    #[serde(rename = "invitedBy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<String>,

    #[serde(rename = "joinedAt")]
    pub joined_at: String,

    #[serde(rename = "removedAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removed_at: Option<String>,

    pub attributes: std::collections::HashMap<String, String>,
}
