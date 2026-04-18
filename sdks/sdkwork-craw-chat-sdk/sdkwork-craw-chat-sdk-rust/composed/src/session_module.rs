use crate::CrawChatSdkContext;
use sdkwork_craw_chat_backend_sdk::{
  PresenceDeviceRequest,
  PresenceSnapshotView,
  ResumeSessionRequest,
  SdkworkError,
  SessionResumeView,
};

#[derive(Clone)]
pub struct CrawChatSessionModule {
  context: CrawChatSdkContext,
}

impl CrawChatSessionModule {
  pub(crate) fn new(context: CrawChatSdkContext) -> Self {
    Self { context }
  }

  pub async fn resume(&self, body: ResumeSessionRequest) -> Result<SessionResumeView, SdkworkError> {
    self.context.backend_client().session().resume(&body).await
  }

  pub async fn disconnect(
    &self,
    body: PresenceDeviceRequest,
  ) -> Result<PresenceSnapshotView, SdkworkError> {
    self.context.backend_client().session().disconnect(&body).await
  }
}
