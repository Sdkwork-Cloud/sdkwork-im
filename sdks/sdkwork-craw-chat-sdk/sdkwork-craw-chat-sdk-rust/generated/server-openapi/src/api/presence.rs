use std::sync::Arc;

use crate::api::paths::backend_path;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{PresenceDeviceRequest, PresenceSnapshotView};

#[derive(Clone)]
pub struct PresenceApi {
    client: Arc<SdkworkHttpClient>,
}

impl PresenceApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Refresh device presence
    pub async fn heartbeat(&self, body: &PresenceDeviceRequest) -> Result<PresenceSnapshotView, SdkworkError> {
        let path = backend_path(&"/presence/heartbeat".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Get current presence
    pub async fn get_presence_me(&self) -> Result<PresenceSnapshotView, SdkworkError> {
        let path = backend_path(&"/presence/me".to_string());
        self.client.get(&path, None, None).await
    }
}
