use std::sync::Arc;

use crate::api::base::{QueryParams};
use crate::api::paths::backend_path;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{AttachMediaRequest, CompleteUploadRequest, CreateUploadRequest, MediaAsset, MediaDownloadUrlResponse, PostMessageResult};

#[derive(Clone)]
pub struct MediaApi {
    client: Arc<SdkworkHttpClient>,
}

impl MediaApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Create a media upload record
    pub async fn create_media_upload(&self, body: &CreateUploadRequest) -> Result<MediaAsset, SdkworkError> {
        let path = backend_path(&"/media/uploads".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Complete a media upload
    pub async fn complete_media_upload(&self, media_asset_id: &str, body: &CompleteUploadRequest) -> Result<MediaAsset, SdkworkError> {
        let path = backend_path(&format!("/media/uploads/{}/complete", media_asset_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Issue a signed media download URL
    pub async fn get_media_download_url(&self, media_asset_id: &str, query: Option<&QueryParams>) -> Result<MediaDownloadUrlResponse, SdkworkError> {
        let path = backend_path(&format!("/media/{}/download-url", media_asset_id));
        self.client.get(&path, query, None).await
    }

    /// Get a media asset by id
    pub async fn get_media_asset(&self, media_asset_id: &str) -> Result<MediaAsset, SdkworkError> {
        let path = backend_path(&format!("/media/{}", media_asset_id));
        self.client.get(&path, None, None).await
    }

    /// Attach a ready media asset as a conversation message
    pub async fn attach_media_asset(&self, media_asset_id: &str, body: &AttachMediaRequest) -> Result<PostMessageResult, SdkworkError> {
        let path = backend_path(&format!("/media/{}/attach", media_asset_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }
}
