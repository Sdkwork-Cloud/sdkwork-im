use craw_chat_contract_control::{
    RealtimeCheckpointRecord, RealtimeCheckpointStore, RealtimeSubscriptionRecord,
    RealtimeSubscriptionStore,
};
use craw_chat_contract_core::ContractError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::{
    RealtimeDeliveryRuntime, RealtimeRuntimeError, device_scope_key, lock_realtime_mutex,
    normalize_checkpoint_fields, realtime_checkpoint_timestamp, subscriptions_synced_at,
};

#[derive(Clone, Default)]
pub(super) struct RuntimeMemoryCheckpointStore {
    checkpoints: Arc<Mutex<HashMap<String, RealtimeCheckpointRecord>>>,
}

impl RealtimeCheckpointStore for RuntimeMemoryCheckpointStore {
    fn load_checkpoint(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        Ok(
            lock_realtime_mutex(&self.checkpoints, "runtime checkpoint store")
                .get(device_scope_key(tenant_id, principal_id, device_id).as_str())
                .cloned(),
        )
    }

    fn save_checkpoint(&self, record: RealtimeCheckpointRecord) -> Result<(), ContractError> {
        lock_realtime_mutex(&self.checkpoints, "runtime checkpoint store").insert(
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
pub(super) struct RuntimeMemorySubscriptionStore {
    subscriptions: Arc<Mutex<HashMap<String, RealtimeSubscriptionRecord>>>,
}

impl RealtimeSubscriptionStore for RuntimeMemorySubscriptionStore {
    fn load_subscriptions(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeSubscriptionRecord>, ContractError> {
        Ok(
            lock_realtime_mutex(&self.subscriptions, "runtime subscription store")
                .get(device_scope_key(tenant_id, principal_id, device_id).as_str())
                .cloned(),
        )
    }

    fn save_subscriptions(&self, record: RealtimeSubscriptionRecord) -> Result<(), ContractError> {
        lock_realtime_mutex(&self.subscriptions, "runtime subscription store").insert(
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
        Ok(
            lock_realtime_mutex(&self.subscriptions, "runtime subscription store")
                .remove(device_scope_key(tenant_id, principal_id, device_id).as_str())
                .is_some(),
        )
    }
}

impl RealtimeDeliveryRuntime {
    pub(super) fn persist_checkpoint(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        self.checkpoint_store
            .save_checkpoint(self.checkpoint_record(tenant_id, principal_id, device_id))
            .map_err(RealtimeRuntimeError::checkpoint_store)
    }

    pub(super) fn persist_subscriptions(
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
        let latest_realtime_seq =
            lock_realtime_mutex(&self.latest_sequences, "realtime sequence store")
                .get(scope_key.as_str())
                .copied()
                .unwrap_or(0);
        let acked_through_seq = lock_realtime_mutex(&self.acked_sequences, "realtime ack store")
            .get(scope_key.as_str())
            .copied()
            .unwrap_or(0);
        let trimmed_through_seq =
            lock_realtime_mutex(&self.trimmed_sequences, "realtime trim store")
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
        let items = lock_realtime_mutex(&self.subscriptions, "realtime subscription store")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_checkpoint_store_load_recovers_from_poisoned_lock() {
        let store = RuntimeMemoryCheckpointStore::default();
        let _ = std::panic::catch_unwind({
            let checkpoints = store.checkpoints.clone();
            move || {
                let _guard = checkpoints
                    .lock()
                    .expect("runtime checkpoint store should lock");
                panic!("poison runtime checkpoint store lock");
            }
        });

        let checkpoint = store
            .load_checkpoint("t_demo", "u_demo", "d_poison")
            .expect("poisoned lock should be recovered");
        assert!(checkpoint.is_none());
    }
}
