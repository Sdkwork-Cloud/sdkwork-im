use crate::{CrawChatSdkContext, QueryParams};
use sdkwork_craw_chat_backend_sdk::{
  AttachMediaRequest,
  CompleteUploadRequest,
  CreateUploadRequest,
  MediaAsset,
  MediaDownloadUrlResponse,
  PostMessageResult,
  SdkworkError,
};

#[derive(Clone)]
pub struct CrawChatMediaModule {
  context: CrawChatSdkContext,
}

impl CrawChatMediaModule {
  pub(crate) fn new(context: CrawChatSdkContext) -> Self {
    Self { context }
  }

  pub async fn create_upload(
    &self,
    body: CreateUploadRequest,
  ) -> Result<MediaAsset, SdkworkError> {
    self.context.backend_client().media().create_media_upload(&body).await
  }

  pub async fn complete_upload(
    &self,
    media_asset_id: impl AsRef<str>,
    body: CompleteUploadRequest,
  ) -> Result<MediaAsset, SdkworkError> {
    let media_asset_id = media_asset_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .media()
      .complete_media_upload(&media_asset_id, &body)
      .await
  }

  pub async fn download_url(
    &self,
    media_asset_id: impl AsRef<str>,
    params: Option<&QueryParams>,
  ) -> Result<MediaDownloadUrlResponse, SdkworkError> {
    let media_asset_id = media_asset_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .media()
      .get_media_download_url(&media_asset_id, params)
      .await
  }

  pub async fn get(
    &self,
    media_asset_id: impl AsRef<str>,
  ) -> Result<MediaAsset, SdkworkError> {
    let media_asset_id = media_asset_id.as_ref().to_string();
    self.context.backend_client().media().get_media_asset(&media_asset_id).await
  }

  pub async fn attach(
    &self,
    media_asset_id: impl AsRef<str>,
    body: AttachMediaRequest,
  ) -> Result<PostMessageResult, SdkworkError> {
    let media_asset_id = media_asset_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .media()
      .attach_media_asset(&media_asset_id, &body)
      .await
  }
}
