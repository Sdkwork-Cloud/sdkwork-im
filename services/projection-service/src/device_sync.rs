use im_domain_core::conversation::DeviceSyncFeedEntry;

use crate::model::RealtimeFanoutTarget;

#[derive(Clone, Debug)]
pub(crate) struct DeviceSyncEntryDraft {
    pub(crate) tenant_id: String,
    pub(crate) origin_event_id: String,
    pub(crate) origin_event_type: String,
    pub(crate) conversation_id: Option<String>,
    pub(crate) message_id: Option<String>,
    pub(crate) message_seq: Option<u64>,
    pub(crate) member_id: Option<String>,
    pub(crate) read_seq: Option<u64>,
    pub(crate) last_read_message_id: Option<String>,
    pub(crate) actor_id: Option<String>,
    pub(crate) actor_kind: Option<String>,
    pub(crate) actor_device_id: Option<String>,
    pub(crate) summary: Option<String>,
    pub(crate) payload_schema: Option<String>,
    pub(crate) payload: Option<String>,
    pub(crate) occurred_at: String,
}

impl DeviceSyncEntryDraft {
    pub(crate) fn build_for_target(
        &self,
        target: &RealtimeFanoutTarget,
        sync_seq: u64,
    ) -> DeviceSyncFeedEntry {
        DeviceSyncFeedEntry {
            tenant_id: self.tenant_id.clone(),
            principal_id: target.principal_id.clone(),
            device_id: target.device_id.clone(),
            sync_seq,
            origin_event_id: self.origin_event_id.clone(),
            origin_event_type: self.origin_event_type.clone(),
            conversation_id: self.conversation_id.clone(),
            message_id: self.message_id.clone(),
            message_seq: self.message_seq,
            member_id: self.member_id.clone(),
            read_seq: self.read_seq,
            last_read_message_id: self.last_read_message_id.clone(),
            actor_id: self.actor_id.clone(),
            actor_kind: self.actor_kind.clone(),
            actor_device_id: self.actor_device_id.clone(),
            summary: self.summary.clone(),
            payload_schema: self.payload_schema.clone(),
            payload: self.payload.clone(),
            occurred_at: self.occurred_at.clone(),
        }
    }
}
