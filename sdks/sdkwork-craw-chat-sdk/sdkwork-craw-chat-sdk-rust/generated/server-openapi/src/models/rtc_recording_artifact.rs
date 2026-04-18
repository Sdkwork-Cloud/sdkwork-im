use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RtcRecordingArtifact {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "rtcSessionId")]
    pub rtc_session_id: String,

    pub bucket: String,

    #[serde(rename = "objectKey")]
    pub object_key: String,

    #[serde(rename = "storageProvider")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_provider: Option<String>,

    #[serde(rename = "playbackUrl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub playback_url: Option<String>,
}
