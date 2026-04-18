use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DevicePresenceView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "deviceId")]
    pub device_id: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,

    #[serde(rename = "sessionId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    pub status: String,

    #[serde(rename = "lastSyncSeq")]
    pub last_sync_seq: i64,

    #[serde(rename = "lastResumeAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_resume_at: Option<String>,

    #[serde(rename = "lastSeenAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<String>,
}
