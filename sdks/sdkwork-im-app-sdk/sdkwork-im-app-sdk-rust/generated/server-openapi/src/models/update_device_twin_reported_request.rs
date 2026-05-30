use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateDeviceTwinReportedRequest {
    #[serde(rename = "reportedStateJson")]
    pub reported_state_json: String,
}
