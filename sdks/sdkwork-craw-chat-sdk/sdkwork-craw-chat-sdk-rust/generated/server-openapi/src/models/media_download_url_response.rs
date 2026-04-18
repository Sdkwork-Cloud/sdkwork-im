use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MediaDownloadUrlResponse {
    #[serde(rename = "mediaAssetId")]
    pub media_asset_id: String,

    #[serde(rename = "storageProvider")]
    pub storage_provider: String,

    #[serde(rename = "downloadUrl")]
    pub download_url: String,

    #[serde(rename = "expiresInSeconds")]
    pub expires_in_seconds: i64,
}
