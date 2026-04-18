use crate::{build_text_stream_frame, CrawChatSdkContext, QueryParams, TextFrameOptions};
use sdkwork_craw_chat_backend_sdk::{
  AbortStreamRequest,
  AppendStreamFrameRequest,
  CheckpointStreamRequest,
  CompleteStreamRequest,
  OpenStreamRequest,
  SdkworkError,
  StreamFrame,
  StreamFrameWindow,
  StreamSession,
};

#[derive(Clone)]
pub struct CrawChatStreamsModule {
  context: CrawChatSdkContext,
}

impl CrawChatStreamsModule {
  pub(crate) fn new(context: CrawChatSdkContext) -> Self {
    Self { context }
  }

  pub async fn open(&self, body: OpenStreamRequest) -> Result<StreamSession, SdkworkError> {
    self.context.backend_client().stream().open(&body).await
  }

  pub async fn list_frames(
    &self,
    stream_id: impl AsRef<str>,
    params: Option<&QueryParams>,
  ) -> Result<StreamFrameWindow, SdkworkError> {
    let stream_id = stream_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .stream()
      .list_stream_frames(&stream_id, params)
      .await
  }

  pub async fn append_frame(
    &self,
    stream_id: impl AsRef<str>,
    body: AppendStreamFrameRequest,
  ) -> Result<StreamFrame, SdkworkError> {
    let stream_id = stream_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .stream()
      .append_stream_frame(&stream_id, &body)
      .await
  }

  pub async fn append_text_frame(
    &self,
    stream_id: impl AsRef<str>,
    frame_seq: i64,
    text: impl Into<String>,
    options: TextFrameOptions,
  ) -> Result<StreamFrame, SdkworkError> {
    self
      .append_frame(stream_id, build_text_stream_frame(frame_seq, text, options))
      .await
  }

  pub async fn checkpoint(
    &self,
    stream_id: impl AsRef<str>,
    body: CheckpointStreamRequest,
  ) -> Result<StreamSession, SdkworkError> {
    let stream_id = stream_id.as_ref().to_string();
    self.context.backend_client().stream().checkpoint(&stream_id, &body).await
  }

  pub async fn complete(
    &self,
    stream_id: impl AsRef<str>,
    body: CompleteStreamRequest,
  ) -> Result<StreamSession, SdkworkError> {
    let stream_id = stream_id.as_ref().to_string();
    self.context.backend_client().stream().complete(&stream_id, &body).await
  }

  pub async fn abort(
    &self,
    stream_id: impl AsRef<str>,
    body: AbortStreamRequest,
  ) -> Result<StreamSession, SdkworkError> {
    let stream_id = stream_id.as_ref().to_string();
    self.context.backend_client().stream().abort(&stream_id, &body).await
  }
}
