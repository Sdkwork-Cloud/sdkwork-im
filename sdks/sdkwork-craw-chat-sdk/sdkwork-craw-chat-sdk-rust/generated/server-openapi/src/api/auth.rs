use std::sync::Arc;

use crate::api::paths::backend_path;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{PortalLoginRequest, PortalLoginResponse, PortalMeResponse};

#[derive(Clone)]
pub struct AuthApi {
    client: Arc<SdkworkHttpClient>,
}

impl AuthApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Sign in to the tenant portal
    pub async fn login(&self, body: &PortalLoginRequest) -> Result<PortalLoginResponse, SdkworkError> {
        let path = backend_path(&"/auth/login".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Read the current portal session
    pub async fn me(&self) -> Result<PortalMeResponse, SdkworkError> {
        let path = backend_path(&"/auth/me".to_string());
        self.client.get(&path, None, None).await
    }
}
