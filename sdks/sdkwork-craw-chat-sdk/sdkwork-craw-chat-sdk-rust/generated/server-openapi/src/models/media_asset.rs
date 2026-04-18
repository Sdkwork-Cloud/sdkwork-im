use serde::{Deserialize, Serialize};

use crate::models::{MediaResource};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MediaAsset {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "principalKind")]
    pub principal_kind: String,

    #[serde(rename = "mediaAssetId")]
    pub media_asset_id: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,

    #[serde(rename = "objectKey")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_key: Option<String>,

    #[serde(rename = "storageProvider")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_provider: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,

    #[serde(rename = "processingState")]
    pub processing_state: String,

    pub resource: MediaResource,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "completedAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}
