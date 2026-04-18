use serde::{Deserialize, Serialize};

use crate::models::{ContentPart};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    pub parts: Vec<ContentPart>,

    #[serde(rename = "renderHints")]
    pub render_hints: std::collections::HashMap<String, String>,
}
