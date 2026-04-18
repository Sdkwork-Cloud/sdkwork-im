use serde::{Deserialize, Serialize};

use crate::models::{StreamFrame};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StreamFrameWindow {
    pub items: Vec<StreamFrame>,

    #[serde(rename = "nextAfterFrameSeq")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_after_frame_seq: Option<i64>,

    #[serde(rename = "hasMore")]
    pub has_more: bool,
}
