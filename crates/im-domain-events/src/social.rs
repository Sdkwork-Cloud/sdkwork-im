use serde::{Deserialize, Serialize};

use crate::{AggregateType, CommitEnvelope, EventActor};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialEventType {
    FriendRequestSubmitted,
    FriendshipActivated,
    FriendshipRemoved,
    ExternalConnectionEstablished,
    ExternalMemberLinkBound,
    SharedChannelPolicyApplied,
    UserBlocked,
    UserBlockReleased,
    DirectChatCreated,
    DirectChatBound,
}

impl SocialEventType {
    pub fn as_wire_value(&self) -> &'static str {
        match self {
            Self::FriendRequestSubmitted => "friend_request.submitted",
            Self::FriendshipActivated => "friendship.activated",
            Self::FriendshipRemoved => "friendship.removed",
            Self::ExternalConnectionEstablished => "external_connection.established",
            Self::ExternalMemberLinkBound => "external_member_link.bound",
            Self::SharedChannelPolicyApplied => "shared_channel_policy.applied",
            Self::UserBlocked => "user_block.blocked",
            Self::UserBlockReleased => "user_block.released",
            Self::DirectChatCreated => "direct_chat.created",
            Self::DirectChatBound => "direct_chat.bound",
        }
    }

    pub fn payload_schema(&self) -> &'static str {
        match self {
            Self::FriendRequestSubmitted => "social.friend_request.submitted.v1",
            Self::FriendshipActivated => "social.friendship.activated.v1",
            Self::FriendshipRemoved => "social.friendship.removed.v1",
            Self::ExternalConnectionEstablished => "social.external_connection.established.v1",
            Self::ExternalMemberLinkBound => "social.external_member_link.bound.v1",
            Self::SharedChannelPolicyApplied => "social.shared_channel_policy.applied.v1",
            Self::UserBlocked => "social.user_block.blocked.v1",
            Self::UserBlockReleased => "social.user_block.released.v1",
            Self::DirectChatCreated => "social.direct_chat.created.v1",
            Self::DirectChatBound => "social.direct_chat.bound.v1",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendRequestSubmittedPayload {
    pub request_id: String,
    pub requester_user_id: String,
    pub target_user_id: String,
    pub request_message: Option<String>,
    pub requested_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendshipActivatedPayload {
    pub friendship_id: String,
    pub user_low_id: String,
    pub user_high_id: String,
    pub initiator_user_id: String,
    pub direct_chat_id: Option<String>,
    pub established_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalConnectionEstablishedPayload {
    pub connection_id: String,
    pub external_tenant_id: String,
    pub external_org_name: Option<String>,
    pub connection_kind: String,
    pub established_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalMemberLinkBoundPayload {
    pub link_id: String,
    pub connection_id: String,
    pub local_actor_id: String,
    pub local_actor_kind: Option<String>,
    pub external_member_id: String,
    pub external_display_name: Option<String>,
    pub linked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedChannelPolicyAppliedPayload {
    pub policy_id: String,
    pub connection_id: String,
    pub channel_id: String,
    pub conversation_id: Option<String>,
    pub policy_version: u64,
    pub history_visibility: String,
    pub applied_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBlockedPayload {
    pub block_id: String,
    pub blocker_user_id: String,
    pub blocked_user_id: String,
    pub scope: String,
    pub direct_chat_id: Option<String>,
    pub expires_at: Option<String>,
    pub effective_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectChatBoundPayload {
    pub direct_chat_id: String,
    pub conversation_id: String,
    pub left_actor_id: String,
    pub right_actor_id: String,
    pub pair_hash: String,
    pub bound_at: String,
}

pub fn social_commit_envelope(
    event_id: &str,
    tenant_id: &str,
    aggregate_type: AggregateType,
    aggregate_id: &str,
    event_type: SocialEventType,
    ordering_seq: u64,
    actor: EventActor,
    occurred_at: &str,
    committed_at: &str,
    payload: &str,
) -> CommitEnvelope {
    let scope_type = aggregate_type.as_wire_value();
    CommitEnvelope {
        event_id: event_id.into(),
        tenant_id: tenant_id.into(),
        event_type: event_type.as_wire_value().into(),
        event_version: 1,
        aggregate_type,
        aggregate_id: aggregate_id.into(),
        scope_type: scope_type.into(),
        scope_id: aggregate_id.into(),
        ordering_key: CommitEnvelope::ordering_key(tenant_id, aggregate_id),
        ordering_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor,
        occurred_at: occurred_at.into(),
        committed_at: committed_at.into(),
        payload_schema: Some(event_type.payload_schema().into()),
        payload: payload.into(),
        retention_class: "standard".into(),
        audit_class: "social".into(),
    }
}
