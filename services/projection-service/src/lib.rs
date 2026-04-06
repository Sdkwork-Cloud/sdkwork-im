use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, Request};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use im_auth_context::{
    AuthContext, AuthContextError, resolve_auth_context, resolve_public_bearer_auth_context,
};
use im_domain_core::conversation::{
    ConversationActorView, ConversationAgentHandoffView, ConversationInboxEntry,
    ConversationMember, ConversationReadCursor, ConversationReadCursorView, DeviceSyncFeedEntry,
};
use im_domain_core::message::{Message, MessageEdited, MessageRecalled};
use im_domain_events::CommitEnvelope;
use im_time::utc_now_rfc3339_millis;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineViewEntry {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub summary: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SummarySenderView {
    pub id: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredDeviceView {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
    pub registered_at: String,
}

#[derive(Default)]
pub struct TimelineProjectionService {
    entries: Mutex<HashMap<String, Vec<TimelineViewEntry>>>,
    summaries: Mutex<HashMap<String, ConversationSummaryView>>,
    members: Mutex<HashMap<String, HashMap<String, ConversationMember>>>,
    read_cursors: Mutex<HashMap<String, HashMap<String, ConversationReadCursor>>>,
    conversations: Mutex<HashMap<String, ConversationCatalogEntry>>,
    registered_devices: Mutex<HashMap<String, HashMap<String, RegisteredDeviceView>>>,
    device_sync_feeds: Mutex<HashMap<String, Vec<DeviceSyncFeedEntry>>>,
    device_sync_sequences: Mutex<HashMap<String, u64>>,
}

#[derive(Clone, Debug)]
struct ConversationCatalogEntry {
    conversation_type: String,
    created_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConversationCreatedPayload {
    conversation_type: String,
    source: Option<ProjectionActorView>,
    target: Option<ProjectionActorView>,
    handoff: Option<ConversationCreatedHandoffPayload>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConversationMemberRoleChangedPayload {
    updated_member: ConversationMember,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProjectionActorView {
    id: String,
    kind: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConversationCreatedHandoffPayload {
    session_id: String,
    reason: Option<String>,
    status: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AgentHandoffStatusChangedProjectionPayload {
    changed_by: ProjectionActorView,
    changed_at: String,
    state: ProjectionAgentHandoffStatePayload,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProjectionAgentHandoffStatePayload {
    conversation_id: String,
    status: String,
    source: ProjectionActorView,
    target: ProjectionActorView,
    handoff_session_id: String,
    handoff_reason: Option<String>,
    accepted_at: Option<String>,
    accepted_by: Option<ProjectionActorView>,
    resolved_at: Option<String>,
    resolved_by: Option<ProjectionActorView>,
    closed_at: Option<String>,
    closed_by: Option<ProjectionActorView>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterDeviceRequest {
    device_id: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct SyncFeedQuery {
    after_seq: Option<u64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TimelineResponse {
    items: Vec<TimelineViewEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct InboxResponse {
    items: Vec<ConversationInboxEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DeviceSyncFeedResponse {
    items: Vec<DeviceSyncFeedEntry>,
}

impl TimelineProjectionService {
    pub fn is_active_member(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        principal_id: &str,
    ) -> bool {
        self.members
            .lock()
            .expect("member store should lock")
            .get(scope_key(tenant_id, conversation_id).as_str())
            .and_then(|scope_members| scope_members.get(principal_id))
            .is_some_and(ConversationMember::is_active)
    }

    pub fn apply(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        match event.event_type.as_str() {
            "conversation.created" => self.apply_conversation_created(event),
            "conversation.agent_handoff_status_changed" => {
                self.apply_agent_handoff_status_changed(event)
            }
            "message.posted" => self.apply_message_posted(event),
            "message.edited" => self.apply_message_edited(event),
            "message.recalled" => self.apply_message_recalled(event),
            "conversation.member_joined" => self.apply_member_joined(event),
            "conversation.member_role_changed" => self.apply_member_role_changed(event),
            "conversation.member_removed" => self.apply_member_removed(event),
            "conversation.member_left" => self.apply_member_left(event),
            "conversation.read_cursor_updated" => self.apply_read_cursor_updated(event),
            _ => Ok(()),
        }
    }

    pub fn register_device(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> RegisteredDeviceView {
        let device = RegisteredDeviceView {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            device_id: device_id.into(),
            registered_at: registered_device_at(),
        };
        let scope = principal_scope_key(tenant_id, principal_id);
        self.registered_devices
            .lock()
            .expect("registered device store should lock")
            .entry(scope)
            .or_default()
            .insert(device_id.into(), device.clone());
        self.device_sync_feeds
            .lock()
            .expect("device sync feed store should lock")
            .entry(device_feed_scope_key(tenant_id, principal_id, device_id))
            .or_default();
        self.device_sync_sequences
            .lock()
            .expect("device sync sequence store should lock")
            .entry(device_feed_scope_key(tenant_id, principal_id, device_id))
            .or_insert(0);
        device
    }

    pub fn device_sync_feed(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        after_seq: Option<u64>,
    ) -> Vec<DeviceSyncFeedEntry> {
        let min_seq = after_seq.unwrap_or_default();
        self.device_sync_feeds
            .lock()
            .expect("device sync feed store should lock")
            .get(device_feed_scope_key(tenant_id, principal_id, device_id).as_str())
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter(|entry| entry.sync_seq > min_seq)
            .collect()
    }

    pub fn registered_devices(
        &self,
        tenant_id: &str,
        principal_id: &str,
    ) -> Vec<RegisteredDeviceView> {
        let mut devices = self
            .registered_devices
            .lock()
            .expect("registered device store should lock")
            .get(principal_scope_key(tenant_id, principal_id).as_str())
            .map(|items| items.values().cloned().collect::<Vec<_>>())
            .unwrap_or_default();
        devices.sort_by(|left, right| left.device_id.cmp(&right.device_id));
        devices
    }

    pub fn latest_device_sync_seq(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> u64 {
        self.device_sync_sequences
            .lock()
            .expect("device sync sequence store should lock")
            .get(device_feed_scope_key(tenant_id, principal_id, device_id).as_str())
            .copied()
            .unwrap_or_default()
    }

    fn apply_conversation_created(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let payload: ConversationCreatedPayload =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let handoff_view = handoff_view_from_created_payload(&payload)?;
        let key = scope_key(event.tenant_id.as_str(), event.aggregate_id.as_str());
        self.conversations
            .lock()
            .expect("conversation store should lock")
            .insert(
                key.clone(),
                ConversationCatalogEntry {
                    conversation_type: payload.conversation_type,
                    created_at: event.committed_at.clone(),
                },
            );
        let conversation_id = event.aggregate_id.clone();
        let tenant_id = event.tenant_id.clone();
        let mut summaries = self.summaries.lock().expect("summary store should lock");
        let summary = summaries
            .entry(key)
            .or_insert_with(|| ConversationSummaryView {
                tenant_id: tenant_id.clone(),
                conversation_id: conversation_id.clone(),
                message_count: 0,
                last_message_id: None,
                last_message_seq: 0,
                last_sender_id: None,
                last_sender_kind: None,
                last_sender: None,
                last_summary: None,
                last_message_at: None,
                agent_handoff: None,
            });
        if handoff_view.is_some() {
            summary.agent_handoff = handoff_view;
        }
        Ok(())
    }

    fn apply_agent_handoff_status_changed(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload: AgentHandoffStatusChangedProjectionPayload =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let handoff_view = handoff_view_from_state_payload(&payload.state);
        let key = scope_key(
            event.tenant_id.as_str(),
            payload.state.conversation_id.as_str(),
        );
        let mut summaries = self.summaries.lock().expect("summary store should lock");
        let summary = summaries
            .entry(key)
            .or_insert_with(|| ConversationSummaryView {
                tenant_id: event.tenant_id.clone(),
                conversation_id: payload.state.conversation_id.clone(),
                message_count: 0,
                last_message_id: None,
                last_message_seq: 0,
                last_sender_id: None,
                last_sender_kind: None,
                last_sender: None,
                last_summary: None,
                last_message_at: None,
                agent_handoff: None,
            });
        summary.agent_handoff = Some(handoff_view);
        drop(summaries);
        self.fan_out_agent_handoff_status_to_device_sync_feeds(event, &payload);
        Ok(())
    }

    fn apply_message_posted(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let message: Message =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let tenant_id = message.tenant_id.clone();
        let conversation_id = message.conversation_id.clone();
        let message_id = message.message_id.clone();
        let message_seq = message.message_seq;
        let summary = message.body.summary.clone();
        let sender_id = message.sender.id.clone();
        let sender_kind = message.sender.kind.clone();
        let last_message_at = message
            .committed_at
            .clone()
            .unwrap_or_else(|| message.occurred_at.clone());
        let key = scope_key(tenant_id.as_str(), conversation_id.as_str());
        let entry = TimelineViewEntry {
            tenant_id: tenant_id.clone(),
            conversation_id: conversation_id.clone(),
            message_id: message_id.clone(),
            message_seq,
            summary: summary.clone(),
        };

        let mut entries = self.entries.lock().expect("projection store should lock");
        entries.entry(key).or_default().push(entry);
        drop(entries);

        let mut summaries = self.summaries.lock().expect("summary store should lock");
        let existing_handoff = summaries
            .get(scope_key(tenant_id.as_str(), conversation_id.as_str()).as_str())
            .and_then(|view| view.agent_handoff.clone());
        summaries.insert(
            scope_key(tenant_id.as_str(), conversation_id.as_str()),
            ConversationSummaryView {
                tenant_id,
                conversation_id,
                message_count: message_seq,
                last_message_id: Some(message_id),
                last_message_seq: message_seq,
                last_sender_id: Some(sender_id.clone()),
                last_sender_kind: Some(sender_kind.clone()),
                last_sender: Some(SummarySenderView {
                    id: sender_id,
                    kind: sender_kind,
                }),
                last_summary: summary,
                last_message_at: Some(last_message_at),
                agent_handoff: existing_handoff,
            },
        );
        drop(summaries);

        self.fan_out_message_to_device_sync_feeds(event, &message);
        Ok(())
    }

    fn apply_message_edited(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let message: MessageEdited =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        self.update_timeline_summary(
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
            message.message_id.as_str(),
            message.body.summary.clone(),
        );
        self.update_conversation_summary_if_last(
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
            message.message_id.as_str(),
            message.body.summary.clone(),
            message.edited_at.clone(),
        );
        self.fan_out_message_mutation_to_device_sync_feeds(
            event,
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
            message.message_id.as_str(),
            message.message_seq,
            message.editor.id.as_str(),
            message.editor.device_id.clone(),
            message.body.summary,
        );
        Ok(())
    }

    fn apply_message_recalled(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let message: MessageRecalled =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let recalled_summary = Some("[recalled]".into());
        self.update_timeline_summary(
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
            message.message_id.as_str(),
            recalled_summary.clone(),
        );
        self.update_conversation_summary_if_last(
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
            message.message_id.as_str(),
            recalled_summary.clone(),
            message.recalled_at.clone(),
        );
        self.fan_out_message_mutation_to_device_sync_feeds(
            event,
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
            message.message_id.as_str(),
            message.message_seq,
            message.recalled_by.id.as_str(),
            message.recalled_by.device_id.clone(),
            recalled_summary,
        );
        Ok(())
    }

    fn apply_member_joined(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let member: ConversationMember =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let key = scope_key(member.tenant_id.as_str(), member.conversation_id.as_str());
        let mut members = self.members.lock().expect("member store should lock");
        members
            .entry(key.clone())
            .or_default()
            .insert(member.principal_id.clone(), member.clone());
        drop(members);

        let mut cursors = self.read_cursors.lock().expect("cursor store should lock");
        cursors
            .entry(key)
            .or_default()
            .entry(member.member_id.clone())
            .or_insert_with(|| ConversationReadCursor {
                tenant_id: member.tenant_id.clone(),
                conversation_id: member.conversation_id.clone(),
                member_id: member.member_id.clone(),
                principal_id: member.principal_id.clone(),
                read_seq: 0,
                last_read_message_id: None,
                updated_at: member.joined_at.clone(),
            });
        drop(cursors);

        self.fan_out_member_governance_to_device_sync_feeds(
            event,
            member.tenant_id.as_str(),
            member.conversation_id.as_str(),
            member.member_id.as_str(),
            member.principal_id.as_str(),
            false,
            member.joined_at.as_str(),
        );
        Ok(())
    }

    fn apply_member_role_changed(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let payload: ConversationMemberRoleChangedPayload =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let member = payload.updated_member;
        let key = scope_key(member.tenant_id.as_str(), member.conversation_id.as_str());
        let mut members = self.members.lock().expect("member store should lock");
        members
            .entry(key)
            .or_default()
            .insert(member.principal_id.clone(), member.clone());
        drop(members);

        self.fan_out_member_governance_to_device_sync_feeds(
            event,
            member.tenant_id.as_str(),
            member.conversation_id.as_str(),
            member.member_id.as_str(),
            member.principal_id.as_str(),
            false,
            event.committed_at.as_str(),
        );
        Ok(())
    }

    fn apply_member_removed(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let member: ConversationMember =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let key = scope_key(member.tenant_id.as_str(), member.conversation_id.as_str());
        let mut members = self.members.lock().expect("member store should lock");
        if let Some(scope_members) = members.get_mut(key.as_str()) {
            scope_members.remove(member.principal_id.as_str());
        }
        drop(members);

        self.fan_out_member_governance_to_device_sync_feeds(
            event,
            member.tenant_id.as_str(),
            member.conversation_id.as_str(),
            member.member_id.as_str(),
            member.principal_id.as_str(),
            true,
            member
                .removed_at
                .as_deref()
                .unwrap_or(event.committed_at.as_str()),
        );
        Ok(())
    }

    fn apply_member_left(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let member: ConversationMember =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let key = scope_key(member.tenant_id.as_str(), member.conversation_id.as_str());
        let mut members = self.members.lock().expect("member store should lock");
        if let Some(scope_members) = members.get_mut(key.as_str()) {
            scope_members.remove(member.principal_id.as_str());
        }
        drop(members);

        self.fan_out_member_governance_to_device_sync_feeds(
            event,
            member.tenant_id.as_str(),
            member.conversation_id.as_str(),
            member.member_id.as_str(),
            member.principal_id.as_str(),
            true,
            member
                .removed_at
                .as_deref()
                .unwrap_or(event.committed_at.as_str()),
        );
        Ok(())
    }

    fn apply_read_cursor_updated(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let cursor: ConversationReadCursor =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let key = scope_key(cursor.tenant_id.as_str(), cursor.conversation_id.as_str());
        let mut cursors = self.read_cursors.lock().expect("cursor store should lock");
        cursors
            .entry(key)
            .or_default()
            .insert(cursor.member_id.clone(), cursor.clone());
        drop(cursors);

        self.fan_out_read_cursor_to_device_sync_feeds(event, &cursor);
        Ok(())
    }

    pub fn timeline(&self, tenant_id: &str, conversation_id: &str) -> Vec<TimelineViewEntry> {
        self.entries
            .lock()
            .expect("projection store should lock")
            .get(scope_key(tenant_id, conversation_id).as_str())
            .cloned()
            .unwrap_or_default()
    }

    pub fn conversation_summary(
        &self,
        tenant_id: &str,
        conversation_id: &str,
    ) -> Option<ConversationSummaryView> {
        self.summaries
            .lock()
            .expect("summary store should lock")
            .get(scope_key(tenant_id, conversation_id).as_str())
            .cloned()
    }

    pub fn read_cursor(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        principal_id: &str,
    ) -> Option<ConversationReadCursorView> {
        let key = scope_key(tenant_id, conversation_id);
        let member_id = self
            .members
            .lock()
            .expect("member store should lock")
            .get(key.as_str())
            .and_then(|scope_members| scope_members.get(principal_id))
            .map(|member| member.member_id.clone())?;

        let cursor = self
            .read_cursors
            .lock()
            .expect("cursor store should lock")
            .get(key.as_str())
            .and_then(|scope_cursors| scope_cursors.get(member_id.as_str()))
            .cloned()?;

        let unread_count = self
            .conversation_summary(tenant_id, conversation_id)
            .map(|summary| summary.last_message_seq.saturating_sub(cursor.read_seq))
            .unwrap_or_default();

        Some(ConversationReadCursorView::from_cursor(
            &cursor,
            unread_count,
        ))
    }

    pub fn member_snapshot(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        principal_id: &str,
    ) -> Option<ConversationMember> {
        self.members
            .lock()
            .expect("member store should lock")
            .get(scope_key(tenant_id, conversation_id).as_str())
            .and_then(|scope_members| scope_members.get(principal_id))
            .cloned()
    }

    pub fn inbox(&self, tenant_id: &str, principal_id: &str) -> Vec<ConversationInboxEntry> {
        let members = self.members.lock().expect("member store should lock");
        let summaries = self.summaries.lock().expect("summary store should lock");
        let cursors = self.read_cursors.lock().expect("cursor store should lock");
        let conversations = self
            .conversations
            .lock()
            .expect("conversation store should lock");

        let mut items = Vec::new();

        for (scope, scope_members) in members.iter() {
            let Some(member) = scope_members.get(principal_id) else {
                continue;
            };
            if !member.is_active() {
                continue;
            }
            if member.tenant_id != tenant_id {
                continue;
            }

            let summary = summaries.get(scope);
            let cursor = cursors
                .get(scope)
                .and_then(|scope_cursors| scope_cursors.get(member.member_id.as_str()));
            let conversation = conversations.get(scope);
            let unread_count = summary
                .map(|view| view.last_message_seq)
                .unwrap_or_default()
                .saturating_sub(cursor.map(|view| view.read_seq).unwrap_or_default());

            items.push(ConversationInboxEntry {
                tenant_id: member.tenant_id.clone(),
                principal_id: member.principal_id.clone(),
                member_id: member.member_id.clone(),
                conversation_id: member.conversation_id.clone(),
                conversation_type: conversation
                    .map(|entry| entry.conversation_type.clone())
                    .unwrap_or_else(|| "unknown".into()),
                message_count: summary.map(|view| view.message_count).unwrap_or_default(),
                last_message_id: summary.and_then(|view| view.last_message_id.clone()),
                last_message_seq: summary
                    .map(|view| view.last_message_seq)
                    .unwrap_or_default(),
                last_sender_id: summary.and_then(|view| view.last_sender_id.clone()),
                last_sender_kind: summary.and_then(|view| view.last_sender_kind.clone()),
                last_summary: summary.and_then(|view| view.last_summary.clone()),
                unread_count,
                last_activity_at: summary
                    .and_then(latest_summary_activity_at)
                    .or_else(|| conversation.map(|entry| entry.created_at.clone()))
                    .unwrap_or_else(|| member.joined_at.clone()),
                agent_handoff: summary.and_then(|view| view.agent_handoff.clone()),
            });
        }

        items.sort_by(|left, right| right.last_activity_at.cmp(&left.last_activity_at));
        items
    }

    fn fan_out_message_to_device_sync_feeds(&self, event: &CommitEnvelope, message: &Message) {
        let scope = scope_key(message.tenant_id.as_str(), message.conversation_id.as_str());
        let mut principals = self
            .members
            .lock()
            .expect("member store should lock")
            .get(scope.as_str())
            .map(|scope_members| {
                scope_members
                    .values()
                    .filter(|member| member.is_active())
                    .map(|member| member.principal_id.clone())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if principals.is_empty() {
            principals.push(message.sender.id.clone());
        }

        for principal_id in principals {
            for device in self.registered_devices(message.tenant_id.as_str(), principal_id.as_str())
            {
                self.append_device_sync_entry(
                    message.tenant_id.as_str(),
                    principal_id.as_str(),
                    device.device_id.as_str(),
                    |sync_seq| DeviceSyncFeedEntry {
                        tenant_id: message.tenant_id.clone(),
                        principal_id: principal_id.clone(),
                        device_id: device.device_id.clone(),
                        sync_seq,
                        origin_event_id: event.event_id.clone(),
                        origin_event_type: event.event_type.clone(),
                        conversation_id: Some(message.conversation_id.clone()),
                        message_id: Some(message.message_id.clone()),
                        message_seq: Some(message.message_seq),
                        member_id: message.sender.member_id.clone(),
                        read_seq: None,
                        last_read_message_id: None,
                        actor_id: Some(message.sender.id.clone()),
                        actor_kind: Some(message.sender.kind.clone()),
                        actor_device_id: message.sender.device_id.clone(),
                        summary: message.body.summary.clone(),
                        payload_schema: None,
                        payload: None,
                        occurred_at: message
                            .committed_at
                            .clone()
                            .unwrap_or_else(|| message.occurred_at.clone()),
                    },
                );
            }
        }
    }

    fn fan_out_message_mutation_to_device_sync_feeds(
        &self,
        event: &CommitEnvelope,
        tenant_id: &str,
        conversation_id: &str,
        message_id: &str,
        message_seq: u64,
        actor_id: &str,
        actor_device_id: Option<String>,
        summary: Option<String>,
    ) {
        let scope = scope_key(tenant_id, conversation_id);
        let mut principals = self
            .members
            .lock()
            .expect("member store should lock")
            .get(scope.as_str())
            .map(|scope_members| {
                scope_members
                    .values()
                    .filter(|member| member.is_active())
                    .map(|member| member.principal_id.clone())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if principals.is_empty() {
            principals.push(actor_id.into());
        }

        for principal_id in principals {
            for device in self.registered_devices(tenant_id, principal_id.as_str()) {
                self.append_device_sync_entry(
                    tenant_id,
                    principal_id.as_str(),
                    device.device_id.as_str(),
                    |sync_seq| DeviceSyncFeedEntry {
                        tenant_id: tenant_id.into(),
                        principal_id: principal_id.clone(),
                        device_id: device.device_id.clone(),
                        sync_seq,
                        origin_event_id: event.event_id.clone(),
                        origin_event_type: event.event_type.clone(),
                        conversation_id: Some(conversation_id.into()),
                        message_id: Some(message_id.into()),
                        message_seq: Some(message_seq),
                        member_id: None,
                        read_seq: None,
                        last_read_message_id: None,
                        actor_id: Some(actor_id.into()),
                        actor_kind: Some(event.actor.actor_kind.clone()),
                        actor_device_id: actor_device_id.clone(),
                        summary: summary.clone(),
                        payload_schema: None,
                        payload: None,
                        occurred_at: event.committed_at.clone(),
                    },
                );
            }
        }
    }

    fn fan_out_read_cursor_to_device_sync_feeds(
        &self,
        event: &CommitEnvelope,
        cursor: &ConversationReadCursor,
    ) {
        for device in
            self.registered_devices(cursor.tenant_id.as_str(), cursor.principal_id.as_str())
        {
            self.append_device_sync_entry(
                cursor.tenant_id.as_str(),
                cursor.principal_id.as_str(),
                device.device_id.as_str(),
                |sync_seq| DeviceSyncFeedEntry {
                    tenant_id: cursor.tenant_id.clone(),
                    principal_id: cursor.principal_id.clone(),
                    device_id: device.device_id.clone(),
                    sync_seq,
                    origin_event_id: event.event_id.clone(),
                    origin_event_type: event.event_type.clone(),
                    conversation_id: Some(cursor.conversation_id.clone()),
                    message_id: None,
                    message_seq: None,
                    member_id: Some(cursor.member_id.clone()),
                    read_seq: Some(cursor.read_seq),
                    last_read_message_id: cursor.last_read_message_id.clone(),
                    actor_id: Some(cursor.principal_id.clone()),
                    actor_kind: Some(event.actor.actor_kind.clone()),
                    actor_device_id: None,
                    summary: None,
                    payload_schema: None,
                    payload: None,
                    occurred_at: cursor.updated_at.clone(),
                },
            );
        }
    }

    fn fan_out_agent_handoff_status_to_device_sync_feeds(
        &self,
        event: &CommitEnvelope,
        payload: &AgentHandoffStatusChangedProjectionPayload,
    ) {
        let scope = scope_key(
            event.tenant_id.as_str(),
            payload.state.conversation_id.as_str(),
        );
        let mut principals = self
            .members
            .lock()
            .expect("member store should lock")
            .get(scope.as_str())
            .map(|scope_members| {
                scope_members
                    .values()
                    .filter(|member| member.is_active())
                    .map(|member| member.principal_id.clone())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if principals.is_empty() {
            principals.push(payload.changed_by.id.clone());
        }

        for principal_id in principals {
            for device in self.registered_devices(event.tenant_id.as_str(), principal_id.as_str()) {
                self.append_device_sync_entry(
                    event.tenant_id.as_str(),
                    principal_id.as_str(),
                    device.device_id.as_str(),
                    |sync_seq| DeviceSyncFeedEntry {
                        tenant_id: event.tenant_id.clone(),
                        principal_id: principal_id.clone(),
                        device_id: device.device_id.clone(),
                        sync_seq,
                        origin_event_id: event.event_id.clone(),
                        origin_event_type: event.event_type.clone(),
                        conversation_id: Some(payload.state.conversation_id.clone()),
                        message_id: None,
                        message_seq: None,
                        member_id: None,
                        read_seq: None,
                        last_read_message_id: None,
                        actor_id: Some(payload.changed_by.id.clone()),
                        actor_kind: Some(payload.changed_by.kind.clone()),
                        actor_device_id: None,
                        summary: Some(payload.state.status.clone()),
                        payload_schema: event.payload_schema.clone(),
                        payload: Some(event.payload.clone()),
                        occurred_at: payload.changed_at.clone(),
                    },
                );
            }
        }
    }

    fn fan_out_member_governance_to_device_sync_feeds(
        &self,
        event: &CommitEnvelope,
        tenant_id: &str,
        conversation_id: &str,
        member_id: &str,
        affected_principal_id: &str,
        include_affected_principal_fallback: bool,
        occurred_at: &str,
    ) {
        let scope = scope_key(tenant_id, conversation_id);
        let mut principals = self
            .members
            .lock()
            .expect("member store should lock")
            .get(scope.as_str())
            .map(|scope_members| {
                scope_members
                    .values()
                    .filter(|member| member.is_active())
                    .map(|member| member.principal_id.clone())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        if include_affected_principal_fallback
            && !principals.iter().any(|item| item == affected_principal_id)
        {
            principals.push(affected_principal_id.into());
        }

        if principals.is_empty() {
            principals.push(affected_principal_id.into());
        }

        for principal_id in principals {
            for device in self.registered_devices(tenant_id, principal_id.as_str()) {
                self.append_device_sync_entry(
                    tenant_id,
                    principal_id.as_str(),
                    device.device_id.as_str(),
                    |sync_seq| DeviceSyncFeedEntry {
                        tenant_id: tenant_id.into(),
                        principal_id: principal_id.clone(),
                        device_id: device.device_id.clone(),
                        sync_seq,
                        origin_event_id: event.event_id.clone(),
                        origin_event_type: event.event_type.clone(),
                        conversation_id: Some(conversation_id.into()),
                        message_id: None,
                        message_seq: None,
                        member_id: Some(member_id.into()),
                        read_seq: None,
                        last_read_message_id: None,
                        actor_id: Some(event.actor.actor_id.clone()),
                        actor_kind: Some(event.actor.actor_kind.clone()),
                        actor_device_id: None,
                        summary: None,
                        payload_schema: event.payload_schema.clone(),
                        payload: Some(event.payload.clone()),
                        occurred_at: occurred_at.into(),
                    },
                );
            }
        }
    }

    fn append_device_sync_entry<F>(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        build: F,
    ) where
        F: FnOnce(u64) -> DeviceSyncFeedEntry,
    {
        let scope = device_feed_scope_key(tenant_id, principal_id, device_id);
        let sync_seq = {
            let mut sequences = self
                .device_sync_sequences
                .lock()
                .expect("device sync sequence store should lock");
            let entry = sequences.entry(scope.clone()).or_insert(0);
            *entry += 1;
            *entry
        };

        self.device_sync_feeds
            .lock()
            .expect("device sync feed store should lock")
            .entry(scope)
            .or_default()
            .push(build(sync_seq));
    }

    fn update_timeline_summary(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        message_id: &str,
        summary: Option<String>,
    ) {
        let key = scope_key(tenant_id, conversation_id);
        if let Some(entries) = self
            .entries
            .lock()
            .expect("projection store should lock")
            .get_mut(key.as_str())
        {
            if let Some(entry) = entries
                .iter_mut()
                .find(|item| item.message_id.as_str() == message_id)
            {
                entry.summary = summary;
            }
        }
    }

    fn update_conversation_summary_if_last(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        message_id: &str,
        summary: Option<String>,
        occurred_at: String,
    ) {
        if let Some(view) = self
            .summaries
            .lock()
            .expect("summary store should lock")
            .get_mut(scope_key(tenant_id, conversation_id).as_str())
        {
            if view.last_message_id.as_deref() == Some(message_id) {
                view.last_summary = summary;
                view.last_message_at = Some(occurred_at);
            }
        }
    }
}

fn handoff_view_from_created_payload(
    payload: &ConversationCreatedPayload,
) -> Result<Option<ConversationAgentHandoffView>, ProjectionError> {
    if payload.conversation_type != "agent_handoff" {
        return Ok(None);
    }

    let source = payload
        .source
        .as_ref()
        .ok_or_else(|| ProjectionError::InvalidEvent("agent_handoff source missing".into()))?;
    let target = payload
        .target
        .as_ref()
        .ok_or_else(|| ProjectionError::InvalidEvent("agent_handoff target missing".into()))?;
    let handoff = payload
        .handoff
        .as_ref()
        .ok_or_else(|| ProjectionError::InvalidEvent("agent_handoff payload missing".into()))?;

    Ok(Some(ConversationAgentHandoffView {
        status: handoff.status.clone(),
        source: projection_actor_to_view(source),
        target: projection_actor_to_view(target),
        handoff_session_id: handoff.session_id.clone(),
        handoff_reason: handoff.reason.clone(),
        accepted_at: None,
        accepted_by: None,
        resolved_at: None,
        resolved_by: None,
        closed_at: None,
        closed_by: None,
    }))
}

fn handoff_view_from_state_payload(
    state: &ProjectionAgentHandoffStatePayload,
) -> ConversationAgentHandoffView {
    ConversationAgentHandoffView {
        status: state.status.clone(),
        source: projection_actor_to_view(&state.source),
        target: projection_actor_to_view(&state.target),
        handoff_session_id: state.handoff_session_id.clone(),
        handoff_reason: state.handoff_reason.clone(),
        accepted_at: state.accepted_at.clone(),
        accepted_by: state.accepted_by.as_ref().map(projection_actor_to_view),
        resolved_at: state.resolved_at.clone(),
        resolved_by: state.resolved_by.as_ref().map(projection_actor_to_view),
        closed_at: state.closed_at.clone(),
        closed_by: state.closed_by.as_ref().map(projection_actor_to_view),
    }
}

fn projection_actor_to_view(actor: &ProjectionActorView) -> ConversationActorView {
    ConversationActorView {
        id: actor.id.clone(),
        kind: actor.kind.clone(),
    }
}

fn latest_summary_activity_at(summary: &ConversationSummaryView) -> Option<String> {
    let mut candidates = Vec::new();
    if let Some(last_message_at) = summary.last_message_at.clone() {
        candidates.push(last_message_at);
    }
    if let Some(handoff) = summary.agent_handoff.as_ref() {
        if let Some(accepted_at) = handoff.accepted_at.clone() {
            candidates.push(accepted_at);
        }
        if let Some(resolved_at) = handoff.resolved_at.clone() {
            candidates.push(resolved_at);
        }
        if let Some(closed_at) = handoff.closed_at.clone() {
            candidates.push(closed_at);
        }
    }
    candidates.into_iter().max()
}

#[derive(Debug)]
pub enum ProjectionError {
    InvalidPayload(serde_json::Error),
    InvalidEvent(String),
}

#[derive(Debug)]
pub struct ProjectionApiError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl ProjectionApiError {
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

impl From<AuthContextError> for ProjectionApiError {
    fn from(value: AuthContextError) -> Self {
        Self {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl axum::response::IntoResponse for ProjectionApiError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status,
            Json(serde_json::json!({
                "code": self.code,
                "message": self.message
            })),
        )
            .into_response()
    }
}

pub fn build_default_app() -> Router {
    build_app(Arc::new(TimelineProjectionService::default()))
}

pub fn build_public_app() -> Router {
    build_default_app().layer(middleware::from_fn(require_public_bearer_auth))
}

pub fn build_public_app_with_service(service: Arc<TimelineProjectionService>) -> Router {
    build_app(service).layer(middleware::from_fn(require_public_bearer_auth))
}

pub fn build_app(service: Arc<TimelineProjectionService>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/api/v1/devices/register", post(register_device))
        .route(
            "/api/v1/devices/{device_id}/sync-feed",
            get(get_device_sync_feed),
        )
        .route("/api/v1/inbox", get(get_inbox))
        .route(
            "/api/v1/conversations/{conversation_id}",
            get(get_conversation_summary),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/read-cursor",
            get(get_read_cursor),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/messages",
            get(get_timeline),
        )
        .with_state(service)
}

async fn require_public_bearer_auth(request: Request<axum::body::Body>, next: Next) -> Response {
    match request.uri().path() {
        "/healthz" | "/readyz" => next.run(request).await,
        _ => match resolve_public_bearer_auth_context(request.headers()) {
            Ok(_) => next.run(request).await,
            Err(error) => ProjectionApiError::from(error).into_response(),
        },
    }
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "projection-service",
    })
}

async fn readyz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "projection-service",
    })
}

async fn register_device(
    headers: HeaderMap,
    State(service): State<Arc<TimelineProjectionService>>,
    Json(request): Json<RegisterDeviceRequest>,
) -> Result<Json<RegisteredDeviceView>, ProjectionApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    Ok(Json(service.register_device(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
    )))
}

async fn get_device_sync_feed(
    Path(device_id): Path<String>,
    Query(query): Query<SyncFeedQuery>,
    headers: HeaderMap,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Result<Json<DeviceSyncFeedResponse>, ProjectionApiError> {
    let auth = resolve_auth_context(&headers)?;
    validate_device_scope(&auth, device_id.as_str())?;
    Ok(Json(DeviceSyncFeedResponse {
        items: service.device_sync_feed(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            device_id.as_str(),
            query.after_seq,
        ),
    }))
}

async fn get_timeline(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Result<Json<TimelineResponse>, ProjectionApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_conversation_member_access(
        service.as_ref(),
        auth.tenant_id.as_str(),
        conversation_id.as_str(),
        auth.actor_id.as_str(),
    )?;
    Ok(Json(TimelineResponse {
        items: service.timeline(auth.tenant_id.as_str(), conversation_id.as_str()),
    }))
}

async fn get_inbox(
    headers: HeaderMap,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Result<Json<InboxResponse>, ProjectionApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(InboxResponse {
        items: service.inbox(auth.tenant_id.as_str(), auth.actor_id.as_str()),
    }))
}

async fn get_conversation_summary(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Result<Json<ConversationSummaryView>, ProjectionApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_conversation_member_access(
        service.as_ref(),
        auth.tenant_id.as_str(),
        conversation_id.as_str(),
        auth.actor_id.as_str(),
    )?;
    let summary = service
        .conversation_summary(auth.tenant_id.as_str(), conversation_id.as_str())
        .ok_or_else(|| ProjectionApiError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "conversation_summary_not_found",
            message: format!("conversation summary not found: {conversation_id}"),
        })?;
    Ok(Json(summary))
}

async fn get_read_cursor(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Result<Json<ConversationReadCursorView>, ProjectionApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_conversation_member_access(
        service.as_ref(),
        auth.tenant_id.as_str(),
        conversation_id.as_str(),
        auth.actor_id.as_str(),
    )?;
    let cursor = service
        .read_cursor(
            auth.tenant_id.as_str(),
            conversation_id.as_str(),
            auth.actor_id.as_str(),
        )
        .ok_or_else(|| ProjectionApiError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "conversation_read_cursor_not_found",
            message: format!("conversation read cursor not found: {conversation_id}"),
        })?;
    Ok(Json(cursor))
}

fn resolve_requested_device_id(
    auth: &AuthContext,
    requested_device_id: Option<String>,
) -> Result<String, ProjectionApiError> {
    match (requested_device_id, auth.device_id.clone()) {
        (Some(requested), Some(bound)) => {
            if requested != bound {
                return Err(ProjectionApiError::bad_request(
                    "device_id_mismatch",
                    format!("device id does not match auth context: {requested}"),
                ));
            }
            Ok(requested)
        }
        (Some(requested), None) => Ok(requested),
        (None, Some(bound)) => Ok(bound),
        (None, None) => Err(ProjectionApiError::bad_request(
            "device_id_missing",
            "device id must be provided by auth context or request body",
        )),
    }
}

fn validate_device_scope(auth: &AuthContext, device_id: &str) -> Result<(), ProjectionApiError> {
    if let Some(bound_device_id) = auth.device_id.as_deref() {
        if bound_device_id != device_id {
            return Err(ProjectionApiError::forbidden(
                "device_scope_forbidden",
                format!("device scope forbidden: {device_id}"),
            ));
        }
    }
    Ok(())
}

fn ensure_conversation_member_access(
    service: &TimelineProjectionService,
    tenant_id: &str,
    conversation_id: &str,
    principal_id: &str,
) -> Result<(), ProjectionApiError> {
    if service.is_active_member(tenant_id, conversation_id, principal_id) {
        return Ok(());
    }

    Err(ProjectionApiError::forbidden(
        "conversation_permission_denied",
        format!("principal is not active conversation member: {principal_id}"),
    ))
}

fn scope_key(tenant_id: &str, conversation_id: &str) -> String {
    format!("{tenant_id}:{conversation_id}")
}

fn principal_scope_key(tenant_id: &str, principal_id: &str) -> String {
    format!("{tenant_id}:{principal_id}")
}

fn device_feed_scope_key(tenant_id: &str, principal_id: &str, device_id: &str) -> String {
    format!("{tenant_id}:{principal_id}:{device_id}")
}

fn registered_device_at() -> String {
    utc_now_rfc3339_millis()
}
