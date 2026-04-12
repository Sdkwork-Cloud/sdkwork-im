use craw_chat_contract_core::ContractError;
use craw_chat_contract_message::{CommitJournal, CommitPosition};
use im_auth_context::AuthContext;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex, MutexGuard};

pub use im_domain_core::conversation::{
    AgentHandoffStateView, ChangeAgentHandoffStatusView, ConversationBusinessBinding,
};
use im_domain_core::conversation::{
    ConversationAggregateState, ConversationMember, ConversationPolicy, ConversationReadCursor,
    ConversationReadCursorView, ConversationRoster, MembershipRole, MembershipState,
    build_conversation_member_with_attributes, build_default_read_cursor, member_episode_id,
    member_id,
};
use im_domain_core::message::{
    ConversationMessageLog, Message, MessageBody, MessageEdited, MessageLocatorIndex,
    MessagePinned, MessageReactionAdded, MessageReactionRemoved, MessageRecalled, MessageType,
    MessageUnpinned, Sender,
};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_time::utc_now_rfc3339_millis;
use serde::{Deserialize, Serialize};
use serde_json::json;

mod binding;
mod creation;
mod governance;
mod handoff;
mod http;
mod membership;
mod policy;
mod recovery;
mod support;

use self::governance::ConversationPolicyAppliedPayload;
use self::policy::MessagePostPolicy;
use self::support::{
    build_agent_handoff_status_changed_envelope, build_conversation_policy_applied_envelope,
    build_member_envelope, build_member_role_changed_envelope, build_message_edited_envelope,
    build_message_pinned_envelope, build_message_reaction_added_envelope,
    build_message_reaction_removed_envelope, build_message_recalled_envelope,
    build_message_unpinned_envelope, build_owner_transfer_envelope, build_read_cursor_envelope,
    conversation_business_scope_key, conversation_retention_class, conversation_scope_key,
    conversation_timestamp, event_id_component, next_member_episode, resolve_active_member,
    resolve_active_member_id, upsert_member, upsert_read_cursor,
};
pub use http::{build_default_app, build_public_app};

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
pub struct CreateThreadConversationCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub parent_conversation_id: String,
    pub root_message_id: String,
    pub creator_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BindDirectChatConversationCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub direct_chat_id: String,
    pub left_actor_id: String,
    pub left_actor_kind: String,
    pub right_actor_id: String,
    pub right_actor_kind: String,
    pub bound_by: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncSharedChannelLinkedMemberCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub shared_channel_policy_id: String,
    pub external_connection_id: String,
    pub local_actor_id: String,
    pub local_actor_kind: String,
    pub external_member_id: String,
    pub synced_by: String,
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
pub struct ApplyConversationPolicyCommand {
    pub tenant_id: String,
    pub conversation_id: String,
    pub applied_by: String,
    pub policy: ConversationPolicy,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMessageReactionCommand {
    pub tenant_id: String,
    pub message_id: String,
    pub reaction_key: String,
    pub reacted_by: Sender,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveMessageReactionCommand {
    pub tenant_id: String,
    pub message_id: String,
    pub reaction_key: String,
    pub removed_by: Sender,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PinMessageCommand {
    pub tenant_id: String,
    pub message_id: String,
    pub pinned_by: Sender,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnpinMessageCommand {
    pub tenant_id: String,
    pub message_id: String,
    pub unpinned_by: Sender,
}

impl CreateConversationCommand {
    pub fn from_auth_context(
        auth: &AuthContext,
        conversation_id: String,
        conversation_type: String,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            creator_id: auth.actor_id.clone(),
            conversation_type,
        }
    }
}

impl CreateAgentDialogCommand {
    pub fn from_auth_context(
        auth: &AuthContext,
        conversation_id: String,
        agent_id: String,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            requester_id: auth.actor_id.clone(),
            agent_id,
        }
    }
}

impl CreateAgentHandoffCommand {
    pub fn from_auth_context(
        auth: &AuthContext,
        conversation_id: String,
        target_id: String,
        target_kind: String,
        handoff_session_id: String,
        handoff_reason: Option<String>,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            source_id: auth.actor_id.clone(),
            target_id,
            target_kind,
            handoff_session_id,
            handoff_reason,
        }
    }
}

impl CreateSystemChannelCommand {
    pub fn from_auth_context(
        auth: &AuthContext,
        conversation_id: String,
        subscriber_id: String,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            requester_id: auth.actor_id.clone(),
            subscriber_id,
        }
    }
}

impl CreateThreadConversationCommand {
    pub fn from_auth_context(
        auth: &AuthContext,
        conversation_id: String,
        parent_conversation_id: String,
        root_message_id: String,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            parent_conversation_id,
            root_message_id,
            creator_id: auth.actor_id.clone(),
        }
    }
}

impl BindDirectChatConversationCommand {
    pub fn from_auth_context(
        auth: &AuthContext,
        conversation_id: String,
        direct_chat_id: String,
        left_actor_id: String,
        left_actor_kind: String,
        right_actor_id: String,
        right_actor_kind: String,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            direct_chat_id,
            left_actor_id,
            left_actor_kind,
            right_actor_id,
            right_actor_kind,
            bound_by: auth.actor_id.clone(),
        }
    }
}

impl SyncSharedChannelLinkedMemberCommand {
    pub fn from_auth_context(
        auth: &AuthContext,
        conversation_id: String,
        shared_channel_policy_id: String,
        external_connection_id: String,
        local_actor_id: String,
        local_actor_kind: String,
        external_member_id: String,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            shared_channel_policy_id,
            external_connection_id,
            local_actor_id,
            local_actor_kind,
            external_member_id,
            synced_by: auth.actor_id.clone(),
        }
    }
}

impl AcceptAgentHandoffCommand {
    pub fn from_auth_context(auth: &AuthContext, conversation_id: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            accepted_by: auth.actor_id.clone(),
        }
    }
}

impl ResolveAgentHandoffCommand {
    pub fn from_auth_context(auth: &AuthContext, conversation_id: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            resolved_by: auth.actor_id.clone(),
        }
    }
}

impl CloseAgentHandoffCommand {
    pub fn from_auth_context(auth: &AuthContext, conversation_id: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            closed_by: auth.actor_id.clone(),
        }
    }
}

impl AddConversationMemberCommand {
    pub fn from_auth_context(
        auth: &AuthContext,
        conversation_id: String,
        principal_id: String,
        principal_kind: String,
        role: MembershipRole,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            principal_id,
            principal_kind,
            role,
            invited_by: auth.actor_id.clone(),
        }
    }
}

impl RemoveConversationMemberCommand {
    pub fn from_auth_context(
        auth: &AuthContext,
        conversation_id: String,
        member_id: String,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            member_id,
            removed_by: auth.actor_id.clone(),
        }
    }
}

impl LeaveConversationCommand {
    pub fn from_auth_context(auth: &AuthContext, conversation_id: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            principal_id: auth.actor_id.clone(),
        }
    }
}

impl TransferConversationOwnerCommand {
    pub fn from_auth_context(
        auth: &AuthContext,
        conversation_id: String,
        target_member_id: String,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            target_member_id,
            transferred_by: auth.actor_id.clone(),
        }
    }
}

impl ChangeConversationMemberRoleCommand {
    pub fn from_auth_context(
        auth: &AuthContext,
        conversation_id: String,
        target_member_id: String,
        new_role: MembershipRole,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            target_member_id,
            new_role,
            changed_by: auth.actor_id.clone(),
        }
    }
}

impl UpdateReadCursorCommand {
    pub fn from_auth_context(
        auth: &AuthContext,
        conversation_id: String,
        read_seq: u64,
        last_read_message_id: Option<String>,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            principal_id: auth.actor_id.clone(),
            read_seq,
            last_read_message_id,
        }
    }
}

impl ApplyConversationPolicyCommand {
    pub fn from_auth_context(
        auth: &AuthContext,
        conversation_id: String,
        policy: ConversationPolicy,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            applied_by: auth.actor_id.clone(),
            policy,
        }
    }
}

fn sender_from_auth_context(auth: &AuthContext) -> Sender {
    Sender {
        id: auth.actor_id.clone(),
        kind: auth.actor_kind.clone(),
        member_id: None,
        device_id: auth.device_id.clone(),
        session_id: auth.session_id.clone(),
        metadata: BTreeMap::new(),
    }
}

impl PostMessageCommand {
    pub fn from_auth_context(
        auth: &AuthContext,
        conversation_id: String,
        client_msg_id: Option<String>,
        message_type: MessageType,
        body: MessageBody,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            sender: sender_from_auth_context(auth),
            client_msg_id,
            message_type,
            body,
        }
    }
}

impl PublishSystemChannelMessageCommand {
    pub fn from_auth_context(
        auth: &AuthContext,
        conversation_id: String,
        client_msg_id: Option<String>,
        body: MessageBody,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            conversation_id,
            publisher: sender_from_auth_context(auth),
            client_msg_id,
            body,
        }
    }
}

impl EditMessageCommand {
    pub fn from_auth_context(auth: &AuthContext, message_id: String, body: MessageBody) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            message_id,
            editor: sender_from_auth_context(auth),
            body,
        }
    }
}

impl RecallMessageCommand {
    pub fn from_auth_context(auth: &AuthContext, message_id: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            message_id,
            recalled_by: sender_from_auth_context(auth),
        }
    }
}

impl AddMessageReactionCommand {
    pub fn from_auth_context(auth: &AuthContext, message_id: String, reaction_key: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            message_id,
            reaction_key,
            reacted_by: sender_from_auth_context(auth),
        }
    }
}

impl RemoveMessageReactionCommand {
    pub fn from_auth_context(auth: &AuthContext, message_id: String, reaction_key: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            message_id,
            reaction_key,
            removed_by: sender_from_auth_context(auth),
        }
    }
}

impl PinMessageCommand {
    pub fn from_auth_context(auth: &AuthContext, message_id: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            message_id,
            pinned_by: sender_from_auth_context(auth),
        }
    }
}

impl UnpinMessageCommand {
    pub fn from_auth_context(auth: &AuthContext, message_id: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            message_id,
            unpinned_by: sender_from_auth_context(auth),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageMutationResult {
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub event_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageHistoryResult {
    pub items: Vec<im_domain_core::message::StoredMessage>,
    pub high_watermark: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageReactionMutationResult {
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub reaction_key: String,
    pub event_id: Option<String>,
    pub changed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagePinMutationResult {
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub event_id: Option<String>,
    pub changed: bool,
}

#[derive(Debug)]
pub enum RuntimeError {
    ConversationAlreadyExists(String),
    ConversationTypeInvalid(String),
    InvalidInput(String),
    ConversationNotFound(String),
    ConversationBindingNotFound(String),
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
    aggregate: ConversationAggregateState,
    roster: ConversationRoster,
    message_log: ConversationMessageLog,
}

#[derive(Default)]
struct RuntimeState {
    conversations: HashMap<String, ConversationState>,
    business_index: HashMap<String, String>,
    message_locator: MessageLocatorIndex,
}

fn lock_runtime_mutex<'a, T>(mutex: &'a Mutex<T>, label: &'static str) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("warning: recovering poisoned mutex in conversation-runtime: {label}");
            poisoned.into_inner()
        }
    }
}

// This internal transition result keeps the full idempotent view and mutation
// payload inline because the runtime immediately pattern-matches and forwards it
// within a single command path; boxing would add indirection without changing
// release behavior.
#[allow(clippy::large_enum_variant)]
enum AgentHandoffStatusTransitionOutcome {
    Idempotent(AgentHandoffStateView),
    Mutated {
        payload: AgentHandoffStatusChangedPayload,
        ordering_seq: u64,
        actor_id: String,
        actor_kind: String,
        retention_class: String,
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
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let mut events = self.events.lock().expect("journal should lock");
        events.push(envelope);
        Ok(CommitPosition::new("p0", events.len() as u64))
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
        let (message_seq, message_id, message, retention_class) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let (message_seq, message_id, message, retention_class) = {
                let conversation =
                    state
                        .conversations
                        .get_mut(scope_key.as_str())
                        .ok_or_else(|| {
                            RuntimeError::ConversationNotFound(command.conversation_id.clone())
                        })?;
                let sender_member =
                    resolve_active_member(conversation, command.sender.id.as_str())?;
                policy::ensure_actor_kind_matches_member(
                    &sender_member,
                    command.sender.kind.as_str(),
                )?;
                match policy {
                    MessagePostPolicy::GenericPost => {
                        policy::ensure_message_post_allowed(conversation, &sender_member)?
                    }
                    MessagePostPolicy::SystemChannelPublish => {
                        policy::ensure_system_channel_publish_command_allowed(
                            conversation,
                            &sender_member,
                        )?
                    }
                }
                let message_seq = conversation.message_log.next_message_seq();

                let mut sender = command.sender.clone();
                if sender.member_id.is_none() {
                    sender.member_id = Some(sender_member.member_id.clone());
                }

                let message_id = format!("msg_{}_{}", command.conversation_id, message_seq);
                let message_timestamp = conversation_timestamp();
                let message = Message {
                    tenant_id: command.tenant_id.clone(),
                    conversation_id: command.conversation_id.clone(),
                    message_id: message_id.clone(),
                    message_seq,
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
                conversation.message_log.store_posted(message.clone());
                let retention_class = conversation_retention_class(conversation);
                (message_seq, message_id, message, retention_class)
            };

            state.message_locator.register_message(&message);

            (message_seq, message_id, message, retention_class)
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
            retention_class,
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
        let (edited, retention_class) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation_id = state
                .message_locator
                .conversation_id(command.tenant_id.as_str(), command.message_id.as_str())
                .map(str::to_owned)
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let scope_key =
                conversation_scope_key(command.tenant_id.as_str(), conversation_id.as_str());
            let conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let editor_member = resolve_active_member(conversation, command.editor.id.as_str())?;
            policy::ensure_actor_kind_matches_member(&editor_member, command.editor.kind.as_str())?;
            let scenario = conversation.aggregate.scenario();
            let handoff_closed = conversation.aggregate.has_closed_handoff();
            let stored = conversation
                .message_log
                .message(command.message_id.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            if stored.recalled {
                return Err(RuntimeError::MessageAlreadyRecalled(command.message_id));
            }
            policy::ensure_message_edit_allowed(
                command.editor.id.as_str(),
                &editor_member,
                scenario,
                handoff_closed,
                &stored.message,
            )?;
            let conversation_id = stored.message.conversation_id.clone();
            let message_id = stored.message.message_id.clone();
            let message_seq = stored.message.message_seq;

            let mut editor = command.editor.clone();
            if editor.member_id.is_none() {
                editor.member_id = Some(editor_member.member_id.clone());
            }

            let edited_at = conversation_timestamp();
            let edited = MessageEdited {
                tenant_id: command.tenant_id.clone(),
                conversation_id,
                message_id,
                message_seq,
                body: command.body,
                editor,
                edited_at,
            };
            conversation
                .message_log
                .apply_edited(&edited)
                .expect("stored message should exist before edit mutation");
            let retention_class = conversation_retention_class(conversation);
            (edited, retention_class)
        };

        let event_id = format!("evt_{}_edited", edited.message_id);
        self.journal.append(build_message_edited_envelope(
            &edited,
            event_id.as_str(),
            retention_class.as_str(),
        ))?;

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
        let (recalled, retention_class) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation_id = state
                .message_locator
                .conversation_id(command.tenant_id.as_str(), command.message_id.as_str())
                .map(str::to_owned)
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let scope_key =
                conversation_scope_key(command.tenant_id.as_str(), conversation_id.as_str());
            let conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let recalled_member =
                resolve_active_member(conversation, command.recalled_by.id.as_str())?;
            policy::ensure_actor_kind_matches_member(
                &recalled_member,
                command.recalled_by.kind.as_str(),
            )?;
            let scenario = conversation.aggregate.scenario();
            let handoff_closed = conversation.aggregate.has_closed_handoff();
            let stored = conversation
                .message_log
                .message(command.message_id.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            if stored.recalled {
                return Err(RuntimeError::MessageAlreadyRecalled(command.message_id));
            }
            policy::ensure_message_recall_allowed(
                command.recalled_by.id.as_str(),
                &recalled_member,
                scenario,
                handoff_closed,
                &stored.message,
            )?;
            let conversation_id = stored.message.conversation_id.clone();
            let message_id = stored.message.message_id.clone();
            let message_seq = stored.message.message_seq;

            let mut recalled_by = command.recalled_by.clone();
            if recalled_by.member_id.is_none() {
                recalled_by.member_id = Some(recalled_member.member_id.clone());
            }

            let recalled_at = conversation_timestamp();
            let recalled = MessageRecalled {
                tenant_id: command.tenant_id.clone(),
                conversation_id,
                message_id,
                message_seq,
                recalled_by,
                recalled_at,
            };
            conversation
                .message_log
                .apply_recalled(&recalled)
                .expect("stored message should exist before recall mutation");
            let retention_class = conversation_retention_class(conversation);
            (recalled, retention_class)
        };

        let event_id = format!("evt_{}_recalled", recalled.message_id);
        self.journal.append(build_message_recalled_envelope(
            &recalled,
            event_id.as_str(),
            retention_class.as_str(),
        ))?;

        Ok(MessageMutationResult {
            conversation_id: recalled.conversation_id,
            message_id: recalled.message_id,
            message_seq: recalled.message_seq,
            event_id,
        })
    }

    pub fn add_message_reaction(
        &self,
        command: AddMessageReactionCommand,
    ) -> Result<MessageReactionMutationResult, RuntimeError> {
        let (reaction, changed, retention_class) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation_id = state
                .message_locator
                .conversation_id(command.tenant_id.as_str(), command.message_id.as_str())
                .map(str::to_owned)
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let scope_key =
                conversation_scope_key(command.tenant_id.as_str(), conversation_id.as_str());
            let conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let reacted_member =
                resolve_active_member(conversation, command.reacted_by.id.as_str())?;
            policy::ensure_actor_kind_matches_member(
                &reacted_member,
                command.reacted_by.kind.as_str(),
            )?;
            policy::ensure_message_reaction_allowed(conversation, &reacted_member)?;
            let stored = conversation
                .message_log
                .message(command.message_id.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            if stored.recalled {
                return Err(RuntimeError::MessageAlreadyRecalled(command.message_id));
            }
            let conversation_id = stored.message.conversation_id.clone();
            let message_id = stored.message.message_id.clone();
            let message_seq = stored.message.message_seq;

            let mut reacted_by = command.reacted_by.clone();
            if reacted_by.member_id.is_none() {
                reacted_by.member_id = Some(reacted_member.member_id.clone());
            }

            let reaction = MessageReactionAdded {
                tenant_id: command.tenant_id.clone(),
                conversation_id,
                message_id,
                message_seq,
                reaction_key: command.reaction_key,
                reacted_by,
                reacted_at: conversation_timestamp(),
            };
            let changed = conversation
                .message_log
                .apply_reaction_added(&reaction)
                .expect("stored message should exist before reaction add");
            let retention_class = conversation_retention_class(conversation);
            (reaction, changed, retention_class)
        };

        let event_id = if changed {
            let event_id = format!(
                "evt_{}_reaction_added_{}_{}_{}",
                reaction.message_id,
                event_id_component(reaction.reaction_key.as_str()),
                event_id_component(reaction.reacted_by.id.as_str()),
                event_id_component(reaction.reacted_at.as_str())
            );
            self.journal.append(build_message_reaction_added_envelope(
                &reaction,
                event_id.as_str(),
                retention_class.as_str(),
            ))?;
            Some(event_id)
        } else {
            None
        };

        Ok(MessageReactionMutationResult {
            conversation_id: reaction.conversation_id,
            message_id: reaction.message_id,
            message_seq: reaction.message_seq,
            reaction_key: reaction.reaction_key,
            event_id,
            changed,
        })
    }

    pub fn remove_message_reaction(
        &self,
        command: RemoveMessageReactionCommand,
    ) -> Result<MessageReactionMutationResult, RuntimeError> {
        let (reaction, changed, retention_class) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation_id = state
                .message_locator
                .conversation_id(command.tenant_id.as_str(), command.message_id.as_str())
                .map(str::to_owned)
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let scope_key =
                conversation_scope_key(command.tenant_id.as_str(), conversation_id.as_str());
            let conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let removed_member =
                resolve_active_member(conversation, command.removed_by.id.as_str())?;
            policy::ensure_actor_kind_matches_member(
                &removed_member,
                command.removed_by.kind.as_str(),
            )?;
            policy::ensure_message_reaction_allowed(conversation, &removed_member)?;
            let stored = conversation
                .message_log
                .message(command.message_id.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            if stored.recalled {
                return Err(RuntimeError::MessageAlreadyRecalled(command.message_id));
            }
            let conversation_id = stored.message.conversation_id.clone();
            let message_id = stored.message.message_id.clone();
            let message_seq = stored.message.message_seq;

            let mut removed_by = command.removed_by.clone();
            if removed_by.member_id.is_none() {
                removed_by.member_id = Some(removed_member.member_id.clone());
            }

            let reaction = MessageReactionRemoved {
                tenant_id: command.tenant_id.clone(),
                conversation_id,
                message_id,
                message_seq,
                reaction_key: command.reaction_key,
                removed_by,
                removed_at: conversation_timestamp(),
            };
            let changed = conversation
                .message_log
                .apply_reaction_removed(&reaction)
                .expect("stored message should exist before reaction remove");
            let retention_class = conversation_retention_class(conversation);
            (reaction, changed, retention_class)
        };

        let event_id = if changed {
            let event_id = format!(
                "evt_{}_reaction_removed_{}_{}_{}",
                reaction.message_id,
                event_id_component(reaction.reaction_key.as_str()),
                event_id_component(reaction.removed_by.id.as_str()),
                event_id_component(reaction.removed_at.as_str())
            );
            self.journal
                .append(build_message_reaction_removed_envelope(
                    &reaction,
                    event_id.as_str(),
                    retention_class.as_str(),
                ))?;
            Some(event_id)
        } else {
            None
        };

        Ok(MessageReactionMutationResult {
            conversation_id: reaction.conversation_id,
            message_id: reaction.message_id,
            message_seq: reaction.message_seq,
            reaction_key: reaction.reaction_key,
            event_id,
            changed,
        })
    }

    pub fn pin_message(
        &self,
        command: PinMessageCommand,
    ) -> Result<MessagePinMutationResult, RuntimeError> {
        let (pin, changed, retention_class) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation_id = state
                .message_locator
                .conversation_id(command.tenant_id.as_str(), command.message_id.as_str())
                .map(str::to_owned)
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let scope_key =
                conversation_scope_key(command.tenant_id.as_str(), conversation_id.as_str());
            let conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let pinned_member = resolve_active_member(conversation, command.pinned_by.id.as_str())?;
            policy::ensure_actor_kind_matches_member(
                &pinned_member,
                command.pinned_by.kind.as_str(),
            )?;
            policy::ensure_message_pin_allowed(conversation, &pinned_member)?;
            let stored = conversation
                .message_log
                .message(command.message_id.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            if stored.recalled {
                return Err(RuntimeError::MessageAlreadyRecalled(command.message_id));
            }
            let conversation_id = stored.message.conversation_id.clone();
            let message_id = stored.message.message_id.clone();
            let message_seq = stored.message.message_seq;

            let mut pinned_by = command.pinned_by.clone();
            if pinned_by.member_id.is_none() {
                pinned_by.member_id = Some(pinned_member.member_id.clone());
            }

            let pin = MessagePinned {
                tenant_id: command.tenant_id.clone(),
                conversation_id,
                message_id,
                message_seq,
                pinned_by,
                pinned_at: conversation_timestamp(),
            };
            let changed = conversation
                .message_log
                .apply_pinned(&pin)
                .expect("stored message should exist before pin");
            let retention_class = conversation_retention_class(conversation);
            (pin, changed, retention_class)
        };

        let event_id = if changed {
            let event_id = format!(
                "evt_{}_pin_added_{}_{}",
                pin.message_id,
                event_id_component(pin.pinned_by.id.as_str()),
                event_id_component(pin.pinned_at.as_str())
            );
            self.journal.append(build_message_pinned_envelope(
                &pin,
                event_id.as_str(),
                retention_class.as_str(),
            ))?;
            Some(event_id)
        } else {
            None
        };

        Ok(MessagePinMutationResult {
            conversation_id: pin.conversation_id,
            message_id: pin.message_id,
            message_seq: pin.message_seq,
            event_id,
            changed,
        })
    }

    pub fn unpin_message(
        &self,
        command: UnpinMessageCommand,
    ) -> Result<MessagePinMutationResult, RuntimeError> {
        let (pin, changed, retention_class) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation_id = state
                .message_locator
                .conversation_id(command.tenant_id.as_str(), command.message_id.as_str())
                .map(str::to_owned)
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let scope_key =
                conversation_scope_key(command.tenant_id.as_str(), conversation_id.as_str());
            let conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let unpinned_member =
                resolve_active_member(conversation, command.unpinned_by.id.as_str())?;
            policy::ensure_actor_kind_matches_member(
                &unpinned_member,
                command.unpinned_by.kind.as_str(),
            )?;
            policy::ensure_message_pin_allowed(conversation, &unpinned_member)?;
            let stored = conversation
                .message_log
                .message(command.message_id.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            if stored.recalled {
                return Err(RuntimeError::MessageAlreadyRecalled(command.message_id));
            }
            let conversation_id = stored.message.conversation_id.clone();
            let message_id = stored.message.message_id.clone();
            let message_seq = stored.message.message_seq;

            let mut unpinned_by = command.unpinned_by.clone();
            if unpinned_by.member_id.is_none() {
                unpinned_by.member_id = Some(unpinned_member.member_id.clone());
            }

            let pin = MessageUnpinned {
                tenant_id: command.tenant_id.clone(),
                conversation_id,
                message_id,
                message_seq,
                unpinned_by,
                unpinned_at: conversation_timestamp(),
            };
            let changed = conversation
                .message_log
                .apply_unpinned(&pin)
                .expect("stored message should exist before unpin");
            let retention_class = conversation_retention_class(conversation);
            (pin, changed, retention_class)
        };

        let event_id = if changed {
            let event_id = format!(
                "evt_{}_pin_removed_{}_{}",
                pin.message_id,
                event_id_component(pin.unpinned_by.id.as_str()),
                event_id_component(pin.unpinned_at.as_str())
            );
            self.journal.append(build_message_unpinned_envelope(
                &pin,
                event_id.as_str(),
                retention_class.as_str(),
            ))?;
            Some(event_id)
        } else {
            None
        };

        Ok(MessagePinMutationResult {
            conversation_id: pin.conversation_id,
            message_id: pin.message_id,
            message_seq: pin.message_seq,
            event_id,
            changed,
        })
    }
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn require_active_member_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
    ) -> Result<ConversationMember, RuntimeError> {
        self.require_active_member(
            auth.tenant_id.as_str(),
            conversation_id,
            auth.actor_id.as_str(),
        )
    }

    pub fn ensure_conversation_bound_write_allowed_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
        capability: &str,
    ) -> Result<(), RuntimeError> {
        self.ensure_conversation_bound_write_allowed_with_actor_kind(
            auth.tenant_id.as_str(),
            conversation_id,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            capability,
        )
    }

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
        policy::ensure_actor_kind_matches_member(&actor_member, actor_kind)?;
        policy::ensure_conversation_bound_write_allowed(conversation, &actor_member, capability)
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
            .roster
            .member(member_id.as_str())
            .cloned()
            .ok_or_else(|| {
                RuntimeError::PermissionDenied(format!(
                    "principal is not active conversation member: {principal_id}"
                ))
            })
    }
}
