use serde::{Deserialize, Serialize};

use crate::models::{MediaResource};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ContentPart {
    pub kind: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(rename = "schemaRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_ref: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,

    #[serde(rename = "mediaAssetId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_asset_id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<MediaResource>,

    #[serde(rename = "signalType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signal_type: Option<String>,

    #[serde(rename = "streamId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,

    #[serde(rename = "streamType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_type: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
