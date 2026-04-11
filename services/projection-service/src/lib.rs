use std::collections::HashMap;
use std::sync::Mutex;

use im_domain_core::conversation::{
    ConversationInboxEntry, ConversationMember, ConversationReadCursor, ConversationReadCursorView,
    DeviceSyncFeedEntry,
};
use im_domain_core::message::{Message, MessageEdited, MessageRecalled};
use im_domain_events::CommitEnvelope;

mod access;
mod contacts;
mod device_sync;
mod http;
mod interactions;
mod member_directory;
mod model;
mod observability;
mod projection;
mod scope;
mod snapshot;
mod update_delay;

use device_sync::DeviceSyncEntryDraft;
use model::ConversationCatalogEntry;
use observability::ProjectionObservabilityState;
use projection::{
    AgentHandoffStatusChangedProjectionPayload, ConversationCreatedPayload,
    ConversationMemberRoleChangedPayload, handoff_view_from_created_payload,
    handoff_view_from_state_payload, latest_summary_activity_at,
};
use scope::{
    device_feed_scope_key, principal_scope_key, registered_device_at, scope_key,
    tracked_live_projection_lag_scope_id,
};

pub use access::{DeviceSyncSessionState, ProjectionAccessError};
pub use http::{build_app, build_default_app, build_public_app, build_public_app_with_service};
pub use model::{
    ContactView, ConversationMemberDirectoryEntry, ConversationSummaryView, InteractionActorView,
    MessageInteractionSummaryView, MessagePinView, MessageReactionCountView, RealtimeFanoutTarget,
    RegisteredDeviceView, SummarySenderView, TimelineViewEntry,
};
pub use observability::{
    ProjectionLagItemView, ProjectionLogView, ProjectionOperationMetricView,
    ProjectionPlaneMetricsView, ProjectionPlaneObservabilityView, ProjectionReplayMetricsView,
    ProjectionTraceView, ProjectionUpdateDelayView,
};
pub use projection::ProjectionError;

#[derive(Default)]
pub struct TimelineProjectionService {
    entries: Mutex<HashMap<String, Vec<TimelineViewEntry>>>,
    summaries: Mutex<HashMap<String, ConversationSummaryView>>,
    members: Mutex<HashMap<String, HashMap<String, ConversationMember>>>,
    read_cursors: Mutex<HashMap<String, HashMap<String, ConversationReadCursor>>>,
    conversations: Mutex<HashMap<String, ConversationCatalogEntry>>,
    contacts: Mutex<HashMap<String, HashMap<String, ContactView>>>,
    direct_chat_bindings: Mutex<HashMap<String, model::ContactDirectChatBindingView>>,
    message_interactions:
        Mutex<HashMap<String, HashMap<String, interactions::StoredMessageInteractionSummary>>>,
    registered_devices: Mutex<HashMap<String, HashMap<String, RegisteredDeviceView>>>,
    device_sync_feeds: Mutex<HashMap<String, Vec<DeviceSyncFeedEntry>>>,
    device_sync_sequences: Mutex<HashMap<String, u64>>,
    observability: Mutex<ProjectionObservabilityState>,
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
        let live_lag_scope_id = tracked_live_projection_lag_scope_id(event);
        if let Some(scope_id) = live_lag_scope_id.as_deref() {
            self.record_projection_live_lag_observed(scope_id, event.ordering_seq);
        }

        let result = match event.event_type.as_str() {
            "conversation.created" => self.apply_conversation_created(event),
            "conversation.agent_handoff_status_changed" => {
                self.apply_agent_handoff_status_changed(event)
            }
            "message.posted" => self.apply_message_posted(event),
            "message.edited" => self.apply_message_edited(event),
            "message.recalled" => self.apply_message_recalled(event),
            "message.reaction_added" => self.apply_message_reaction_added(event),
            "message.reaction_removed" => self.apply_message_reaction_removed(event),
            "message.pin_added" => self.apply_message_pinned(event),
            "message.pin_removed" => self.apply_message_unpinned(event),
            "conversation.member_joined" => self.apply_member_joined(event),
            "conversation.member_role_changed" => self.apply_member_role_changed(event),
            "conversation.member_removed" => self.apply_member_removed(event),
            "conversation.member_left" => self.apply_member_left(event),
            "conversation.read_cursor_updated" => self.apply_read_cursor_updated(event),
            "friendship.activated" => self.apply_friendship_activated(event),
            "direct_chat.bound" => self.apply_direct_chat_bound(event),
            _ => Ok(()),
        };

        if result.is_ok()
            && let Some(scope_id) = live_lag_scope_id.as_deref()
        {
            self.record_projection_live_lag_committed(scope_id, event.ordering_seq);
        }

        result
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

    pub fn realtime_fanout_targets_for_principals(
        &self,
        tenant_id: &str,
        principal_ids: impl IntoIterator<Item = String>,
    ) -> Vec<RealtimeFanoutTarget> {
        let mut targets = principal_ids
            .into_iter()
            .flat_map(|principal_id| {
                self.registered_devices(tenant_id, principal_id.as_str())
                    .into_iter()
                    .map(move |device| RealtimeFanoutTarget {
                        principal_id: principal_id.clone(),
                        device_id: device.device_id,
                    })
            })
            .collect::<Vec<_>>();
        targets.sort_by(|left, right| {
            left.principal_id
                .cmp(&right.principal_id)
                .then_with(|| left.device_id.cmp(&right.device_id))
        });
        targets
    }

    pub fn device_sync_fanout_targets_for_conversation(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        fallback_principal_ids: Vec<String>,
    ) -> Vec<RealtimeFanoutTarget> {
        let mut principal_ids = self.active_conversation_principal_ids(tenant_id, conversation_id);
        for fallback_principal_id in fallback_principal_ids {
            if !principal_ids
                .iter()
                .any(|item| item == &fallback_principal_id)
            {
                principal_ids.push(fallback_principal_id);
            }
        }
        self.realtime_fanout_targets_for_principals(tenant_id, principal_ids)
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
        self.record_projection_update_delay_for_scope(
            "message.posted",
            scope_key(message.tenant_id.as_str(), message.conversation_id.as_str()).as_str(),
            message
                .committed_at
                .as_deref()
                .unwrap_or(message.occurred_at.as_str()),
        );
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
        self.record_projection_update_delay_for_scope(
            "message.edited",
            scope_key(message.tenant_id.as_str(), message.conversation_id.as_str()).as_str(),
            message.edited_at.as_str(),
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
        self.record_projection_update_delay_for_scope(
            "message.recalled",
            scope_key(message.tenant_id.as_str(), message.conversation_id.as_str()).as_str(),
            message.recalled_at.as_str(),
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
        let draft = DeviceSyncEntryDraft {
            tenant_id: message.tenant_id.clone(),
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
        };

        for target in self.device_sync_fanout_targets_for_conversation(
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
            vec![message.sender.id.clone()],
        ) {
            self.append_device_sync_draft(&target, &draft);
        }
    }

    // These fanout helpers keep event and conversation identity fields explicit
    // because they bridge journal payloads into device-sync artifacts.
    #[allow(clippy::too_many_arguments)]
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
        let draft = DeviceSyncEntryDraft {
            tenant_id: tenant_id.into(),
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
            actor_device_id,
            summary,
            payload_schema: None,
            payload: None,
            occurred_at: event.committed_at.clone(),
        };

        for target in self.device_sync_fanout_targets_for_conversation(
            tenant_id,
            conversation_id,
            vec![actor_id.into()],
        ) {
            self.append_device_sync_draft(&target, &draft);
        }
    }

    fn fan_out_read_cursor_to_device_sync_feeds(
        &self,
        event: &CommitEnvelope,
        cursor: &ConversationReadCursor,
    ) {
        let draft = DeviceSyncEntryDraft {
            tenant_id: cursor.tenant_id.clone(),
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
        };

        for target in self.realtime_fanout_targets_for_principals(
            cursor.tenant_id.as_str(),
            vec![cursor.principal_id.clone()],
        ) {
            self.append_device_sync_draft(&target, &draft);
        }
    }

    fn fan_out_agent_handoff_status_to_device_sync_feeds(
        &self,
        event: &CommitEnvelope,
        payload: &AgentHandoffStatusChangedProjectionPayload,
    ) {
        let draft = DeviceSyncEntryDraft {
            tenant_id: event.tenant_id.clone(),
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
        };

        for target in self.device_sync_fanout_targets_for_conversation(
            event.tenant_id.as_str(),
            payload.state.conversation_id.as_str(),
            vec![payload.changed_by.id.clone()],
        ) {
            self.append_device_sync_draft(&target, &draft);
        }
    }

    #[allow(clippy::too_many_arguments)]
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
        let include_fallback = include_affected_principal_fallback
            || self
                .active_conversation_principal_ids(tenant_id, conversation_id)
                .is_empty();
        let fallback_principal_ids = if include_fallback {
            vec![affected_principal_id.into()]
        } else {
            Vec::new()
        };
        let draft = DeviceSyncEntryDraft {
            tenant_id: tenant_id.into(),
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
        };

        for target in self.device_sync_fanout_targets_for_conversation(
            tenant_id,
            conversation_id,
            fallback_principal_ids,
        ) {
            self.append_device_sync_draft(&target, &draft);
        }
    }

    pub(crate) fn active_conversation_principal_ids(
        &self,
        tenant_id: &str,
        conversation_id: &str,
    ) -> Vec<String> {
        let scope = scope_key(tenant_id, conversation_id);
        let mut principal_ids = self
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
        principal_ids.sort();
        principal_ids.dedup();
        principal_ids
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

    fn append_device_sync_draft(
        &self,
        target: &RealtimeFanoutTarget,
        draft: &DeviceSyncEntryDraft,
    ) {
        self.append_device_sync_entry(
            draft.tenant_id.as_str(),
            target.principal_id.as_str(),
            target.device_id.as_str(),
            |sync_seq| draft.build_for_target(target, sync_seq),
        );
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
            && let Some(entry) = entries
                .iter_mut()
                .find(|item| item.message_id.as_str() == message_id)
        {
            entry.summary = summary;
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
            && view.last_message_id.as_deref() == Some(message_id)
        {
            view.last_summary = summary;
            view.last_message_at = Some(occurred_at);
        }
    }
}
