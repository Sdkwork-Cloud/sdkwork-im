use std::sync::Arc;

use crate::api::base::{QueryParams};
use crate::api::paths::backend_path;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{DeviceSyncFeedResponse, RegisterDeviceRequest, RegisteredDeviceView};

#[derive(Clone)]
pub struct DeviceApi {
    client: Arc<SdkworkHttpClient>,
}

impl DeviceApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Register the current device
    pub async fn register(&self, body: &RegisterDeviceRequest) -> Result<RegisteredDeviceView, SdkworkError> {
        let path = backend_path(&"/devices/register".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Get device sync feed entries
    pub async fn get_device_sync_feed(&self, device_id: &str, query: Option<&QueryParams>) -> Result<DeviceSyncFeedResponse, SdkworkError> {
        let path = backend_path(&format!("/devices/{}/sync-feed", device_id));
        self.client.get(&path, query, None).await
    }
}
