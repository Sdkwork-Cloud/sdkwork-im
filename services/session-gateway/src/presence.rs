use craw_chat_contract_control::{PresenceStateRecord, PresenceStateStore};
use craw_chat_contract_core::ContractError;
use craw_chat_runtime_link::decide_resume;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::{Arc, Mutex};

use im_auth_context::AuthContext;
use im_domain_core::session::{
    DevicePresenceStatus, DevicePresenceView, PresenceSnapshotView, SessionResumeView,
};
use im_time::utc_now_rfc3339_millis;

#[derive(Clone, Debug, PartialEq, Eq)]
struct PresenceRuntimeEntry {
    view: DevicePresenceView,
    resume_required: bool,
}

#[derive(Clone)]
pub struct SessionPresenceRuntime {
    entries: Arc<Mutex<HashMap<String, HashMap<String, PresenceRuntimeEntry>>>>,
    restored_principals: Arc<Mutex<HashSet<String>>>,
    state_store: Arc<dyn PresenceStateStore>,
}

#[derive(Clone, Default)]
struct RuntimeMemoryPresenceStateStore {
    states: Arc<Mutex<HashMap<String, PresenceStateRecord>>>,
}

impl PresenceStateStore for RuntimeMemoryPresenceStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError> {
        Ok(self
            .states
            .lock()
            .expect("presence state store should lock")
            .get(device_scope_key(tenant_id, principal_id, device_id).as_str())
            .cloned())
    }

    fn save_state(&self, record: PresenceStateRecord) -> Result<(), ContractError> {
        self.states
            .lock()
            .expect("presence state store should lock")
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

    fn list_states_for_principal(
        &self,
        tenant_id: &str,
        principal_id: &str,
    ) -> Result<Vec<PresenceStateRecord>, ContractError> {
        Ok(self
            .states
            .lock()
            .expect("presence state store should lock")
            .values()
            .filter(|record| record.tenant_id == tenant_id && record.principal_id == principal_id)
            .cloned()
            .collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresenceRuntimeError {
    code: &'static str,
    message: String,
}

impl PresenceRuntimeError {
    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    fn reconnect_required(device_id: &str) -> Self {
        Self {
            code: "reconnect_required",
            message: format!("device must resume a fresh session before continuing: {device_id}"),
        }
    }

    fn presence_store(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                code: "presence_store_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                code: "presence_store_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                code: "presence_store_unsupported",
                message,
            },
        }
    }
}

impl SessionPresenceRuntime {
    pub fn with_store<S>(state_store: Arc<S>) -> Self
    where
        S: PresenceStateStore + 'static,
    {
        Self {
            entries: Arc::new(Mutex::new(HashMap::new())),
            restored_principals: Arc::new(Mutex::new(HashSet::new())),
            state_store,
        }
    }

    pub fn register_device(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<(), PresenceRuntimeError> {
        self.ensure_device_entry(tenant_id, principal_id, device_id)
            .map(|_| ())
    }

    pub fn ensure_device_resume_not_required(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<(), PresenceRuntimeError> {
        let entry = self.ensure_device_entry(tenant_id, principal_id, device_id)?;
        if entry.resume_required {
            return Err(PresenceRuntimeError::reconnect_required(device_id));
        }
        Ok(())
    }

    pub fn resume(
        &self,
        auth: &AuthContext,
        device_id: String,
        last_seen_sync_seq: u64,
        latest_sync_seq: u64,
        registered_devices: Vec<String>,
    ) -> Result<SessionResumeView, PresenceRuntimeError> {
        self.ensure_device_entry(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            device_id.as_str(),
        )?;
        let resumed_at = session_timestamp();
        let updated_entry = {
            let scope = principal_scope_key(auth.tenant_id.as_str(), auth.actor_id.as_str());
            let mut entries = self.entries.lock().expect("presence store should lock");
            let scope_entries = entries.entry(scope).or_default();
            let entry =
                scope_entries
                    .entry(device_id.clone())
                    .or_insert_with(|| PresenceRuntimeEntry {
                        view: empty_presence_view(auth, device_id.as_str()),
                        resume_required: false,
                    });
            entry.view.session_id = auth.session_id.clone();
            entry.view.status = DevicePresenceStatus::Online;
            entry.view.last_sync_seq = latest_sync_seq;
            entry.view.last_resume_at = Some(resumed_at.clone());
            entry.view.last_seen_at = Some(resumed_at.clone());
            entry.resume_required = false;
            entry.clone()
        };
        self.persist_entry(updated_entry, resumed_at.clone())?;

        let presence = self.presence_snapshot(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            Some(device_id.clone()),
            registered_devices,
        )?;
        let resume = decide_resume(last_seen_sync_seq, latest_sync_seq);

        Ok(SessionResumeView {
            tenant_id: auth.tenant_id.clone(),
            actor_id: auth.actor_id.clone(),
            actor_kind: auth.actor_kind.clone(),
            session_id: auth.session_id.clone(),
            device_id,
            resume_required: resume.resume_required,
            resume_from_sync_seq: resume.resume_from_sync_seq,
            latest_sync_seq: resume.latest_sync_seq,
            resumed_at,
            presence,
        })
    }

    pub fn presence_snapshot(
        &self,
        tenant_id: &str,
        principal_id: &str,
        current_device_id: Option<String>,
        registered_devices: Vec<String>,
    ) -> Result<PresenceSnapshotView, PresenceRuntimeError> {
        self.ensure_principal_state(tenant_id, principal_id)?;
        let scope = principal_scope_key(tenant_id, principal_id);
        let stored_devices = self
            .entries
            .lock()
            .expect("presence store should lock")
            .get(scope.as_str())
            .cloned()
            .unwrap_or_default();

        let mut device_ids = BTreeSet::new();
        for device_id in registered_devices {
            device_ids.insert(device_id);
        }
        if let Some(device_id) = current_device_id.clone() {
            device_ids.insert(device_id);
        }
        for device_id in stored_devices.keys() {
            device_ids.insert(device_id.clone());
        }

        let mut devices = device_ids
            .into_iter()
            .map(|device_id| {
                stored_devices
                    .get(device_id.as_str())
                    .map(|entry| entry.view.clone())
                    .unwrap_or_else(|| {
                        empty_presence_view_for_scope(tenant_id, principal_id, &device_id)
                    })
            })
            .collect::<Vec<_>>();

        devices.sort_by(|left, right| {
            let left_current = current_device_id
                .as_deref()
                .map(|value| value == left.device_id.as_str())
                .unwrap_or(false);
            let right_current = current_device_id
                .as_deref()
                .map(|value| value == right.device_id.as_str())
                .unwrap_or(false);
            right_current
                .cmp(&left_current)
                .then_with(|| left.device_id.cmp(&right.device_id))
        });

        Ok(PresenceSnapshotView {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            current_device_id,
            devices,
        })
    }

    pub fn heartbeat(
        &self,
        auth: &AuthContext,
        device_id: String,
        latest_sync_seq: u64,
        registered_devices: Vec<String>,
    ) -> Result<PresenceSnapshotView, PresenceRuntimeError> {
        self.ensure_device_resume_not_required(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            device_id.as_str(),
        )?;
        let observed_at = session_timestamp().to_owned();
        self.update_presence_entry(
            auth,
            device_id.clone(),
            latest_sync_seq,
            Some(auth.session_id.clone()),
            DevicePresenceStatus::Online,
            observed_at,
            false,
            false,
        )?;
        self.presence_snapshot(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            Some(device_id),
            registered_devices,
        )
    }

    pub fn disconnect(
        &self,
        auth: &AuthContext,
        device_id: String,
        registered_devices: Vec<String>,
    ) -> Result<PresenceSnapshotView, PresenceRuntimeError> {
        self.ensure_device_entry(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            device_id.as_str(),
        )?;
        let latest_sync_seq = self
            .entries
            .lock()
            .expect("presence store should lock")
            .get(principal_scope_key(auth.tenant_id.as_str(), auth.actor_id.as_str()).as_str())
            .and_then(|scope_entries| scope_entries.get(device_id.as_str()))
            .map(|entry| entry.view.last_sync_seq)
            .unwrap_or_default();
        self.update_presence_entry(
            auth,
            device_id.clone(),
            latest_sync_seq,
            Some(None),
            DevicePresenceStatus::Offline,
            session_timestamp(),
            false,
            true,
        )?;
        self.presence_snapshot(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            Some(device_id),
            registered_devices,
        )
    }

    fn ensure_principal_state(
        &self,
        tenant_id: &str,
        principal_id: &str,
    ) -> Result<(), PresenceRuntimeError> {
        let scope_key = principal_scope_key(tenant_id, principal_id);
        if self
            .restored_principals
            .lock()
            .expect("presence runtime should lock")
            .contains(scope_key.as_str())
        {
            return Ok(());
        }

        let restored = self
            .state_store
            .list_states_for_principal(tenant_id, principal_id)
            .map_err(PresenceRuntimeError::presence_store)?;
        let mut normalized_records = Vec::new();
        let mut runtime_entries = Vec::new();
        for record in restored {
            let (entry, normalized_record) = normalize_presence_record(record);
            if let Some(normalized_record) = normalized_record {
                normalized_records.push(normalized_record);
            }
            runtime_entries.push((entry.view.device_id.clone(), entry));
        }

        for normalized_record in normalized_records {
            self.state_store
                .save_state(normalized_record)
                .map_err(PresenceRuntimeError::presence_store)?;
        }

        let mut entries = self.entries.lock().expect("presence runtime should lock");
        let scope_entries = entries.entry(scope_key.clone()).or_default();
        for (device_id, entry) in runtime_entries {
            scope_entries.entry(device_id).or_insert(entry);
        }
        drop(entries);
        self.restored_principals
            .lock()
            .expect("presence runtime should lock")
            .insert(scope_key);

        Ok(())
    }

    fn ensure_device_entry(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<PresenceRuntimeEntry, PresenceRuntimeError> {
        self.ensure_principal_state(tenant_id, principal_id)?;

        if let Some(entry) = self
            .entries
            .lock()
            .expect("presence store should lock")
            .get(principal_scope_key(tenant_id, principal_id).as_str())
            .and_then(|scope_entries| scope_entries.get(device_id))
            .cloned()
        {
            return Ok(entry);
        }

        let entry = PresenceRuntimeEntry {
            view: empty_presence_view_for_scope(tenant_id, principal_id, device_id),
            resume_required: false,
        };
        let scope = principal_scope_key(tenant_id, principal_id);
        let mut entries = self.entries.lock().expect("presence store should lock");
        entries
            .entry(scope)
            .or_default()
            .insert(device_id.to_owned(), entry.clone());
        drop(entries);
        self.persist_entry(entry.clone(), session_timestamp())?;

        Ok(entry)
    }

    // Presence updates mirror the stored device session tuple and explicit flags
    // so reconnect and resume decisions stay readable at each call site.
    #[allow(clippy::too_many_arguments)]
    fn update_presence_entry(
        &self,
        auth: &AuthContext,
        device_id: String,
        latest_sync_seq: u64,
        session_id: Option<Option<String>>,
        status: DevicePresenceStatus,
        observed_at: String,
        refresh_resume_at: bool,
        resume_required: bool,
    ) -> Result<(), PresenceRuntimeError> {
        self.ensure_device_entry(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            device_id.as_str(),
        )?;
        let scope = principal_scope_key(auth.tenant_id.as_str(), auth.actor_id.as_str());
        let mut entries = self.entries.lock().expect("presence store should lock");
        let scope_entries = entries.entry(scope).or_default();
        let entry =
            scope_entries
                .entry(device_id.clone())
                .or_insert_with(|| PresenceRuntimeEntry {
                    view: empty_presence_view(auth, device_id.as_str()),
                    resume_required: false,
                });
        if let Some(session_id) = session_id {
            entry.view.session_id = session_id;
        }
        entry.view.status = status;
        entry.view.last_sync_seq = latest_sync_seq;
        if refresh_resume_at || entry.view.last_resume_at.is_none() {
            entry.view.last_resume_at = Some(observed_at.clone());
        }
        entry.view.last_seen_at = Some(observed_at.clone());
        entry.resume_required = resume_required;
        let updated = entry.clone();
        drop(entries);
        self.persist_entry(updated, observed_at)
    }

    fn persist_entry(
        &self,
        entry: PresenceRuntimeEntry,
        updated_at: String,
    ) -> Result<(), PresenceRuntimeError> {
        self.state_store
            .save_state(PresenceStateRecord {
                tenant_id: entry.view.tenant_id.clone(),
                principal_id: entry.view.principal_id.clone(),
                device_id: entry.view.device_id.clone(),
                presence: entry.view,
                resume_required: entry.resume_required,
                updated_at,
            })
            .map_err(PresenceRuntimeError::presence_store)
    }
}

impl Default for SessionPresenceRuntime {
    fn default() -> Self {
        Self::with_store(Arc::new(RuntimeMemoryPresenceStateStore::default()))
    }
}

pub(crate) fn principal_scope_key(tenant_id: &str, principal_id: &str) -> String {
    format!("{tenant_id}:{principal_id}")
}

pub(crate) fn device_scope_key(tenant_id: &str, principal_id: &str, device_id: &str) -> String {
    format!("{tenant_id}:{principal_id}:{device_id}")
}

fn empty_presence_view(auth: &AuthContext, device_id: &str) -> DevicePresenceView {
    empty_presence_view_for_scope(auth.tenant_id.as_str(), auth.actor_id.as_str(), device_id)
}

fn empty_presence_view_for_scope(
    tenant_id: &str,
    principal_id: &str,
    device_id: &str,
) -> DevicePresenceView {
    DevicePresenceView {
        tenant_id: tenant_id.into(),
        principal_id: principal_id.into(),
        device_id: device_id.into(),
        platform: None,
        session_id: None,
        status: DevicePresenceStatus::Offline,
        last_sync_seq: 0,
        last_resume_at: None,
        last_seen_at: None,
    }
}

fn normalize_presence_record(
    record: PresenceStateRecord,
) -> (PresenceRuntimeEntry, Option<PresenceStateRecord>) {
    let mut presence = record.presence.clone();
    let mut resume_required = record.resume_required;
    let mut normalized = false;

    if matches!(presence.status, DevicePresenceStatus::Online) {
        presence.status = DevicePresenceStatus::Offline;
        presence.session_id = None;
        resume_required = true;
        normalized = true;
    } else if presence.session_id.is_some() {
        presence.session_id = None;
        normalized = true;
    }

    let entry = PresenceRuntimeEntry {
        view: presence.clone(),
        resume_required,
    };
    let normalized_record = if normalized || resume_required != record.resume_required {
        Some(PresenceStateRecord {
            tenant_id: record.tenant_id,
            principal_id: record.principal_id,
            device_id: record.device_id,
            presence,
            resume_required,
            updated_at: session_timestamp(),
        })
    } else {
        None
    };

    (entry, normalized_record)
}

fn session_timestamp() -> String {
    utc_now_rfc3339_millis()
}
