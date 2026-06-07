use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RtcSession {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "rtcSessionId")]
    pub rtc_session_id: String,

    #[serde(rename = "conversationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,

    #[serde(rename = "providerPluginId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_plugin_id: Option<String>,

    #[serde(rename = "providerSessionId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_session_id: Option<String>,

    #[serde(rename = "rtcMode")]
    pub rtc_mode: String,

    pub state: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
