use serde::{Deserialize, Serialize};

use crate::models::{MediaResource};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateUploadRequest {
    #[serde(rename = "mediaAssetId")]
    pub media_asset_id: String,

    pub resource: MediaResource,
}
