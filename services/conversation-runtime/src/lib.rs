use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

use axum::extract::{Path, State};
use axum::http::{HeaderMap, Request};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use im_auth_context::{AuthContextError, resolve_auth_context, resolve_public_bearer_auth_context};
use im_domain_core::conversation::{
    ConversationMember, ConversationReadCursor, ConversationReadCursorView, MembershipRole,
    MembershipState,
};
use im_domain_core::message::{
    ContentPart, Message, MessageBody, MessageEdited, MessageRecalled, MessageType, Sender,
};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::{CommitJournal, ContractError};
use im_time::utc_now_rfc3339_millis;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConversationCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub creator_id: String,
    pub conversation_type: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAgentDialogCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub requester_id: String,
    pub agent_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAgentHandoffCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub source_id: String,
    pub target_id: String,
    pub target_kind: String,
    pub handoff_session_id: String,
    pub handoff_reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSystemChannelCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub requester_id: String,
    pub subscriber_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptAgentHandoffCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub accepted_by: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolveAgentHandoffCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub resolved_by: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseAgentHandoffCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub closed_by: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConversationResult {
    pub conversation_id: String,
    pub event_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAgentHandoffStatusView {
    pub id: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentHandoffStateView {
    pub tenant_id: String,
    pub conversation_id: String,
    pub status: String,
    pub source: ChangeAgentHandoffStatusView,
    pub target: ChangeAgentHandoffStatusView,
    pub handoff_session_id: String,
    pub handoff_reason: Option<String>,
    pub accepted_at: Option<String>,
    pub accepted_by: Option<ChangeAgentHandoffStatusView>,
    pub resolved_at: Option<String>,
    pub resolved_by: Option<ChangeAgentHandoffStatusView>,
    pub closed_at: Option<String>,
    pub closed_by: Option<ChangeAgentHandoffStatusView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentHandoffStatusChangedPayload {
    pub tenant_id: String,
    pub conversation_id: String,
    pub previous_status: String,
    pub current_status: String,
    pub changed_by: ChangeAgentHandoffStatusView,
    pub changed_at: String,
    pub state: AgentHandoffStateView,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecoveredConversationCreatedPayload {
    conversation_type: String,
    source: Option<ChangeAgentHandoffStatusView>,
    target: Option<ChangeAgentHandoffStatusView>,
    handoff: Option<RecoveredConversationHandoffPayload>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecoveredConversationHandoffPayload {
    session_id: String,
    reason: Option<String>,
    status: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddConversationMemberCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub principal_id: String,
    pub principal_kind: String,
    pub role: MembershipRole,
    pub invited_by: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveConversationMemberCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub member_id: String,
    pub removed_by: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaveConversationCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub principal_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferConversationOwnerCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub target_member_id: String,
    pub transferred_by: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferConversationOwnerPayload {
    pub tenant_id: String,
    pub conversation_id: String,
    pub previous_owner: ConversationMember,
    pub new_owner: ConversationMember,
    pub transferred_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferConversationOwnerResult {
    pub event_id: String,
    pub transferred_at: String,
    pub previous_owner: ConversationMember,
    pub new_owner: ConversationMember,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConversationMemberRoleCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub target_member_id: String,
    pub new_role: MembershipRole,
    pub changed_by: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConversationMemberRolePayload {
    pub tenant_id: String,
    pub conversation_id: String,
    pub previous_member: ConversationMember,
    pub updated_member: ConversationMember,
    pub changed_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConversationMemberRoleResult {
    pub event_id: String,
    pub changed_at: String,
    pub previous_member: ConversationMember,
    pub updated_member: ConversationMember,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReadCursorCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub principal_id: String,
    pub read_seq: u64,
    pub last_read_message_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMessageCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender: Sender,
    pub client_msg_id: Option<String>,
    pub message_type: MessageType,
    pub body: MessageBody,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishSystemChannelMessageCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub publisher: Sender,
    pub client_msg_id: Option<String>,
    pub body: MessageBody,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMessageResult {
    pub message_id: String,
    pub message_seq: u64,
    pub event_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditMessageCommand {
    pub tenant_id: String,
    pub message_id: String,
    pub editor: Sender,
    pub body: MessageBody,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecallMessageCommand {
    pub tenant_id: String,
    pub message_id: String,
    pub recalled_by: Sender,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageMutationResult {
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub event_id: String,
}

#[derive(Debug)]
pub enum RuntimeError {
    ConversationAlreadyExists(String),
    ConversationTypeInvalid(String),
    ConversationNotFound(String),
    MessageNotFound(String),
    MessageAlreadyRecalled(String),
    MemberAlreadyExists(String),
    MemberNotFound(String),
    PermissionDenied(String),
    Conflict(String),
    ReadCursorInvalid(String),
    Contract(ContractError),
}

impl From<ContractError> for RuntimeError {
    fn from(value: ContractError) -> Self {
        Self::Contract(value)
    }
}

#[derive(Default)]
struct ConversationState {
    conversation_type: String,
    message_seq: u64,
    member_epoch: u64,
    handoff_status_epoch: u64,
    members: BTreeMap<String, ConversationMember>,
    principal_members: HashMap<String, String>,
    read_cursors: BTreeMap<String, ConversationReadCursor>,
    messages: HashMap<String, StoredMessage>,
    handoff_state: Option<AgentHandoffStateView>,
}

#[derive(Clone)]
struct StoredMessage {
    message: Message,
    recalled: bool,
}

#[derive(Default)]
struct RuntimeState {
    conversations: HashMap<String, ConversationState>,
    message_index: HashMap<String, String>,
}

#[derive(Clone, Copy)]
enum AgentHandoffLifecycleAction {
    Accept,
    Resolve,
    Close,
}

enum AgentHandoffStatusTransitionOutcome {
    Idempotent(AgentHandoffStateView),
    Mutated {
        payload: AgentHandoffStatusChangedPayload,
        ordering_seq: u64,
        actor_id: String,
        actor_kind: String,
    },
}

#[derive(Clone, Default)]
pub struct InMemoryJournal {
    events: Arc<Mutex<Vec<CommitEnvelope>>>,
}

impl InMemoryJournal {
    pub fn recorded(&self) -> Vec<CommitEnvelope> {
        self.events.lock().expect("journal should lock").clone()
    }
}

impl CommitJournal for InMemoryJournal {
    fn append(
        &self,
        envelope: CommitEnvelope,
    ) -> Result<im_platform_contracts::CommitPosition, ContractError> {
        let mut events = self.events.lock().expect("journal should lock");
        events.push(envelope);
        Ok(im_platform_contracts::CommitPosition::new(
            "p0",
            events.len() as u64,
        ))
    }
}

pub struct ConversationRuntime<J> {
    journal: J,
    state: Mutex<RuntimeState>,
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn new(journal: J) -> Self {
        Self {
            journal,
            state: Mutex::new(RuntimeState::default()),
        }
    }

    pub fn apply_recovered_envelope(&self, envelope: &CommitEnvelope) -> Result<(), RuntimeError> {
        match envelope.event_type.as_str() {
            "conversation.created" => self.apply_recovered_conversation_created(envelope),
            "conversation.member_joined" => self.apply_recovered_member_joined(envelope),
            "conversation.member_removed" | "conversation.member_left" => {
                self.apply_recovered_member_deactivated(envelope)
            }
            "conversation.read_cursor_updated" => self.apply_recovered_read_cursor(envelope),
            "conversation.owner_transferred" => self.apply_recovered_owner_transfer(envelope),
            "conversation.member_role_changed" => {
                self.apply_recovered_member_role_changed(envelope)
            }
            "conversation.agent_handoff_status_changed" => {
                self.apply_recovered_handoff_status_changed(envelope)
            }
            "message.posted" => self.apply_recovered_message_posted(envelope),
            "message.edited" => self.apply_recovered_message_edited(envelope),
            "message.recalled" => self.apply_recovered_message_recalled(envelope),
            _ => Ok(()),
        }
    }

    pub fn create_conversation(
        &self,
        command: CreateConversationCommand,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_conversation_with_creator_kind(command, "user")
    }

    pub fn create_conversation_with_creator_kind(
        &self,
        command: CreateConversationCommand,
        creator_kind: &str,
    ) -> Result<CreateConversationResult, RuntimeError> {
        ensure_generic_creatable_conversation_type(command.conversation_type.as_str())?;
        let creator_id = command.creator_id.clone();
        let created_at = conversation_timestamp();
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let creator_member = build_conversation_member(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(command.conversation_id.as_str(), creator_id.as_str()),
            creator_id.as_str(),
            creator_kind,
            MembershipRole::Owner,
            None,
            created_at.clone(),
        );
        let mut state = self.state.lock().expect("runtime state should lock");
        if state.conversations.contains_key(scope_key.as_str()) {
            return Err(RuntimeError::ConversationAlreadyExists(
                command.conversation_id,
            ));
        }
        let mut conversation = ConversationState::default();
        conversation.conversation_type = command.conversation_type.clone();
        conversation.member_epoch = 1;
        upsert_member(&mut conversation, creator_member.clone());
        upsert_read_cursor(
            &mut conversation,
            build_default_read_cursor(&creator_member),
        );
        state.conversations.insert(scope_key, conversation);
        drop(state);

        let event_id = format!("evt_{}_created", command.conversation_id);
        let envelope = CommitEnvelope {
            event_id: event_id.clone(),
            tenant_id: command.tenant_id.clone(),
            event_type: "conversation.created".into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: command.conversation_id.clone(),
            scope_type: "conversation".into(),
            scope_id: command.conversation_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
            ),
            ordering_seq: 0,
            causation_id: None,
            correlation_id: None,
            idempotency_key: None,
            actor: EventActor {
                actor_id: creator_id.clone(),
                actor_kind: creator_kind.into(),
                actor_session_id: None,
            },
            occurred_at: created_at.clone(),
            committed_at: created_at.clone(),
            payload_schema: Some("conversation.created.v1".into()),
            payload: json!({
                "conversationId": command.conversation_id,
                "conversationType": command.conversation_type
            })
            .to_string(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };

        self.journal.append(envelope)?;
        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            creator_member.clone(),
            1,
            creator_id.as_str(),
            creator_kind,
        ))?;

        Ok(CreateConversationResult {
            conversation_id: command.conversation_id,
            event_id,
        })
    }

    pub fn create_agent_dialog(
        &self,
        command: CreateAgentDialogCommand,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_agent_dialog_with_requester_kind(command, "user")
    }

    pub fn create_agent_dialog_with_requester_kind(
        &self,
        command: CreateAgentDialogCommand,
        requester_kind: &str,
    ) -> Result<CreateConversationResult, RuntimeError> {
        ensure_agent_dialog_requester_kind(requester_kind)?;

        if command.agent_id.trim().is_empty() {
            return Err(RuntimeError::PermissionDenied(
                "agent dialog requires target agent id".into(),
            ));
        }
        if command.requester_id == command.agent_id {
            return Err(RuntimeError::PermissionDenied(
                "agent dialog agent must differ from requester".into(),
            ));
        }

        let created_at = conversation_timestamp();
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let requester_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(
                command.conversation_id.as_str(),
                command.requester_id.as_str(),
            ),
            command.requester_id.as_str(),
            requester_kind,
            MembershipRole::Owner,
            None,
            created_at.clone(),
            BTreeMap::from([("dialogRole".into(), "requester".into())]),
        );
        let agent_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(command.conversation_id.as_str(), command.agent_id.as_str()),
            command.agent_id.as_str(),
            "agent",
            MembershipRole::Member,
            Some(command.requester_id.clone()),
            created_at.clone(),
            BTreeMap::from([
                ("agentId".into(), command.agent_id.clone()),
                ("dialogRole".into(), "assistant".into()),
            ]),
        );

        let mut state = self.state.lock().expect("runtime state should lock");
        if state.conversations.contains_key(scope_key.as_str()) {
            return Err(RuntimeError::ConversationAlreadyExists(
                command.conversation_id,
            ));
        }

        let mut conversation = ConversationState::default();
        conversation.conversation_type = "agent_dialog".into();
        conversation.member_epoch = 2;
        upsert_member(&mut conversation, requester_member.clone());
        upsert_read_cursor(
            &mut conversation,
            build_default_read_cursor(&requester_member),
        );
        upsert_member(&mut conversation, agent_member.clone());
        upsert_read_cursor(&mut conversation, build_default_read_cursor(&agent_member));
        state.conversations.insert(scope_key, conversation);
        drop(state);

        let event_id = format!("evt_{}_created", command.conversation_id);
        let envelope = CommitEnvelope {
            event_id: event_id.clone(),
            tenant_id: command.tenant_id.clone(),
            event_type: "conversation.created".into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: command.conversation_id.clone(),
            scope_type: "conversation".into(),
            scope_id: command.conversation_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
            ),
            ordering_seq: 0,
            causation_id: None,
            correlation_id: None,
            idempotency_key: None,
            actor: EventActor {
                actor_id: command.requester_id.clone(),
                actor_kind: requester_kind.into(),
                actor_session_id: None,
            },
            occurred_at: created_at.clone(),
            committed_at: created_at.clone(),
            payload_schema: Some("conversation.created.v1".into()),
            payload: json!({
                "conversationId": command.conversation_id,
                "conversationType": "agent_dialog"
            })
            .to_string(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };

        self.journal.append(envelope)?;
        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            requester_member,
            1,
            command.requester_id.as_str(),
            requester_kind,
        ))?;
        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            agent_member,
            2,
            command.requester_id.as_str(),
            requester_kind,
        ))?;

        Ok(CreateConversationResult {
            conversation_id: command.conversation_id,
            event_id,
        })
    }

    pub fn create_system_channel(
        &self,
        command: CreateSystemChannelCommand,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_system_channel_with_requester_kind(command, "system")
    }

    pub fn create_system_channel_with_requester_kind(
        &self,
        command: CreateSystemChannelCommand,
        requester_kind: &str,
    ) -> Result<CreateConversationResult, RuntimeError> {
        ensure_system_channel_requester_kind(requester_kind)?;

        if command.subscriber_id.trim().is_empty() {
            return Err(RuntimeError::PermissionDenied(
                "system channel requires subscriber id".into(),
            ));
        }
        if command.requester_id == command.subscriber_id {
            return Err(RuntimeError::PermissionDenied(
                "system channel subscriber must differ from requester".into(),
            ));
        }

        let created_at = conversation_timestamp();
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let publisher_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(
                command.conversation_id.as_str(),
                command.requester_id.as_str(),
            ),
            command.requester_id.as_str(),
            requester_kind,
            MembershipRole::Owner,
            None,
            created_at.clone(),
            BTreeMap::from([("channelRole".into(), "publisher".into())]),
        );
        let subscriber_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(
                command.conversation_id.as_str(),
                command.subscriber_id.as_str(),
            ),
            command.subscriber_id.as_str(),
            "user",
            MembershipRole::Member,
            Some(command.requester_id.clone()),
            created_at.clone(),
            BTreeMap::from([("channelRole".into(), "subscriber".into())]),
        );

        let mut state = self.state.lock().expect("runtime state should lock");
        if state.conversations.contains_key(scope_key.as_str()) {
            return Err(RuntimeError::ConversationAlreadyExists(
                command.conversation_id,
            ));
        }

        let mut conversation = ConversationState::default();
        conversation.conversation_type = "system_channel".into();
        conversation.member_epoch = 2;
        upsert_member(&mut conversation, publisher_member.clone());
        upsert_read_cursor(
            &mut conversation,
            build_default_read_cursor(&publisher_member),
        );
        upsert_member(&mut conversation, subscriber_member.clone());
        upsert_read_cursor(
            &mut conversation,
            build_default_read_cursor(&subscriber_member),
        );
        state.conversations.insert(scope_key, conversation);
        drop(state);

        let event_id = format!("evt_{}_created", command.conversation_id);
        let envelope = CommitEnvelope {
            event_id: event_id.clone(),
            tenant_id: command.tenant_id.clone(),
            event_type: "conversation.created".into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: command.conversation_id.clone(),
            scope_type: "conversation".into(),
            scope_id: command.conversation_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
            ),
            ordering_seq: 0,
            causation_id: None,
            correlation_id: None,
            idempotency_key: None,
            actor: EventActor {
                actor_id: command.requester_id.clone(),
                actor_kind: requester_kind.into(),
                actor_session_id: None,
            },
            occurred_at: created_at.clone(),
            committed_at: created_at.clone(),
            payload_schema: Some("conversation.created.v1".into()),
            payload: json!({
                "conversationId": command.conversation_id,
                "conversationType": "system_channel"
            })
            .to_string(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };

        self.journal.append(envelope)?;
        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            publisher_member,
            1,
            command.requester_id.as_str(),
            requester_kind,
        ))?;
        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            subscriber_member,
            2,
            command.requester_id.as_str(),
            requester_kind,
        ))?;

        Ok(CreateConversationResult {
            conversation_id: command.conversation_id,
            event_id,
        })
    }

    pub fn create_agent_handoff(
        &self,
        command: CreateAgentHandoffCommand,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_agent_handoff_with_source_kind(command, "agent")
    }

    pub fn create_agent_handoff_with_source_kind(
        &self,
        command: CreateAgentHandoffCommand,
        source_kind: &str,
    ) -> Result<CreateConversationResult, RuntimeError> {
        ensure_agent_handoff_source_kind(source_kind)?;
        ensure_agent_handoff_target_kind(command.target_kind.as_str())?;

        if command.handoff_session_id.trim().is_empty() {
            return Err(RuntimeError::PermissionDenied(
                "agent handoff requires handoff session id".into(),
            ));
        }
        if command.source_id == command.target_id {
            return Err(RuntimeError::PermissionDenied(
                "agent handoff target must differ from source".into(),
            ));
        }

        let created_at = conversation_timestamp();
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());

        let mut source_attributes = BTreeMap::from([
            ("handoffRole".into(), "source".into()),
            (
                "handoffSessionId".into(),
                command.handoff_session_id.clone(),
            ),
            ("targetId".into(), command.target_id.clone()),
            ("targetKind".into(), command.target_kind.clone()),
        ]);
        if let Some(reason) = command.handoff_reason.clone() {
            source_attributes.insert("handoffReason".into(), reason);
        }
        let source_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(command.conversation_id.as_str(), command.source_id.as_str()),
            command.source_id.as_str(),
            source_kind,
            MembershipRole::Owner,
            None,
            created_at.clone(),
            source_attributes,
        );

        let mut target_attributes = BTreeMap::from([
            ("handoffRole".into(), "target".into()),
            (
                "handoffSessionId".into(),
                command.handoff_session_id.clone(),
            ),
            ("sourceAgentId".into(), command.source_id.clone()),
        ]);
        if let Some(reason) = command.handoff_reason.clone() {
            target_attributes.insert("handoffReason".into(), reason);
        }
        let target_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(command.conversation_id.as_str(), command.target_id.as_str()),
            command.target_id.as_str(),
            command.target_kind.as_str(),
            MembershipRole::Member,
            Some(command.source_id.clone()),
            created_at.clone(),
            target_attributes,
        );

        let mut state = self.state.lock().expect("runtime state should lock");
        if state.conversations.contains_key(scope_key.as_str()) {
            return Err(RuntimeError::ConversationAlreadyExists(
                command.conversation_id,
            ));
        }

        let mut conversation = ConversationState::default();
        conversation.conversation_type = "agent_handoff".into();
        conversation.member_epoch = 2;
        conversation.handoff_state = Some(AgentHandoffStateView {
            tenant_id: command.tenant_id.clone(),
            conversation_id: command.conversation_id.clone(),
            status: "open".into(),
            source: ChangeAgentHandoffStatusView {
                id: command.source_id.clone(),
                kind: source_kind.into(),
            },
            target: ChangeAgentHandoffStatusView {
                id: command.target_id.clone(),
                kind: command.target_kind.clone(),
            },
            handoff_session_id: command.handoff_session_id.clone(),
            handoff_reason: command.handoff_reason.clone(),
            accepted_at: None,
            accepted_by: None,
            resolved_at: None,
            resolved_by: None,
            closed_at: None,
            closed_by: None,
        });
        upsert_member(&mut conversation, source_member.clone());
        upsert_read_cursor(&mut conversation, build_default_read_cursor(&source_member));
        upsert_member(&mut conversation, target_member.clone());
        upsert_read_cursor(&mut conversation, build_default_read_cursor(&target_member));
        state.conversations.insert(scope_key, conversation);
        drop(state);

        let event_id = format!("evt_{}_created", command.conversation_id);
        let envelope = CommitEnvelope {
            event_id: event_id.clone(),
            tenant_id: command.tenant_id.clone(),
            event_type: "conversation.created".into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: command.conversation_id.clone(),
            scope_type: "conversation".into(),
            scope_id: command.conversation_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
            ),
            ordering_seq: 0,
            causation_id: None,
            correlation_id: None,
            idempotency_key: None,
            actor: EventActor {
                actor_id: command.source_id.clone(),
                actor_kind: source_kind.into(),
                actor_session_id: None,
            },
            occurred_at: created_at.clone(),
            committed_at: created_at.clone(),
            payload_schema: Some("conversation.created.v1".into()),
            payload: json!({
                "conversationId": command.conversation_id,
                "conversationType": "agent_handoff",
                "source": {
                    "id": command.source_id,
                    "kind": source_kind
                },
                "target": {
                    "id": command.target_id,
                    "kind": command.target_kind
                },
                "handoff": {
                    "sessionId": command.handoff_session_id,
                    "reason": command.handoff_reason,
                    "status": "open"
                }
            })
            .to_string(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };

        self.journal.append(envelope)?;
        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            source_member,
            1,
            command.source_id.as_str(),
            source_kind,
        ))?;
        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            target_member,
            2,
            command.source_id.as_str(),
            source_kind,
        ))?;

        Ok(CreateConversationResult {
            conversation_id: command.conversation_id,
            event_id,
        })
    }

    pub fn get_agent_handoff_state(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        principal_id: &str,
    ) -> Result<AgentHandoffStateView, RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, conversation_id);
        let state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        resolve_active_member(conversation, principal_id)?;
        ensure_agent_handoff_conversation(conversation)?;
        conversation.handoff_state.clone().ok_or_else(|| {
            RuntimeError::ConversationTypeInvalid("agent handoff state missing".into())
        })
    }

    pub fn accept_agent_handoff_with_actor_kind(
        &self,
        command: AcceptAgentHandoffCommand,
        actor_kind: &str,
    ) -> Result<AgentHandoffStateView, RuntimeError> {
        let outcome = self.transition_agent_handoff_status(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            command.accepted_by.as_str(),
            actor_kind,
            AgentHandoffLifecycleAction::Accept,
        )?;
        self.finish_agent_handoff_transition(outcome)
    }

    pub fn resolve_agent_handoff_with_actor_kind(
        &self,
        command: ResolveAgentHandoffCommand,
        actor_kind: &str,
    ) -> Result<AgentHandoffStateView, RuntimeError> {
        let outcome = self.transition_agent_handoff_status(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            command.resolved_by.as_str(),
            actor_kind,
            AgentHandoffLifecycleAction::Resolve,
        )?;
        self.finish_agent_handoff_transition(outcome)
    }

    pub fn close_agent_handoff_with_actor_kind(
        &self,
        command: CloseAgentHandoffCommand,
        actor_kind: &str,
    ) -> Result<AgentHandoffStateView, RuntimeError> {
        let outcome = self.transition_agent_handoff_status(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            command.closed_by.as_str(),
            actor_kind,
            AgentHandoffLifecycleAction::Close,
        )?;
        self.finish_agent_handoff_transition(outcome)
    }

    pub fn add_member(
        &self,
        command: AddConversationMemberCommand,
    ) -> Result<ConversationMember, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                command.invited_by.as_str(),
            )?
            .principal_kind;
        self.add_member_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn add_member_with_actor_kind(
        &self,
        command: AddConversationMemberCommand,
        actor_kind: &str,
    ) -> Result<ConversationMember, RuntimeError> {
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let (member, member_epoch, actor_kind) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            let invited_by_member =
                resolve_active_member(conversation, command.invited_by.as_str())?;
            ensure_actor_kind_matches_member(&invited_by_member, actor_kind)?;
            ensure_member_add_actor_allowed(conversation, &invited_by_member)?;

            if let Some(member_id) = conversation
                .principal_members
                .get(command.principal_id.as_str())
            {
                if conversation
                    .members
                    .get(member_id.as_str())
                    .is_some_and(ConversationMember::is_active)
                {
                    return Err(RuntimeError::MemberAlreadyExists(command.principal_id));
                }
            }
            ensure_member_add_request_allowed(conversation, &invited_by_member, &command.role)?;
            let member_episode = next_member_episode(conversation, command.principal_id.as_str());

            let member = build_conversation_member(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                member_episode_id(
                    command.conversation_id.as_str(),
                    command.principal_id.as_str(),
                    member_episode,
                ),
                command.principal_id.as_str(),
                command.principal_kind.as_str(),
                command.role,
                Some(command.invited_by.clone()),
                conversation_timestamp(),
            );

            conversation.member_epoch += 1;
            let member_epoch = conversation.member_epoch;
            upsert_member(conversation, member.clone());
            upsert_read_cursor(conversation, build_default_read_cursor(&member));
            (
                member,
                member_epoch,
                invited_by_member.principal_kind.clone(),
            )
        };

        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            member.clone(),
            member_epoch,
            command.invited_by.as_str(),
            actor_kind.as_str(),
        ))?;

        Ok(member)
    }

    pub fn remove_member(
        &self,
        command: RemoveConversationMemberCommand,
    ) -> Result<ConversationMember, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                command.removed_by.as_str(),
            )?
            .principal_kind;
        self.remove_member_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn remove_member_with_actor_kind(
        &self,
        command: RemoveConversationMemberCommand,
        actor_kind: &str,
    ) -> Result<ConversationMember, RuntimeError> {
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let (member, member_epoch, actor_kind) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            let removed_by_member =
                resolve_active_member(conversation, command.removed_by.as_str())?;
            ensure_actor_kind_matches_member(&removed_by_member, actor_kind)?;

            let mut member = conversation
                .members
                .get(command.member_id.as_str())
                .cloned()
                .ok_or_else(|| RuntimeError::MemberNotFound(command.member_id.clone()))?;
            ensure_current_active_member_target(conversation, &member)?;
            ensure_member_remove_allowed(conversation, &removed_by_member, &member)?;
            member.state = MembershipState::Removed;
            member.removed_at = Some(conversation_timestamp());

            conversation.member_epoch += 1;
            let member_epoch = conversation.member_epoch;
            conversation
                .principal_members
                .remove(member.principal_id.as_str());
            conversation
                .members
                .insert(member.member_id.clone(), member.clone());
            (
                member,
                member_epoch,
                removed_by_member.principal_kind.clone(),
            )
        };

        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_removed",
            member.clone(),
            member_epoch,
            command.removed_by.as_str(),
            actor_kind.as_str(),
        ))?;

        Ok(member)
    }

    pub fn leave_conversation(
        &self,
        command: LeaveConversationCommand,
    ) -> Result<ConversationMember, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                command.principal_id.as_str(),
            )?
            .principal_kind;
        self.leave_conversation_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn leave_conversation_with_actor_kind(
        &self,
        command: LeaveConversationCommand,
        actor_kind: &str,
    ) -> Result<ConversationMember, RuntimeError> {
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let (member, member_epoch, actor_kind) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            let leaving_member =
                resolve_active_member(conversation, command.principal_id.as_str())?;
            ensure_actor_kind_matches_member(&leaving_member, actor_kind)?;
            ensure_member_leave_allowed(conversation, &leaving_member)?;

            let mut member = leaving_member.clone();
            member.state = MembershipState::Left;
            member.removed_at = Some(conversation_timestamp());

            conversation.member_epoch += 1;
            let member_epoch = conversation.member_epoch;
            conversation
                .principal_members
                .remove(member.principal_id.as_str());
            conversation
                .members
                .insert(member.member_id.clone(), member.clone());
            (member, member_epoch, leaving_member.principal_kind.clone())
        };

        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_left",
            member.clone(),
            member_epoch,
            command.principal_id.as_str(),
            actor_kind.as_str(),
        ))?;

        Ok(member)
    }

    pub fn transfer_conversation_owner(
        &self,
        command: TransferConversationOwnerCommand,
    ) -> Result<TransferConversationOwnerResult, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                command.transferred_by.as_str(),
            )?
            .principal_kind;
        self.transfer_conversation_owner_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn transfer_conversation_owner_with_actor_kind(
        &self,
        command: TransferConversationOwnerCommand,
        actor_kind: &str,
    ) -> Result<TransferConversationOwnerResult, RuntimeError> {
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let (payload, ordering_seq, actor_kind) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            let owner_member =
                resolve_active_member(conversation, command.transferred_by.as_str())?;
            ensure_actor_kind_matches_member(&owner_member, actor_kind)?;
            let target_member = conversation
                .members
                .get(command.target_member_id.as_str())
                .cloned()
                .ok_or_else(|| RuntimeError::MemberNotFound(command.target_member_id.clone()))?;
            ensure_owner_transfer_allowed(conversation, &owner_member, &target_member)?;

            let transferred_at = conversation_timestamp();
            let actor_kind = owner_member.principal_kind.clone();
            let previous_owner = ConversationMember {
                role: MembershipRole::Admin,
                ..owner_member
            };
            let new_owner = ConversationMember {
                role: MembershipRole::Owner,
                ..target_member
            };

            conversation
                .members
                .insert(previous_owner.member_id.clone(), previous_owner.clone());
            conversation
                .members
                .insert(new_owner.member_id.clone(), new_owner.clone());
            conversation.member_epoch += 1;
            let ordering_seq = conversation.member_epoch;

            (
                TransferConversationOwnerPayload {
                    tenant_id: command.tenant_id.clone(),
                    conversation_id: command.conversation_id.clone(),
                    previous_owner,
                    new_owner,
                    transferred_at,
                },
                ordering_seq,
                actor_kind,
            )
        };

        let event = build_owner_transfer_envelope(
            payload.clone(),
            ordering_seq,
            command.transferred_by.as_str(),
            actor_kind.as_str(),
        );
        self.journal.append(event.clone())?;

        Ok(TransferConversationOwnerResult {
            event_id: event.event_id,
            transferred_at: payload.transferred_at.clone(),
            previous_owner: payload.previous_owner,
            new_owner: payload.new_owner,
        })
    }

    pub fn change_conversation_member_role(
        &self,
        command: ChangeConversationMemberRoleCommand,
    ) -> Result<ChangeConversationMemberRoleResult, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                command.changed_by.as_str(),
            )?
            .principal_kind;
        self.change_conversation_member_role_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn change_conversation_member_role_with_actor_kind(
        &self,
        command: ChangeConversationMemberRoleCommand,
        actor_kind: &str,
    ) -> Result<ChangeConversationMemberRoleResult, RuntimeError> {
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let (payload, ordering_seq, actor_kind) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            let actor_member = resolve_active_member(conversation, command.changed_by.as_str())?;
            ensure_actor_kind_matches_member(&actor_member, actor_kind)?;
            let target_member = conversation
                .members
                .get(command.target_member_id.as_str())
                .cloned()
                .ok_or_else(|| RuntimeError::MemberNotFound(command.target_member_id.clone()))?;
            ensure_current_active_member_target(conversation, &target_member)?;
            ensure_member_role_change_allowed(
                conversation,
                &actor_member,
                &target_member,
                &command.new_role,
            )?;

            let changed_at = conversation_timestamp();
            let previous_member = target_member.clone();
            let updated_member = ConversationMember {
                role: command.new_role.clone(),
                ..target_member
            };
            conversation
                .members
                .insert(updated_member.member_id.clone(), updated_member.clone());
            conversation.member_epoch += 1;
            let ordering_seq = conversation.member_epoch;

            (
                ChangeConversationMemberRolePayload {
                    tenant_id: command.tenant_id.clone(),
                    conversation_id: command.conversation_id.clone(),
                    previous_member,
                    updated_member,
                    changed_at,
                },
                ordering_seq,
                actor_member.principal_kind.clone(),
            )
        };

        let event = build_member_role_changed_envelope(
            payload.clone(),
            ordering_seq,
            command.changed_by.as_str(),
            actor_kind.as_str(),
        );
        self.journal.append(event.clone())?;

        Ok(ChangeConversationMemberRoleResult {
            event_id: event.event_id,
            changed_at: payload.changed_at.clone(),
            previous_member: payload.previous_member,
            updated_member: payload.updated_member,
        })
    }

    pub fn list_members(
        &self,
        tenant_id: &str,
        conversation_id: &str,
    ) -> Result<Vec<ConversationMember>, RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, conversation_id);
        let state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;

        Ok(conversation
            .members
            .values()
            .filter(|member| member.is_active())
            .cloned()
            .collect())
    }

    pub fn update_read_cursor(
        &self,
        command: UpdateReadCursorCommand,
    ) -> Result<ConversationReadCursor, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                command.principal_id.as_str(),
            )?
            .principal_kind;
        self.update_read_cursor_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn update_read_cursor_with_actor_kind(
        &self,
        command: UpdateReadCursorCommand,
        actor_kind: &str,
    ) -> Result<ConversationReadCursor, RuntimeError> {
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let mut changed_event: Option<(ConversationReadCursor, String)> = None;
        let cursor = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            if command.read_seq > conversation.message_seq {
                return Err(RuntimeError::ReadCursorInvalid(format!(
                    "read cursor exceeds conversation high watermark: {} > {}",
                    command.read_seq, conversation.message_seq
                )));
            }

            let actor_member = resolve_active_member(conversation, command.principal_id.as_str())?;
            ensure_actor_kind_matches_member(&actor_member, actor_kind)?;
            let member_id = actor_member.member_id.clone();
            let cursor = conversation
                .read_cursors
                .entry(member_id.clone())
                .or_insert_with(|| ConversationReadCursor {
                    tenant_id: command.tenant_id.clone(),
                    conversation_id: command.conversation_id.clone(),
                    member_id: member_id.clone(),
                    principal_id: command.principal_id.clone(),
                    read_seq: 0,
                    last_read_message_id: None,
                    updated_at: conversation_timestamp(),
                });

            if command.read_seq > cursor.read_seq {
                cursor.read_seq = command.read_seq;
                cursor.last_read_message_id = command.last_read_message_id.clone();
                cursor.updated_at = conversation_timestamp();
                changed_event = Some((cursor.clone(), actor_member.principal_kind.clone()));
            }

            cursor.clone()
        };

        if let Some((changed_cursor, actor_kind)) = changed_event {
            self.journal.append(build_read_cursor_envelope(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                changed_cursor.clone(),
                changed_cursor.read_seq,
                command.principal_id.as_str(),
                actor_kind.as_str(),
            ))?;
        }

        Ok(cursor)
    }

    pub fn read_cursor_view(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        principal_id: &str,
    ) -> Result<ConversationReadCursorView, RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, conversation_id);
        let state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        let member_id = resolve_active_member_id(conversation, principal_id)?;
        let cursor = conversation
            .read_cursors
            .get(member_id.as_str())
            .ok_or_else(|| {
                RuntimeError::PermissionDenied(format!(
                    "principal is not active conversation member: {principal_id}"
                ))
            })?;

        Ok(ConversationReadCursorView::from_cursor(
            cursor,
            conversation.message_seq.saturating_sub(cursor.read_seq),
        ))
    }

    pub fn post_message(
        &self,
        command: PostMessageCommand,
    ) -> Result<PostMessageResult, RuntimeError> {
        self.post_message_with_policy(command, MessagePostPolicy::GenericPost)
    }

    pub fn publish_system_channel_message(
        &self,
        command: PublishSystemChannelMessageCommand,
    ) -> Result<PostMessageResult, RuntimeError> {
        self.post_message_with_policy(
            PostMessageCommand {
                tenant_id: command.tenant_id,
                conversation_id: command.conversation_id,
                sender: command.publisher,
                client_msg_id: command.client_msg_id,
                message_type: MessageType::Standard,
                body: command.body,
            },
            MessagePostPolicy::SystemChannelPublish,
        )
    }

    fn post_message_with_policy(
        &self,
        command: PostMessageCommand,
        policy: MessagePostPolicy,
    ) -> Result<PostMessageResult, RuntimeError> {
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let (message_seq, message_id, message) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let (message_seq, message_id, message) = {
                let conversation =
                    state
                        .conversations
                        .get_mut(scope_key.as_str())
                        .ok_or_else(|| {
                            RuntimeError::ConversationNotFound(command.conversation_id.clone())
                        })?;
                let sender_member =
                    resolve_active_member(conversation, command.sender.id.as_str())?;
                ensure_actor_kind_matches_member(&sender_member, command.sender.kind.as_str())?;
                match policy {
                    MessagePostPolicy::GenericPost => {
                        ensure_message_post_allowed(conversation, &sender_member)?
                    }
                    MessagePostPolicy::SystemChannelPublish => {
                        ensure_system_channel_publish_command_allowed(conversation, &sender_member)?
                    }
                }
                conversation.message_seq += 1;

                let mut sender = command.sender.clone();
                if sender.member_id.is_none() {
                    sender.member_id = Some(sender_member.member_id.clone());
                }

                let message_id = format!(
                    "msg_{}_{}",
                    command.conversation_id, conversation.message_seq
                );
                let message_timestamp = conversation_timestamp();
                let message = Message {
                    tenant_id: command.tenant_id.clone(),
                    conversation_id: command.conversation_id.clone(),
                    message_id: message_id.clone(),
                    message_seq: conversation.message_seq,
                    sender,
                    message_type: command.message_type.clone(),
                    delivery_mode: "discrete".into(),
                    client_msg_id: command.client_msg_id.clone(),
                    stream_session_id: None,
                    rtc_session_id: None,
                    body: command.body.clone(),
                    attributes: BTreeMap::new(),
                    metadata: BTreeMap::new(),
                    occurred_at: message_timestamp.clone(),
                    committed_at: Some(message_timestamp),
                };
                conversation.messages.insert(
                    message_id.clone(),
                    StoredMessage {
                        message: message.clone(),
                        recalled: false,
                    },
                );
                (conversation.message_seq, message_id, message)
            };

            state.message_index.insert(
                message_scope_key(command.tenant_id.as_str(), message_id.as_str()),
                scope_key,
            );

            (message_seq, message_id, message)
        };

        let envelope = CommitEnvelope {
            event_id: format!("evt_{}_posted", message_id),
            tenant_id: command.tenant_id.clone(),
            event_type: "message.posted".into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: command.conversation_id.clone(),
            scope_type: "conversation".into(),
            scope_id: command.conversation_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
            ),
            ordering_seq: message_seq,
            causation_id: None,
            correlation_id: None,
            idempotency_key: command.client_msg_id,
            actor: EventActor {
                actor_id: message.sender.id.clone(),
                actor_kind: message.sender.kind.clone(),
                actor_session_id: message.sender.session_id.clone(),
            },
            occurred_at: message.occurred_at.clone(),
            committed_at: message
                .committed_at
                .clone()
                .unwrap_or_else(|| message.occurred_at.clone()),
            payload_schema: Some("message.posted.v1".into()),
            payload: serde_json::to_string(&message)
                .expect("message payload should serialize into commit envelope"),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };

        self.journal.append(envelope)?;

        Ok(PostMessageResult {
            message_id,
            message_seq,
            event_id: format!("evt_{}_posted", message.message_id),
        })
    }

    pub fn edit_message(
        &self,
        command: EditMessageCommand,
    ) -> Result<MessageMutationResult, RuntimeError> {
        let edited = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let scope_key = state
                .message_index
                .get(
                    message_scope_key(command.tenant_id.as_str(), command.message_id.as_str())
                        .as_str(),
                )
                .cloned()
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let editor_member = resolve_active_member(conversation, command.editor.id.as_str())?;
            ensure_actor_kind_matches_member(&editor_member, command.editor.kind.as_str())?;
            let conversation_type = conversation.conversation_type.clone();
            let handoff_closed = is_closed_agent_handoff(conversation);
            let stored = conversation
                .messages
                .get_mut(command.message_id.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            if stored.recalled {
                return Err(RuntimeError::MessageAlreadyRecalled(command.message_id));
            }
            ensure_message_edit_allowed(
                command.editor.id.as_str(),
                &editor_member,
                conversation_type.as_str(),
                handoff_closed,
                &stored.message,
            )?;

            let mut editor = command.editor.clone();
            if editor.member_id.is_none() {
                editor.member_id = Some(editor_member.member_id.clone());
            }

            let edited_at = conversation_timestamp();
            stored.message.body = command.body.clone();
            stored.message.committed_at = Some(edited_at.clone());

            MessageEdited {
                tenant_id: command.tenant_id.clone(),
                conversation_id: stored.message.conversation_id.clone(),
                message_id: stored.message.message_id.clone(),
                message_seq: stored.message.message_seq,
                body: command.body,
                editor,
                edited_at,
            }
        };

        let event_id = format!("evt_{}_edited", edited.message_id);
        self.journal
            .append(build_message_edited_envelope(&edited, event_id.as_str()))?;

        Ok(MessageMutationResult {
            conversation_id: edited.conversation_id,
            message_id: edited.message_id,
            message_seq: edited.message_seq,
            event_id,
        })
    }

    pub fn recall_message(
        &self,
        command: RecallMessageCommand,
    ) -> Result<MessageMutationResult, RuntimeError> {
        let recalled = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let scope_key = state
                .message_index
                .get(
                    message_scope_key(command.tenant_id.as_str(), command.message_id.as_str())
                        .as_str(),
                )
                .cloned()
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let recalled_member =
                resolve_active_member(conversation, command.recalled_by.id.as_str())?;
            ensure_actor_kind_matches_member(&recalled_member, command.recalled_by.kind.as_str())?;
            let conversation_type = conversation.conversation_type.clone();
            let handoff_closed = is_closed_agent_handoff(conversation);
            let stored = conversation
                .messages
                .get_mut(command.message_id.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            if stored.recalled {
                return Err(RuntimeError::MessageAlreadyRecalled(command.message_id));
            }
            ensure_message_recall_allowed(
                command.recalled_by.id.as_str(),
                &recalled_member,
                conversation_type.as_str(),
                handoff_closed,
                &stored.message,
            )?;

            let mut recalled_by = command.recalled_by.clone();
            if recalled_by.member_id.is_none() {
                recalled_by.member_id = Some(recalled_member.member_id.clone());
            }

            let recalled_at = conversation_timestamp();
            stored.recalled = true;
            stored.message.body.summary = Some("[recalled]".into());
            stored.message.committed_at = Some(recalled_at.clone());

            MessageRecalled {
                tenant_id: command.tenant_id.clone(),
                conversation_id: stored.message.conversation_id.clone(),
                message_id: stored.message.message_id.clone(),
                message_seq: stored.message.message_seq,
                recalled_by,
                recalled_at,
            }
        };

        let event_id = format!("evt_{}_recalled", recalled.message_id);
        self.journal.append(build_message_recalled_envelope(
            &recalled,
            event_id.as_str(),
        ))?;

        Ok(MessageMutationResult {
            conversation_id: recalled.conversation_id,
            message_id: recalled.message_id,
            message_seq: recalled.message_seq,
            event_id,
        })
    }

    fn transition_agent_handoff_status(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        actor_id: &str,
        actor_kind: &str,
        action: AgentHandoffLifecycleAction,
    ) -> Result<AgentHandoffStatusTransitionOutcome, RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, conversation_id);
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        ensure_agent_handoff_conversation(conversation)?;
        let actor_member = resolve_active_member(conversation, actor_id)?;
        ensure_actor_kind_matches_member(&actor_member, actor_kind)?;
        let actor = build_handoff_actor_view(&actor_member);
        let handoff_state = conversation.handoff_state.as_mut().ok_or_else(|| {
            RuntimeError::ConversationTypeInvalid("agent handoff state missing".into())
        })?;
        let current_status = handoff_state.status.clone();

        match action {
            AgentHandoffLifecycleAction::Accept => {
                ensure_target_handoff_actor(handoff_state, &actor)?;
                if current_status == "accepted"
                    && handoff_state.accepted_by.as_ref() == Some(&actor)
                {
                    return Ok(AgentHandoffStatusTransitionOutcome::Idempotent(
                        handoff_state.clone(),
                    ));
                }
                if current_status != "open" {
                    return Err(RuntimeError::Conflict(format!(
                        "agent handoff cannot accept from status {}",
                        current_status
                    )));
                }

                let changed_at = conversation_timestamp();
                handoff_state.status = "accepted".into();
                handoff_state.accepted_at = Some(changed_at.clone());
                handoff_state.accepted_by = Some(actor.clone());
                conversation.handoff_status_epoch += 1;
                let payload = AgentHandoffStatusChangedPayload {
                    tenant_id: tenant_id.into(),
                    conversation_id: conversation_id.into(),
                    previous_status: current_status,
                    current_status: handoff_state.status.clone(),
                    changed_by: actor.clone(),
                    changed_at: changed_at.clone(),
                    state: handoff_state.clone(),
                };
                return Ok(AgentHandoffStatusTransitionOutcome::Mutated {
                    payload,
                    ordering_seq: conversation.handoff_status_epoch,
                    actor_id: actor_id.into(),
                    actor_kind: actor_kind.into(),
                });
            }
            AgentHandoffLifecycleAction::Resolve => {
                ensure_target_handoff_actor(handoff_state, &actor)?;
                if current_status == "resolved"
                    && handoff_state.resolved_by.as_ref() == Some(&actor)
                {
                    return Ok(AgentHandoffStatusTransitionOutcome::Idempotent(
                        handoff_state.clone(),
                    ));
                }
                if current_status != "accepted" {
                    return Err(RuntimeError::Conflict(format!(
                        "agent handoff cannot resolve from status {}",
                        current_status
                    )));
                }

                let changed_at = conversation_timestamp();
                handoff_state.status = "resolved".into();
                handoff_state.resolved_at = Some(changed_at.clone());
                handoff_state.resolved_by = Some(actor.clone());
                conversation.handoff_status_epoch += 1;
                let payload = AgentHandoffStatusChangedPayload {
                    tenant_id: tenant_id.into(),
                    conversation_id: conversation_id.into(),
                    previous_status: current_status,
                    current_status: handoff_state.status.clone(),
                    changed_by: actor.clone(),
                    changed_at: changed_at.clone(),
                    state: handoff_state.clone(),
                };
                return Ok(AgentHandoffStatusTransitionOutcome::Mutated {
                    payload,
                    ordering_seq: conversation.handoff_status_epoch,
                    actor_id: actor_id.into(),
                    actor_kind: actor_kind.into(),
                });
            }
            AgentHandoffLifecycleAction::Close => {
                ensure_source_or_target_handoff_actor(handoff_state, &actor)?;
                if current_status == "closed" {
                    return Ok(AgentHandoffStatusTransitionOutcome::Idempotent(
                        handoff_state.clone(),
                    ));
                }

                let changed_at = conversation_timestamp();
                handoff_state.status = "closed".into();
                handoff_state.closed_at = Some(changed_at.clone());
                handoff_state.closed_by = Some(actor.clone());
                conversation.handoff_status_epoch += 1;
                let payload = AgentHandoffStatusChangedPayload {
                    tenant_id: tenant_id.into(),
                    conversation_id: conversation_id.into(),
                    previous_status: current_status,
                    current_status: handoff_state.status.clone(),
                    changed_by: actor.clone(),
                    changed_at: changed_at.clone(),
                    state: handoff_state.clone(),
                };
                return Ok(AgentHandoffStatusTransitionOutcome::Mutated {
                    payload,
                    ordering_seq: conversation.handoff_status_epoch,
                    actor_id: actor_id.into(),
                    actor_kind: actor_kind.into(),
                });
            }
        }
    }

    fn finish_agent_handoff_transition(
        &self,
        outcome: AgentHandoffStatusTransitionOutcome,
    ) -> Result<AgentHandoffStateView, RuntimeError> {
        match outcome {
            AgentHandoffStatusTransitionOutcome::Idempotent(state) => Ok(state),
            AgentHandoffStatusTransitionOutcome::Mutated {
                payload,
                ordering_seq,
                actor_id,
                actor_kind,
            } => {
                let envelope = build_agent_handoff_status_changed_envelope(
                    payload.clone(),
                    ordering_seq,
                    actor_id.as_str(),
                    actor_kind.as_str(),
                );
                self.journal.append(envelope)?;
                Ok(payload.state)
            }
        }
    }

    fn apply_recovered_conversation_created(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let payload: RecoveredConversationCreatedPayload =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.created {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state.conversations.entry(scope_key).or_default();
        conversation.conversation_type = payload.conversation_type;
        if let (Some(source), Some(target), Some(handoff)) =
            (payload.source, payload.target, payload.handoff)
        {
            conversation.handoff_state = Some(AgentHandoffStateView {
                tenant_id: envelope.tenant_id.clone(),
                conversation_id: envelope.scope_id.clone(),
                status: handoff.status,
                source,
                target,
                handoff_session_id: handoff.session_id,
                handoff_reason: handoff.reason,
                accepted_at: None,
                accepted_by: None,
                resolved_at: None,
                resolved_by: None,
                closed_at: None,
                closed_by: None,
            });
        }
        Ok(())
    }

    fn apply_recovered_member_joined(&self, envelope: &CommitEnvelope) -> Result<(), RuntimeError> {
        let member: ConversationMember =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.member_joined {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        conversation.member_epoch = conversation.member_epoch.max(envelope.ordering_seq);
        upsert_member(conversation, member.clone());
        conversation
            .read_cursors
            .entry(member.member_id.clone())
            .or_insert_with(|| build_default_read_cursor(&member));
        Ok(())
    }

    fn apply_recovered_member_deactivated(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let member: ConversationMember =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay {} {}: {error}",
                    envelope.event_type, envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        conversation.member_epoch = conversation.member_epoch.max(envelope.ordering_seq);
        conversation
            .principal_members
            .remove(member.principal_id.as_str());
        conversation
            .members
            .insert(member.member_id.clone(), member);
        Ok(())
    }

    fn apply_recovered_read_cursor(&self, envelope: &CommitEnvelope) -> Result<(), RuntimeError> {
        let cursor: ConversationReadCursor = serde_json::from_str(envelope.payload.as_str())
            .map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.read_cursor_updated {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        upsert_read_cursor(conversation, cursor);
        Ok(())
    }

    fn apply_recovered_owner_transfer(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let payload: TransferConversationOwnerPayload =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.owner_transferred {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        conversation.member_epoch = conversation.member_epoch.max(envelope.ordering_seq);
        upsert_member(conversation, payload.previous_owner);
        upsert_member(conversation, payload.new_owner);
        Ok(())
    }

    fn apply_recovered_member_role_changed(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let payload: ChangeConversationMemberRolePayload =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.member_role_changed {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        conversation.member_epoch = conversation.member_epoch.max(envelope.ordering_seq);
        upsert_member(conversation, payload.updated_member);
        Ok(())
    }

    fn apply_recovered_handoff_status_changed(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let payload: AgentHandoffStatusChangedPayload =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.agent_handoff_status_changed {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        conversation.handoff_status_epoch =
            conversation.handoff_status_epoch.max(envelope.ordering_seq);
        conversation.handoff_state = Some(payload.state);
        Ok(())
    }

    fn apply_recovered_message_posted(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let message: Message =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay message.posted {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.posted without conversation {}",
                    envelope.scope_id
                ))
            })?;
        conversation.message_seq = conversation.message_seq.max(message.message_seq);
        conversation.messages.insert(
            message.message_id.clone(),
            StoredMessage {
                message: message.clone(),
                recalled: false,
            },
        );
        state.message_index.insert(
            message_scope_key(message.tenant_id.as_str(), message.message_id.as_str()),
            scope_key,
        );
        Ok(())
    }

    fn apply_recovered_message_edited(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let edited: MessageEdited =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay message.edited {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        let stored = conversation
            .messages
            .get_mut(edited.message_id.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.edited without message {}",
                    edited.message_id
                ))
            })?;
        stored.message.body = edited.body;
        stored.message.committed_at = Some(edited.edited_at);
        Ok(())
    }

    fn apply_recovered_message_recalled(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let recalled: MessageRecalled =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay message.recalled {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        let stored = conversation
            .messages
            .get_mut(recalled.message_id.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.recalled without message {}",
                    recalled.message_id
                ))
            })?;
        stored.recalled = true;
        stored.message.body.summary = Some("[recalled]".into());
        stored.message.committed_at = Some(recalled.recalled_at);
        Ok(())
    }
}

#[derive(Clone)]
struct AppState {
    runtime: Arc<ConversationRuntime<InMemoryJournal>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MessagePostPolicy {
    GenericPost,
    SystemChannelPublish,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PostMessageRequest {
    client_msg_id: Option<String>,
    summary: Option<String>,
    text: Option<String>,
    #[serde(default)]
    parts: Vec<ContentPart>,
    #[serde(default)]
    render_hints: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EditMessageRequest {
    summary: Option<String>,
    text: Option<String>,
    #[serde(default)]
    parts: Vec<ContentPart>,
    #[serde(default)]
    render_hints: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateConversationRequest {
    conversation_id: String,
    conversation_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateAgentDialogRequest {
    conversation_id: String,
    agent_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateAgentHandoffRequest {
    conversation_id: String,
    target_id: String,
    target_kind: String,
    handoff_session_id: String,
    handoff_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateSystemChannelRequest {
    conversation_id: String,
    subscriber_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddConversationMemberRequest {
    principal_id: String,
    principal_kind: String,
    role: MembershipRole,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RemoveConversationMemberRequest {
    member_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransferConversationOwnerRequest {
    member_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChangeConversationMemberRoleRequest {
    member_id: String,
    role: MembershipRole,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ListMembersResponse {
    items: Vec<ConversationMember>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateReadCursorRequest {
    read_seq: u64,
    last_read_message_id: Option<String>,
}

#[derive(Debug)]
struct ApiError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl ApiError {
    fn bad_request(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::BAD_REQUEST,
            code,
            message: message.into(),
        }
    }

    fn forbidden(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::FORBIDDEN,
            code,
            message: message.into(),
        }
    }
}

impl From<AuthContextError> for ApiError {
    fn from(value: AuthContextError) -> Self {
        Self {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl From<RuntimeError> for ApiError {
    fn from(value: RuntimeError) -> Self {
        match value {
            RuntimeError::ConversationAlreadyExists(message) => {
                Self::bad_request("conversation_exists", message)
            }
            RuntimeError::ConversationTypeInvalid(message) => {
                Self::bad_request("conversation_type_invalid", message)
            }
            RuntimeError::ConversationNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "conversation_not_found",
                message,
            },
            RuntimeError::MessageNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "message_not_found",
                message,
            },
            RuntimeError::MessageAlreadyRecalled(message) => Self::bad_request(
                "message_already_recalled",
                format!("message already recalled: {message}"),
            ),
            RuntimeError::MemberAlreadyExists(message) => {
                Self::bad_request("conversation_member_exists", message)
            }
            RuntimeError::MemberNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "conversation_member_not_found",
                message,
            },
            RuntimeError::PermissionDenied(message) => {
                Self::forbidden("conversation_permission_denied", message)
            }
            RuntimeError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                code: "conversation_conflict",
                message,
            },
            RuntimeError::ReadCursorInvalid(message) => {
                Self::bad_request("read_cursor_invalid", message)
            }
            RuntimeError::Contract(_) => Self {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "journal_unavailable",
                message: "commit journal unavailable".into(),
            },
        }
    }
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status,
            Json(json!({
                "code": self.code,
                "message": self.message
            })),
        )
            .into_response()
    }
}

pub fn build_default_app() -> Router {
    let state = AppState {
        runtime: Arc::new(ConversationRuntime::new(InMemoryJournal::default())),
    };
    build_app(state)
}

pub fn build_public_app() -> Router {
    build_default_app().layer(middleware::from_fn(require_public_bearer_auth))
}

fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/api/v1/conversations", post(create_conversation))
        .route(
            "/api/v1/conversations/agent-dialogs",
            post(create_agent_dialog),
        )
        .route(
            "/api/v1/conversations/agent-handoffs",
            post(create_agent_handoff),
        )
        .route(
            "/api/v1/conversations/system-channels",
            post(create_system_channel),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/agent-handoff",
            get(get_agent_handoff_state),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/agent-handoff/accept",
            post(accept_agent_handoff),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/agent-handoff/resolve",
            post(resolve_agent_handoff),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/agent-handoff/close",
            post(close_agent_handoff),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members",
            get(list_members),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members/add",
            post(add_member),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members/remove",
            post(remove_member),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members/transfer-owner",
            post(transfer_conversation_owner),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members/change-role",
            post(change_conversation_member_role),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members/leave",
            post(leave_conversation),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/read-cursor",
            get(get_read_cursor).post(update_read_cursor),
        )
        .route("/api/v1/messages/{message_id}/edit", post(edit_message))
        .route("/api/v1/messages/{message_id}/recall", post(recall_message))
        .route(
            "/api/v1/conversations/{conversation_id}/messages",
            post(post_message),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/system-channel/publish",
            post(publish_system_channel_message),
        )
        .with_state(state)
}

async fn require_public_bearer_auth(request: Request<axum::body::Body>, next: Next) -> Response {
    match request.uri().path() {
        "/healthz" | "/readyz" => next.run(request).await,
        _ => match resolve_public_bearer_auth_context(request.headers()) {
            Ok(_) => next.run(request).await,
            Err(error) => ApiError::from(error).into_response(),
        },
    }
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "conversation-runtime",
    })
}

async fn readyz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "conversation-runtime",
    })
}

async fn create_conversation(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateConversationRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let result = state.runtime.create_conversation_with_creator_kind(
        CreateConversationCommand {
            tenant_id: auth.tenant_id,
            conversation_id: request.conversation_id,
            creator_id: auth.actor_id,
            conversation_type: request.conversation_type,
        },
        auth.actor_kind.as_str(),
    )?;
    Ok(Json(result))
}

async fn create_agent_dialog(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateAgentDialogRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state.runtime.create_agent_dialog_with_requester_kind(
            CreateAgentDialogCommand {
                tenant_id: auth.tenant_id,
                conversation_id: request.conversation_id,
                requester_id: auth.actor_id,
                agent_id: request.agent_id,
            },
            auth.actor_kind.as_str(),
        )?,
    ))
}

async fn create_agent_handoff(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateAgentHandoffRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.create_agent_handoff_with_source_kind(
        CreateAgentHandoffCommand {
            tenant_id: auth.tenant_id,
            conversation_id: request.conversation_id,
            source_id: auth.actor_id,
            target_id: request.target_id,
            target_kind: request.target_kind,
            handoff_session_id: request.handoff_session_id,
            handoff_reason: request.handoff_reason,
        },
        auth.actor_kind.as_str(),
    )?))
}

async fn create_system_channel(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateSystemChannelRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state.runtime.create_system_channel_with_requester_kind(
            CreateSystemChannelCommand {
                tenant_id: auth.tenant_id,
                conversation_id: request.conversation_id,
                requester_id: auth.actor_id,
                subscriber_id: request.subscriber_id,
            },
            auth.actor_kind.as_str(),
        )?,
    ))
}

async fn get_agent_handoff_state(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AgentHandoffStateView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.get_agent_handoff_state(
        auth.tenant_id.as_str(),
        conversation_id.as_str(),
        auth.actor_id.as_str(),
    )?))
}

async fn accept_agent_handoff(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AgentHandoffStateView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.accept_agent_handoff_with_actor_kind(
        AcceptAgentHandoffCommand {
            tenant_id: auth.tenant_id,
            conversation_id,
            accepted_by: auth.actor_id,
        },
        auth.actor_kind.as_str(),
    )?))
}

async fn resolve_agent_handoff(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AgentHandoffStateView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.resolve_agent_handoff_with_actor_kind(
        ResolveAgentHandoffCommand {
            tenant_id: auth.tenant_id,
            conversation_id,
            resolved_by: auth.actor_id,
        },
        auth.actor_kind.as_str(),
    )?))
}

async fn close_agent_handoff(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AgentHandoffStateView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.close_agent_handoff_with_actor_kind(
        CloseAgentHandoffCommand {
            tenant_id: auth.tenant_id,
            conversation_id,
            closed_by: auth.actor_id,
        },
        auth.actor_kind.as_str(),
    )?))
}

async fn list_members(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ListMembersResponse>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    state.runtime.require_active_member(
        auth.tenant_id.as_str(),
        conversation_id.as_str(),
        auth.actor_id.as_str(),
    )?;
    Ok(Json(ListMembersResponse {
        items: state
            .runtime
            .list_members(auth.tenant_id.as_str(), conversation_id.as_str())?,
    }))
}

async fn add_member(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AddConversationMemberRequest>,
) -> Result<Json<ConversationMember>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.add_member_with_actor_kind(
        AddConversationMemberCommand {
            tenant_id: auth.tenant_id,
            conversation_id,
            principal_id: request.principal_id,
            principal_kind: request.principal_kind,
            role: request.role,
            invited_by: auth.actor_id,
        },
        auth.actor_kind.as_str(),
    )?))
}

async fn remove_member(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RemoveConversationMemberRequest>,
) -> Result<Json<ConversationMember>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.remove_member_with_actor_kind(
        RemoveConversationMemberCommand {
            tenant_id: auth.tenant_id,
            conversation_id,
            member_id: request.member_id,
            removed_by: auth.actor_id,
        },
        auth.actor_kind.as_str(),
    )?))
}

async fn transfer_conversation_owner(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<TransferConversationOwnerRequest>,
) -> Result<Json<TransferConversationOwnerResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state.runtime.transfer_conversation_owner_with_actor_kind(
            TransferConversationOwnerCommand {
                tenant_id: auth.tenant_id,
                conversation_id,
                target_member_id: request.member_id,
                transferred_by: auth.actor_id,
            },
            auth.actor_kind.as_str(),
        )?,
    ))
}

async fn change_conversation_member_role(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<ChangeConversationMemberRoleRequest>,
) -> Result<Json<ChangeConversationMemberRoleResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state
            .runtime
            .change_conversation_member_role_with_actor_kind(
                ChangeConversationMemberRoleCommand {
                    tenant_id: auth.tenant_id,
                    conversation_id,
                    target_member_id: request.member_id,
                    new_role: request.role,
                    changed_by: auth.actor_id,
                },
                auth.actor_kind.as_str(),
            )?,
    ))
}

async fn leave_conversation(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ConversationMember>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.leave_conversation_with_actor_kind(
        LeaveConversationCommand {
            tenant_id: auth.tenant_id,
            conversation_id,
            principal_id: auth.actor_id,
        },
        auth.actor_kind.as_str(),
    )?))
}

async fn get_read_cursor(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ConversationReadCursorView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    state.runtime.require_active_member(
        auth.tenant_id.as_str(),
        conversation_id.as_str(),
        auth.actor_id.as_str(),
    )?;
    Ok(Json(state.runtime.read_cursor_view(
        auth.tenant_id.as_str(),
        conversation_id.as_str(),
        auth.actor_id.as_str(),
    )?))
}

async fn update_read_cursor(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateReadCursorRequest>,
) -> Result<Json<ConversationReadCursorView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    state.runtime.update_read_cursor_with_actor_kind(
        UpdateReadCursorCommand {
            tenant_id: auth.tenant_id.clone(),
            conversation_id: conversation_id.clone(),
            principal_id: auth.actor_id.clone(),
            read_seq: request.read_seq,
            last_read_message_id: request.last_read_message_id,
        },
        auth.actor_kind.as_str(),
    )?;

    Ok(Json(state.runtime.read_cursor_view(
        auth.tenant_id.as_str(),
        conversation_id.as_str(),
        auth.actor_id.as_str(),
    )?))
}

async fn post_message(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PostMessageRequest>,
) -> Result<Json<PostMessageResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let body = build_message_body(
        request.summary,
        request.text,
        request.parts,
        request.render_hints,
    )?;

    let result = state.runtime.post_message(PostMessageCommand {
        tenant_id: auth.tenant_id,
        conversation_id,
        sender: Sender {
            id: auth.actor_id,
            kind: auth.actor_kind,
            member_id: None,
            device_id: auth.device_id,
            session_id: auth.session_id,
            metadata: BTreeMap::new(),
        },
        client_msg_id: request.client_msg_id,
        message_type: MessageType::Standard,
        body,
    })?;
    Ok(Json(result))
}

async fn publish_system_channel_message(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PostMessageRequest>,
) -> Result<Json<PostMessageResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let body = build_message_body(
        request.summary,
        request.text,
        request.parts,
        request.render_hints,
    )?;

    let result =
        state
            .runtime
            .publish_system_channel_message(PublishSystemChannelMessageCommand {
                tenant_id: auth.tenant_id,
                conversation_id,
                publisher: Sender {
                    id: auth.actor_id,
                    kind: auth.actor_kind,
                    member_id: None,
                    device_id: None,
                    session_id: auth.session_id,
                    metadata: BTreeMap::new(),
                },
                client_msg_id: request.client_msg_id,
                body,
            })?;

    Ok(Json(result))
}

async fn edit_message(
    Path(message_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<EditMessageRequest>,
) -> Result<Json<MessageMutationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let body = build_message_body(
        request.summary,
        request.text,
        request.parts,
        request.render_hints,
    )?;
    Ok(Json(state.runtime.edit_message(EditMessageCommand {
        tenant_id: auth.tenant_id,
        message_id,
        editor: Sender {
            id: auth.actor_id,
            kind: auth.actor_kind,
            member_id: None,
            device_id: auth.device_id,
            session_id: auth.session_id,
            metadata: BTreeMap::new(),
        },
        body,
    })?))
}

async fn recall_message(
    Path(message_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<MessageMutationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.recall_message(
        RecallMessageCommand {
            tenant_id: auth.tenant_id,
            message_id,
            recalled_by: Sender {
                id: auth.actor_id,
                kind: auth.actor_kind,
                member_id: None,
                device_id: auth.device_id,
                session_id: auth.session_id,
                metadata: BTreeMap::new(),
            },
        },
    )?))
}

fn build_message_body(
    summary: Option<String>,
    text: Option<String>,
    parts: Vec<ContentPart>,
    render_hints: BTreeMap<String, String>,
) -> Result<MessageBody, ApiError> {
    let mut resolved_parts = Vec::new();
    if let Some(text) = text {
        if !text.trim().is_empty() {
            resolved_parts.push(ContentPart::text(text));
        }
    }
    resolved_parts.extend(parts);

    if resolved_parts.is_empty() {
        return Err(ApiError::bad_request(
            "message_body_empty",
            "message body must contain text or parts",
        ));
    }

    Ok(MessageBody {
        summary,
        parts: resolved_parts,
        render_hints,
    })
}

fn conversation_scope_key(tenant_id: &str, conversation_id: &str) -> String {
    format!("{tenant_id}:{conversation_id}")
}

fn ensure_generic_creatable_conversation_type(conversation_type: &str) -> Result<(), RuntimeError> {
    if matches!(conversation_type, "group" | "direct") {
        return Ok(());
    }

    if matches!(
        conversation_type,
        "agent_dialog" | "agent_handoff" | "system_channel"
    ) {
        return Err(RuntimeError::ConversationTypeInvalid(format!(
            "conversation type {conversation_type} requires a dedicated create command"
        )));
    }

    Err(RuntimeError::ConversationTypeInvalid(format!(
        "unsupported conversation type: {conversation_type}"
    )))
}

fn ensure_agent_dialog_requester_kind(requester_kind: &str) -> Result<(), RuntimeError> {
    if requester_kind == "user" {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "agent dialog requires user requester, got {requester_kind}"
    )))
}

fn ensure_agent_handoff_source_kind(source_kind: &str) -> Result<(), RuntimeError> {
    if source_kind == "agent" {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "agent handoff requires agent source, got {source_kind}"
    )))
}

fn ensure_agent_handoff_target_kind(target_kind: &str) -> Result<(), RuntimeError> {
    if matches!(target_kind, "user" | "agent") {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "agent handoff target kind must be user or agent, got {target_kind}"
    )))
}

fn ensure_system_channel_requester_kind(requester_kind: &str) -> Result<(), RuntimeError> {
    if requester_kind == "system" {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "system channel requires system requester, got {requester_kind}"
    )))
}

fn build_conversation_member(
    tenant_id: &str,
    conversation_id: &str,
    member_id: String,
    principal_id: &str,
    principal_kind: &str,
    role: MembershipRole,
    invited_by: Option<String>,
    joined_at: String,
) -> ConversationMember {
    build_conversation_member_with_attributes(
        tenant_id,
        conversation_id,
        member_id,
        principal_id,
        principal_kind,
        role,
        invited_by,
        joined_at,
        BTreeMap::new(),
    )
}

fn build_conversation_member_with_attributes(
    tenant_id: &str,
    conversation_id: &str,
    member_id: String,
    principal_id: &str,
    principal_kind: &str,
    role: MembershipRole,
    invited_by: Option<String>,
    joined_at: String,
    attributes: BTreeMap<String, String>,
) -> ConversationMember {
    ConversationMember {
        tenant_id: tenant_id.into(),
        conversation_id: conversation_id.into(),
        member_id,
        principal_id: principal_id.into(),
        principal_kind: principal_kind.into(),
        role,
        state: MembershipState::Joined,
        invited_by,
        joined_at,
        removed_at: None,
        attributes,
    }
}

fn upsert_member(conversation: &mut ConversationState, member: ConversationMember) {
    conversation
        .principal_members
        .insert(member.principal_id.clone(), member.member_id.clone());
    conversation
        .members
        .insert(member.member_id.clone(), member);
}

fn next_member_episode(conversation: &ConversationState, principal_id: &str) -> u64 {
    conversation
        .members
        .values()
        .filter(|member| member.principal_id == principal_id)
        .count() as u64
        + 1
}

fn resolve_active_member_id(
    conversation: &ConversationState,
    principal_id: &str,
) -> Result<String, RuntimeError> {
    let member_id = conversation
        .principal_members
        .get(principal_id)
        .ok_or_else(|| {
            RuntimeError::PermissionDenied(format!(
                "principal is not active conversation member: {principal_id}"
            ))
        })?;
    let member = conversation
        .members
        .get(member_id.as_str())
        .ok_or_else(|| {
            RuntimeError::PermissionDenied(format!(
                "principal is not active conversation member: {principal_id}"
            ))
        })?;
    if !member.is_active() {
        return Err(RuntimeError::PermissionDenied(format!(
            "principal is not active conversation member: {principal_id}"
        )));
    }
    Ok(member_id.clone())
}

fn resolve_active_member(
    conversation: &ConversationState,
    principal_id: &str,
) -> Result<ConversationMember, RuntimeError> {
    let member_id = resolve_active_member_id(conversation, principal_id)?;
    conversation
        .members
        .get(member_id.as_str())
        .cloned()
        .ok_or_else(|| {
            RuntimeError::PermissionDenied(format!(
                "principal is not active conversation member: {principal_id}"
            ))
        })
}

fn ensure_agent_handoff_conversation(conversation: &ConversationState) -> Result<(), RuntimeError> {
    if conversation.conversation_type == "agent_handoff" {
        return Ok(());
    }

    Err(RuntimeError::ConversationTypeInvalid(format!(
        "conversation type {} is not agent_handoff",
        conversation.conversation_type
    )))
}

fn ensure_actor_kind_matches_member(
    actor_member: &ConversationMember,
    actor_kind: &str,
) -> Result<(), RuntimeError> {
    if actor_member.principal_kind == actor_kind {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "actor kind {} does not match member principal kind {}",
        actor_kind, actor_member.principal_kind
    )))
}

fn build_handoff_actor_view(member: &ConversationMember) -> ChangeAgentHandoffStatusView {
    ChangeAgentHandoffStatusView {
        id: member.principal_id.clone(),
        kind: member.principal_kind.clone(),
    }
}

fn ensure_target_handoff_actor(
    handoff_state: &AgentHandoffStateView,
    actor: &ChangeAgentHandoffStatusView,
) -> Result<(), RuntimeError> {
    if &handoff_state.target == actor {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "actor {} is not the handoff target",
        actor.id
    )))
}

fn ensure_source_or_target_handoff_actor(
    handoff_state: &AgentHandoffStateView,
    actor: &ChangeAgentHandoffStatusView,
) -> Result<(), RuntimeError> {
    if &handoff_state.source == actor || &handoff_state.target == actor {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "actor {} is not part of the handoff lifecycle",
        actor.id
    )))
}

fn is_closed_agent_handoff(conversation: &ConversationState) -> bool {
    conversation
        .handoff_state
        .as_ref()
        .is_some_and(|state| state.status == "closed")
}

fn ensure_member_add_actor_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
) -> Result<(), RuntimeError> {
    match conversation.conversation_type.as_str() {
        "group" => {
            if matches!(
                actor_member.role,
                MembershipRole::Owner | MembershipRole::Admin
            ) {
                return Ok(());
            }

            Err(RuntimeError::PermissionDenied(format!(
                "member {} cannot add members in group conversation",
                actor_member.principal_id
            )))
        }
        "direct" => {
            if matches!(actor_member.role, MembershipRole::Owner) {
                return Ok(());
            }

            Err(RuntimeError::PermissionDenied(format!(
                "member {} cannot add peer in direct conversation",
                actor_member.principal_id
            )))
        }
        conversation_type => Err(RuntimeError::PermissionDenied(format!(
            "conversation type {conversation_type} does not support generic member add"
        ))),
    }
}

fn ensure_member_add_request_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
    requested_role: &MembershipRole,
) -> Result<(), RuntimeError> {
    match conversation.conversation_type.as_str() {
        "group" => match actor_member.role {
            MembershipRole::Owner => {
                if matches!(requested_role, MembershipRole::Owner) {
                    return Err(RuntimeError::PermissionDenied(
                        "group conversation does not support creating a second owner".into(),
                    ));
                }

                Ok(())
            }
            MembershipRole::Admin => {
                if matches!(
                    requested_role,
                    MembershipRole::Member | MembershipRole::Guest
                ) {
                    return Ok(());
                }

                Err(RuntimeError::PermissionDenied(format!(
                    "admin member {} cannot assign elevated role",
                    actor_member.principal_id
                )))
            }
            _ => Err(RuntimeError::PermissionDenied(format!(
                "member {} cannot add members",
                actor_member.principal_id
            ))),
        },
        "direct" => {
            if conversation.principal_members.len() >= 2 {
                return Err(RuntimeError::PermissionDenied(
                    "direct conversation already has the maximum number of active participants"
                        .into(),
                ));
            }
            if matches!(
                requested_role,
                MembershipRole::Owner | MembershipRole::Admin
            ) {
                return Err(RuntimeError::PermissionDenied(
                    "direct conversation peer cannot be assigned owner/admin role".into(),
                ));
            }

            Ok(())
        }
        conversation_type => Err(RuntimeError::PermissionDenied(format!(
            "conversation type {conversation_type} does not support generic member add"
        ))),
    }
}

fn ensure_member_remove_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
    target_member: &ConversationMember,
) -> Result<(), RuntimeError> {
    match conversation.conversation_type.as_str() {
        "group" => match actor_member.role {
            MembershipRole::Owner => {
                if matches!(target_member.role, MembershipRole::Owner) {
                    return Err(RuntimeError::PermissionDenied(
                        "group conversation owner cannot be removed via remove_member".into(),
                    ));
                }

                Ok(())
            }
            MembershipRole::Admin => {
                if matches!(
                    target_member.role,
                    MembershipRole::Member | MembershipRole::Guest
                ) {
                    return Ok(());
                }

                Err(RuntimeError::PermissionDenied(format!(
                    "admin member {} cannot remove privileged member {}",
                    actor_member.principal_id, target_member.principal_id
                )))
            }
            _ => Err(RuntimeError::PermissionDenied(format!(
                "member {} cannot remove members in group conversation",
                actor_member.principal_id
            ))),
        },
        "direct" => Err(RuntimeError::PermissionDenied(
            "direct conversation does not support member removal".into(),
        )),
        conversation_type => Err(RuntimeError::PermissionDenied(format!(
            "conversation type {conversation_type} does not support generic member removal"
        ))),
    }
}

fn ensure_current_active_member_target(
    conversation: &ConversationState,
    target_member: &ConversationMember,
) -> Result<(), RuntimeError> {
    let active_member_id = conversation
        .principal_members
        .get(target_member.principal_id.as_str())
        .ok_or_else(|| RuntimeError::MemberNotFound(target_member.member_id.clone()))?;
    if active_member_id != target_member.member_id.as_str() || !target_member.is_active() {
        return Err(RuntimeError::MemberNotFound(
            target_member.member_id.clone(),
        ));
    }

    Ok(())
}

fn ensure_member_leave_allowed(
    conversation: &ConversationState,
    member: &ConversationMember,
) -> Result<(), RuntimeError> {
    match conversation.conversation_type.as_str() {
        "group" => {
            if matches!(member.role, MembershipRole::Owner) {
                return Err(RuntimeError::PermissionDenied(
                    "group conversation owner cannot leave until owner transfer is supported"
                        .into(),
                ));
            }

            Ok(())
        }
        "direct" => Err(RuntimeError::PermissionDenied(
            "direct conversation does not support self leave".into(),
        )),
        conversation_type => Err(RuntimeError::PermissionDenied(format!(
            "conversation type {conversation_type} does not support generic self leave"
        ))),
    }
}

fn ensure_owner_transfer_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
    target_member: &ConversationMember,
) -> Result<(), RuntimeError> {
    match conversation.conversation_type.as_str() {
        "group" => {
            if !matches!(actor_member.role, MembershipRole::Owner) {
                return Err(RuntimeError::PermissionDenied(format!(
                    "member {} cannot transfer group ownership",
                    actor_member.principal_id
                )));
            }
            if !target_member.is_active() {
                return Err(RuntimeError::PermissionDenied(
                    "owner transfer target must be an active member".into(),
                ));
            }
            if actor_member.member_id == target_member.member_id {
                return Err(RuntimeError::PermissionDenied(
                    "owner transfer target must be another active member".into(),
                ));
            }

            Ok(())
        }
        "direct" => Err(RuntimeError::PermissionDenied(
            "direct conversation does not support owner transfer".into(),
        )),
        conversation_type => Err(RuntimeError::PermissionDenied(format!(
            "conversation type {conversation_type} does not support owner transfer"
        ))),
    }
}

fn ensure_member_role_change_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
    target_member: &ConversationMember,
    requested_role: &MembershipRole,
) -> Result<(), RuntimeError> {
    match conversation.conversation_type.as_str() {
        "group" => {
            if !matches!(actor_member.role, MembershipRole::Owner) {
                return Err(RuntimeError::PermissionDenied(format!(
                    "member {} cannot change member roles in group conversation",
                    actor_member.principal_id
                )));
            }
            if matches!(target_member.role, MembershipRole::Owner)
                || matches!(requested_role, MembershipRole::Owner)
            {
                return Err(RuntimeError::PermissionDenied(
                    "group owner role must be changed via owner transfer".into(),
                ));
            }
            if target_member.role == *requested_role {
                return Err(RuntimeError::PermissionDenied(
                    "target member already has the requested role".into(),
                ));
            }

            Ok(())
        }
        "direct" => Err(RuntimeError::PermissionDenied(
            "direct conversation does not support generic member role change".into(),
        )),
        conversation_type => Err(RuntimeError::PermissionDenied(format!(
            "conversation type {conversation_type} does not support generic member role change"
        ))),
    }
}

fn ensure_message_post_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
) -> Result<(), RuntimeError> {
    if is_closed_agent_handoff(conversation) {
        return Err(RuntimeError::Conflict(format!(
            "agent handoff {} is already closed",
            actor_member.conversation_id
        )));
    }

    match conversation.conversation_type.as_str() {
        "group" | "direct" | "agent_dialog" | "agent_handoff" => Ok(()),
        "system_channel" => Err(RuntimeError::PermissionDenied(
            "system channel requires dedicated publish command".into(),
        )),
        _ => Err(RuntimeError::PermissionDenied(format!(
            "conversation type {} does not support message post",
            conversation.conversation_type
        ))),
    }
}

fn ensure_system_channel_publish_command_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
) -> Result<(), RuntimeError> {
    if conversation.conversation_type != "system_channel" {
        return Err(RuntimeError::ConversationTypeInvalid(format!(
            "conversation type {} does not support system channel publish",
            conversation.conversation_type
        )));
    }

    ensure_system_channel_publisher_write_allowed(actor_member, "system_channel.publish")
}

fn ensure_message_edit_allowed(
    actor_id: &str,
    actor_member: &ConversationMember,
    conversation_type: &str,
    handoff_closed: bool,
    message: &Message,
) -> Result<(), RuntimeError> {
    if conversation_type == "agent_handoff" && handoff_closed {
        return Err(RuntimeError::Conflict(format!(
            "agent handoff {} is already closed",
            actor_member.conversation_id
        )));
    }
    if message.sender.id == actor_id {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "member {} cannot edit message owned by {}",
        actor_member.principal_id, message.sender.id
    )))
}

fn ensure_message_recall_allowed(
    actor_id: &str,
    actor_member: &ConversationMember,
    conversation_type: &str,
    handoff_closed: bool,
    message: &Message,
) -> Result<(), RuntimeError> {
    if conversation_type == "agent_handoff" && handoff_closed {
        return Err(RuntimeError::Conflict(format!(
            "agent handoff {} is already closed",
            actor_member.conversation_id
        )));
    }
    if message.sender.id == actor_id {
        return Ok(());
    }

    if conversation_type == "group"
        && matches!(
            actor_member.role,
            MembershipRole::Owner | MembershipRole::Admin
        )
    {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "member {} cannot recall message owned by {}",
        actor_member.principal_id, message.sender.id
    )))
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn ensure_conversation_bound_write_allowed(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        principal_id: &str,
        capability: &str,
    ) -> Result<(), RuntimeError> {
        let actor_kind = self
            .require_active_member(tenant_id, conversation_id, principal_id)?
            .principal_kind;
        self.ensure_conversation_bound_write_allowed_with_actor_kind(
            tenant_id,
            conversation_id,
            principal_id,
            actor_kind.as_str(),
            capability,
        )
    }

    pub fn ensure_conversation_bound_write_allowed_with_actor_kind(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        principal_id: &str,
        actor_kind: &str,
        capability: &str,
    ) -> Result<(), RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, conversation_id);
        let state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        let actor_member = resolve_active_member(conversation, principal_id)?;
        ensure_actor_kind_matches_member(&actor_member, actor_kind)?;
        ensure_conversation_bound_write_allowed(conversation, &actor_member, capability)
    }

    pub fn require_active_member(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        principal_id: &str,
    ) -> Result<ConversationMember, RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, conversation_id);
        let state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        let member_id = resolve_active_member_id(conversation, principal_id)?;
        conversation
            .members
            .get(member_id.as_str())
            .cloned()
            .ok_or_else(|| {
                RuntimeError::PermissionDenied(format!(
                    "principal is not active conversation member: {principal_id}"
                ))
            })
    }
}

fn ensure_conversation_bound_write_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
    capability: &str,
) -> Result<(), RuntimeError> {
    if conversation.conversation_type == "agent_handoff" && is_closed_agent_handoff(conversation) {
        return Err(RuntimeError::Conflict(format!(
            "agent handoff {} is already closed for {capability}",
            actor_member.conversation_id
        )));
    }

    if conversation.conversation_type == "system_channel" {
        return ensure_system_channel_publisher_write_allowed(actor_member, capability);
    }

    Ok(())
}

fn ensure_system_channel_publisher_write_allowed(
    actor_member: &ConversationMember,
    capability: &str,
) -> Result<(), RuntimeError> {
    let is_publisher = actor_member.principal_kind == "system"
        && actor_member
            .attributes
            .get("channelRole")
            .map(String::as_str)
            == Some("publisher");
    if is_publisher {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "member {} cannot perform {capability} in system channel",
        actor_member.principal_id
    )))
}

fn upsert_read_cursor(conversation: &mut ConversationState, cursor: ConversationReadCursor) {
    conversation
        .read_cursors
        .insert(cursor.member_id.clone(), cursor);
}

fn build_member_envelope(
    tenant_id: &str,
    conversation_id: &str,
    event_type: &'static str,
    member: ConversationMember,
    ordering_seq: u64,
    actor_id: &str,
    actor_kind: &str,
) -> CommitEnvelope {
    let event_suffix = match event_type {
        "conversation.member_removed" => "removed",
        "conversation.member_left" => "left",
        _ => "joined",
    };
    let event_timestamp = if matches!(
        event_type,
        "conversation.member_removed" | "conversation.member_left"
    ) {
        member
            .removed_at
            .clone()
            .unwrap_or_else(|| member.joined_at.clone())
    } else {
        member.joined_at.clone()
    };

    CommitEnvelope {
        event_id: format!("evt_{}_{}", member.member_id, event_suffix),
        tenant_id: tenant_id.into(),
        event_type: event_type.into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: conversation_id.into(),
        scope_type: "conversation".into(),
        scope_id: conversation_id.into(),
        ordering_key: CommitEnvelope::ordering_key(tenant_id, conversation_id),
        ordering_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: actor_id.into(),
            actor_kind: actor_kind.into(),
            actor_session_id: None,
        },
        occurred_at: event_timestamp.clone(),
        committed_at: event_timestamp,
        payload_schema: Some("conversation.member.v1".into()),
        payload: serde_json::to_string(&member)
            .expect("conversation member payload should serialize into commit envelope"),
        retention_class: "standard".into(),
        audit_class: "default".into(),
    }
}

fn build_default_read_cursor(member: &ConversationMember) -> ConversationReadCursor {
    ConversationReadCursor {
        tenant_id: member.tenant_id.clone(),
        conversation_id: member.conversation_id.clone(),
        member_id: member.member_id.clone(),
        principal_id: member.principal_id.clone(),
        read_seq: 0,
        last_read_message_id: None,
        updated_at: member.joined_at.clone(),
    }
}

fn build_read_cursor_envelope(
    tenant_id: &str,
    conversation_id: &str,
    cursor: ConversationReadCursor,
    ordering_seq: u64,
    actor_id: &str,
    actor_kind: &str,
) -> CommitEnvelope {
    CommitEnvelope {
        event_id: format!("evt_{}_cursor_{}", cursor.member_id, ordering_seq),
        tenant_id: tenant_id.into(),
        event_type: "conversation.read_cursor_updated".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: conversation_id.into(),
        scope_type: "conversation".into(),
        scope_id: conversation_id.into(),
        ordering_key: CommitEnvelope::ordering_key(tenant_id, conversation_id),
        ordering_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: actor_id.into(),
            actor_kind: actor_kind.into(),
            actor_session_id: None,
        },
        occurred_at: cursor.updated_at.clone(),
        committed_at: cursor.updated_at.clone(),
        payload_schema: Some("conversation.read_cursor.v1".into()),
        payload: serde_json::to_string(&cursor)
            .expect("read cursor payload should serialize into commit envelope"),
        retention_class: "standard".into(),
        audit_class: "default".into(),
    }
}

fn build_owner_transfer_envelope(
    payload: TransferConversationOwnerPayload,
    ordering_seq: u64,
    actor_id: &str,
    actor_kind: &str,
) -> CommitEnvelope {
    CommitEnvelope {
        event_id: format!(
            "evt_{}_owner_transfer_{}",
            payload.conversation_id, ordering_seq
        ),
        tenant_id: payload.tenant_id.clone(),
        event_type: "conversation.owner_transferred".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: payload.conversation_id.clone(),
        scope_type: "conversation".into(),
        scope_id: payload.conversation_id.clone(),
        ordering_key: CommitEnvelope::ordering_key(
            payload.tenant_id.as_str(),
            payload.conversation_id.as_str(),
        ),
        ordering_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: actor_id.into(),
            actor_kind: actor_kind.into(),
            actor_session_id: None,
        },
        occurred_at: payload.transferred_at.clone(),
        committed_at: payload.transferred_at.clone(),
        payload_schema: Some("conversation.owner_transferred.v1".into()),
        payload: serde_json::to_string(&payload)
            .expect("owner transfer payload should serialize into commit envelope"),
        retention_class: "standard".into(),
        audit_class: "default".into(),
    }
}

fn build_member_role_changed_envelope(
    payload: ChangeConversationMemberRolePayload,
    ordering_seq: u64,
    actor_id: &str,
    actor_kind: &str,
) -> CommitEnvelope {
    CommitEnvelope {
        event_id: format!(
            "evt_{}_role_change_{}",
            payload.updated_member.member_id, ordering_seq
        ),
        tenant_id: payload.tenant_id.clone(),
        event_type: "conversation.member_role_changed".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: payload.conversation_id.clone(),
        scope_type: "conversation".into(),
        scope_id: payload.conversation_id.clone(),
        ordering_key: CommitEnvelope::ordering_key(
            payload.tenant_id.as_str(),
            payload.conversation_id.as_str(),
        ),
        ordering_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: actor_id.into(),
            actor_kind: actor_kind.into(),
            actor_session_id: None,
        },
        occurred_at: payload.changed_at.clone(),
        committed_at: payload.changed_at.clone(),
        payload_schema: Some("conversation.member_role_changed.v1".into()),
        payload: serde_json::to_string(&payload)
            .expect("member role changed payload should serialize into commit envelope"),
        retention_class: "standard".into(),
        audit_class: "default".into(),
    }
}

fn build_agent_handoff_status_changed_envelope(
    payload: AgentHandoffStatusChangedPayload,
    ordering_seq: u64,
    actor_id: &str,
    actor_kind: &str,
) -> CommitEnvelope {
    CommitEnvelope {
        event_id: format!(
            "evt_{}_agent_handoff_status_{}",
            payload.conversation_id, ordering_seq
        ),
        tenant_id: payload.tenant_id.clone(),
        event_type: "conversation.agent_handoff_status_changed".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: payload.conversation_id.clone(),
        scope_type: "conversation".into(),
        scope_id: payload.conversation_id.clone(),
        ordering_key: CommitEnvelope::ordering_key(
            payload.tenant_id.as_str(),
            payload.conversation_id.as_str(),
        ),
        ordering_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: actor_id.into(),
            actor_kind: actor_kind.into(),
            actor_session_id: None,
        },
        occurred_at: payload.changed_at.clone(),
        committed_at: payload.changed_at.clone(),
        payload_schema: Some("conversation.agent_handoff_status_changed.v1".into()),
        payload: serde_json::to_string(&payload)
            .expect("agent handoff status payload should serialize into commit envelope"),
        retention_class: "standard".into(),
        audit_class: "default".into(),
    }
}

fn build_message_edited_envelope(message: &MessageEdited, event_id: &str) -> CommitEnvelope {
    CommitEnvelope {
        event_id: event_id.into(),
        tenant_id: message.tenant_id.clone(),
        event_type: "message.edited".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: message.conversation_id.clone(),
        scope_type: "conversation".into(),
        scope_id: message.conversation_id.clone(),
        ordering_key: CommitEnvelope::ordering_key(
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
        ),
        ordering_seq: message.message_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: message.editor.id.clone(),
            actor_kind: message.editor.kind.clone(),
            actor_session_id: message.editor.session_id.clone(),
        },
        occurred_at: message.edited_at.clone(),
        committed_at: message.edited_at.clone(),
        payload_schema: Some("message.edited.v1".into()),
        payload: serde_json::to_string(message)
            .expect("message edited payload should serialize into commit envelope"),
        retention_class: "standard".into(),
        audit_class: "default".into(),
    }
}

fn build_message_recalled_envelope(message: &MessageRecalled, event_id: &str) -> CommitEnvelope {
    CommitEnvelope {
        event_id: event_id.into(),
        tenant_id: message.tenant_id.clone(),
        event_type: "message.recalled".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: message.conversation_id.clone(),
        scope_type: "conversation".into(),
        scope_id: message.conversation_id.clone(),
        ordering_key: CommitEnvelope::ordering_key(
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
        ),
        ordering_seq: message.message_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: message.recalled_by.id.clone(),
            actor_kind: message.recalled_by.kind.clone(),
            actor_session_id: message.recalled_by.session_id.clone(),
        },
        occurred_at: message.recalled_at.clone(),
        committed_at: message.recalled_at.clone(),
        payload_schema: Some("message.recalled.v1".into()),
        payload: serde_json::to_string(message)
            .expect("message recalled payload should serialize into commit envelope"),
        retention_class: "standard".into(),
        audit_class: "default".into(),
    }
}

fn message_scope_key(tenant_id: &str, message_id: &str) -> String {
    format!("{tenant_id}:{message_id}")
}

fn member_id(conversation_id: &str, principal_id: &str) -> String {
    member_episode_id(conversation_id, principal_id, 1)
}

fn member_episode_id(conversation_id: &str, principal_id: &str, episode: u64) -> String {
    if episode <= 1 {
        return format!("cm_{conversation_id}_{principal_id}");
    }

    format!("cm_{conversation_id}_{principal_id}_e{episode}")
}

fn conversation_timestamp() -> String {
    utc_now_rfc3339_millis()
}
