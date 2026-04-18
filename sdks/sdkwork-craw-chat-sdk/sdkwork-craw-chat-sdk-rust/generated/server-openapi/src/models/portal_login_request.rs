use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalLoginRequest {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    pub login: String,

    pub password: String,

    #[serde(rename = "deviceId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,

    #[serde(rename = "sessionId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    #[serde(rename = "clientKind")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_kind: Option<String>,
}
