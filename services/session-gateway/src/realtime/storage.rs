use craw_chat_contract_control::{
    RealtimeCheckpointRecord, RealtimeCheckpointStore, RealtimeSubscriptionRecord,
    RealtimeSubscriptionStore,
};
use craw_chat_contract_core::ContractError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::{
    RealtimeDeliveryRuntime, RealtimeRuntimeError, device_scope_key, normalize_checkpoint_fields,
    realtime_checkpoint_timestamp, subscriptions_synced_at,
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
        Ok(self
            .checkpoints
            .lock()
            .expect("runtime checkpoint store should lock")
            .get(device_scope_key(tenant_id, principal_id, device_id).as_str())
            .cloned())
    }

    fn save_checkpoint(&self, record: RealtimeCheckpointRecord) -> Result<(), ContractError> {
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
