use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AttachMediaRequest {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "clientMsgId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_msg_id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(rename = "renderHints")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub render_hints: Option<std::collections::HashMap<String, String>>,
}
