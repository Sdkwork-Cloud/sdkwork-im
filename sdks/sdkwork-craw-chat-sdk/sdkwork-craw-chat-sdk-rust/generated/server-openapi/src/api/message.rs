use std::sync::Arc;

use crate::api::paths::backend_path;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{EditMessageRequest, MessageMutationResult};

#[derive(Clone)]
pub struct MessageApi {
    client: Arc<SdkworkHttpClient>,
}

impl MessageApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Edit a posted message
    pub async fn edit(&self, message_id: &str, body: &EditMessageRequest) -> Result<MessageMutationResult, SdkworkError> {
        let path = backend_path(&format!("/messages/{}/edit", message_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Recall a posted message
    pub async fn recall(&self, message_id: &str) -> Result<MessageMutationResult, SdkworkError> {
        let path = backend_path(&format!("/messages/{}/recall", message_id));
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }
}
