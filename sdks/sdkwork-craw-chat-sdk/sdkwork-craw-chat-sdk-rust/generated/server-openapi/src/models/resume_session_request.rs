use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ResumeSessionRequest {
    #[serde(rename = "deviceId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,

    #[serde(rename = "lastSeenSyncSeq")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_sync_seq: Option<i64>,
}
