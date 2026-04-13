use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, Mutex, MutexGuard};

use im_auth_context::AuthContext;

use super::ApiError;
use super::principal_scope::{
    tenant_device_scope_key, typed_device_scope_key, typed_principal_scope_key,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DeviceSyncSessionState {
    pub(crate) registered_devices: Vec<String>,
    pub(crate) latest_sync_seq: u64,
}

#[derive(Clone, Default)]
pub(crate) struct SessionSyncState {
    registered_devices: Arc<Mutex<HashMap<String, BTreeSet<String>>>>,
    latest_sync_sequences: Arc<Mutex<HashMap<String, u64>>>,
    device_owner_scopes: Arc<Mutex<HashMap<String, String>>>,
}

impl SessionSyncState {
    pub(crate) fn ensure_device_kind_available(
        &self,
        auth: &AuthContext,
        device_id: &str,
    ) -> Result<(), ApiError> {
        // Device IDs are principal-scoped for sync state, but must remain tenant-global
        // at registration time so the same external device cannot be rebound to another owner.
        let scope_key = tenant_device_scope_key(auth.tenant_id.as_str(), device_id);
        let requested_owner_scope = typed_principal_scope_key(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        );
        if let Some(existing_owner_scope) =
            lock_session_sync_mutex(&self.device_owner_scopes, "registered device owner store")
                .get(scope_key.as_str())
                .cloned()
            && existing_owner_scope != requested_owner_scope
        {
            return Err(ApiError::conflict(
                "device_scope_conflict",
                format!("device scope already bound to a different principal: {device_id}"),
            ));
        }

        Ok(())
    }

    pub(crate) fn register_device(&self, auth: &AuthContext, device_id: &str) {
        lock_session_sync_mutex(&self.registered_devices, "registered device store")
            .entry(typed_principal_scope_key(
                auth.tenant_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
            ))
            .or_default()
            .insert(device_id.into());
        lock_session_sync_mutex(&self.latest_sync_sequences, "latest sync sequence store")
            .entry(typed_device_scope_key(
                auth.tenant_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                device_id,
            ))
            .or_insert(0);
        lock_session_sync_mutex(&self.device_owner_scopes, "registered device owner store")
            .entry(tenant_device_scope_key(auth.tenant_id.as_str(), device_id))
            .or_insert_with(|| {
                typed_principal_scope_key(
                    auth.tenant_id.as_str(),
                    auth.actor_id.as_str(),
                    auth.actor_kind.as_str(),
                )
            });
    }

    pub(crate) fn has_registered_device(&self, auth: &AuthContext, device_id: &str) -> bool {
        lock_session_sync_mutex(&self.registered_devices, "registered device store")
            .get(
                typed_principal_scope_key(
                    auth.tenant_id.as_str(),
                    auth.actor_id.as_str(),
                    auth.actor_kind.as_str(),
                )
                .as_str(),
            )
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
                    auth.actor_kind.as_str(),
                    device_id,
                )
            })
            .unwrap_or_default();

        Ok(DeviceSyncSessionState {
            registered_devices: self.registered_devices(
                auth.tenant_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
            ),
            latest_sync_seq,
        })
    }

    fn registered_devices(&self, tenant_id: &str, principal_id: &str, principal_kind: &str) -> Vec<String> {
        lock_session_sync_mutex(&self.registered_devices, "registered device store")
            .get(typed_principal_scope_key(tenant_id, principal_id, principal_kind).as_str())
            .map(|items| items.iter().cloned().collect())
            .unwrap_or_default()
    }

    fn latest_device_sync_seq(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> u64 {
        lock_session_sync_mutex(&self.latest_sync_sequences, "latest sync sequence store")
            .get(
                typed_device_scope_key(tenant_id, principal_id, principal_kind, device_id).as_str(),
            )
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

    fn auth_context(principal_id: &str, actor_kind: &str, device_id: &str) -> AuthContext {
        AuthContext {
            tenant_id: "t_demo".into(),
            actor_id: principal_id.into(),
            actor_kind: actor_kind.into(),
            session_id: Some(format!("s_{actor_kind}")),
            device_id: Some(device_id.into()),
            permissions: Default::default(),
        }
    }

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

        let auth = auth_context("u_demo", "user", "d_poison");
        state.register_device(&auth, "d_poison");
        assert!(state.has_registered_device(&auth, "d_poison"));
    }

    #[test]
    fn test_device_sync_state_isolated_by_actor_kind_for_same_actor_id() {
        let state = SessionSyncState::default();
        let user_auth = auth_context("u_demo", "user", "d_user");
        let agent_auth = auth_context("u_demo", "agent", "d_agent");

        state.register_device(&user_auth, "d_user");
        state.register_device(&agent_auth, "d_agent");

        let user_state = state
            .device_sync_session_state(&user_auth, Some("d_user"))
            .expect("user sync state should resolve");
        let agent_state = state
            .device_sync_session_state(&agent_auth, Some("d_agent"))
            .expect("agent sync state should resolve");

        assert_eq!(user_state.registered_devices, vec!["d_user"]);
        assert_eq!(agent_state.registered_devices, vec!["d_agent"]);
    }

    #[test]
    fn test_device_kind_conflict_rejected_for_same_actor_and_device() {
        let state = SessionSyncState::default();
        let user_auth = auth_context("u_demo", "user", "d_shared");
        let agent_auth = auth_context("u_demo", "agent", "d_shared");

        state.register_device(&user_auth, "d_shared");
        let error = state
            .ensure_device_kind_available(&agent_auth, "d_shared")
            .expect_err("different actor kind should be rejected for same actor/device");
        assert_eq!(error.code, "device_scope_conflict");
    }

    #[test]
    fn test_device_owner_conflict_rejected_for_different_actor_same_device() {
        let state = SessionSyncState::default();
        let owner_a = auth_context("u_owner_a", "user", "d_shared");
        let owner_b = auth_context("u_owner_b", "user", "d_shared");

        state.register_device(&owner_a, "d_shared");
        let error = state
            .ensure_device_kind_available(&owner_b, "d_shared")
            .expect_err("different owner should be rejected for same tenant/device");
        assert_eq!(error.code, "device_scope_conflict");
    }
}
