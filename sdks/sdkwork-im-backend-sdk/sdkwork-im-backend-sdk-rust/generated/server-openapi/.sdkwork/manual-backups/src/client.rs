use std::sync::Arc;

use crate::api::{OpsApi, AuditApi, ProviderApi, IotApi, RtcApi, AutomationApi};
use crate::http::{SdkworkConfig, SdkworkError, SdkworkHttpClient};

#[derive(Clone)]
pub struct SdkworkBackendClient {
    http: Arc<SdkworkHttpClient>,
}

impl SdkworkBackendClient {
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

    pub fn ops(&self) -> OpsApi {
            OpsApi::new(Arc::clone(&self.http))
        }

    pub fn audit(&self) -> AuditApi {
            AuditApi::new(Arc::clone(&self.http))
        }

    pub fn provider(&self) -> ProviderApi {
            ProviderApi::new(Arc::clone(&self.http))
        }

    pub fn iot(&self) -> IotApi {
            IotApi::new(Arc::clone(&self.http))
        }

    pub fn rtc(&self) -> RtcApi {
            RtcApi::new(Arc::clone(&self.http))
        }

    pub fn automation(&self) -> AutomationApi {
            AutomationApi::new(Arc::clone(&self.http))
        }
}
