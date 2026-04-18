use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AddConversationMemberRequest {
    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "principalKind")]
    pub principal_kind: String,

    pub role: String,
}
