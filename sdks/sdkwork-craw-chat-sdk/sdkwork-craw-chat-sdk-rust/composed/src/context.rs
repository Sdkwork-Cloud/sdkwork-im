use sdkwork_craw_chat_backend_sdk::{SdkworkBackendClient, SdkworkConfig, SdkworkError};

#[derive(Clone)]
pub struct CrawChatSdkContext {
  backend_client: SdkworkBackendClient,
}

impl CrawChatSdkContext {
  pub fn new(backend_client: SdkworkBackendClient) -> Self {
    Self { backend_client }
  }

  pub fn new_with_base_url(base_url: impl Into<String>) -> Result<Self, SdkworkError> {
    let backend_client = SdkworkBackendClient::new(SdkworkConfig::new(base_url))?;
    Ok(Self::new(backend_client))
  }

  pub fn backend_client(&self) -> &SdkworkBackendClient {
    &self.backend_client
  }

  pub fn set_auth_token(&self, token: impl Into<String>) -> &Self {
    self.backend_client.set_auth_token(token);
    self
  }
}
