use serde::{Deserialize, Serialize};

use crate::models::{ConversationMember};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TransferConversationOwnerResult {
    #[serde(rename = "eventId")]
    pub event_id: String,

    #[serde(rename = "transferredAt")]
    pub transferred_at: String,

    #[serde(rename = "previousOwner")]
    pub previous_owner: ConversationMember,

    #[serde(rename = "newOwner")]
    pub new_owner: ConversationMember,
}
