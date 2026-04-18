use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CheckpointStreamRequest {
    #[serde(rename = "frameSeq")]
    pub frame_seq: i64,
}
