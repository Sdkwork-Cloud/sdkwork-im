use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeviceTwinView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "deviceId")]
    pub device_id: String,

    #[serde(rename = "desiredStateJson")]
    pub desired_state_json: String,

    #[serde(rename = "reportedStateJson")]
    pub reported_state_json: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
