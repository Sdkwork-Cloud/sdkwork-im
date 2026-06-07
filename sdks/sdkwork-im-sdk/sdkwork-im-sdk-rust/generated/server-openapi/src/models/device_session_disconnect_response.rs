use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeviceSessionDisconnectResponse {
    #[serde(rename = "deviceId")]
    pub device_id: String,

    pub disconnected: bool,
}
