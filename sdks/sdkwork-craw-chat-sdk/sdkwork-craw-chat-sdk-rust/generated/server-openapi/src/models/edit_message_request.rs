use serde::{Deserialize, Serialize};

use crate::models::{ContentPart};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EditMessageRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<ContentPart>>,

    #[serde(rename = "renderHints")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub render_hints: Option<std::collections::HashMap<String, String>>,
}
