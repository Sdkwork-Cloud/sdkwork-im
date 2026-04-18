use crate::CrawChatSdkContext;
use sdkwork_craw_chat_backend_sdk::{PresenceDeviceRequest, PresenceSnapshotView, SdkworkError};

#[derive(Clone)]
pub struct CrawChatPresenceModule {
  context: CrawChatSdkContext,
}

impl CrawChatPresenceModule {
  pub(crate) fn new(context: CrawChatSdkContext) -> Self {
    Self { context }
  }

  pub async fn heartbeat(
    &self,
    body: PresenceDeviceRequest,
  ) -> Result<PresenceSnapshotView, SdkworkError> {
    self.context.backend_client().presence().heartbeat(&body).await
  }

  pub async fn me(&self) -> Result<PresenceSnapshotView, SdkworkError> {
    self.context.backend_client().presence().get_presence_me().await
  }
}
