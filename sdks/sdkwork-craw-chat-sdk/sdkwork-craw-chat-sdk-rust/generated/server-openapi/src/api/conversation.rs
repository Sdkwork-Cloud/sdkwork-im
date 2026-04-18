use std::sync::Arc;

use crate::api::paths::backend_path;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{AddConversationMemberRequest, AgentHandoffStateView, ChangeConversationMemberRoleRequest, ChangeConversationMemberRoleResult, ConversationMember, ConversationReadCursorView, ConversationSummaryView, CreateAgentDialogRequest, CreateAgentHandoffRequest, CreateConversationRequest, CreateConversationResult, CreateSystemChannelRequest, ListMembersResponse, PostMessageRequest, PostMessageResult, RemoveConversationMemberRequest, TimelineResponse, TransferConversationOwnerRequest, TransferConversationOwnerResult, UpdateReadCursorRequest};

#[derive(Clone)]
pub struct ConversationApi {
    client: Arc<SdkworkHttpClient>,
}

impl ConversationApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Create a conversation
    pub async fn create_conversation(&self, body: &CreateConversationRequest) -> Result<CreateConversationResult, SdkworkError> {
        let path = backend_path(&"/conversations".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Create an agent dialog conversation
    pub async fn create_agent_dialog(&self, body: &CreateAgentDialogRequest) -> Result<CreateConversationResult, SdkworkError> {
        let path = backend_path(&"/conversations/agent-dialogs".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Create an agent handoff conversation
    pub async fn create_agent_handoff(&self, body: &CreateAgentHandoffRequest) -> Result<CreateConversationResult, SdkworkError> {
        let path = backend_path(&"/conversations/agent-handoffs".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Create a system channel conversation
    pub async fn create_system_channel(&self, body: &CreateSystemChannelRequest) -> Result<CreateConversationResult, SdkworkError> {
        let path = backend_path(&"/conversations/system-channels".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Get projected conversation summary
    pub async fn get_conversation_summary(&self, conversation_id: &str) -> Result<ConversationSummaryView, SdkworkError> {
        let path = backend_path(&format!("/conversations/{}", conversation_id));
        self.client.get(&path, None, None).await
    }

    /// Get current agent handoff state
    pub async fn get_agent_handoff_state(&self, conversation_id: &str) -> Result<AgentHandoffStateView, SdkworkError> {
        let path = backend_path(&format!("/conversations/{}/agent-handoff", conversation_id));
        self.client.get(&path, None, None).await
    }

    /// Accept an agent handoff
    pub async fn accept_agent_handoff(&self, conversation_id: &str) -> Result<AgentHandoffStateView, SdkworkError> {
        let path = backend_path(&format!("/conversations/{}/agent-handoff/accept", conversation_id));
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Resolve an accepted agent handoff
    pub async fn resolve_agent_handoff(&self, conversation_id: &str) -> Result<AgentHandoffStateView, SdkworkError> {
        let path = backend_path(&format!("/conversations/{}/agent-handoff/resolve", conversation_id));
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Close an agent handoff
    pub async fn close_agent_handoff(&self, conversation_id: &str) -> Result<AgentHandoffStateView, SdkworkError> {
        let path = backend_path(&format!("/conversations/{}/agent-handoff/close", conversation_id));
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// List members in a conversation
    pub async fn list_conversation_members(&self, conversation_id: &str) -> Result<ListMembersResponse, SdkworkError> {
        let path = backend_path(&format!("/conversations/{}/members", conversation_id));
        self.client.get(&path, None, None).await
    }

    /// Add a member to a conversation
    pub async fn add_conversation_member(&self, conversation_id: &str, body: &AddConversationMemberRequest) -> Result<ConversationMember, SdkworkError> {
        let path = backend_path(&format!("/conversations/{}/members/add", conversation_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Remove a member from a conversation
    pub async fn remove_conversation_member(&self, conversation_id: &str, body: &RemoveConversationMemberRequest) -> Result<ConversationMember, SdkworkError> {
        let path = backend_path(&format!("/conversations/{}/members/remove", conversation_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Transfer conversation ownership
    pub async fn transfer_conversation_owner(&self, conversation_id: &str, body: &TransferConversationOwnerRequest) -> Result<TransferConversationOwnerResult, SdkworkError> {
        let path = backend_path(&format!("/conversations/{}/members/transfer-owner", conversation_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Change a conversation member role
    pub async fn change_conversation_member_role(&self, conversation_id: &str, body: &ChangeConversationMemberRoleRequest) -> Result<ChangeConversationMemberRoleResult, SdkworkError> {
        let path = backend_path(&format!("/conversations/{}/members/change-role", conversation_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Leave a conversation
    pub async fn leave(&self, conversation_id: &str) -> Result<ConversationMember, SdkworkError> {
        let path = backend_path(&format!("/conversations/{}/members/leave", conversation_id));
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Get the current member read cursor
    pub async fn get_conversation_read_cursor(&self, conversation_id: &str) -> Result<ConversationReadCursorView, SdkworkError> {
        let path = backend_path(&format!("/conversations/{}/read-cursor", conversation_id));
        self.client.get(&path, None, None).await
    }

    /// Update the current member read cursor
    pub async fn update_conversation_read_cursor(&self, conversation_id: &str, body: &UpdateReadCursorRequest) -> Result<ConversationReadCursorView, SdkworkError> {
        let path = backend_path(&format!("/conversations/{}/read-cursor", conversation_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// List projected conversation timeline entries
    pub async fn list_conversation_messages(&self, conversation_id: &str) -> Result<TimelineResponse, SdkworkError> {
        let path = backend_path(&format!("/conversations/{}/messages", conversation_id));
        self.client.get(&path, None, None).await
    }

    /// Post a standard conversation message
    pub async fn post_conversation_message(&self, conversation_id: &str, body: &PostMessageRequest) -> Result<PostMessageResult, SdkworkError> {
        let path = backend_path(&format!("/conversations/{}/messages", conversation_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Publish a message into a system channel conversation
    pub async fn publish_system_channel_message(&self, conversation_id: &str, body: &PostMessageRequest) -> Result<PostMessageResult, SdkworkError> {
        let path = backend_path(&format!("/conversations/{}/system-channel/publish", conversation_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }
}
