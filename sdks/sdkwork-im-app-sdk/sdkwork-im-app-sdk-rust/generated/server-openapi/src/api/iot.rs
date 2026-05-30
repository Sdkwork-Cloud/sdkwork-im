use std::sync::Arc;

use crate::api::paths::app_path;
use crate::http::{SdkworkError, SdkworkHttpClient};

#[derive(Clone)]
pub struct IotApi {
    client: Arc<SdkworkHttpClient>,
}

impl IotApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Retrieve IoT access provider health
    pub async fn access_provider_health_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = app_path(&"/iot/access/provider_health".to_string());
        self.client.get(&path, None, None).await
    }

    /// Retrieve IoT protocol provider health
    pub async fn protocol_provider_health_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = app_path(&"/iot/protocol/provider_health".to_string());
        self.client.get(&path, None, None).await
    }

    /// Ingest IoT protocol uplink
    pub async fn protocol_uplink_create(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = app_path(&"/iot/protocol/uplink".to_string());
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Ingest IoT protocol downlink
    pub async fn protocol_downlink_create(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = app_path(&"/iot/protocol/downlink".to_string());
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

}
