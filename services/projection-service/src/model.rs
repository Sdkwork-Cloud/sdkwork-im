use im_domain_core::conversation::{
    ConversationAgentHandoffView, ConversationMember, MembershipRole, MembershipState,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineViewEntry {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub summary: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummarySenderView {
    pub id: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageReactionCountView {
    pub reaction_key: String,
    pub count: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionActorView {
    pub id: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagePinView {
    pub pinned_by: InteractionActorView,
    pub pinned_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageInteractionSummaryView {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub total_reaction_count: u64,
    pub reaction_counts: Vec<MessageReactionCountView>,
    pub pin: Option<MessagePinView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationMemberDirectoryEntry {
    pub tenant_id: String,
    pub conversation_id: String,
    pub member_id: String,
    pub principal_id: String,
    pub principal_kind: String,
    pub role: MembershipRole,
    pub state: MembershipState,
    pub invited_by: Option<String>,
    pub joined_at: String,
    pub removed_at: Option<String>,
    pub attributes: std::collections::BTreeMap<String, String>,
}

impl ConversationMemberDirectoryEntry {
    pub fn from_member(member: &ConversationMember) -> Self {
        Self {
            tenant_id: member.tenant_id.clone(),
            conversation_id: member.conversation_id.clone(),
            member_id: member.member_id.clone(),
            principal_id: member.principal_id.clone(),
            principal_kind: member.principal_kind.clone(),
            role: member.role.clone(),
            state: member.state.clone(),
            invited_by: member.invited_by.clone(),
            joined_at: member.joined_at.clone(),
            removed_at: member.removed_at.clone(),
            attributes: member.attributes.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationSummaryView {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_count: u64,
    pub last_message_id: Option<String>,
    pub last_message_seq: u64,
    pub last_sender_id: Option<String>,
    pub last_sender_kind: Option<String>,
    pub last_sender: Option<SummarySenderView>,
    pub last_summary: Option<String>,
    pub last_message_at: Option<String>,
    pub agent_handoff: Option<ConversationAgentHandoffView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredDeviceView {
    pub tenant_id: String,
    pub principal_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal_kind: Option<String>,
    pub device_id: String,
    pub registered_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeFanoutTarget {
    pub principal_id: String,
    pub principal_kind: Option<String>,
    pub device_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRecipientView {
    pub principal_id: String,
    pub principal_kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactView {
    pub tenant_id: String,
    pub owner_user_id: String,
    pub target_user_id: String,
    pub contact_type: String,
    pub relationship_state: String,
    pub friendship_id: String,
    pub direct_chat_id: Option<String>,
    pub conversation_id: Option<String>,
    pub established_at: String,
    pub last_interaction_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ContactDirectChatBindingView {
    pub(super) direct_chat_id: String,
    pub(super) conversation_id: String,
    pub(super) bound_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct ConversationCatalogEntry {
    pub(super) conversation_type: String,
    pub(super) created_at: String,
}
