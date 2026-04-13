use craw_chat_contract_control::{RealtimeCheckpointStore, RealtimeSubscriptionStore};
use craw_chat_contract_core::ContractError;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};

use im_domain_core::realtime::{
    RealtimeAckState, RealtimeEvent, RealtimeEventWindow, RealtimeSubscription,
    RealtimeSubscriptionSnapshot,
};
use im_time::utc_now_rfc3339_millis;
use serde::Deserialize;
use tokio::sync::watch;

use crate::principal_scope::{actor_device_scope_key, typed_device_scope_key, typed_principal_id};

mod storage;

use storage::{RuntimeMemoryCheckpointStore, RuntimeMemorySubscriptionStore};

const REALTIME_MAX_SCOPE_TYPE_BYTES: usize = 64;
const REALTIME_MAX_SCOPE_ID_BYTES: usize = 512;
const REALTIME_MAX_EVENT_TYPE_BYTES: usize = 128;
const REALTIME_MAX_EVENT_TYPES_TOTAL_BYTES: usize = 16 * 1024;
const REALTIME_MAX_SUBSCRIPTION_ITEMS: usize = 256;
pub(crate) const REALTIME_EVENT_WINDOW_MAX_LIMIT: usize = 1000;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeSubscriptionItemInput {
    pub scope_type: String,
    pub scope_id: String,
    #[serde(default)]
    pub event_types: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SyncRealtimeSubscriptionsRequest {
    pub device_id: Option<String>,
    #[serde(default)]
    pub items: Vec<RealtimeSubscriptionItemInput>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListRealtimeEventsQuery {
    pub after_seq: Option<u64>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AckRealtimeEventsRequest {
    pub device_id: Option<String>,
    pub acked_seq: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeRuntimeError {
    pub code: &'static str,
    pub message: String,
}

impl From<ContractError> for RealtimeRuntimeError {
    fn from(value: ContractError) -> Self {
        Self::checkpoint_store(value)
    }
}

impl RealtimeRuntimeError {
    fn payload_too_large(field: &'static str, max_bytes: usize, actual_bytes: usize) -> Self {
        Self {
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
            ),
        }
    }

    fn collection_too_large(field: &'static str, max_items: usize, actual_items: usize) -> Self {
        Self {
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_items} items, actual={actual_items} items"
            ),
        }
    }

    fn limit_invalid(limit: usize) -> Self {
        Self {
            code: "limit_invalid",
            message: format!(
                "limit must be less than or equal to {REALTIME_EVENT_WINDOW_MAX_LIMIT}; actual={limit}"
            ),
        }
    }

    fn checkpoint_store(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                code: "checkpoint_store_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                code: "checkpoint_store_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                code: "checkpoint_store_unsupported",
                message,
            },
        }
    }

    fn subscription_store(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                code: "subscription_store_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                code: "subscription_store_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                code: "subscription_store_unsupported",
                message,
            },
        }
    }
}

#[derive(Clone)]
pub struct RealtimeDeliveryRuntime {
    subscriptions: Arc<Mutex<HashMap<String, Vec<RealtimeSubscription>>>>,
    windows: Arc<Mutex<HashMap<String, Vec<RealtimeEvent>>>>,
    latest_sequences: Arc<Mutex<HashMap<String, u64>>>,
    acked_sequences: Arc<Mutex<HashMap<String, u64>>>,
    trimmed_sequences: Arc<Mutex<HashMap<String, u64>>>,
    notifiers: Arc<Mutex<HashMap<String, watch::Sender<u64>>>>,
    disconnect_generations: Arc<Mutex<HashMap<String, u64>>>,
    disconnect_notifiers: Arc<Mutex<HashMap<String, watch::Sender<u64>>>>,
    checkpoint_store: Arc<dyn RealtimeCheckpointStore>,
    subscription_store: Arc<dyn RealtimeSubscriptionStore>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeWindowCheckpoint {
    pub latest_realtime_seq: u64,
    pub acked_through_seq: u64,
    pub trimmed_through_seq: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeDeviceStateSnapshot {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
    pub subscriptions: Vec<RealtimeSubscription>,
    pub events: Vec<RealtimeEvent>,
    pub latest_realtime_seq: u64,
    pub acked_through_seq: u64,
    pub trimmed_through_seq: u64,
}

impl Default for RealtimeDeliveryRuntime {
    fn default() -> Self {
        Self::with_checkpoint_store(Arc::new(RuntimeMemoryCheckpointStore::default()))
    }
}

impl fmt::Debug for RealtimeDeliveryRuntime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RealtimeDeliveryRuntime")
            .finish_non_exhaustive()
    }
}

impl RealtimeDeliveryRuntime {
    pub fn with_checkpoint_store(checkpoint_store: Arc<dyn RealtimeCheckpointStore>) -> Self {
        Self::with_stores(
            checkpoint_store,
            Arc::new(RuntimeMemorySubscriptionStore::default()),
        )
    }

    pub fn with_stores(
        checkpoint_store: Arc<dyn RealtimeCheckpointStore>,
        subscription_store: Arc<dyn RealtimeSubscriptionStore>,
    ) -> Self {
        Self {
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            windows: Arc::new(Mutex::new(HashMap::new())),
            latest_sequences: Arc::new(Mutex::new(HashMap::new())),
            acked_sequences: Arc::new(Mutex::new(HashMap::new())),
            trimmed_sequences: Arc::new(Mutex::new(HashMap::new())),
            notifiers: Arc::new(Mutex::new(HashMap::new())),
            disconnect_generations: Arc::new(Mutex::new(HashMap::new())),
            disconnect_notifiers: Arc::new(Mutex::new(HashMap::new())),
            checkpoint_store,
            subscription_store,
        }
    }

    pub fn ensure_device_state(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        self.ensure_device_state_internal(tenant_id, principal_id, None, device_id)
    }

    pub fn ensure_device_state_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        self.ensure_device_state_internal(tenant_id, principal_id, Some(principal_kind), device_id)
    }

    fn ensure_device_state_internal(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: Option<&str>,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        let scope_key = device_scope_key(tenant_id, principal_id, principal_kind, device_id);
        let stored_principal_id = storage_principal_id(principal_id, principal_kind);
        let needs_restore = !lock_realtime_mutex(&self.latest_sequences, "realtime sequence store")
            .contains_key(scope_key.as_str());
        let restored = if needs_restore {
            self.checkpoint_store
                .load_checkpoint(tenant_id, stored_principal_id.as_str(), device_id)
                .map_err(RealtimeRuntimeError::checkpoint_store)?
        } else {
            None
        };
        let restored_subscriptions = if needs_restore {
            self.subscription_store
                .load_subscriptions(tenant_id, stored_principal_id.as_str(), device_id)
                .map_err(RealtimeRuntimeError::subscription_store)?
        } else {
            None
        };
        let normalized_restored = restored.as_ref().map(|record| {
            normalize_checkpoint_fields(
                record.latest_realtime_seq,
                record.acked_through_seq,
                record.trimmed_through_seq,
            )
        });
        let checkpoint_needs_normalization = restored.as_ref().is_some_and(|record| {
            normalized_restored
                .map(|(latest, acked, trimmed)| {
                    latest != record.latest_realtime_seq
                        || acked != record.acked_through_seq
                        || trimmed != record.trimmed_through_seq
                })
                .unwrap_or(false)
        });

        lock_realtime_mutex(&self.windows, "realtime window store")
            .entry(scope_key.clone())
            .or_default();
        if let Some(restored_subscriptions) =
            restored_subscriptions.filter(|record| !record.items.is_empty())
        {
            lock_realtime_mutex(&self.subscriptions, "realtime subscription store")
                .entry(scope_key.clone())
                .or_insert(restored_subscriptions.items);
        }
        let latest_seq = {
            let mut latest_sequences =
                lock_realtime_mutex(&self.latest_sequences, "realtime sequence store");
            *latest_sequences
                .entry(scope_key.clone())
                .or_insert_with(|| normalized_restored.map(|item| item.0).unwrap_or(0))
        };
        lock_realtime_mutex(&self.acked_sequences, "realtime ack store")
            .entry(scope_key.clone())
            .or_insert_with(|| normalized_restored.map(|item| item.1).unwrap_or(0));
        lock_realtime_mutex(&self.trimmed_sequences, "realtime trim store")
            .entry(scope_key.clone())
            .or_insert_with(|| normalized_restored.map(|item| item.2).unwrap_or(0));
        lock_realtime_mutex(&self.notifiers, "realtime notifier store")
            .entry(scope_key.clone())
            .or_insert_with(|| {
                let (sender, _) = watch::channel(latest_seq);
                sender
            });
        let disconnect_generation = {
            let mut disconnect_generations = lock_realtime_mutex(
                &self.disconnect_generations,
                "realtime disconnect generation store",
            );
            *disconnect_generations.entry(scope_key.clone()).or_insert(0)
        };
        lock_realtime_mutex(
            &self.disconnect_notifiers,
            "realtime disconnect notifier store",
        )
        .entry(scope_key)
        .or_insert_with(|| {
            let (sender, _) = watch::channel(disconnect_generation);
            sender
        });
        if checkpoint_needs_normalization {
            self.persist_checkpoint_internal(tenant_id, principal_id, principal_kind, device_id)?;
        }

        Ok(())
    }

    pub fn subscribe_device(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<watch::Receiver<u64>, RealtimeRuntimeError> {
        self.subscribe_device_internal(tenant_id, principal_id, None, device_id)
    }

    pub fn subscribe_device_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<watch::Receiver<u64>, RealtimeRuntimeError> {
        self.subscribe_device_internal(tenant_id, principal_id, Some(principal_kind), device_id)
    }

    fn subscribe_device_internal(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: Option<&str>,
        device_id: &str,
    ) -> Result<watch::Receiver<u64>, RealtimeRuntimeError> {
        self.ensure_device_state_internal(tenant_id, principal_id, principal_kind, device_id)?;
        let scope_key = device_scope_key(tenant_id, principal_id, principal_kind, device_id);
        let sender = lock_realtime_mutex(&self.notifiers, "realtime notifier store")
            .entry(scope_key)
            .or_insert_with(|| {
                eprintln!("warn: realtime notifier missing after ensure; reconstructing sender");
                let (sender, _) = watch::channel(0);
                sender
            })
            .clone();
        Ok(sender.subscribe())
    }

    pub fn subscribe_disconnect_signal(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<watch::Receiver<u64>, RealtimeRuntimeError> {
        self.subscribe_disconnect_signal_internal(tenant_id, principal_id, None, device_id)
    }

    pub fn subscribe_disconnect_signal_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<watch::Receiver<u64>, RealtimeRuntimeError> {
        self.subscribe_disconnect_signal_internal(
            tenant_id,
            principal_id,
            Some(principal_kind),
            device_id,
        )
    }

    fn subscribe_disconnect_signal_internal(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: Option<&str>,
        device_id: &str,
    ) -> Result<watch::Receiver<u64>, RealtimeRuntimeError> {
        self.ensure_device_state_internal(tenant_id, principal_id, principal_kind, device_id)?;
        let scope_key = device_scope_key(tenant_id, principal_id, principal_kind, device_id);
        let sender = lock_realtime_mutex(
            &self.disconnect_notifiers,
            "realtime disconnect notifier store",
        )
        .entry(scope_key)
        .or_insert_with(|| {
            eprintln!(
                "warn: realtime disconnect notifier missing after ensure; reconstructing sender"
            );
            let (sender, _) = watch::channel(0);
            sender
        })
        .clone();
        Ok(sender.subscribe())
    }

    pub fn disconnect_generation(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<u64, RealtimeRuntimeError> {
        self.disconnect_generation_internal(tenant_id, principal_id, None, device_id)
    }

    pub fn disconnect_generation_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<u64, RealtimeRuntimeError> {
        self.disconnect_generation_internal(
            tenant_id,
            principal_id,
            Some(principal_kind),
            device_id,
        )
    }

    fn disconnect_generation_internal(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: Option<&str>,
        device_id: &str,
    ) -> Result<u64, RealtimeRuntimeError> {
        self.ensure_device_state_internal(tenant_id, principal_id, principal_kind, device_id)?;
        Ok(lock_realtime_mutex(
            &self.disconnect_generations,
            "realtime disconnect generation store",
        )
        .get(device_scope_key(tenant_id, principal_id, principal_kind, device_id).as_str())
        .copied()
        .unwrap_or(0))
    }

    pub fn signal_device_disconnect(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        self.signal_device_disconnect_internal(tenant_id, principal_id, None, device_id)
    }

    pub fn signal_device_disconnect_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        self.signal_device_disconnect_internal(
            tenant_id,
            principal_id,
            Some(principal_kind),
            device_id,
        )
    }

    fn signal_device_disconnect_internal(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: Option<&str>,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        self.ensure_device_state_internal(tenant_id, principal_id, principal_kind, device_id)?;
        let scope_key = device_scope_key(tenant_id, principal_id, principal_kind, device_id);
        let next_generation = {
            let mut disconnect_generations = lock_realtime_mutex(
                &self.disconnect_generations,
                "realtime disconnect generation store",
            );
            let entry = disconnect_generations.entry(scope_key.clone()).or_insert(0);
            *entry += 1;
            *entry
        };
        if let Some(sender) = lock_realtime_mutex(
            &self.disconnect_notifiers,
            "realtime disconnect notifier store",
        )
        .get(scope_key.as_str())
        .cloned()
        {
            let _ = sender.send(next_generation);
        }

        Ok(())
    }

    pub fn window_checkpoint(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<RealtimeWindowCheckpoint, RealtimeRuntimeError> {
        self.window_checkpoint_internal(tenant_id, principal_id, None, device_id)
    }

    pub fn window_checkpoint_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<RealtimeWindowCheckpoint, RealtimeRuntimeError> {
        self.window_checkpoint_internal(tenant_id, principal_id, Some(principal_kind), device_id)
    }

    fn window_checkpoint_internal(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: Option<&str>,
        device_id: &str,
    ) -> Result<RealtimeWindowCheckpoint, RealtimeRuntimeError> {
        self.ensure_device_state_internal(tenant_id, principal_id, principal_kind, device_id)?;
        let scope_key = device_scope_key(tenant_id, principal_id, principal_kind, device_id);
        Ok(RealtimeWindowCheckpoint {
            latest_realtime_seq: lock_realtime_mutex(
                &self.latest_sequences,
                "realtime sequence store",
            )
            .get(scope_key.as_str())
            .copied()
            .unwrap_or(0),
            acked_through_seq: lock_realtime_mutex(&self.acked_sequences, "realtime ack store")
                .get(scope_key.as_str())
                .copied()
                .unwrap_or(0),
            trimmed_through_seq: lock_realtime_mutex(
                &self.trimmed_sequences,
                "realtime trim store",
            )
            .get(scope_key.as_str())
            .copied()
            .unwrap_or(0),
        })
    }

    pub fn sync_subscriptions(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        items: Vec<RealtimeSubscriptionItemInput>,
    ) -> Result<RealtimeSubscriptionSnapshot, RealtimeRuntimeError> {
        self.sync_subscriptions_internal(tenant_id, principal_id, None, device_id, items)
    }

    pub fn sync_subscriptions_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        items: Vec<RealtimeSubscriptionItemInput>,
    ) -> Result<RealtimeSubscriptionSnapshot, RealtimeRuntimeError> {
        self.sync_subscriptions_internal(
            tenant_id,
            principal_id,
            Some(principal_kind),
            device_id,
            items,
        )
    }

    fn sync_subscriptions_internal(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: Option<&str>,
        device_id: &str,
        items: Vec<RealtimeSubscriptionItemInput>,
    ) -> Result<RealtimeSubscriptionSnapshot, RealtimeRuntimeError> {
        validate_realtime_subscription_items(&items)?;
        let synced_at = realtime_timestamp();
        self.ensure_device_state_internal(tenant_id, principal_id, principal_kind, device_id)?;
        let subscriptions = items
            .into_iter()
            .map(|item| RealtimeSubscription {
                scope_type: item.scope_type,
                scope_id: item.scope_id,
                event_types: item.event_types,
                subscribed_at: synced_at.clone(),
            })
            .collect::<Vec<_>>();
        let scope_key = device_scope_key(tenant_id, principal_id, principal_kind, device_id);

        lock_realtime_mutex(&self.subscriptions, "realtime subscription store")
            .insert(scope_key.clone(), subscriptions.clone());
        self.persist_subscriptions_internal(tenant_id, principal_id, principal_kind, device_id)?;

        Ok(RealtimeSubscriptionSnapshot {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            device_id: device_id.into(),
            items: subscriptions,
            synced_at,
        })
    }

    pub fn clear_device_subscriptions(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        self.clear_device_subscriptions_internal(tenant_id, principal_id, None, device_id)
    }

    pub fn clear_device_subscriptions_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        self.clear_device_subscriptions_internal(
            tenant_id,
            principal_id,
            Some(principal_kind),
            device_id,
        )
    }

    fn clear_device_subscriptions_internal(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: Option<&str>,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        let scope_key = device_scope_key(tenant_id, principal_id, principal_kind, device_id);
        lock_realtime_mutex(&self.subscriptions, "realtime subscription store")
            .remove(scope_key.as_str());
        self.persist_subscriptions_internal(tenant_id, principal_id, principal_kind, device_id)
    }

    pub fn list_events(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        after_seq: u64,
        limit: usize,
    ) -> Result<RealtimeEventWindow, RealtimeRuntimeError> {
        self.list_events_internal(tenant_id, principal_id, None, device_id, after_seq, limit)
    }

    pub fn list_events_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        after_seq: u64,
        limit: usize,
    ) -> Result<RealtimeEventWindow, RealtimeRuntimeError> {
        self.list_events_internal(
            tenant_id,
            principal_id,
            Some(principal_kind),
            device_id,
            after_seq,
            limit,
        )
    }

    fn list_events_internal(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: Option<&str>,
        device_id: &str,
        after_seq: u64,
        limit: usize,
    ) -> Result<RealtimeEventWindow, RealtimeRuntimeError> {
        validate_realtime_event_limit(limit)?;
        self.ensure_device_state_internal(tenant_id, principal_id, principal_kind, device_id)?;
        let scope_key = device_scope_key(tenant_id, principal_id, principal_kind, device_id);
        let acked_through_seq = lock_realtime_mutex(&self.acked_sequences, "realtime ack store")
            .get(scope_key.as_str())
            .copied()
            .unwrap_or(0);
        let trimmed_through_seq =
            lock_realtime_mutex(&self.trimmed_sequences, "realtime trim store")
                .get(scope_key.as_str())
                .copied()
                .unwrap_or(0);
        let windows = lock_realtime_mutex(&self.windows, "realtime window store");
        let total_after = windows
            .get(scope_key.as_str())
            .map(|items| {
                items
                    .iter()
                    .filter(|item| item.realtime_seq > after_seq)
                    .count()
            })
            .unwrap_or(0);
        let items = windows
            .get(scope_key.as_str())
            .into_iter()
            .flat_map(|items| items.iter())
            .filter(|item| item.realtime_seq > after_seq)
            .take(limit)
            .cloned()
            .collect::<Vec<_>>();
        let has_more = total_after > items.len();
        let next_after_seq = items.last().map(|item| item.realtime_seq);

        Ok(RealtimeEventWindow {
            device_id: device_id.into(),
            items,
            next_after_seq,
            has_more,
            acked_through_seq,
            trimmed_through_seq,
        })
    }

    pub fn ack_events(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        acked_seq: u64,
    ) -> Result<RealtimeAckState, RealtimeRuntimeError> {
        self.ack_events_internal(tenant_id, principal_id, None, device_id, acked_seq)
    }

    pub fn ack_events_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        acked_seq: u64,
    ) -> Result<RealtimeAckState, RealtimeRuntimeError> {
        self.ack_events_internal(
            tenant_id,
            principal_id,
            Some(principal_kind),
            device_id,
            acked_seq,
        )
    }

    fn ack_events_internal(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: Option<&str>,
        device_id: &str,
        acked_seq: u64,
    ) -> Result<RealtimeAckState, RealtimeRuntimeError> {
        self.ensure_device_state_internal(tenant_id, principal_id, principal_kind, device_id)?;
        let scope_key = device_scope_key(tenant_id, principal_id, principal_kind, device_id);
        let latest_seq = lock_realtime_mutex(&self.latest_sequences, "realtime sequence store")
            .get(scope_key.as_str())
            .copied()
            .unwrap_or(0);
        let effective_ack = acked_seq.min(latest_seq);
        let acked_through_seq = {
            let mut acked_sequences =
                lock_realtime_mutex(&self.acked_sequences, "realtime ack store");
            let entry = acked_sequences.entry(scope_key.clone()).or_insert(0);
            if effective_ack > *entry {
                *entry = effective_ack;
            }
            *entry
        };

        let (trimmed_through_seq, retained_event_count) = {
            let mut windows = lock_realtime_mutex(&self.windows, "realtime window store");
            let mut trimmed_sequences =
                lock_realtime_mutex(&self.trimmed_sequences, "realtime trim store");
            let trimmed_entry = trimmed_sequences.entry(scope_key.clone()).or_insert(0);
            if acked_through_seq > *trimmed_entry {
                *trimmed_entry = acked_through_seq;
            }
            let items = windows.entry(scope_key).or_default();
            items.retain(|item| item.realtime_seq > acked_through_seq);
            (*trimmed_entry, items.len())
        };
        self.persist_checkpoint_internal(tenant_id, principal_id, principal_kind, device_id)?;

        Ok(RealtimeAckState {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            device_id: device_id.into(),
            acked_through_seq,
            trimmed_through_seq,
            retained_event_count,
            acked_at: realtime_timestamp(),
        })
    }

    pub fn take_device_state(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<RealtimeDeviceStateSnapshot, RealtimeRuntimeError> {
        self.take_device_state_internal(tenant_id, principal_id, None, device_id)
    }

    pub fn take_device_state_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<RealtimeDeviceStateSnapshot, RealtimeRuntimeError> {
        self.take_device_state_internal(tenant_id, principal_id, Some(principal_kind), device_id)
    }

    fn take_device_state_internal(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: Option<&str>,
        device_id: &str,
    ) -> Result<RealtimeDeviceStateSnapshot, RealtimeRuntimeError> {
        self.ensure_device_state_internal(tenant_id, principal_id, principal_kind, device_id)?;
        let scope_key = device_scope_key(tenant_id, principal_id, principal_kind, device_id);
        let subscriptions = lock_realtime_mutex(&self.subscriptions, "realtime subscription store")
            .remove(scope_key.as_str())
            .unwrap_or_default();
        let events = lock_realtime_mutex(&self.windows, "realtime window store")
            .remove(scope_key.as_str())
            .unwrap_or_default();
        let latest_realtime_seq =
            lock_realtime_mutex(&self.latest_sequences, "realtime sequence store")
                .remove(scope_key.as_str())
                .unwrap_or(0);
        let acked_through_seq = lock_realtime_mutex(&self.acked_sequences, "realtime ack store")
            .remove(scope_key.as_str())
            .unwrap_or(0);
        let trimmed_through_seq =
            lock_realtime_mutex(&self.trimmed_sequences, "realtime trim store")
                .remove(scope_key.as_str())
                .unwrap_or(0);
        lock_realtime_mutex(&self.notifiers, "realtime notifier store").remove(scope_key.as_str());

        Ok(RealtimeDeviceStateSnapshot {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            device_id: device_id.into(),
            subscriptions,
            events,
            latest_realtime_seq,
            acked_through_seq,
            trimmed_through_seq,
        })
    }

    pub fn restore_device_state(
        &self,
        snapshot: RealtimeDeviceStateSnapshot,
    ) -> Result<(), RealtimeRuntimeError> {
        self.restore_device_state_internal(snapshot, None)
    }

    pub fn restore_device_state_for_principal_kind(
        &self,
        principal_kind: &str,
        snapshot: RealtimeDeviceStateSnapshot,
    ) -> Result<(), RealtimeRuntimeError> {
        self.restore_device_state_internal(snapshot, Some(principal_kind))
    }

    fn restore_device_state_internal(
        &self,
        snapshot: RealtimeDeviceStateSnapshot,
        principal_kind: Option<&str>,
    ) -> Result<(), RealtimeRuntimeError> {
        let latest_realtime_seq = snapshot
            .events
            .iter()
            .map(|event| event.realtime_seq)
            .max()
            .unwrap_or(snapshot.latest_realtime_seq)
            .max(snapshot.latest_realtime_seq);
        let (latest_realtime_seq, acked_through_seq, trimmed_through_seq) =
            normalize_checkpoint_fields(
                latest_realtime_seq,
                snapshot.acked_through_seq,
                snapshot.trimmed_through_seq,
            );
        let normalized_events = normalize_window_events(snapshot.events, trimmed_through_seq);
        let scope_key = device_scope_key(
            snapshot.tenant_id.as_str(),
            snapshot.principal_id.as_str(),
            principal_kind,
            snapshot.device_id.as_str(),
        );

        lock_realtime_mutex(&self.subscriptions, "realtime subscription store")
            .insert(scope_key.clone(), snapshot.subscriptions);
        lock_realtime_mutex(&self.windows, "realtime window store")
            .insert(scope_key.clone(), normalized_events);
        lock_realtime_mutex(&self.latest_sequences, "realtime sequence store")
            .insert(scope_key.clone(), latest_realtime_seq);
        lock_realtime_mutex(&self.acked_sequences, "realtime ack store")
            .insert(scope_key.clone(), acked_through_seq);
        lock_realtime_mutex(&self.trimmed_sequences, "realtime trim store")
            .insert(scope_key.clone(), trimmed_through_seq);
        lock_realtime_mutex(&self.notifiers, "realtime notifier store")
            .insert(scope_key, watch::channel(latest_realtime_seq).0);
        self.persist_subscriptions_internal(
            snapshot.tenant_id.as_str(),
            snapshot.principal_id.as_str(),
            principal_kind,
            snapshot.device_id.as_str(),
        )?;
        self.persist_checkpoint_internal(
            snapshot.tenant_id.as_str(),
            snapshot.principal_id.as_str(),
            principal_kind,
            snapshot.device_id.as_str(),
        )?;

        Ok(())
    }

    // Scope fanout takes explicit addressing and payload fields because this is
    // the runtime's main delivery boundary and call sites benefit from keeping
    // the event identity fully visible.
    #[allow(clippy::too_many_arguments)]
    pub fn publish_scope_event(
        &self,
        tenant_id: &str,
        principal_id: &str,
        scope_type: &str,
        scope_id: &str,
        event_type: &str,
        payload: String,
        registered_devices: Vec<String>,
    ) -> Result<usize, RealtimeRuntimeError> {
        self.publish_scope_event_internal(
            tenant_id,
            principal_id,
            None,
            scope_type,
            scope_id,
            event_type,
            payload,
            registered_devices,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn publish_scope_event_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        scope_type: &str,
        scope_id: &str,
        event_type: &str,
        payload: String,
        registered_devices: Vec<String>,
    ) -> Result<usize, RealtimeRuntimeError> {
        self.publish_scope_event_internal(
            tenant_id,
            principal_id,
            Some(principal_kind),
            scope_type,
            scope_id,
            event_type,
            payload,
            registered_devices,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn publish_scope_event_internal(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: Option<&str>,
        scope_type: &str,
        scope_id: &str,
        event_type: &str,
        payload: String,
        registered_devices: Vec<String>,
    ) -> Result<usize, RealtimeRuntimeError> {
        let matched_targets = {
            let subscriptions =
                lock_realtime_mutex(&self.subscriptions, "realtime subscription store");
            collect_matched_delivery_targets(
                &subscriptions,
                tenant_id,
                principal_id,
                principal_kind,
                scope_type,
                scope_id,
                event_type,
                registered_devices,
            )
        };
        let mut windows = lock_realtime_mutex(&self.windows, "realtime window store");
        let mut latest_sequences =
            lock_realtime_mutex(&self.latest_sequences, "realtime sequence store");
        let mut delivered = 0;
        let mut notified = Vec::new();
        let mut persisted_devices = Vec::new();

        for (scope_key, device_id) in matched_targets {
            let next_seq = latest_sequences.entry(scope_key.clone()).or_insert(0);
            *next_seq += 1;
            windows
                .entry(scope_key.clone())
                .or_default()
                .push(RealtimeEvent {
                    tenant_id: tenant_id.into(),
                    principal_id: principal_id.into(),
                    device_id: device_id.clone(),
                    realtime_seq: *next_seq,
                    scope_type: scope_type.into(),
                    scope_id: scope_id.into(),
                    event_type: event_type.into(),
                    delivery_class: "ephemeral".into(),
                    payload: payload.clone(),
                    occurred_at: realtime_timestamp(),
                });
            delivered += 1;
            notified.push((scope_key, *next_seq));
            persisted_devices.push(device_id);
        }

        drop(latest_sequences);
        drop(windows);
        let mut notifiers = lock_realtime_mutex(&self.notifiers, "realtime notifier store");
        for (scope_key, next_seq) in notified {
            let sender = notifiers.entry(scope_key).or_insert_with(|| {
                let (sender, _) = watch::channel(0);
                sender
            });
            let _ = sender.send(next_seq);
        }
        drop(notifiers);
        for device_id in persisted_devices {
            self.persist_checkpoint_internal(
                tenant_id,
                principal_id,
                principal_kind,
                device_id.as_str(),
            )?;
        }

        Ok(delivered)
    }
}

pub(super) fn lock_realtime_mutex<'a, T>(
    mutex: &'a Mutex<T>,
    lock_name: &'static str,
) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("warn: recovered poisoned realtime runtime mutex lock={lock_name}");
            poisoned.into_inner()
        }
    }
}

fn device_scope_key(
    tenant_id: &str,
    principal_id: &str,
    principal_kind: Option<&str>,
    device_id: &str,
) -> String {
    match principal_kind {
        Some(principal_kind) => {
            typed_device_scope_key(tenant_id, principal_id, principal_kind, device_id)
        }
        None => actor_device_scope_key(tenant_id, principal_id, device_id),
    }
}

fn storage_principal_id(principal_id: &str, principal_kind: Option<&str>) -> String {
    match principal_kind {
        Some(principal_kind) => typed_principal_id(principal_id, principal_kind),
        None => principal_id.to_owned(),
    }
}

fn validate_payload_size(
    field: &'static str,
    payload: &str,
    max_bytes: usize,
) -> Result<(), RealtimeRuntimeError> {
    let actual_bytes = payload.len();
    if actual_bytes > max_bytes {
        return Err(RealtimeRuntimeError::payload_too_large(
            field,
            max_bytes,
            actual_bytes,
        ));
    }
    Ok(())
}

fn validate_realtime_subscription_items(
    items: &[RealtimeSubscriptionItemInput],
) -> Result<(), RealtimeRuntimeError> {
    if items.len() > REALTIME_MAX_SUBSCRIPTION_ITEMS {
        return Err(RealtimeRuntimeError::collection_too_large(
            "items",
            REALTIME_MAX_SUBSCRIPTION_ITEMS,
            items.len(),
        ));
    }
    for item in items {
        validate_payload_size(
            "scopeType",
            item.scope_type.as_str(),
            REALTIME_MAX_SCOPE_TYPE_BYTES,
        )?;
        validate_payload_size(
            "scopeId",
            item.scope_id.as_str(),
            REALTIME_MAX_SCOPE_ID_BYTES,
        )?;
        let event_types_total_bytes = item
            .event_types
            .iter()
            .fold(0usize, |total, event_type| total.saturating_add(event_type.len()));
        if event_types_total_bytes > REALTIME_MAX_EVENT_TYPES_TOTAL_BYTES {
            return Err(RealtimeRuntimeError::payload_too_large(
                "eventTypes",
                REALTIME_MAX_EVENT_TYPES_TOTAL_BYTES,
                event_types_total_bytes,
            ));
        }
        for event_type in &item.event_types {
            validate_payload_size(
                "eventTypes",
                event_type.as_str(),
                REALTIME_MAX_EVENT_TYPE_BYTES,
            )?;
        }
    }
    Ok(())
}

pub(crate) fn validate_realtime_event_limit(limit: usize) -> Result<(), RealtimeRuntimeError> {
    if limit > REALTIME_EVENT_WINDOW_MAX_LIMIT {
        return Err(RealtimeRuntimeError::limit_invalid(limit));
    }
    Ok(())
}

fn normalize_checkpoint_fields(
    latest_realtime_seq: u64,
    acked_through_seq: u64,
    trimmed_through_seq: u64,
) -> (u64, u64, u64) {
    let acked_through_seq = acked_through_seq.min(latest_realtime_seq);
    let trimmed_through_seq = trimmed_through_seq.min(acked_through_seq);
    (latest_realtime_seq, acked_through_seq, trimmed_through_seq)
}

fn normalize_window_events(
    events: Vec<RealtimeEvent>,
    trimmed_through_seq: u64,
) -> Vec<RealtimeEvent> {
    events
        .into_iter()
        .filter(|event| event.realtime_seq > trimmed_through_seq)
        .fold(BTreeMap::new(), |mut deduped, event| {
            deduped.insert(event.realtime_seq, event);
            deduped
        })
        .into_values()
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn collect_matched_delivery_targets(
    subscriptions: &HashMap<String, Vec<RealtimeSubscription>>,
    tenant_id: &str,
    principal_id: &str,
    principal_kind: Option<&str>,
    scope_type: &str,
    scope_id: &str,
    event_type: &str,
    registered_devices: Vec<String>,
) -> Vec<(String, String)> {
    registered_devices
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter_map(|device_id| {
            let scope_key =
                device_scope_key(tenant_id, principal_id, principal_kind, device_id.as_str());
            let matches_subscription =
                subscriptions
                    .get(scope_key.as_str())
                    .is_some_and(|device_subscriptions| {
                        device_subscriptions.iter().any(|subscription| {
                            subscription.scope_type == scope_type
                                && subscription.scope_id == scope_id
                                && (subscription.event_types.is_empty()
                                    || subscription
                                        .event_types
                                        .iter()
                                        .any(|item| item == event_type))
                        })
                    });
            matches_subscription.then_some((scope_key, device_id))
        })
        .collect()
}

fn realtime_timestamp() -> String {
    utc_now_rfc3339_millis()
}

fn realtime_checkpoint_timestamp() -> String {
    utc_now_rfc3339_millis()
}

fn subscriptions_synced_at(items: &[RealtimeSubscription]) -> String {
    items
        .iter()
        .map(|item| item.subscribed_at.as_str())
        .max()
        .map(str::to_owned)
        .unwrap_or_else(realtime_timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use im_adapters_local_memory::MemoryRealtimeCheckpointStore;

    #[test]
    fn test_collect_matched_delivery_targets_filters_to_registered_matching_devices() {
        let mut subscriptions = HashMap::new();
        subscriptions.insert(
            device_scope_key("t_demo", "u_demo", None, "d_match"),
            vec![subscription(
                "conversation",
                "c_demo",
                vec!["message.posted"],
            )],
        );
        subscriptions.insert(
            device_scope_key("t_demo", "u_demo", None, "d_other_scope"),
            vec![subscription(
                "conversation",
                "c_other",
                vec!["message.posted"],
            )],
        );
        subscriptions.insert(
            device_scope_key("t_demo", "u_demo", None, "d_other_event"),
            vec![subscription("conversation", "c_demo", vec!["message.read"])],
        );

        let matched = collect_matched_delivery_targets(
            &subscriptions,
            "t_demo",
            "u_demo",
            None,
            "conversation",
            "c_demo",
            "message.posted",
            vec![
                "d_other_scope".into(),
                "d_match".into(),
                "d_other_event".into(),
                "d_match".into(),
                "d_missing".into(),
            ],
        );

        assert_eq!(
            matched,
            vec![(
                device_scope_key("t_demo", "u_demo", None, "d_match"),
                "d_match".into()
            )]
        );
    }

    #[test]
    fn test_collect_matched_delivery_targets_accepts_wildcard_event_subscriptions() {
        let mut subscriptions = HashMap::new();
        subscriptions.insert(
            device_scope_key("t_demo", "u_demo", None, "d_wildcard"),
            vec![subscription("conversation", "c_demo", vec![])],
        );

        let matched = collect_matched_delivery_targets(
            &subscriptions,
            "t_demo",
            "u_demo",
            None,
            "conversation",
            "c_demo",
            "message.edited",
            vec!["d_wildcard".into()],
        );

        assert_eq!(
            matched,
            vec![(
                device_scope_key("t_demo", "u_demo", None, "d_wildcard"),
                "d_wildcard".into()
            )]
        );
    }

    #[test]
    fn test_persist_checkpoint_normalizes_transient_inconsistent_sequence_state() {
        let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
        let runtime = RealtimeDeliveryRuntime::with_checkpoint_store(checkpoint_store.clone());
        let scope_key = device_scope_key("t_demo", "u_demo", None, "d_pad");

        runtime
            .latest_sequences
            .lock()
            .expect("realtime sequence store should lock")
            .insert(scope_key.clone(), 3);
        runtime
            .acked_sequences
            .lock()
            .expect("realtime ack store should lock")
            .insert(scope_key.clone(), 9);
        runtime
            .trimmed_sequences
            .lock()
            .expect("realtime trim store should lock")
            .insert(scope_key, 11);

        runtime
            .persist_checkpoint("t_demo", "u_demo", "d_pad")
            .expect("checkpoint persist should succeed");

        let persisted = checkpoint_store
            .checkpoint("t_demo", "u_demo", "d_pad")
            .expect("checkpoint should be persisted");
        assert_eq!(persisted.latest_realtime_seq, 3);
        assert_eq!(persisted.acked_through_seq, 3);
        assert_eq!(persisted.trimmed_through_seq, 3);
    }

    #[test]
    fn test_ensure_device_state_recovers_from_poisoned_sequence_store_lock() {
        let runtime = RealtimeDeliveryRuntime::default();
        let _ = std::panic::catch_unwind({
            let latest_sequences = runtime.latest_sequences.clone();
            move || {
                let _guard = latest_sequences
                    .lock()
                    .expect("realtime sequence store should lock");
                panic!("poison realtime sequence store lock");
            }
        });

        runtime
            .ensure_device_state("t_demo", "u_demo", "d_poison")
            .expect("poisoned lock should be recovered");
        let checkpoint = runtime
            .window_checkpoint("t_demo", "u_demo", "d_poison")
            .expect("window checkpoint should still be available");
        assert_eq!(checkpoint.latest_realtime_seq, 0);
        assert_eq!(checkpoint.acked_through_seq, 0);
        assert_eq!(checkpoint.trimmed_through_seq, 0);
    }

    fn subscription(
        scope_type: &str,
        scope_id: &str,
        event_types: Vec<&str>,
    ) -> RealtimeSubscription {
        RealtimeSubscription {
            scope_type: scope_type.into(),
            scope_id: scope_id.into(),
            event_types: event_types.into_iter().map(|item| item.into()).collect(),
            subscribed_at: "2026-04-05T10:10:00Z".into(),
        }
    }
}
