use serde::{Deserialize, Serialize};

use crate::models::{DeviceSyncFeedEntry};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeviceSyncFeedResponse {
    pub items: Vec<DeviceSyncFeedEntry>,

    #[serde(rename = "nextAfterSeq")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_after_seq: Option<i64>,

    #[serde(rename = "hasMore")]
    pub has_more: bool,

    #[serde(rename = "trimmedThroughSeq")]
    pub trimmed_through_seq: i64,
}
