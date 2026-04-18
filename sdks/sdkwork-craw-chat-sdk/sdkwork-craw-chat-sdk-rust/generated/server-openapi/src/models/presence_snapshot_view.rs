use serde::{Deserialize, Serialize};

use crate::models::{DevicePresenceView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PresenceSnapshotView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "currentDeviceId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_device_id: Option<String>,

    pub devices: Vec<DevicePresenceView>,
}
