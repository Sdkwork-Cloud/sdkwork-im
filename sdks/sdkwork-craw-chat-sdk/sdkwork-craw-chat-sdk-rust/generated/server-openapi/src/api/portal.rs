use std::sync::Arc;

use crate::api::paths::backend_path;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{PortalWorkspaceView};

#[derive(Clone)]
pub struct PortalApi {
    client: Arc<SdkworkHttpClient>,
}

impl PortalApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Read the tenant portal home snapshot
    pub async fn get_home(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/portal/home".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the tenant portal sign-in snapshot
    pub async fn get_auth(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/portal/auth".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the current tenant workspace snapshot
    pub async fn get_workspace(&self) -> Result<PortalWorkspaceView, SdkworkError> {
        let path = backend_path(&"/portal/workspace".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the tenant dashboard snapshot
    pub async fn get_dashboard(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/portal/dashboard".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the tenant conversations snapshot
    pub async fn get_conversations(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/portal/conversations".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the tenant realtime snapshot
    pub async fn get_realtime(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/portal/realtime".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the tenant media snapshot
    pub async fn get_media(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/portal/media".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the tenant automation snapshot
    pub async fn get_automation(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/portal/automation".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the tenant governance snapshot
    pub async fn get_governance(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/portal/governance".to_string());
        self.client.get(&path, None, None).await
    }
}
