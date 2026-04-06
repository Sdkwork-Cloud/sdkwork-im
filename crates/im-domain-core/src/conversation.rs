use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipRole {
    Owner,
    Admin,
    Member,
    Guest,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipState {
    Joined,
    Invited,
    Left,
    Removed,
}

impl MembershipState {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Joined | Self::Invited)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationMember {
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
    pub attributes: BTreeMap<String, String>,
}

impl ConversationMember {
    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationReadCursor {
    pub tenant_id: String,
    pub conversation_id: String,
    pub member_id: String,
    pub principal_id: String,
    pub read_seq: u64,
    pub last_read_message_id: Option<String>,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationReadCursorView {
    pub tenant_id: String,
    pub conversation_id: String,
    pub member_id: String,
    pub principal_id: String,
    pub read_seq: u64,
    pub last_read_message_id: Option<String>,
    pub updated_at: String,
    pub unread_count: u64,
}

impl ConversationReadCursorView {
    pub fn from_cursor(cursor: &ConversationReadCursor, unread_count: u64) -> Self {
        Self {
            tenant_id: cursor.tenant_id.clone(),
            conversation_id: cursor.conversation_id.clone(),
            member_id: cursor.member_id.clone(),
            principal_id: cursor.principal_id.clone(),
            read_seq: cursor.read_seq,
            last_read_message_id: cursor.last_read_message_id.clone(),
            updated_at: cursor.updated_at.clone(),
            unread_count,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationActorView {
    pub id: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationAgentHandoffView {
    pub status: String,
    pub source: ConversationActorView,
    pub target: ConversationActorView,
    pub handoff_session_id: String,
    pub handoff_reason: Option<String>,
    pub accepted_at: Option<String>,
    pub accepted_by: Option<ConversationActorView>,
    pub resolved_at: Option<String>,
    pub resolved_by: Option<ConversationActorView>,
    pub closed_at: Option<String>,
    pub closed_by: Option<ConversationActorView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationInboxEntry {
    pub tenant_id: String,
    pub principal_id: String,
    pub member_id: String,
    pub conversation_id: String,
    pub conversation_type: String,
    pub message_count: u64,
    pub last_message_id: Option<String>,
    pub last_message_seq: u64,
    pub last_sender_id: Option<String>,
    pub last_sender_kind: Option<String>,
    pub last_summary: Option<String>,
    pub unread_count: u64,
    pub last_activity_at: String,
    pub agent_handoff: Option<ConversationAgentHandoffView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSyncFeedEntry {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
    pub sync_seq: u64,
    pub origin_event_id: String,
    pub origin_event_type: String,
    pub conversation_id: Option<String>,
    pub message_id: Option<String>,
    pub message_seq: Option<u64>,
    pub member_id: Option<String>,
    pub read_seq: Option<u64>,
    pub last_read_message_id: Option<String>,
    pub actor_id: Option<String>,
    pub actor_kind: Option<String>,
    pub actor_device_id: Option<String>,
    pub summary: Option<String>,
    pub payload_schema: Option<String>,
    pub payload: Option<String>,
    pub occurred_at: String,
}
