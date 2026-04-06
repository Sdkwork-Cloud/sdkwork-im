use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;
use std::sync::{Arc, Mutex};

use im_domain_core::realtime::{
    RealtimeAckState, RealtimeEvent, RealtimeEventWindow, RealtimeSubscription,
    RealtimeSubscriptionSnapshot,
};
use im_platform_contracts::{
    ContractError, RealtimeCheckpointRecord, RealtimeCheckpointStore, RealtimeSubscriptionRecord,
    RealtimeSubscriptionStore,
};
use im_time::utc_now_rfc3339_millis;
use serde::Deserialize;
use tokio::sync::watch;

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

#[derive(Clone, Default)]
struct RuntimeMemoryCheckpointStore {
    checkpoints: Arc<Mutex<HashMap<String, RealtimeCheckpointRecord>>>,
}

impl RealtimeCheckpointStore for RuntimeMemoryCheckpointStore {
    fn load_checkpoint(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, im_platform_contracts::ContractError> {
        Ok(self
            .checkpoints
            .lock()
            .expect("runtime checkpoint store should lock")
            .get(device_scope_key(tenant_id, principal_id, device_id).as_str())
            .cloned())
    }

    fn save_checkpoint(
        &self,
        record: RealtimeCheckpointRecord,
    ) -> Result<(), im_platform_contracts::ContractError> {
        self.checkpoints
            .lock()
            .expect("runtime checkpoint store should lock")
            .insert(
                device_scope_key(
                    record.tenant_id.as_str(),
                    record.principal_id.as_str(),
                    record.device_id.as_str(),
                ),
                record,
            );
        Ok(())
    }
}

#[derive(Clone, Default)]
struct RuntimeMemorySubscriptionStore {
    subscriptions: Arc<Mutex<HashMap<String, RealtimeSubscriptionRecord>>>,
}

impl RealtimeSubscriptionStore for RuntimeMemorySubscriptionStore {
    fn load_subscriptions(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeSubscriptionRecord>, ContractError> {
        Ok(self
            .subscriptions
            .lock()
            .expect("runtime subscription store should lock")
            .get(device_scope_key(tenant_id, principal_id, device_id).as_str())
            .cloned())
    }

    fn save_subscriptions(&self, record: RealtimeSubscriptionRecord) -> Result<(), ContractError> {
        self.subscriptions
            .lock()
            .expect("runtime subscription store should lock")
            .insert(
                device_scope_key(
                    record.tenant_id.as_str(),
                    record.principal_id.as_str(),
                    record.device_id.as_str(),
                ),
                record,
            );
        Ok(())
    }

    fn clear_subscriptions(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        Ok(self
            .subscriptions
            .lock()
            .expect("runtime subscription store should lock")
            .remove(device_scope_key(tenant_id, principal_id, device_id).as_str())
            .is_some())
    }
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
        let scope_key = device_scope_key(tenant_id, principal_id, device_id);
        let needs_restore = !self
            .latest_sequences
            .lock()
            .expect("realtime sequence store should lock")
            .contains_key(scope_key.as_str());
        let restored = if needs_restore {
            self.checkpoint_store
                .load_checkpoint(tenant_id, principal_id, device_id)
                .map_err(RealtimeRuntimeError::checkpoint_store)?
        } else {
            None
        };
        let restored_subscriptions = if needs_restore {
            self.subscription_store
                .load_subscriptions(tenant_id, principal_id, device_id)
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

        self.windows
            .lock()
            .expect("realtime window store should lock")
            .entry(scope_key.clone())
            .or_default();
        if let Some(restored_subscriptions) =
            restored_subscriptions.filter(|record| !record.items.is_empty())
        {
            self.subscriptions
                .lock()
                .expect("realtime subscription store should lock")
                .entry(scope_key.clone())
                .or_insert(restored_subscriptions.items);
        }
        let latest_seq = {
            let mut latest_sequences = self
                .latest_sequences
                .lock()
                .expect("realtime sequence store should lock");
            *latest_sequences
                .entry(scope_key.clone())
                .or_insert_with(|| normalized_restored.map(|item| item.0).unwrap_or(0))
        };
        self.acked_sequences
            .lock()
            .expect("realtime ack store should lock")
            .entry(scope_key.clone())
            .or_insert_with(|| normalized_restored.map(|item| item.1).unwrap_or(0));
        self.trimmed_sequences
            .lock()
            .expect("realtime trim store should lock")
            .entry(scope_key.clone())
            .or_insert_with(|| normalized_restored.map(|item| item.2).unwrap_or(0));
        self.notifiers
            .lock()
            .expect("realtime notifier store should lock")
            .entry(scope_key.clone())
            .or_insert_with(|| {
                let (sender, _) = watch::channel(latest_seq);
                sender
            });
        let disconnect_generation = {
            let mut disconnect_generations = self
                .disconnect_generations
                .lock()
                .expect("realtime disconnect generation store should lock");
            *disconnect_generations.entry(scope_key.clone()).or_insert(0)
        };
        self.disconnect_notifiers
            .lock()
            .expect("realtime disconnect notifier store should lock")
            .entry(scope_key)
            .or_insert_with(|| {
                let (sender, _) = watch::channel(disconnect_generation);
                sender
            });
        if checkpoint_needs_normalization {
            self.persist_checkpoint(tenant_id, principal_id, device_id)?;
        }

        Ok(())
    }

    pub fn subscribe_device(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<watch::Receiver<u64>, RealtimeRuntimeError> {
        self.ensure_device_state(tenant_id, principal_id, device_id)?;
        let scope_key = device_scope_key(tenant_id, principal_id, device_id);
        Ok(self
            .notifiers
            .lock()
            .expect("realtime notifier store should lock")
            .get(scope_key.as_str())
            .expect("realtime notifier should exist after ensure")
            .subscribe())
    }

    pub fn subscribe_disconnect_signal(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<watch::Receiver<u64>, RealtimeRuntimeError> {
        self.ensure_device_state(tenant_id, principal_id, device_id)?;
        let scope_key = device_scope_key(tenant_id, principal_id, device_id);
        Ok(self
            .disconnect_notifiers
            .lock()
            .expect("realtime disconnect notifier store should lock")
            .get(scope_key.as_str())
            .expect("realtime disconnect notifier should exist after ensure")
            .subscribe())
    }

    pub fn disconnect_generation(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<u64, RealtimeRuntimeError> {
        self.ensure_device_state(tenant_id, principal_id, device_id)?;
        Ok(self
            .disconnect_generations
            .lock()
            .expect("realtime disconnect generation store should lock")
            .get(device_scope_key(tenant_id, principal_id, device_id).as_str())
            .copied()
            .unwrap_or(0))
    }

    pub fn signal_device_disconnect(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        self.ensure_device_state(tenant_id, principal_id, device_id)?;
        let scope_key = device_scope_key(tenant_id, principal_id, device_id);
        let next_generation = {
            let mut disconnect_generations = self
                .disconnect_generations
                .lock()
                .expect("realtime disconnect generation store should lock");
            let entry = disconnect_generations.entry(scope_key.clone()).or_insert(0);
            *entry += 1;
            *entry
        };
        if let Some(sender) = self
            .disconnect_notifiers
            .lock()
            .expect("realtime disconnect notifier store should lock")
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
        self.ensure_device_state(tenant_id, principal_id, device_id)?;
        let scope_key = device_scope_key(tenant_id, principal_id, device_id);
        Ok(RealtimeWindowCheckpoint {
            latest_realtime_seq: self
                .latest_sequences
                .lock()
                .expect("realtime sequence store should lock")
                .get(scope_key.as_str())
                .copied()
                .unwrap_or(0),
            acked_through_seq: self
                .acked_sequences
                .lock()
                .expect("realtime ack store should lock")
                .get(scope_key.as_str())
                .copied()
                .unwrap_or(0),
            trimmed_through_seq: self
                .trimmed_sequences
                .lock()
                .expect("realtime trim store should lock")
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
        let synced_at = realtime_timestamp();
        self.ensure_device_state(tenant_id, principal_id, device_id)?;
        let subscriptions = items
            .into_iter()
            .map(|item| RealtimeSubscription {
                scope_type: item.scope_type,
                scope_id: item.scope_id,
                event_types: item.event_types,
                subscribed_at: synced_at.clone(),
            })
            .collect::<Vec<_>>();
        let scope_key = device_scope_key(tenant_id, principal_id, device_id);

        self.subscriptions
            .lock()
            .expect("realtime subscription store should lock")
            .insert(scope_key.clone(), subscriptions.clone());
        self.persist_subscriptions(tenant_id, principal_id, device_id)?;

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
        let scope_key = device_scope_key(tenant_id, principal_id, device_id);
        self.subscriptions
            .lock()
            .expect("realtime subscription store should lock")
            .remove(scope_key.as_str());
        self.persist_subscriptions(tenant_id, principal_id, device_id)
    }

    pub fn list_events(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        after_seq: u64,
        limit: usize,
    ) -> Result<RealtimeEventWindow, RealtimeRuntimeError> {
        self.ensure_device_state(tenant_id, principal_id, device_id)?;
        let scope_key = device_scope_key(tenant_id, principal_id, device_id);
        let acked_through_seq = self
            .acked_sequences
            .lock()
            .expect("realtime ack store should lock")
            .get(scope_key.as_str())
            .copied()
            .unwrap_or(0);
        let trimmed_through_seq = self
            .trimmed_sequences
            .lock()
            .expect("realtime trim store should lock")
            .get(scope_key.as_str())
            .copied()
            .unwrap_or(0);
        let windows = self
            .windows
            .lock()
            .expect("realtime window store should lock");
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
        self.ensure_device_state(tenant_id, principal_id, device_id)?;
        let scope_key = device_scope_key(tenant_id, principal_id, device_id);
        let latest_seq = self
            .latest_sequences
            .lock()
            .expect("realtime sequence store should lock")
            .get(scope_key.as_str())
            .copied()
            .unwrap_or(0);
        let effective_ack = acked_seq.min(latest_seq);
        let acked_through_seq = {
            let mut acked_sequences = self
                .acked_sequences
                .lock()
                .expect("realtime ack store should lock");
            let entry = acked_sequences.entry(scope_key.clone()).or_insert(0);
            if effective_ack > *entry {
                *entry = effective_ack;
            }
            *entry
        };

        let (trimmed_through_seq, retained_event_count) = {
            let mut windows = self
                .windows
                .lock()
                .expect("realtime window store should lock");
            let mut trimmed_sequences = self
                .trimmed_sequences
                .lock()
                .expect("realtime trim store should lock");
            let trimmed_entry = trimmed_sequences.entry(scope_key.clone()).or_insert(0);
            if acked_through_seq > *trimmed_entry {
                *trimmed_entry = acked_through_seq;
            }
            let items = windows.entry(scope_key).or_default();
            items.retain(|item| item.realtime_seq > acked_through_seq);
            (*trimmed_entry, items.len())
        };
        self.persist_checkpoint(tenant_id, principal_id, device_id)?;

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
        self.ensure_device_state(tenant_id, principal_id, device_id)?;
        let scope_key = device_scope_key(tenant_id, principal_id, device_id);
        let subscriptions = self
            .subscriptions
            .lock()
            .expect("realtime subscription store should lock")
            .remove(scope_key.as_str())
            .unwrap_or_default();
        let events = self
            .windows
            .lock()
            .expect("realtime window store should lock")
            .remove(scope_key.as_str())
            .unwrap_or_default();
        let latest_realtime_seq = self
            .latest_sequences
            .lock()
            .expect("realtime sequence store should lock")
            .remove(scope_key.as_str())
            .unwrap_or(0);
        let acked_through_seq = self
            .acked_sequences
            .lock()
            .expect("realtime ack store should lock")
            .remove(scope_key.as_str())
            .unwrap_or(0);
        let trimmed_through_seq = self
            .trimmed_sequences
            .lock()
            .expect("realtime trim store should lock")
            .remove(scope_key.as_str())
            .unwrap_or(0);
        self.notifiers
            .lock()
            .expect("realtime notifier store should lock")
            .remove(scope_key.as_str());

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
            snapshot.device_id.as_str(),
        );

        self.subscriptions
            .lock()
            .expect("realtime subscription store should lock")
            .insert(scope_key.clone(), snapshot.subscriptions);
        self.windows
            .lock()
            .expect("realtime window store should lock")
            .insert(scope_key.clone(), normalized_events);
        self.latest_sequences
            .lock()
            .expect("realtime sequence store should lock")
            .insert(scope_key.clone(), latest_realtime_seq);
        self.acked_sequences
            .lock()
            .expect("realtime ack store should lock")
            .insert(scope_key.clone(), acked_through_seq);
        self.trimmed_sequences
            .lock()
            .expect("realtime trim store should lock")
            .insert(scope_key.clone(), trimmed_through_seq);
        self.notifiers
            .lock()
            .expect("realtime notifier store should lock")
            .insert(scope_key, watch::channel(latest_realtime_seq).0);
        self.persist_subscriptions(
            snapshot.tenant_id.as_str(),
            snapshot.principal_id.as_str(),
            snapshot.device_id.as_str(),
        )?;
        self.persist_checkpoint(
            snapshot.tenant_id.as_str(),
            snapshot.principal_id.as_str(),
            snapshot.device_id.as_str(),
        )?;

        Ok(())
    }

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
        let matched_targets = {
            let subscriptions = self
                .subscriptions
                .lock()
                .expect("realtime subscription store should lock");
            collect_matched_delivery_targets(
                &subscriptions,
                tenant_id,
                principal_id,
                scope_type,
                scope_id,
                event_type,
                registered_devices,
            )
        };
        let mut windows = self
            .windows
            .lock()
            .expect("realtime window store should lock");
        let mut latest_sequences = self
            .latest_sequences
            .lock()
            .expect("realtime sequence store should lock");
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
        let mut notifiers = self
            .notifiers
            .lock()
            .expect("realtime notifier store should lock");
        for (scope_key, next_seq) in notified {
            let sender = notifiers.entry(scope_key).or_insert_with(|| {
                let (sender, _) = watch::channel(0);
                sender
            });
            let _ = sender.send(next_seq);
        }
        drop(notifiers);
        for device_id in persisted_devices {
            self.persist_checkpoint(tenant_id, principal_id, device_id.as_str())?;
        }

        Ok(delivered)
    }

    fn persist_checkpoint(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        self.checkpoint_store
            .save_checkpoint(self.checkpoint_record(tenant_id, principal_id, device_id))
            .map_err(RealtimeRuntimeError::checkpoint_store)
    }

    fn persist_subscriptions(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        match self.subscription_record(tenant_id, principal_id, device_id) {
            Some(record) => self
                .subscription_store
                .save_subscriptions(record)
                .map_err(RealtimeRuntimeError::subscription_store),
            None => self
                .subscription_store
                .clear_subscriptions(tenant_id, principal_id, device_id)
                .map(|_| ())
                .map_err(RealtimeRuntimeError::subscription_store),
        }
    }

    fn checkpoint_record(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> RealtimeCheckpointRecord {
        let scope_key = device_scope_key(tenant_id, principal_id, device_id);
        let latest_realtime_seq = self
            .latest_sequences
            .lock()
            .expect("realtime sequence store should lock")
            .get(scope_key.as_str())
            .copied()
            .unwrap_or(0);
        let acked_through_seq = self
            .acked_sequences
            .lock()
            .expect("realtime ack store should lock")
            .get(scope_key.as_str())
            .copied()
            .unwrap_or(0);
        let trimmed_through_seq = self
            .trimmed_sequences
            .lock()
            .expect("realtime trim store should lock")
            .get(scope_key.as_str())
            .copied()
            .unwrap_or(0);
        let (latest_realtime_seq, acked_through_seq, trimmed_through_seq) =
            normalize_checkpoint_fields(
                latest_realtime_seq,
                acked_through_seq,
                trimmed_through_seq,
            );
        RealtimeCheckpointRecord {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            device_id: device_id.into(),
            latest_realtime_seq,
            acked_through_seq,
            trimmed_through_seq,
            updated_at: realtime_checkpoint_timestamp(),
        }
    }

    fn subscription_record(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Option<RealtimeSubscriptionRecord> {
        let scope_key = device_scope_key(tenant_id, principal_id, device_id);
        let items = self
            .subscriptions
            .lock()
            .expect("realtime subscription store should lock")
            .get(scope_key.as_str())
            .cloned()
            .unwrap_or_default();
        if items.is_empty() {
            return None;
        }

        Some(RealtimeSubscriptionRecord {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            device_id: device_id.into(),
            synced_at: subscriptions_synced_at(items.as_slice()),
            items,
        })
    }
}

fn device_scope_key(tenant_id: &str, principal_id: &str, device_id: &str) -> String {
    format!("{tenant_id}:{principal_id}:{device_id}")
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

fn collect_matched_delivery_targets(
    subscriptions: &HashMap<String, Vec<RealtimeSubscription>>,
    tenant_id: &str,
    principal_id: &str,
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
            let scope_key = device_scope_key(tenant_id, principal_id, device_id.as_str());
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
            device_scope_key("t_demo", "u_demo", "d_match"),
            vec![subscription(
                "conversation",
                "c_demo",
                vec!["message.posted"],
            )],
        );
        subscriptions.insert(
            device_scope_key("t_demo", "u_demo", "d_other_scope"),
            vec![subscription(
                "conversation",
                "c_other",
                vec!["message.posted"],
            )],
        );
        subscriptions.insert(
            device_scope_key("t_demo", "u_demo", "d_other_event"),
            vec![subscription("conversation", "c_demo", vec!["message.read"])],
        );

        let matched = collect_matched_delivery_targets(
            &subscriptions,
            "t_demo",
            "u_demo",
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
                device_scope_key("t_demo", "u_demo", "d_match"),
                "d_match".into()
            )]
        );
    }

    #[test]
    fn test_collect_matched_delivery_targets_accepts_wildcard_event_subscriptions() {
        let mut subscriptions = HashMap::new();
        subscriptions.insert(
            device_scope_key("t_demo", "u_demo", "d_wildcard"),
            vec![subscription("conversation", "c_demo", vec![])],
        );

        let matched = collect_matched_delivery_targets(
            &subscriptions,
            "t_demo",
            "u_demo",
            "conversation",
            "c_demo",
            "message.edited",
            vec!["d_wildcard".into()],
        );

        assert_eq!(
            matched,
            vec![(
                device_scope_key("t_demo", "u_demo", "d_wildcard"),
                "d_wildcard".into()
            )]
        );
    }

    #[test]
    fn test_persist_checkpoint_normalizes_transient_inconsistent_sequence_state() {
        let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
        let runtime = RealtimeDeliveryRuntime::with_checkpoint_store(checkpoint_store.clone());
        let scope_key = device_scope_key("t_demo", "u_demo", "d_pad");

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
