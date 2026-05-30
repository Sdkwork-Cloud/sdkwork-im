use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateDeviceTwinDesiredRequest {
    #[serde(rename = "desiredStateJson")]
    pub desired_state_json: String,
}
