use std::sync::Arc;

use crate::api::{PortalApi, DeviceApi, PresenceApi, RealtimeApi, SocialApi, ChatApi, MediaApi, StreamApi, RtcApi, NotificationApi, AutomationApi};
use crate::http::{SdkworkConfig, SdkworkError, SdkworkHttpClient};

#[derive(Clone)]
pub struct SdkworkAppClient {
    http: Arc<SdkworkHttpClient>,
}

impl SdkworkAppClient {
    pub fn new(config: SdkworkConfig) -> Result<Self, SdkworkError> {
        Ok(Self {
            http: Arc::new(SdkworkHttpClient::new(config)?),
        })
    }

    pub fn new_with_base_url(base_url: impl Into<String>) -> Result<Self, SdkworkError> {
        Self::new(SdkworkConfig::new(base_url))
    }


    pub fn set_auth_token(&self, token: impl Into<String>) -> &Self {
        self.http.set_auth_token(token);
        self
    }

    pub fn set_access_token(&self, token: impl Into<String>) -> &Self {
        self.http.set_access_token(token);
        self
    }

    pub fn set_header(&self, key: impl Into<String>, value: impl Into<String>) -> &Self {
        self.http.set_header(key, value);
        self
    }

    pub fn http_client(&self) -> Arc<SdkworkHttpClient> {
        Arc::clone(&self.http)
    }

    pub fn portal(&self) -> PortalApi {
            PortalApi::new(Arc::clone(&self.http))
        }

    pub fn device(&self) -> DeviceApi {
            DeviceApi::new(Arc::clone(&self.http))
        }

    pub fn presence(&self) -> PresenceApi {
            PresenceApi::new(Arc::clone(&self.http))
        }

    pub fn realtime(&self) -> RealtimeApi {
            RealtimeApi::new(Arc::clone(&self.http))
        }

    pub fn social(&self) -> SocialApi {
            SocialApi::new(Arc::clone(&self.http))
        }

    pub fn chat(&self) -> ChatApi {
            ChatApi::new(Arc::clone(&self.http))
        }

    pub fn media(&self) -> MediaApi {
            MediaApi::new(Arc::clone(&self.http))
        }

    pub fn stream(&self) -> StreamApi {
            StreamApi::new(Arc::clone(&self.http))
        }

    pub fn rtc(&self) -> RtcApi {
            RtcApi::new(Arc::clone(&self.http))
        }

    pub fn notification(&self) -> NotificationApi {
            NotificationApi::new(Arc::clone(&self.http))
        }

    pub fn automation(&self) -> AutomationApi {
            AutomationApi::new(Arc::clone(&self.http))
        }
}
