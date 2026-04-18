use crate::{CrawChatSdkContext, QueryParams};
use sdkwork_craw_chat_backend_sdk::{
  DeviceSyncFeedResponse,
  RegisterDeviceRequest,
  RegisteredDeviceView,
  SdkworkError,
};

#[derive(Clone)]
pub struct CrawChatDevicesModule {
  context: CrawChatSdkContext,
}

impl CrawChatDevicesModule {
  pub(crate) fn new(context: CrawChatSdkContext) -> Self {
    Self { context }
  }

  pub async fn register(
    &self,
    body: RegisterDeviceRequest,
  ) -> Result<RegisteredDeviceView, SdkworkError> {
    self.context.backend_client().device().register(&body).await
  }

  pub async fn sync_feed(
    &self,
    device_id: impl AsRef<str>,
    params: Option<&QueryParams>,
  ) -> Result<DeviceSyncFeedResponse, SdkworkError> {
    let device_id = device_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .device()
      .get_device_sync_feed(&device_id, params)
      .await
  }
}
