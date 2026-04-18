use std::collections::BTreeMap;

use im_domain_core::conversation::DeviceSyncFeedEntry;

use crate::model::{NotificationRecipientView, RealtimeFanoutTarget, RegisteredDeviceView};
use crate::scope::{
    DeviceFeedScopeKey, DevicePrincipalScopeKey, device_feed_scope_key, device_principal_scope_key,
    registered_device_at, scope_key,
};
use crate::{TimelineProjectionService, lock_projection_mutex};

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

pub(crate) fn register_device_for_principal_kind(
    service: &TimelineProjectionService,
    tenant_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
) -> RegisteredDeviceView {
    register_device_with_optional_principal_kind(
        service,
        tenant_id,
        principal_id,
        Some(principal_kind),
        device_id,
    )
}

pub(crate) fn register_device_legacy(
    service: &TimelineProjectionService,
    tenant_id: &str,
    principal_id: &str,
    device_id: &str,
) -> RegisteredDeviceView {
    register_device_with_optional_principal_kind(service, tenant_id, principal_id, None, device_id)
}

fn register_device_with_optional_principal_kind(
    service: &TimelineProjectionService,
    tenant_id: &str,
    principal_id: &str,
    principal_kind: Option<&str>,
    device_id: &str,
) -> RegisteredDeviceView {
    let device = RegisteredDeviceView {
        tenant_id: tenant_id.into(),
        principal_id: principal_id.into(),
        principal_kind: principal_kind.map(str::to_owned),
        device_id: device_id.into(),
        registered_at: registered_device_at(),
    };
    let scope = device_principal_scope_key(tenant_id, principal_id, principal_kind);
    lock_projection_mutex(&service.registered_devices, "registered device store")
        .entry(scope)
        .or_default()
        .insert(device_id.into(), device.clone());
    lock_projection_mutex(&service.device_sync_feeds, "device sync feed store")
        .entry(device_feed_scope_key(
            tenant_id,
            principal_id,
            principal_kind,
            device_id,
        ))
        .or_default();
    lock_projection_mutex(&service.device_sync_sequences, "device sync sequence store")
        .entry(device_feed_scope_key(
            tenant_id,
            principal_id,
            principal_kind,
            device_id,
        ))
        .or_insert(0);
    device
}

pub(crate) fn registered_devices_for_principal_kind(
    service: &TimelineProjectionService,
    tenant_id: &str,
    principal_id: &str,
    principal_kind: &str,
) -> Vec<RegisteredDeviceView> {
    let registered_devices =
        lock_projection_mutex(&service.registered_devices, "registered device store");
    let mut devices_by_id = BTreeMap::new();

    if let Some(legacy_devices) =
        registered_devices.get(&device_principal_scope_key(tenant_id, principal_id, None))
    {
        for device in legacy_devices.values() {
            devices_by_id.insert(device.device_id.clone(), device.clone());
        }
    }

    if let Some(typed_devices) = registered_devices.get(&device_principal_scope_key(
        tenant_id,
        principal_id,
        Some(principal_kind),
    )) {
        for device in typed_devices.values() {
            devices_by_id.insert(device.device_id.clone(), device.clone());
        }
    }

    let mut devices = devices_by_id.into_values().collect::<Vec<_>>();
    devices.sort_by(|left, right| left.device_id.cmp(&right.device_id));
    devices
}

pub(crate) fn device_sync_feed_for_principal_kind(
    service: &TimelineProjectionService,
    tenant_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
    after_seq: Option<u64>,
) -> Vec<DeviceSyncFeedEntry> {
    let min_seq = after_seq.unwrap_or_default();
    let scope = resolved_device_feed_scope_key(
        service,
        tenant_id,
        principal_id,
        Some(principal_kind),
        device_id,
    );
    lock_projection_mutex(&service.device_sync_feeds, "device sync feed store")
        .get(&scope)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter(|entry| entry.sync_seq > min_seq)
        .filter(|entry| {
            entry
                .conversation_id
                .as_deref()
                .is_none_or(|conversation_id| {
                    !service.is_archived_direct_chat_conversation(tenant_id, conversation_id)
                })
        })
        .collect()
}

pub(crate) fn latest_device_sync_seq_for_principal_kind(
    service: &TimelineProjectionService,
    tenant_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
) -> u64 {
    let scope = resolved_device_feed_scope_key(
        service,
        tenant_id,
        principal_id,
        Some(principal_kind),
        device_id,
    );
    lock_projection_mutex(&service.device_sync_sequences, "device sync sequence store")
        .get(&scope)
        .copied()
        .unwrap_or_default()
}

pub(crate) fn active_conversation_principal_recipients(
    service: &TimelineProjectionService,
    tenant_id: &str,
    conversation_id: &str,
) -> Vec<NotificationRecipientView> {
    let scope = scope_key(tenant_id, conversation_id);
    let mut recipients = lock_projection_mutex(&service.members, "member store")
        .get(scope.as_str())
        .map(|scope_members| {
            scope_members
                .values()
                .filter(|member| member.is_active())
                .map(|member| NotificationRecipientView {
                    principal_id: member.principal_id.clone(),
                    principal_kind: member.principal_kind.clone(),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    recipients.sort();
    recipients.dedup();
    recipients
}

pub(crate) fn realtime_fanout_targets_for_recipients(
    service: &TimelineProjectionService,
    tenant_id: &str,
    recipients: impl IntoIterator<Item = NotificationRecipientView>,
) -> Vec<RealtimeFanoutTarget> {
    let mut targets = recipients
        .into_iter()
        .flat_map(|recipient| {
            registered_devices_for_principal_kind(
                service,
                tenant_id,
                recipient.principal_id.as_str(),
                recipient.principal_kind.as_str(),
            )
            .into_iter()
            .map(move |device| RealtimeFanoutTarget {
                principal_id: recipient.principal_id.clone(),
                principal_kind: Some(recipient.principal_kind.clone()),
                device_id: device.device_id,
            })
        })
        .collect::<Vec<_>>();
    targets.sort_by(|left, right| {
        left.principal_id
            .cmp(&right.principal_id)
            .then_with(|| left.principal_kind.cmp(&right.principal_kind))
            .then_with(|| left.device_id.cmp(&right.device_id))
    });
    targets
}

pub(crate) fn device_sync_fanout_targets_for_conversation(
    service: &TimelineProjectionService,
    tenant_id: &str,
    conversation_id: &str,
    fallback_recipients: Vec<NotificationRecipientView>,
) -> Vec<RealtimeFanoutTarget> {
    let mut recipients =
        active_conversation_principal_recipients(service, tenant_id, conversation_id);
    for fallback in fallback_recipients {
        if !recipients.iter().any(|item| item == &fallback) {
            recipients.push(fallback);
        }
    }
    realtime_fanout_targets_for_recipients(service, tenant_id, recipients)
}

impl TimelineProjectionService {
    pub fn register_device(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> RegisteredDeviceView {
        register_device_legacy(self, tenant_id, principal_id, device_id)
    }

    pub fn register_device_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> RegisteredDeviceView {
        self::register_device_for_principal_kind(
            self,
            tenant_id,
            principal_id,
            principal_kind,
            device_id,
        )
    }

    pub fn device_sync_feed(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        after_seq: Option<u64>,
    ) -> Vec<DeviceSyncFeedEntry> {
        let min_seq = after_seq.unwrap_or_default();
        lock_projection_mutex(&self.device_sync_feeds, "device sync feed store")
            .get(&device_feed_scope_key(
                tenant_id,
                principal_id,
                None,
                device_id,
            ))
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter(|entry| entry.sync_seq > min_seq)
            .filter(|entry| {
                entry
                    .conversation_id
                    .as_deref()
                    .is_none_or(|conversation_id| {
                        !self.is_archived_direct_chat_conversation(tenant_id, conversation_id)
                    })
            })
            .collect()
    }

    pub fn device_sync_feed_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        after_seq: Option<u64>,
    ) -> Vec<DeviceSyncFeedEntry> {
        self::device_sync_feed_for_principal_kind(
            self,
            tenant_id,
            principal_id,
            principal_kind,
            device_id,
            after_seq,
        )
    }

    pub fn registered_devices(
        &self,
        tenant_id: &str,
        principal_id: &str,
    ) -> Vec<RegisteredDeviceView> {
        let mut devices =
            lock_projection_mutex(&self.registered_devices, "registered device store")
                .iter()
                .filter(|(scope, _)| {
                    scope.tenant_id == tenant_id && scope.principal_id == principal_id
                })
                .flat_map(|(_, items)| items.values().cloned())
                .collect::<Vec<_>>();
        devices.sort_by(|left, right| {
            left.device_id
                .cmp(&right.device_id)
                .then_with(|| left.principal_kind.cmp(&right.principal_kind))
        });
        devices
    }

    pub fn registered_devices_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Vec<RegisteredDeviceView> {
        self::registered_devices_for_principal_kind(self, tenant_id, principal_id, principal_kind)
    }

    pub fn latest_device_sync_seq(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> u64 {
        lock_projection_mutex(&self.device_sync_sequences, "device sync sequence store")
            .get(&device_feed_scope_key(
                tenant_id,
                principal_id,
                None,
                device_id,
            ))
            .copied()
            .unwrap_or_default()
    }

    pub fn latest_device_sync_seq_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> u64 {
        self::latest_device_sync_seq_for_principal_kind(
            self,
            tenant_id,
            principal_id,
            principal_kind,
            device_id,
        )
    }

    pub(crate) fn append_device_sync_entry<F>(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: Option<&str>,
        device_id: &str,
        build: F,
    ) where
        F: FnOnce(u64) -> DeviceSyncFeedEntry,
    {
        let scope = device_feed_scope_key(tenant_id, principal_id, principal_kind, device_id);
        let sync_seq = {
            let mut sequences =
                lock_projection_mutex(&self.device_sync_sequences, "device sync sequence store");
            let entry = sequences.entry(scope.clone()).or_insert(0);
            *entry += 1;
            *entry
        };

        lock_projection_mutex(&self.device_sync_feeds, "device sync feed store")
            .entry(scope)
            .or_default()
            .push(build(sync_seq));
    }

    pub(crate) fn append_device_sync_draft(
        &self,
        target: &RealtimeFanoutTarget,
        draft: &DeviceSyncEntryDraft,
    ) {
        let principal_kind = resolved_device_scope_principal_kind(
            self,
            draft.tenant_id.as_str(),
            target.principal_id.as_str(),
            target.principal_kind.as_deref(),
            target.device_id.as_str(),
        );
        self.append_device_sync_entry(
            draft.tenant_id.as_str(),
            target.principal_id.as_str(),
            principal_kind.as_deref(),
            target.device_id.as_str(),
            |sync_seq| draft.build_for_target(target, sync_seq),
        );
    }
}

fn resolved_device_feed_scope_key(
    service: &TimelineProjectionService,
    tenant_id: &str,
    principal_id: &str,
    principal_kind: Option<&str>,
    device_id: &str,
) -> DeviceFeedScopeKey {
    let resolved_principal_kind = resolved_device_scope_principal_kind(
        service,
        tenant_id,
        principal_id,
        principal_kind,
        device_id,
    );
    device_feed_scope_key(
        tenant_id,
        principal_id,
        resolved_principal_kind.as_deref(),
        device_id,
    )
}

fn resolved_device_scope_principal_kind(
    service: &TimelineProjectionService,
    tenant_id: &str,
    principal_id: &str,
    principal_kind: Option<&str>,
    device_id: &str,
) -> Option<String> {
    let registered_devices =
        lock_projection_mutex(&service.registered_devices, "registered device store");
    if let Some(principal_kind) = principal_kind
        && scope_contains_device(
            &registered_devices,
            &device_principal_scope_key(tenant_id, principal_id, Some(principal_kind)),
            device_id,
        )
    {
        return Some(principal_kind.to_owned());
    }

    if scope_contains_device(
        &registered_devices,
        &device_principal_scope_key(tenant_id, principal_id, None),
        device_id,
    ) {
        return None;
    }

    principal_kind.map(str::to_owned)
}

fn scope_contains_device(
    registered_devices: &std::collections::HashMap<
        DevicePrincipalScopeKey,
        std::collections::HashMap<String, RegisteredDeviceView>,
    >,
    scope: &DevicePrincipalScopeKey,
    device_id: &str,
) -> bool {
    registered_devices
        .get(scope)
        .is_some_and(|devices| devices.contains_key(device_id))
}
