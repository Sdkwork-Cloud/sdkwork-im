use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateReadCursorRequest {
    #[serde(rename = "readSeq")]
    pub read_seq: i64,

    #[serde(rename = "lastReadMessageId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_read_message_id: Option<String>,
}
