use serde::{Deserialize, Serialize};

use crate::models::{ConversationMember};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ChangeConversationMemberRoleResult {
    #[serde(rename = "eventId")]
    pub event_id: String,

    #[serde(rename = "changedAt")]
    pub changed_at: String,

    #[serde(rename = "previousMember")]
    pub previous_member: ConversationMember,

    #[serde(rename = "updatedMember")]
    pub updated_member: ConversationMember,
}
