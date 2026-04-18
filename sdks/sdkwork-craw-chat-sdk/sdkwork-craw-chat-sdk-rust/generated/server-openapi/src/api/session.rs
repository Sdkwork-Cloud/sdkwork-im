use std::sync::Arc;

use crate::api::paths::backend_path;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{PresenceDeviceRequest, PresenceSnapshotView, ResumeSessionRequest, SessionResumeView};

#[derive(Clone)]
pub struct SessionApi {
    client: Arc<SdkworkHttpClient>,
}

impl SessionApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Resume the current app session
    pub async fn resume(&self, body: &ResumeSessionRequest) -> Result<SessionResumeView, SdkworkError> {
        let path = backend_path(&"/sessions/resume".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Disconnect the current app session device route
    pub async fn disconnect(&self, body: &PresenceDeviceRequest) -> Result<PresenceSnapshotView, SdkworkError> {
        let path = backend_path(&"/sessions/disconnect".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }
}
