use im_domain_core::conversation::ConversationInboxEntry;

use crate::projection::latest_summary_activity_at;
use crate::{TimelineProjectionService, lock_projection_mutex};

impl TimelineProjectionService {
    pub fn inbox_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Vec<ConversationInboxEntry> {
        let members = lock_projection_mutex(&self.members, "member store");
        let summaries = lock_projection_mutex(&self.summaries, "summary store");
        let cursors = lock_projection_mutex(&self.read_cursors, "cursor store");
        let conversations = lock_projection_mutex(&self.conversations, "conversation store");
        let mut items = Vec::new();

        for scope in
            members.active_member_scopes_for_principal_kind(tenant_id, principal_kind, principal_id)
        {
            let Some(scope_members) = members.get(scope.as_str()) else {
                continue;
            };
            for member in scope_members.values().filter(|member| {
                member.principal_id == principal_id
                    && member.principal_kind == principal_kind
                    && member.is_active()
                    && member.tenant_id == tenant_id
            }) {
                let summary = summaries.get(scope.as_str());
                let cursor = cursors
                    .get(scope.as_str())
                    .and_then(|scope_cursors| scope_cursors.get(member.member_id.as_str()));
                let conversation = conversations.get(scope.as_str());
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
        }

        items.sort_by(|left, right| right.last_activity_at.cmp(&left.last_activity_at));
        items
    }
}
