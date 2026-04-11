use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, Mutex, MutexGuard};

use im_auth_context::AuthContext;

use super::ApiError;
use super::presence::{device_scope_key, principal_scope_key};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DeviceSyncSessionState {
    pub(crate) registered_devices: Vec<String>,
    pub(crate) latest_sync_seq: u64,
}

#[derive(Clone, Default)]
pub(crate) struct SessionSyncState {
    registered_devices: Arc<Mutex<HashMap<String, BTreeSet<String>>>>,
    latest_sync_sequences: Arc<Mutex<HashMap<String, u64>>>,
}

impl SessionSyncState {
    pub(crate) fn register_device(&self, tenant_id: &str, principal_id: &str, device_id: &str) {
        lock_session_sync_mutex(&self.registered_devices, "registered device store")
            .entry(principal_scope_key(tenant_id, principal_id))
            .or_default()
            .insert(device_id.into());
        lock_session_sync_mutex(&self.latest_sync_sequences, "latest sync sequence store")
            .entry(device_scope_key(tenant_id, principal_id, device_id))
            .or_insert(0);
    }

    pub(crate) fn has_registered_device(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> bool {
        lock_session_sync_mutex(&self.registered_devices, "registered device store")
            .get(principal_scope_key(tenant_id, principal_id).as_str())
            .is_some_and(|items| items.contains(device_id))
    }

    pub(crate) fn device_sync_session_state(
        &self,
        auth: &AuthContext,
        requested_device_id: Option<&str>,
    ) -> Result<DeviceSyncSessionState, ApiError> {
        if let (Some(requested), Some(bound)) = (requested_device_id, auth.device_id.as_deref())
            && requested != bound
        {
            return Err(ApiError::bad_request(
                "device_id_mismatch",
                format!("device id does not match auth context: {requested}"),
            ));
        }

        let latest_sync_seq = requested_device_id
            .or(auth.device_id.as_deref())
            .map(|device_id| {
                self.latest_device_sync_seq(
                    auth.tenant_id.as_str(),
                    auth.actor_id.as_str(),
                    device_id,
                )
            })
            .unwrap_or_default();

        Ok(DeviceSyncSessionState {
            registered_devices: self
                .registered_devices(auth.tenant_id.as_str(), auth.actor_id.as_str()),
            latest_sync_seq,
        })
    }

    fn registered_devices(&self, tenant_id: &str, principal_id: &str) -> Vec<String> {
        lock_session_sync_mutex(&self.registered_devices, "registered device store")
            .get(principal_scope_key(tenant_id, principal_id).as_str())
            .map(|items| items.iter().cloned().collect())
            .unwrap_or_default()
    }

    fn latest_device_sync_seq(&self, tenant_id: &str, principal_id: &str, device_id: &str) -> u64 {
        lock_session_sync_mutex(&self.latest_sync_sequences, "latest sync sequence store")
            .get(device_scope_key(tenant_id, principal_id, device_id).as_str())
            .copied()
            .unwrap_or_default()
    }
}

fn lock_session_sync_mutex<'a, T>(
    mutex: &'a Mutex<T>,
    lock_name: &'static str,
) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("warn: recovered poisoned session-sync mutex lock={lock_name}");
            poisoned.into_inner()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_device_recovers_from_poisoned_registered_device_lock() {
        let state = SessionSyncState::default();
        let _ = std::panic::catch_unwind({
            let registered_devices = state.registered_devices.clone();
            move || {
                let _guard = registered_devices
                    .lock()
                    .expect("registered device store should lock");
                panic!("poison registered device store lock");
            }
        });

        state.register_device("t_demo", "u_demo", "d_poison");
        assert!(state.has_registered_device("t_demo", "u_demo", "d_poison"));
    }
}
