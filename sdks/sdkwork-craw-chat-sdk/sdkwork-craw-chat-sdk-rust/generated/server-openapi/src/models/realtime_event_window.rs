use serde::{Deserialize, Serialize};

use crate::models::{RealtimeEvent};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RealtimeEventWindow {
    #[serde(rename = "deviceId")]
    pub device_id: String,

    pub items: Vec<RealtimeEvent>,

    #[serde(rename = "nextAfterSeq")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_after_seq: Option<i64>,

    #[serde(rename = "hasMore")]
    pub has_more: bool,

    #[serde(rename = "ackedThroughSeq")]
    pub acked_through_seq: i64,

    #[serde(rename = "trimmedThroughSeq")]
    pub trimmed_through_seq: i64,
}
