use crate::{build_text_message, CrawChatSdkContext, PostTextOptions};
use sdkwork_craw_chat_backend_sdk::{
  AddConversationMemberRequest,
  AgentHandoffStateView,
  ChangeConversationMemberRoleRequest,
  ChangeConversationMemberRoleResult,
  ConversationMember,
  ConversationReadCursorView,
  ConversationSummaryView,
  CreateAgentDialogRequest,
  CreateAgentHandoffRequest,
  CreateConversationRequest,
  CreateConversationResult,
  CreateSystemChannelRequest,
  ListMembersResponse,
  PostMessageRequest,
  PostMessageResult,
  RemoveConversationMemberRequest,
  SdkworkError,
  TimelineResponse,
  TransferConversationOwnerRequest,
  TransferConversationOwnerResult,
  UpdateReadCursorRequest,
};

#[derive(Clone)]
pub struct CrawChatConversationsModule {
  context: CrawChatSdkContext,
}

impl CrawChatConversationsModule {
  pub(crate) fn new(context: CrawChatSdkContext) -> Self {
    Self { context }
  }

  pub async fn create(
    &self,
    body: CreateConversationRequest,
  ) -> Result<CreateConversationResult, SdkworkError> {
    self
      .context
      .backend_client()
      .conversation()
      .create_conversation(&body)
      .await
  }

  pub async fn create_agent_dialog(
    &self,
    body: CreateAgentDialogRequest,
  ) -> Result<CreateConversationResult, SdkworkError> {
    self
      .context
      .backend_client()
      .conversation()
      .create_agent_dialog(&body)
      .await
  }

  pub async fn create_agent_handoff(
    &self,
    body: CreateAgentHandoffRequest,
  ) -> Result<CreateConversationResult, SdkworkError> {
    self
      .context
      .backend_client()
      .conversation()
      .create_agent_handoff(&body)
      .await
  }

  pub async fn create_system_channel(
    &self,
    body: CreateSystemChannelRequest,
  ) -> Result<CreateConversationResult, SdkworkError> {
    self
      .context
      .backend_client()
      .conversation()
      .create_system_channel(&body)
      .await
  }

  pub async fn get(
    &self,
    conversation_id: impl AsRef<str>,
  ) -> Result<ConversationSummaryView, SdkworkError> {
    let conversation_id = conversation_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .conversation()
      .get_conversation_summary(&conversation_id)
      .await
  }

  pub async fn get_agent_handoff_state(
    &self,
    conversation_id: impl AsRef<str>,
  ) -> Result<AgentHandoffStateView, SdkworkError> {
    let conversation_id = conversation_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .conversation()
      .get_agent_handoff_state(&conversation_id)
      .await
  }

  pub async fn accept_agent_handoff(
    &self,
    conversation_id: impl AsRef<str>,
  ) -> Result<AgentHandoffStateView, SdkworkError> {
    let conversation_id = conversation_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .conversation()
      .accept_agent_handoff(&conversation_id)
      .await
  }

  pub async fn resolve_agent_handoff(
    &self,
    conversation_id: impl AsRef<str>,
  ) -> Result<AgentHandoffStateView, SdkworkError> {
    let conversation_id = conversation_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .conversation()
      .resolve_agent_handoff(&conversation_id)
      .await
  }

  pub async fn close_agent_handoff(
    &self,
    conversation_id: impl AsRef<str>,
  ) -> Result<AgentHandoffStateView, SdkworkError> {
    let conversation_id = conversation_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .conversation()
      .close_agent_handoff(&conversation_id)
      .await
  }

  pub async fn list_members(
    &self,
    conversation_id: impl AsRef<str>,
  ) -> Result<ListMembersResponse, SdkworkError> {
    let conversation_id = conversation_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .conversation()
      .list_conversation_members(&conversation_id)
      .await
  }

  pub async fn add_member(
    &self,
    conversation_id: impl AsRef<str>,
    body: AddConversationMemberRequest,
  ) -> Result<ConversationMember, SdkworkError> {
    let conversation_id = conversation_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .conversation()
      .add_conversation_member(&conversation_id, &body)
      .await
  }

  pub async fn remove_member(
    &self,
    conversation_id: impl AsRef<str>,
    body: RemoveConversationMemberRequest,
  ) -> Result<ConversationMember, SdkworkError> {
    let conversation_id = conversation_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .conversation()
      .remove_conversation_member(&conversation_id, &body)
      .await
  }

  pub async fn transfer_owner(
    &self,
    conversation_id: impl AsRef<str>,
    body: TransferConversationOwnerRequest,
  ) -> Result<TransferConversationOwnerResult, SdkworkError> {
    let conversation_id = conversation_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .conversation()
      .transfer_conversation_owner(&conversation_id, &body)
      .await
  }

  pub async fn change_member_role(
    &self,
    conversation_id: impl AsRef<str>,
    body: ChangeConversationMemberRoleRequest,
  ) -> Result<ChangeConversationMemberRoleResult, SdkworkError> {
    let conversation_id = conversation_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .conversation()
      .change_conversation_member_role(&conversation_id, &body)
      .await
  }

  pub async fn leave(
    &self,
    conversation_id: impl AsRef<str>,
  ) -> Result<ConversationMember, SdkworkError> {
    let conversation_id = conversation_id.as_ref().to_string();
    self.context.backend_client().conversation().leave(&conversation_id).await
  }

  pub async fn get_read_cursor(
    &self,
    conversation_id: impl AsRef<str>,
  ) -> Result<ConversationReadCursorView, SdkworkError> {
    let conversation_id = conversation_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .conversation()
      .get_conversation_read_cursor(&conversation_id)
      .await
  }

  pub async fn update_read_cursor(
    &self,
    conversation_id: impl AsRef<str>,
    body: UpdateReadCursorRequest,
  ) -> Result<ConversationReadCursorView, SdkworkError> {
    let conversation_id = conversation_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .conversation()
      .update_conversation_read_cursor(&conversation_id, &body)
      .await
  }

  pub async fn list_messages(
    &self,
    conversation_id: impl AsRef<str>,
  ) -> Result<TimelineResponse, SdkworkError> {
    let conversation_id = conversation_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .conversation()
      .list_conversation_messages(&conversation_id)
      .await
  }

  pub async fn post_message(
    &self,
    conversation_id: impl AsRef<str>,
    body: PostMessageRequest,
  ) -> Result<PostMessageResult, SdkworkError> {
    let conversation_id = conversation_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .conversation()
      .post_conversation_message(&conversation_id, &body)
      .await
  }

  pub async fn post_text(
    &self,
    conversation_id: impl AsRef<str>,
    text: impl Into<String>,
    options: PostTextOptions,
  ) -> Result<PostMessageResult, SdkworkError> {
    self.post_message(conversation_id, build_text_message(text, options)).await
  }

  pub async fn publish_system_message(
    &self,
    conversation_id: impl AsRef<str>,
    body: PostMessageRequest,
  ) -> Result<PostMessageResult, SdkworkError> {
    let conversation_id = conversation_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .conversation()
      .publish_system_channel_message(&conversation_id, &body)
      .await
  }

  pub async fn publish_system_text(
    &self,
    conversation_id: impl AsRef<str>,
    text: impl Into<String>,
    options: PostTextOptions,
  ) -> Result<PostMessageResult, SdkworkError> {
    self
      .publish_system_message(conversation_id, build_text_message(text, options))
      .await
  }
}
