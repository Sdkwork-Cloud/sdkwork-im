use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeviceSessionView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "principalKind")]
    pub principal_kind: String,

    #[serde(rename = "deviceId")]
    pub device_id: String,

    #[serde(rename = "resumedAt")]
    pub resumed_at: String,
}
