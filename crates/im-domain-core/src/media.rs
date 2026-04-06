use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub type MediaMetadata = BTreeMap<String, String>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaResourceType {
    Image,
    Video,
    Audio,
    File,
}

impl MediaResourceType {
    pub fn as_wire_value(&self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Video => "video",
            Self::Audio => "audio",
            Self::File => "file",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaResource {
    pub id: Option<u64>,
    pub uuid: Option<String>,
    pub url: Option<String>,
    pub bytes: Option<Vec<u8>>,
    pub local_file: Option<String>,
    pub base64: Option<String>,
    #[serde(rename = "type")]
    pub resource_type: Option<MediaResourceType>,
    pub mime_type: Option<String>,
    pub size: Option<u64>,
    pub name: Option<String>,
    pub extension: Option<String>,
    pub tags: Option<MediaMetadata>,
    pub metadata: Option<MediaMetadata>,
    pub prompt: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MediaProcessingState {
    PendingUpload,
    Ready,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaAsset {
    pub tenant_id: String,
    pub principal_id: String,
    pub principal_kind: String,
    pub media_asset_id: String,
    pub bucket: Option<String>,
    pub object_key: Option<String>,
    pub storage_provider: Option<String>,
    pub checksum: Option<String>,
    pub processing_state: MediaProcessingState,
    pub resource: MediaResource,
    pub created_at: String,
    pub completed_at: Option<String>,
}
