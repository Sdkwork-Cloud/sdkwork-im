use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RegisteredDeviceView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "deviceId")]
    pub device_id: String,

    #[serde(rename = "registeredAt")]
    pub registered_at: String,
}
