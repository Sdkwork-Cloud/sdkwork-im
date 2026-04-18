use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OpenStreamRequest {
    #[serde(rename = "streamId")]
    pub stream_id: String,

    #[serde(rename = "streamType")]
    pub stream_type: String,

    #[serde(rename = "scopeKind")]
    pub scope_kind: String,

    #[serde(rename = "scopeId")]
    pub scope_id: String,

    #[serde(rename = "durabilityClass")]
    pub durability_class: String,

    #[serde(rename = "schemaRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_ref: Option<String>,
}
