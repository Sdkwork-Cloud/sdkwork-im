use std::sync::Arc;

use crate::api::paths::backend_path;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{InboxResponse};

#[derive(Clone)]
pub struct InboxApi {
    client: Arc<SdkworkHttpClient>,
}

impl InboxApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Get inbox entries
    pub async fn get_inbox(&self) -> Result<InboxResponse, SdkworkError> {
        let path = backend_path(&"/inbox".to_string());
        self.client.get(&path, None, None).await
    }
}
