use crate::{
  CrawChatConversationsModule,
  CrawChatDevicesModule,
  CrawChatInboxModule,
  CrawChatMediaModule,
  CrawChatMessagesModule,
  CrawChatPresenceModule,
  CrawChatRealtimeModule,
  CrawChatRtcModule,
  CrawChatSdkContext,
  CrawChatSessionModule,
  CrawChatStreamsModule,
};
use sdkwork_craw_chat_backend_sdk::{SdkworkBackendClient, SdkworkConfig, SdkworkError};

#[derive(Clone)]
pub struct CrawChatClient {
  context: CrawChatSdkContext,
}

impl CrawChatClient {
  pub fn new(backend_client: SdkworkBackendClient) -> Self {
    Self {
      context: CrawChatSdkContext::new(backend_client),
    }
  }

  pub fn new_with_base_url(base_url: impl Into<String>) -> Result<Self, SdkworkError> {
    let backend_client = SdkworkBackendClient::new(SdkworkConfig::new(base_url))?;
    Ok(Self::new(backend_client))
  }

  pub fn context(&self) -> &CrawChatSdkContext {
    &self.context
  }

  pub fn backend_client(&self) -> &SdkworkBackendClient {
    self.context.backend_client()
  }

  pub fn session(&self) -> CrawChatSessionModule {
    CrawChatSessionModule::new(self.context.clone())
  }

  pub fn presence(&self) -> CrawChatPresenceModule {
    CrawChatPresenceModule::new(self.context.clone())
  }

  pub fn realtime(&self) -> CrawChatRealtimeModule {
    CrawChatRealtimeModule::new(self.context.clone())
  }

  pub fn devices(&self) -> CrawChatDevicesModule {
    CrawChatDevicesModule::new(self.context.clone())
  }

  pub fn inbox(&self) -> CrawChatInboxModule {
    CrawChatInboxModule::new(self.context.clone())
  }

  pub fn conversations(&self) -> CrawChatConversationsModule {
    CrawChatConversationsModule::new(self.context.clone())
  }

  pub fn messages(&self) -> CrawChatMessagesModule {
    CrawChatMessagesModule::new(self.context.clone())
  }

  pub fn media(&self) -> CrawChatMediaModule {
    CrawChatMediaModule::new(self.context.clone())
  }

  pub fn streams(&self) -> CrawChatStreamsModule {
    CrawChatStreamsModule::new(self.context.clone())
  }

  pub fn rtc(&self) -> CrawChatRtcModule {
    CrawChatRtcModule::new(self.context.clone())
  }

  pub fn set_auth_token(&self, token: impl Into<String>) -> &Self {
    self.context.set_auth_token(token);
    self
  }
}
