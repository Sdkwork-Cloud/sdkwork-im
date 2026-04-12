use craw_chat_contract_control::{RealtimeDisconnectFenceRecord, RealtimeDisconnectFenceStore};
use craw_chat_contract_core::ContractError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::{RealtimeClusterBridge, RealtimeClusterError, cluster_timestamp, device_scope_key};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct RealtimeDisconnectFence {
    tenant_id: String,
    principal_id: String,
    device_id: String,
    session_id: Option<String>,
    owner_node_id: String,
    disconnected_at: String,
}

#[derive(Clone, Default)]
pub(super) struct ClusterMemoryDisconnectFenceStore {
    fences: Arc<Mutex<HashMap<String, RealtimeDisconnectFenceRecord>>>,
}

impl RealtimeDisconnectFenceStore for ClusterMemoryDisconnectFenceStore {
    fn load_fence(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeDisconnectFenceRecord>, ContractError> {
        Ok(self
            .fences
            .lock_cluster_disconnect_fences()
            .get(device_scope_key(tenant_id, principal_id, device_id).as_str())
            .cloned())
    }

    fn save_fence(&self, record: RealtimeDisconnectFenceRecord) -> Result<(), ContractError> {
        self.fences.lock_cluster_disconnect_fences().insert(
            device_scope_key(
                record.tenant_id.as_str(),
                record.principal_id.as_str(),
                record.device_id.as_str(),
            ),
            record,
        );
        Ok(())
    }

    fn clear_fence(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        Ok(self
            .fences
            .lock_cluster_disconnect_fences()
            .remove(device_scope_key(tenant_id, principal_id, device_id).as_str())
            .is_some())
    }
}

impl RealtimeClusterBridge {
    pub fn mark_device_disconnected(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        session_id: Option<&str>,
        owner_node_id: &str,
    ) -> Result<(), RealtimeClusterError> {
        let scope_key = device_scope_key(tenant_id, principal_id, device_id);
        let fence = RealtimeDisconnectFence {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            device_id: device_id.into(),
            session_id: session_id.map(str::to_owned),
            owner_node_id: owner_node_id.into(),
            disconnected_at: cluster_timestamp(),
        };
        self.disconnect_fence_store
            .save_fence(fence.to_record())
            .map_err(|error| {
                self.disconnect_fence_store_error("persist disconnect fence", owner_node_id, error)
            })?;
        self.disconnect_fences
            .lock_cluster_disconnect_fence_cache()
            .insert(scope_key, fence);
        Ok(())
    }

    pub fn clear_device_disconnect_fence(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, RealtimeClusterError> {
        let persisted_removed = self
            .disconnect_fence_store
            .clear_fence(tenant_id, principal_id, device_id)
            .map_err(|error| {
                self.disconnect_fence_store_error("clear disconnect fence", "storage", error)
            })?;
        let removed = self
            .disconnect_fences
            .lock_cluster_disconnect_fence_cache()
            .remove(device_scope_key(tenant_id, principal_id, device_id).as_str())
            .is_some();
        Ok(removed || persisted_removed)
    }

    pub fn ensure_device_resume_not_required(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<(), RealtimeClusterError> {
        let Some(fence) = self.load_disconnect_fence(tenant_id, principal_id, device_id)? else {
            return Ok(());
        };
        Err(self.node_error(
            "reconnect_required",
            fence.owner_node_id.as_str(),
            format!(
                "device must resume a fresh session before continuing after disconnect on node {}",
                fence.owner_node_id
            ),
        ))
    }

    pub fn disconnect_fence_matches_session(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        session_id: Option<&str>,
    ) -> Result<bool, RealtimeClusterError> {
        Ok(self
            .load_disconnect_fence(tenant_id, principal_id, device_id)?
            .as_ref()
            .map(|fence| fence.session_id.as_deref() == session_id)
            .unwrap_or(false))
    }

    fn load_disconnect_fence(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeDisconnectFence>, RealtimeClusterError> {
        let scope_key = device_scope_key(tenant_id, principal_id, device_id);
        if let Some(fence) = self
            .disconnect_fences
            .lock_cluster_disconnect_fence_cache()
            .get(scope_key.as_str())
            .cloned()
        {
            return Ok(Some(fence));
        }

        let restored = self
            .disconnect_fence_store
            .load_fence(tenant_id, principal_id, device_id)
            .map_err(|error| {
                self.disconnect_fence_store_error("load disconnect fence", "storage", error)
            })?
            .map(RealtimeDisconnectFence::from_record);
        if let Some(fence) = restored.as_ref() {
            self.disconnect_fences
                .lock_cluster_disconnect_fence_cache()
                .insert(scope_key, fence.clone());
        }
        Ok(restored)
    }

    fn disconnect_fence_store_error(
        &self,
        action: &str,
        node_id: &str,
        error: ContractError,
    ) -> RealtimeClusterError {
        self.node_error(
            "disconnect_fence_store_unavailable",
            node_id,
            format!("{action} failed: {error:?}"),
        )
    }
}

impl RealtimeDisconnectFence {
    fn to_record(&self) -> RealtimeDisconnectFenceRecord {
        RealtimeDisconnectFenceRecord {
            tenant_id: self.tenant_id.clone(),
            principal_id: self.principal_id.clone(),
            device_id: self.device_id.clone(),
            session_id: self.session_id.clone(),
            owner_node_id: self.owner_node_id.clone(),
            disconnected_at: self.disconnected_at.clone(),
        }
    }

    fn from_record(record: RealtimeDisconnectFenceRecord) -> Self {
        Self {
            tenant_id: record.tenant_id,
            principal_id: record.principal_id,
            device_id: record.device_id,
            session_id: record.session_id,
            owner_node_id: record.owner_node_id,
            disconnected_at: record.disconnected_at,
        }
    }
}

trait ClusterDisconnectMutexExt<T> {
    fn lock_cluster_disconnect_fences(&self) -> std::sync::MutexGuard<'_, T>;
}

impl<T> ClusterDisconnectMutexExt<T> for Mutex<T> {
    fn lock_cluster_disconnect_fences(&self) -> std::sync::MutexGuard<'_, T> {
        super::lock_cluster_mutex(self, "disconnect_fence_store")
    }
}

trait ClusterDisconnectCacheMutexExt<T> {
    fn lock_cluster_disconnect_fence_cache(&self) -> std::sync::MutexGuard<'_, T>;
}

impl<T> ClusterDisconnectCacheMutexExt<T> for Mutex<T> {
    fn lock_cluster_disconnect_fence_cache(&self) -> std::sync::MutexGuard<'_, T> {
        super::lock_cluster_mutex(self, "disconnect_fence_cache")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{self, AssertUnwindSafe};

    fn poison_mutex<T>(mutex: &Mutex<T>) {
        let _ = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = mutex.lock().expect("test poison lock should succeed");
            panic!("intentional poison for regression coverage");
        }));
    }

    #[test]
    fn test_disconnect_fence_store_load_recovers_from_poisoned_lock() {
        let store = ClusterMemoryDisconnectFenceStore::default();
        poison_mutex(&store.fences);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            store.load_fence("t_demo", "u_demo", "d_demo")
        }));
        assert!(
            result.is_ok(),
            "disconnect fence store load should not panic when lock is poisoned"
        );
        let load_result = result.expect("panic status should be captured");
        assert!(load_result.is_ok());
    }

    #[test]
    fn test_mark_device_disconnected_recovers_from_poisoned_disconnect_cache_lock() {
        let cluster = RealtimeClusterBridge::default();
        cluster.bind_node_runtime(
            "node_a",
            std::sync::Arc::new(crate::RealtimeDeliveryRuntime::default()),
        );
        poison_mutex(&cluster.disconnect_fences);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            cluster.mark_device_disconnected("t_demo", "u_demo", "d_demo", Some("s_demo"), "node_a")
        }));
        assert!(
            result.is_ok(),
            "mark_device_disconnected should not panic when disconnect cache lock is poisoned"
        );
        let mark_result = result.expect("panic status should be captured");
        assert!(mark_result.is_ok());
    }
}
