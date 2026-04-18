use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CompleteUploadRequest {
    pub bucket: String,

    #[serde(rename = "objectKey")]
    pub object_key: String,

    #[serde(rename = "storageProvider")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_provider: Option<String>,

    pub url: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
}
