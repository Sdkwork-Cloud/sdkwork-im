use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AbortStreamRequest {
    #[serde(rename = "frameSeq")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame_seq: Option<i64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
