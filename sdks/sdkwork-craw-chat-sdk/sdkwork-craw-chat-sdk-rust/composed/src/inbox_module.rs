use crate::CrawChatSdkContext;
use sdkwork_craw_chat_backend_sdk::{InboxResponse, SdkworkError};

#[derive(Clone)]
pub struct CrawChatInboxModule {
  context: CrawChatSdkContext,
}

impl CrawChatInboxModule {
  pub(crate) fn new(context: CrawChatSdkContext) -> Self {
    Self { context }
  }

  pub async fn get(&self) -> Result<InboxResponse, SdkworkError> {
    self.context.backend_client().inbox().get_inbox().await
  }
}
