use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ChangeAgentHandoffStatusView {
    pub id: String,

    pub kind: String,
}
