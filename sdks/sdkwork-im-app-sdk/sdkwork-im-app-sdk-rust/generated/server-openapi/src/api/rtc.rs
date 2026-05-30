use std::sync::Arc;

use crate::api::paths::app_path;
use crate::http::{SdkworkError, SdkworkHttpClient};

#[derive(Clone)]
pub struct RtcApi {
    client: Arc<SdkworkHttpClient>,
}

impl RtcApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Map RTC provider callback
    pub async fn provider_callbacks_create(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = app_path(&"/rtc/provider_callbacks".to_string());
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Retrieve RTC provider health
    pub async fn provider_health_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = app_path(&"/rtc/provider_health".to_string());
        self.client.get(&path, None, None).await
    }

}
