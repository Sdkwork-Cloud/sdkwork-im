use crate::{build_text_edit, CrawChatSdkContext, EditTextOptions};
use sdkwork_craw_chat_backend_sdk::{
  EditMessageRequest,
  MessageMutationResult,
  SdkworkError,
};

#[derive(Clone)]
pub struct CrawChatMessagesModule {
  context: CrawChatSdkContext,
}

impl CrawChatMessagesModule {
  pub(crate) fn new(context: CrawChatSdkContext) -> Self {
    Self { context }
  }

  pub async fn edit(
    &self,
    message_id: impl AsRef<str>,
    body: EditMessageRequest,
  ) -> Result<MessageMutationResult, SdkworkError> {
    let message_id = message_id.as_ref().to_string();
    self.context.backend_client().message().edit(&message_id, &body).await
  }

  pub async fn edit_text(
    &self,
    message_id: impl AsRef<str>,
    text: impl Into<String>,
    options: EditTextOptions,
  ) -> Result<MessageMutationResult, SdkworkError> {
    self.edit(message_id, build_text_edit(text, options)).await
  }

  pub async fn recall(
    &self,
    message_id: impl AsRef<str>,
  ) -> Result<MessageMutationResult, SdkworkError> {
    let message_id = message_id.as_ref().to_string();
    self.context.backend_client().message().recall(&message_id).await
  }
}
