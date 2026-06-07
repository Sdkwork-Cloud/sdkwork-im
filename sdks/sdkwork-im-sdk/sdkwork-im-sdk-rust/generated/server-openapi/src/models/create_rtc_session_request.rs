use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateRtcSessionRequest {
    #[serde(rename = "conversationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,

    #[serde(rename = "mediaKind")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_kind: Option<String>,
}
