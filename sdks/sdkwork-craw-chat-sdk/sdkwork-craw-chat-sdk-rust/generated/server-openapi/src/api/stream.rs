use std::sync::Arc;

use crate::api::base::{QueryParams};
use crate::api::paths::backend_path;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{AbortStreamRequest, AppendStreamFrameRequest, CheckpointStreamRequest, CompleteStreamRequest, OpenStreamRequest, StreamFrame, StreamFrameWindow, StreamSession};

#[derive(Clone)]
pub struct StreamApi {
    client: Arc<SdkworkHttpClient>,
}

impl StreamApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Open a stream session
    pub async fn open(&self, body: &OpenStreamRequest) -> Result<StreamSession, SdkworkError> {
        let path = backend_path(&"/streams".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// List stream frames
    pub async fn list_stream_frames(&self, stream_id: &str, query: Option<&QueryParams>) -> Result<StreamFrameWindow, SdkworkError> {
        let path = backend_path(&format!("/streams/{}/frames", stream_id));
        self.client.get(&path, query, None).await
    }

    /// Append a frame to a stream
    pub async fn append_stream_frame(&self, stream_id: &str, body: &AppendStreamFrameRequest) -> Result<StreamFrame, SdkworkError> {
        let path = backend_path(&format!("/streams/{}/frames", stream_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Checkpoint a stream session
    pub async fn checkpoint(&self, stream_id: &str, body: &CheckpointStreamRequest) -> Result<StreamSession, SdkworkError> {
        let path = backend_path(&format!("/streams/{}/checkpoint", stream_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Complete a stream session
    pub async fn complete(&self, stream_id: &str, body: &CompleteStreamRequest) -> Result<StreamSession, SdkworkError> {
        let path = backend_path(&format!("/streams/{}/complete", stream_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Abort a stream session
    pub async fn abort(&self, stream_id: &str, body: &AbortStreamRequest) -> Result<StreamSession, SdkworkError> {
        let path = backend_path(&format!("/streams/{}/abort", stream_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }
}
