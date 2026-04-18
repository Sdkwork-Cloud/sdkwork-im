use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PostMessageResult {
    #[serde(rename = "messageId")]
    pub message_id: String,

    #[serde(rename = "messageSeq")]
    pub message_seq: i64,

    #[serde(rename = "eventId")]
    pub event_id: String,
}
