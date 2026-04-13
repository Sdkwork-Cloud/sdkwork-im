use std::collections::BTreeSet;
use std::sync::{Mutex, MutexGuard};

use im_domain_core::conversation::DeviceSyncFeedEntry;
use im_domain_core::conversation::{
    ConversationMember, ConversationReadCursor, principal_member_key,
};
use im_platform_contracts::{MetadataStore, TimelineProjectionStore};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::contacts::{
    contact_map_from_items, contact_runtime_scope, contact_snapshot_items, parse_contact_scope,
};
use crate::interactions::{
    StoredMessageInteractionSummary, interaction_map_from_items, interaction_snapshot_items,
};
use crate::observability::ProjectionSnapshotOperation;
use crate::projection::ProjectionError;
use crate::scope::{
    DeviceFeedScopeKey, DevicePrincipalScopeKey, device_feed_scope_key, device_principal_scope_key,
};
use crate::{
    ContactView, ConversationSummaryView, TimelineProjectionService, TimelineViewEntry,
    model::ConversationCatalogEntry,
};

const CONVERSATION_SUMMARY_KEY: &str = "conversation-summary";
const CONVERSATION_CATALOG_KEY: &str = "conversation-catalog";
const CONVERSATION_MEMBERS_KEY: &str = "conversation-members";
const CONVERSATION_READ_CURSORS_KEY: &str = "conversation-read-cursors";
const MESSAGE_INTERACTIONS_KEY: &str = "message-interactions";
const CONTACTS_KEY: &str = "contacts";
const CONTACT_OWNERS_KEY: &str = "contact-owners";
const CONTACT_DIRECT_CHAT_BINDINGS_KEY: &str = "contact-direct-chat-bindings";
const REGISTERED_DEVICES_KEY: &str = "registered-devices";
const REGISTERED_DEVICE_PRINCIPALS_KEY: &str = "registered-device-principals";
const DEVICE_SYNC_SEQUENCE_KEY: &str = "device-sync-sequence";
const DEVICE_SYNC_SCOPE_CATALOG_KEY: &str = "device-sync-scopes";
const CONTACT_CATALOG_SCOPE: &str = "projection-contacts";
const PRINCIPAL_SNAPSHOT_SCOPE_PREFIX: &str = "principal";
const DEVICE_SYNC_SNAPSHOT_SCOPE_PREFIX: &str = "device-sync";
const DEVICE_SYNC_CATALOG_SCOPE: &str = "projection-device-sync";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
struct PrincipalSnapshotCatalogEntry {
    tenant_id: String,
    principal_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    principal_kind: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
struct DeviceSyncScopeCatalogEntry {
    tenant_id: String,
    principal_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    principal_kind: Option<String>,
    device_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
struct ContactOwnerCatalogEntry {
    tenant_id: String,
    owner_user_id: String,
}

trait ProjectionMutexExt<T> {
    fn lock_projection(&self, lock_name: &'static str) -> MutexGuard<'_, T>;
}

impl<T> ProjectionMutexExt<T> for Mutex<T> {
    fn lock_projection(&self, lock_name: &'static str) -> MutexGuard<'_, T> {
        super::lock_projection_mutex(self, lock_name)
    }
}

impl TimelineProjectionService {
    pub fn persist_conversation_snapshot(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        metadata_store: &dyn MetadataStore,
        timeline_store: &dyn TimelineProjectionStore,
    ) -> Result<bool, ProjectionError> {
        let scope = conversation_snapshot_scope(tenant_id, conversation_id);
        let result = (|| {
            let Some(summary) = self.conversation_summary(tenant_id, conversation_id) else {
                return Ok(false);
            };
            let conversation = self
                .conversations
                .lock_projection("conversation store")
                .get(scope.as_str())
                .cloned();
            let mut members = self
                .members
                .lock_projection("member store")
                .get(scope.as_str())
                .map(|scope_members| scope_members.values().cloned().collect::<Vec<_>>())
                .unwrap_or_default();
            members.sort_by(|left, right| {
                left.principal_id
                    .cmp(&right.principal_id)
                    .then_with(|| left.principal_kind.cmp(&right.principal_kind))
            });
            let mut read_cursors = self
                .read_cursors
                .lock_projection("cursor store")
                .get(scope.as_str())
                .map(|scope_cursors| scope_cursors.values().cloned().collect::<Vec<_>>())
                .unwrap_or_default();
            read_cursors.sort_by(|left, right| left.member_id.cmp(&right.member_id));
            let interaction_items = self
                .message_interactions
                .lock_projection("message interaction store")
                .get(scope.as_str())
                .map(interaction_snapshot_items)
                .unwrap_or_default();

            persist_metadata_snapshot(
                metadata_store,
                scope.as_str(),
                CONVERSATION_SUMMARY_KEY,
                &summary,
            )?;
            if let Some(conversation) = conversation.as_ref() {
                persist_metadata_snapshot(
                    metadata_store,
                    scope.as_str(),
                    CONVERSATION_CATALOG_KEY,
                    conversation,
                )?;
            }
            persist_metadata_snapshot(
                metadata_store,
                scope.as_str(),
                CONVERSATION_MEMBERS_KEY,
                &members,
            )?;
            persist_metadata_snapshot(
                metadata_store,
                scope.as_str(),
                CONVERSATION_READ_CURSORS_KEY,
                &read_cursors,
            )?;
            persist_metadata_snapshot(
                metadata_store,
                scope.as_str(),
                MESSAGE_INTERACTIONS_KEY,
                &interaction_items,
            )?;

            for entry in self.timeline(tenant_id, conversation_id) {
                let entry_payload =
                    serde_json::to_string(&entry).map_err(ProjectionError::InvalidSnapshot)?;
                timeline_store
                    .upsert_timeline_entry(
                        scope.as_str(),
                        entry.message_seq,
                        entry_payload.as_str(),
                    )
                    .map_err(ProjectionError::StoreFailure)?;
            }
            self.persist_contact_snapshot(metadata_store)?;
            self.persist_device_sync_snapshot(metadata_store, timeline_store)?;

            Ok(true)
        })();

        match &result {
            Ok(true) => self.record_projection_snapshot_success(
                ProjectionSnapshotOperation::ConversationSnapshotPersist,
                "conversation",
                scope.as_str(),
                format!("persisted conversation projection snapshot for {scope}"),
            ),
            Ok(false) => {}
            Err(error) => self.record_projection_snapshot_failure(
                ProjectionSnapshotOperation::ConversationSnapshotPersist,
                "conversation",
                scope.as_str(),
                error,
            ),
        }

        result
    }

    pub fn restore_conversation_snapshot(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        metadata_store: &dyn MetadataStore,
        timeline_store: &dyn TimelineProjectionStore,
    ) -> Result<bool, ProjectionError> {
        let scope = conversation_snapshot_scope(tenant_id, conversation_id);
        let result = (|| {
            let Some(summary) = load_metadata_snapshot::<ConversationSummaryView>(
                metadata_store,
                scope.as_str(),
                CONVERSATION_SUMMARY_KEY,
            )?
            else {
                return Ok(false);
            };
            let mut timeline = timeline_store
                .load_timeline(scope.as_str())
                .map_err(ProjectionError::StoreFailure)?
                .into_iter()
                .map(|(_, payload)| {
                    serde_json::from_str::<TimelineViewEntry>(&payload)
                        .map_err(ProjectionError::InvalidSnapshot)
                })
                .collect::<Result<Vec<_>, _>>()?;
            timeline.sort_by_key(|entry| entry.message_seq);
            let conversation = load_metadata_snapshot::<ConversationCatalogEntry>(
                metadata_store,
                scope.as_str(),
                CONVERSATION_CATALOG_KEY,
            )?;
            let members = load_metadata_snapshot::<Vec<ConversationMember>>(
                metadata_store,
                scope.as_str(),
                CONVERSATION_MEMBERS_KEY,
            )?
            .unwrap_or_default()
            .into_iter()
            .map(|member| {
                (
                    principal_member_key(
                        member.principal_id.as_str(),
                        member.principal_kind.as_str(),
                    ),
                    member,
                )
            })
            .collect();
            let read_cursors = load_metadata_snapshot::<Vec<ConversationReadCursor>>(
                metadata_store,
                scope.as_str(),
                CONVERSATION_READ_CURSORS_KEY,
            )?
            .unwrap_or_default()
            .into_iter()
            .map(|cursor| (cursor.member_id.clone(), cursor))
            .collect();
            let interactions = load_metadata_snapshot::<Vec<StoredMessageInteractionSummary>>(
                metadata_store,
                scope.as_str(),
                MESSAGE_INTERACTIONS_KEY,
            )?
            .map(interaction_map_from_items)
            .unwrap_or_default();

            self.summaries
                .lock_projection("summary store")
                .insert(scope.clone(), summary);
            self.entries
                .lock_projection("projection store")
                .insert(scope.clone(), timeline);
            match conversation {
                Some(conversation) => {
                    self.conversations
                        .lock_projection("conversation store")
                        .insert(scope.clone(), conversation);
                }
                None => {
                    self.conversations
                        .lock_projection("conversation store")
                        .remove(scope.as_str());
                }
            }
            self.members
                .lock_projection("member store")
                .insert(scope.clone(), members);
            self.read_cursors
                .lock_projection("cursor store")
                .insert(scope.clone(), read_cursors);
            self.message_interactions
                .lock_projection("message interaction store")
                .insert(scope.clone(), interactions);
            self.restore_contact_snapshot(metadata_store)?;
            self.restore_device_sync_snapshot(metadata_store, timeline_store)?;

            Ok(true)
        })();

        match &result {
            Ok(true) => self.record_projection_snapshot_success(
                ProjectionSnapshotOperation::ConversationSnapshotRestore,
                "conversation",
                scope.as_str(),
                format!("restored conversation projection snapshot for {scope}"),
            ),
            Ok(false) => {}
            Err(error) => self.record_projection_snapshot_failure(
                ProjectionSnapshotOperation::ConversationSnapshotRestore,
                "conversation",
                scope.as_str(),
                error,
            ),
        }

        result
    }

    pub fn persist_device_sync_snapshot(
        &self,
        metadata_store: &dyn MetadataStore,
        timeline_store: &dyn TimelineProjectionStore,
    ) -> Result<bool, ProjectionError> {
        let result = (|| {
            let registered_device_snapshots = self
                .registered_devices
                .lock_projection("registered device store")
                .iter()
                .map(|(scope, devices)| {
                    let mut items = devices.values().cloned().collect::<Vec<_>>();
                    items.sort_by(|left, right| left.device_id.cmp(&right.device_id));
                    (scope.clone(), items)
                })
                .collect::<Vec<_>>();
            let device_sync_feeds = self
                .device_sync_feeds
                .lock_projection("device sync feed store")
                .clone();
            let device_sync_sequences = self
                .device_sync_sequences
                .lock_projection("device sync sequence store")
                .clone();
            if registered_device_snapshots.is_empty()
                && device_sync_feeds.is_empty()
                && device_sync_sequences.is_empty()
            {
                return Ok(false);
            }

            let mut persisted_device_scopes = BTreeSet::new();
            let mut principal_catalog = BTreeSet::new();
            for (runtime_scope, devices) in &registered_device_snapshots {
                principal_catalog.insert(PrincipalSnapshotCatalogEntry {
                    tenant_id: runtime_scope.tenant_id.clone(),
                    principal_id: runtime_scope.principal_id.clone(),
                    principal_kind: runtime_scope.principal_kind.clone(),
                });
                let snapshot_scope = device_principal_snapshot_scope(runtime_scope);
                persist_metadata_snapshot(
                    metadata_store,
                    snapshot_scope.as_str(),
                    REGISTERED_DEVICES_KEY,
                    devices,
                )?;

                for device in devices {
                    persisted_device_scopes.insert(device_feed_scope_key(
                        runtime_scope.tenant_id.as_str(),
                        runtime_scope.principal_id.as_str(),
                        runtime_scope.principal_kind.as_deref(),
                        device.device_id.as_str(),
                    ));
                }
            }
            for runtime_scope in device_sync_feeds.keys() {
                persisted_device_scopes.insert(runtime_scope.clone());
            }
            for runtime_scope in device_sync_sequences.keys() {
                persisted_device_scopes.insert(runtime_scope.clone());
            }

            let principal_catalog = principal_catalog.into_iter().collect::<Vec<_>>();
            let device_scope_catalog = persisted_device_scopes
                .iter()
                .map(|scope| DeviceSyncScopeCatalogEntry {
                    tenant_id: scope.tenant_id.clone(),
                    principal_id: scope.principal_id.clone(),
                    principal_kind: scope.principal_kind.clone(),
                    device_id: scope.device_id.clone(),
                })
                .collect::<Vec<_>>();
            persist_metadata_snapshot(
                metadata_store,
                DEVICE_SYNC_CATALOG_SCOPE,
                REGISTERED_DEVICE_PRINCIPALS_KEY,
                &principal_catalog,
            )?;
            persist_metadata_snapshot(
                metadata_store,
                DEVICE_SYNC_CATALOG_SCOPE,
                DEVICE_SYNC_SCOPE_CATALOG_KEY,
                &device_scope_catalog,
            )?;

            for runtime_scope in persisted_device_scopes {
                let snapshot_scope = device_sync_snapshot_scope(&runtime_scope);
                let feed_entries = device_sync_feeds
                    .get(&runtime_scope)
                    .cloned()
                    .unwrap_or_default();
                let latest_sync_seq = device_sync_sequences
                    .get(&runtime_scope)
                    .copied()
                    .unwrap_or_else(|| {
                        feed_entries
                            .iter()
                            .map(|entry| entry.sync_seq)
                            .max()
                            .unwrap_or_default()
                    });
                persist_metadata_snapshot(
                    metadata_store,
                    snapshot_scope.as_str(),
                    DEVICE_SYNC_SEQUENCE_KEY,
                    &latest_sync_seq,
                )?;

                for entry in feed_entries {
                    let payload =
                        serde_json::to_string(&entry).map_err(ProjectionError::InvalidSnapshot)?;
                    timeline_store
                        .upsert_timeline_entry(
                            snapshot_scope.as_str(),
                            entry.sync_seq,
                            payload.as_str(),
                        )
                        .map_err(ProjectionError::StoreFailure)?;
                }
            }

            Ok(true)
        })();

        match &result {
            Ok(true) => self.record_projection_snapshot_success(
                ProjectionSnapshotOperation::DeviceSyncSnapshotPersist,
                "device-sync",
                DEVICE_SYNC_CATALOG_SCOPE,
                "persisted device sync projection snapshot catalog".to_owned(),
            ),
            Ok(false) => {}
            Err(error) => self.record_projection_snapshot_failure(
                ProjectionSnapshotOperation::DeviceSyncSnapshotPersist,
                "device-sync",
                DEVICE_SYNC_CATALOG_SCOPE,
                error,
            ),
        }

        result
    }

    pub fn persist_contact_snapshot(
        &self,
        metadata_store: &dyn MetadataStore,
    ) -> Result<bool, ProjectionError> {
        (|| {
            let contacts = self.contacts.lock_projection("contact store").clone();
            let direct_chat_bindings = self
                .direct_chat_bindings
                .lock_projection("contact direct chat binding store")
                .values()
                .cloned()
                .collect::<Vec<_>>();

            if contacts.is_empty() && direct_chat_bindings.is_empty() {
                return Ok(false);
            }

            let mut owner_catalog = BTreeSet::new();
            for (runtime_scope, scope_contacts) in &contacts {
                let Some((tenant_id, owner_user_id)) = parse_contact_scope(runtime_scope.as_str())
                else {
                    continue;
                };
                owner_catalog.insert(ContactOwnerCatalogEntry {
                    tenant_id: tenant_id.to_owned(),
                    owner_user_id: owner_user_id.to_owned(),
                });
                let snapshot_scope = principal_snapshot_scope(tenant_id, owner_user_id);
                let items = contact_snapshot_items(scope_contacts);
                persist_metadata_snapshot(
                    metadata_store,
                    snapshot_scope.as_str(),
                    CONTACTS_KEY,
                    &items,
                )?;
            }

            persist_metadata_snapshot(
                metadata_store,
                CONTACT_CATALOG_SCOPE,
                CONTACT_OWNERS_KEY,
                &owner_catalog.into_iter().collect::<Vec<_>>(),
            )?;
            persist_metadata_snapshot(
                metadata_store,
                CONTACT_CATALOG_SCOPE,
                CONTACT_DIRECT_CHAT_BINDINGS_KEY,
                &direct_chat_bindings,
            )?;

            Ok(true)
        })()
    }

    pub fn restore_contact_snapshot(
        &self,
        metadata_store: &dyn MetadataStore,
    ) -> Result<bool, ProjectionError> {
        (|| {
            let owner_catalog = load_metadata_snapshot::<Vec<ContactOwnerCatalogEntry>>(
                metadata_store,
                CONTACT_CATALOG_SCOPE,
                CONTACT_OWNERS_KEY,
            )?
            .unwrap_or_default();
            let direct_chat_bindings =
                load_metadata_snapshot::<Vec<crate::model::ContactDirectChatBindingView>>(
                    metadata_store,
                    CONTACT_CATALOG_SCOPE,
                    CONTACT_DIRECT_CHAT_BINDINGS_KEY,
                )?
                .unwrap_or_default();

            if owner_catalog.is_empty() && direct_chat_bindings.is_empty() {
                return Ok(false);
            }

            for owner in owner_catalog {
                let snapshot_scope = principal_snapshot_scope(
                    owner.tenant_id.as_str(),
                    owner.owner_user_id.as_str(),
                );
                let items = load_metadata_snapshot::<Vec<ContactView>>(
                    metadata_store,
                    snapshot_scope.as_str(),
                    CONTACTS_KEY,
                )?
                .unwrap_or_default();
                self.contacts.lock_projection("contact store").insert(
                    contact_runtime_scope(owner.tenant_id.as_str(), owner.owner_user_id.as_str()),
                    contact_map_from_items(items),
                );
            }

            self.direct_chat_bindings
                .lock_projection("contact direct chat binding store")
                .extend(
                    direct_chat_bindings
                        .into_iter()
                        .map(|binding| (binding.direct_chat_id.clone(), binding)),
                );

            Ok(true)
        })()
    }

    pub fn restore_device_sync_snapshot(
        &self,
        metadata_store: &dyn MetadataStore,
        timeline_store: &dyn TimelineProjectionStore,
    ) -> Result<bool, ProjectionError> {
        let result = (|| {
            let principal_catalog = load_metadata_snapshot::<Vec<PrincipalSnapshotCatalogEntry>>(
                metadata_store,
                DEVICE_SYNC_CATALOG_SCOPE,
                REGISTERED_DEVICE_PRINCIPALS_KEY,
            )?
            .unwrap_or_default();
            let device_scope_catalog = load_metadata_snapshot::<Vec<DeviceSyncScopeCatalogEntry>>(
                metadata_store,
                DEVICE_SYNC_CATALOG_SCOPE,
                DEVICE_SYNC_SCOPE_CATALOG_KEY,
            )?
            .unwrap_or_default();
            if principal_catalog.is_empty() && device_scope_catalog.is_empty() {
                return Ok(false);
            }

            let mut restored_device_scopes = BTreeSet::new();
            for principal in principal_catalog {
                let runtime_scope = device_principal_scope_key(
                    principal.tenant_id.as_str(),
                    principal.principal_id.as_str(),
                    principal.principal_kind.as_deref(),
                );
                let snapshot_scope = device_principal_snapshot_scope(&runtime_scope);
                let devices = load_metadata_snapshot::<Vec<crate::RegisteredDeviceView>>(
                    metadata_store,
                    snapshot_scope.as_str(),
                    REGISTERED_DEVICES_KEY,
                )?
                .unwrap_or_default();
                let device_map = devices
                    .into_iter()
                    .map(|device| {
                        restored_device_scopes.insert(device_feed_scope_key(
                            principal.tenant_id.as_str(),
                            principal.principal_id.as_str(),
                            principal.principal_kind.as_deref(),
                            device.device_id.as_str(),
                        ));
                        (device.device_id.clone(), device)
                    })
                    .collect();
                self.registered_devices
                    .lock_projection("registered device store")
                    .insert(runtime_scope, device_map);
            }

            for device_scope in device_scope_catalog {
                restored_device_scopes.insert(device_feed_scope_key(
                    device_scope.tenant_id.as_str(),
                    device_scope.principal_id.as_str(),
                    device_scope.principal_kind.as_deref(),
                    device_scope.device_id.as_str(),
                ));
            }

            for runtime_scope in restored_device_scopes {
                let snapshot_scope = device_sync_snapshot_scope(&runtime_scope);
                let mut feed_entries = timeline_store
                    .load_timeline(snapshot_scope.as_str())
                    .map_err(ProjectionError::StoreFailure)?
                    .into_iter()
                    .map(|(_, payload)| {
                        serde_json::from_str::<DeviceSyncFeedEntry>(&payload)
                            .map_err(ProjectionError::InvalidSnapshot)
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                feed_entries.sort_by_key(|entry| entry.sync_seq);

                let restored_latest_sync_seq = load_metadata_snapshot::<u64>(
                    metadata_store,
                    snapshot_scope.as_str(),
                    DEVICE_SYNC_SEQUENCE_KEY,
                )?
                .unwrap_or_else(|| {
                    feed_entries
                        .iter()
                        .map(|entry| entry.sync_seq)
                        .max()
                        .unwrap_or_default()
                });
                self.device_sync_sequences
                    .lock_projection("device sync sequence store")
                    .insert(runtime_scope.clone(), restored_latest_sync_seq);
                self.device_sync_feeds
                    .lock_projection("device sync feed store")
                    .insert(runtime_scope, feed_entries);
            }

            Ok(true)
        })();

        match &result {
            Ok(true) => self.record_projection_snapshot_success(
                ProjectionSnapshotOperation::DeviceSyncSnapshotRestore,
                "device-sync",
                DEVICE_SYNC_CATALOG_SCOPE,
                "restored device sync projection snapshot catalog".to_owned(),
            ),
            Ok(false) => {}
            Err(error) => self.record_projection_snapshot_failure(
                ProjectionSnapshotOperation::DeviceSyncSnapshotRestore,
                "device-sync",
                DEVICE_SYNC_CATALOG_SCOPE,
                error,
            ),
        }

        result
    }
}

fn conversation_snapshot_scope(tenant_id: &str, conversation_id: &str) -> String {
    super::scope_key(tenant_id, conversation_id)
}

fn principal_snapshot_scope(tenant_id: &str, principal_id: &str) -> String {
    format!("{PRINCIPAL_SNAPSHOT_SCOPE_PREFIX}:{tenant_id}:{principal_id}")
}

fn device_principal_snapshot_scope(scope: &DevicePrincipalScopeKey) -> String {
    match scope.principal_kind.as_deref() {
        Some(principal_kind) => format!(
            "{PRINCIPAL_SNAPSHOT_SCOPE_PREFIX}:typed:{}:{principal_kind}:{}",
            scope.tenant_id, scope.principal_id
        ),
        None => format!(
            "{PRINCIPAL_SNAPSHOT_SCOPE_PREFIX}:{}:{}",
            scope.tenant_id, scope.principal_id
        ),
    }
}

fn device_sync_snapshot_scope(scope: &DeviceFeedScopeKey) -> String {
    match scope.principal_kind.as_deref() {
        Some(principal_kind) => format!(
            "{DEVICE_SYNC_SNAPSHOT_SCOPE_PREFIX}:typed:{}:{principal_kind}:{}:{}",
            scope.tenant_id, scope.principal_id, scope.device_id
        ),
        None => format!(
            "{DEVICE_SYNC_SNAPSHOT_SCOPE_PREFIX}:{}:{}:{}",
            scope.tenant_id, scope.principal_id, scope.device_id
        ),
    }
}

fn persist_metadata_snapshot<T: Serialize>(
    metadata_store: &dyn MetadataStore,
    scope: &str,
    key: &str,
    value: &T,
) -> Result<(), ProjectionError> {
    let payload = serde_json::to_string(value).map_err(ProjectionError::InvalidSnapshot)?;
    metadata_store
        .put_snapshot(scope, key, payload.as_str())
        .map_err(ProjectionError::StoreFailure)
}

fn load_metadata_snapshot<T: DeserializeOwned>(
    metadata_store: &dyn MetadataStore,
    scope: &str,
    key: &str,
) -> Result<Option<T>, ProjectionError> {
    metadata_store
        .load_snapshot(scope, key)
        .map_err(ProjectionError::StoreFailure)?
        .map(|payload| serde_json::from_str(&payload).map_err(ProjectionError::InvalidSnapshot))
        .transpose()
}

#[cfg(test)]
mod tests {
    use std::panic::{self, AssertUnwindSafe};
    use std::sync::Mutex;

    use im_adapters_local_memory::{MemoryMetadataStore, MemoryTimelineProjectionStore};

    use super::*;

    fn poison_mutex<T>(mutex: &Mutex<T>) {
        let _ = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = mutex.lock().expect("test poison lock should succeed");
            panic!("intentional poison for regression coverage");
        }));
    }

    fn demo_summary(tenant_id: &str, conversation_id: &str) -> ConversationSummaryView {
        ConversationSummaryView {
            tenant_id: tenant_id.into(),
            conversation_id: conversation_id.into(),
            message_count: 0,
            last_message_id: None,
            last_message_seq: 0,
            last_sender_id: None,
            last_sender_kind: None,
            last_sender: None,
            last_summary: None,
            last_message_at: None,
            agent_handoff: None,
        }
    }

    #[test]
    fn test_persist_contact_snapshot_recovers_from_poisoned_contact_store_lock() {
        let projection = TimelineProjectionService::default();
        let metadata_store = MemoryMetadataStore::default();
        poison_mutex(&projection.contacts);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            projection.persist_contact_snapshot(&metadata_store)
        }));
        assert!(
            result.is_ok(),
            "persist_contact_snapshot should not panic when contact store lock is poisoned"
        );
        let persist_result = result.expect("panic status should be captured");
        assert!(
            persist_result.is_ok(),
            "persist_contact_snapshot should recover from poisoned lock"
        );
    }

    #[test]
    fn test_restore_conversation_snapshot_recovers_from_poisoned_summary_store_lock() {
        let projection = TimelineProjectionService::default();
        let metadata_store = MemoryMetadataStore::default();
        let timeline_store = MemoryTimelineProjectionStore::default();
        let snapshot_scope = conversation_snapshot_scope("t_demo", "c_demo");
        persist_metadata_snapshot(
            &metadata_store,
            snapshot_scope.as_str(),
            CONVERSATION_SUMMARY_KEY,
            &demo_summary("t_demo", "c_demo"),
        )
        .expect("summary snapshot should persist");

        poison_mutex(&projection.summaries);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            projection.restore_conversation_snapshot(
                "t_demo",
                "c_demo",
                &metadata_store,
                &timeline_store,
            )
        }));
        assert!(
            result.is_ok(),
            "restore_conversation_snapshot should not panic when summary store lock is poisoned"
        );
        let restore_result = result.expect("panic status should be captured");
        assert!(
            restore_result.is_ok(),
            "restore_conversation_snapshot should recover from poisoned lock"
        );
    }

    #[test]
    fn test_persist_device_sync_snapshot_recovers_from_poisoned_registered_device_store_lock() {
        let projection = TimelineProjectionService::default();
        let metadata_store = MemoryMetadataStore::default();
        let timeline_store = MemoryTimelineProjectionStore::default();
        poison_mutex(&projection.registered_devices);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            projection.persist_device_sync_snapshot(&metadata_store, &timeline_store)
        }));
        assert!(
            result.is_ok(),
            "persist_device_sync_snapshot should not panic when registered-device lock is poisoned"
        );
        let persist_result = result.expect("panic status should be captured");
        assert!(
            persist_result.is_ok(),
            "persist_device_sync_snapshot should recover from poisoned lock"
        );
    }

    #[test]
    fn test_restore_contact_snapshot_recovers_from_poisoned_binding_store_lock() {
        let projection = TimelineProjectionService::default();
        let metadata_store = MemoryMetadataStore::default();
        persist_metadata_snapshot(
            &metadata_store,
            CONTACT_CATALOG_SCOPE,
            CONTACT_OWNERS_KEY,
            &Vec::<ContactOwnerCatalogEntry>::new(),
        )
        .expect("contact owner catalog should persist");
        persist_metadata_snapshot(
            &metadata_store,
            CONTACT_CATALOG_SCOPE,
            CONTACT_DIRECT_CHAT_BINDINGS_KEY,
            &vec![crate::model::ContactDirectChatBindingView {
                direct_chat_id: "dc_demo".into(),
                conversation_id: "c_demo".into(),
                bound_at: "2026-04-01T00:00:00.000Z".into(),
            }],
        )
        .expect("contact direct-chat binding snapshot should persist");
        poison_mutex(&projection.direct_chat_bindings);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            projection.restore_contact_snapshot(&metadata_store)
        }));
        assert!(
            result.is_ok(),
            "restore_contact_snapshot should not panic when direct-chat-binding lock is poisoned"
        );
        let restore_result = result.expect("panic status should be captured");
        assert!(
            restore_result.is_ok(),
            "restore_contact_snapshot should recover from poisoned lock"
        );
    }

    #[test]
    fn test_restore_device_sync_snapshot_recovers_from_poisoned_sequence_store_lock() {
        let projection = TimelineProjectionService::default();
        let metadata_store = MemoryMetadataStore::default();
        let timeline_store = MemoryTimelineProjectionStore::default();
        let device_scope = DeviceSyncScopeCatalogEntry {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            principal_kind: None,
            device_id: "d_demo".into(),
        };
        persist_metadata_snapshot(
            &metadata_store,
            DEVICE_SYNC_CATALOG_SCOPE,
            REGISTERED_DEVICE_PRINCIPALS_KEY,
            &Vec::<PrincipalSnapshotCatalogEntry>::new(),
        )
        .expect("device-sync principal catalog should persist");
        persist_metadata_snapshot(
            &metadata_store,
            DEVICE_SYNC_CATALOG_SCOPE,
            DEVICE_SYNC_SCOPE_CATALOG_KEY,
            &vec![device_scope.clone()],
        )
        .expect("device-sync scope catalog should persist");
        persist_metadata_snapshot(
            &metadata_store,
            device_sync_snapshot_scope(&device_feed_scope_key(
                device_scope.tenant_id.as_str(),
                device_scope.principal_id.as_str(),
                device_scope.principal_kind.as_deref(),
                device_scope.device_id.as_str(),
            ))
            .as_str(),
            DEVICE_SYNC_SEQUENCE_KEY,
            &42_u64,
        )
        .expect("device-sync sequence snapshot should persist");
        poison_mutex(&projection.device_sync_sequences);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            projection.restore_device_sync_snapshot(&metadata_store, &timeline_store)
        }));
        assert!(
            result.is_ok(),
            "restore_device_sync_snapshot should not panic when sequence lock is poisoned"
        );
        let restore_result = result.expect("panic status should be captured");
        assert!(
            restore_result.is_ok(),
            "restore_device_sync_snapshot should recover from poisoned lock"
        );
    }
}
