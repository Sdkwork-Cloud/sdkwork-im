use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};

use im_domain_core::rtc::{RtcStateRecord, RtcStateStore, encode_im_call_key_segments};
use sdkwork_rtc_core::RtcContractError;
use serde::de::DeserializeOwned;

#[derive(Clone, Default)]
pub(super) struct MemoryImCallStateStore {
    states: Arc<Mutex<HashMap<String, RtcStateRecord>>>,
}

impl MemoryImCallStateStore {
    pub(super) fn state(&self, tenant_id: &str, rtc_session_id: &str) -> Option<RtcStateRecord> {
        lock_memory_mutex(&self.states, "im call state store")
            .get(im_call_scope_key(tenant_id, rtc_session_id).as_str())
            .cloned()
    }
}

impl RtcStateStore for MemoryImCallStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<Option<RtcStateRecord>, RtcContractError> {
        Ok(self.state(tenant_id, rtc_session_id))
    }

    fn save_state(&self, record: RtcStateRecord) -> Result<(), RtcContractError> {
        let key = im_call_scope_key(record.tenant_id.as_str(), record.rtc_session_id.as_str());
        let mut states = lock_memory_mutex(&self.states, "im call state store");
        let next = states
            .remove(key.as_str())
            .map(|previous| previous.merge_monotonic(record.clone()))
            .unwrap_or(record);
        states.insert(key, next);
        Ok(())
    }

    fn clear_state(&self, tenant_id: &str, rtc_session_id: &str) -> Result<bool, RtcContractError> {
        Ok(lock_memory_mutex(&self.states, "im call state store")
            .remove(im_call_scope_key(tenant_id, rtc_session_id).as_str())
            .is_some())
    }
}

#[derive(Clone, Debug)]
pub(in crate::node) struct FileImCallStateStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileImCallStateStore {
    pub(in crate::node) fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    fn read_records(&self) -> Result<BTreeMap<String, RtcStateRecord>, RtcContractError> {
        read_json_records_or_default(self.file_path.as_path(), "im call state store")
    }
}

impl RtcStateStore for FileImCallStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<Option<RtcStateRecord>, RtcContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("im call state file store lock should lock");
        Ok(self
            .read_records()?
            .remove(im_call_scope_key(tenant_id, rtc_session_id).as_str()))
    }

    fn save_state(&self, record: RtcStateRecord) -> Result<(), RtcContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("im call state file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "im call state store",
            |records: &mut BTreeMap<String, RtcStateRecord>| {
                let key =
                    im_call_scope_key(record.tenant_id.as_str(), record.rtc_session_id.as_str());
                let next = records
                    .remove(key.as_str())
                    .map(|previous| previous.merge_monotonic(record.clone()))
                    .unwrap_or(record);
                records.insert(key, next);
            },
        )
    }

    fn clear_state(&self, tenant_id: &str, rtc_session_id: &str) -> Result<bool, RtcContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("im call state file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "im call state store",
            |records: &mut BTreeMap<String, RtcStateRecord>| {
                records
                    .remove(im_call_scope_key(tenant_id, rtc_session_id).as_str())
                    .is_some()
            },
        )
    }
}

pub(in crate::node) fn validate_im_call_state_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), RtcContractError> {
    let _: BTreeMap<String, RtcStateRecord> =
        read_json_records_or_default(file_path.as_ref(), "im call state store")?;
    Ok(())
}

fn lock_memory_mutex<'a, T>(mutex: &'a Mutex<T>, lock_name: &'static str) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovering poisoned mutex in local-minimal-node: {lock_name}");
            poisoned.into_inner()
        }
    }
}

fn im_call_scope_key(tenant_id: &str, rtc_session_id: &str) -> String {
    encode_im_call_key_segments([tenant_id, rtc_session_id])
}

fn read_json_records_or_default<T>(
    file_path: &Path,
    store_name: &str,
) -> Result<T, RtcContractError>
where
    T: DeserializeOwned + Default,
{
    match fs::read_to_string(file_path) {
        Ok(source) => {
            if source.trim().is_empty() {
                return Ok(T::default());
            }
            serde_json::from_str(source.as_str()).map_err(|error| {
                RtcContractError::Unavailable(format!(
                    "failed to parse {store_name} {}: {error}",
                    file_path.display()
                ))
            })
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(T::default()),
        Err(error) => Err(RtcContractError::Unavailable(format!(
            "failed to read {store_name} {}: {error}",
            file_path.display()
        ))),
    }
}

fn update_json_records<T, R>(
    file_path: &Path,
    store_name: &str,
    update: impl FnOnce(&mut T) -> R,
) -> Result<R, RtcContractError>
where
    T: DeserializeOwned + Default + serde::Serialize,
{
    let mut records = read_json_records_or_default(file_path, store_name)?;
    let result = update(&mut records);
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            RtcContractError::Unavailable(format!(
                "failed to create {store_name} directory {}: {error}",
                parent.display()
            ))
        })?;
    }
    let payload = serde_json::to_string_pretty(&records).map_err(|error| {
        RtcContractError::Unavailable(format!("failed to encode {store_name}: {error}"))
    })?;
    fs::write(file_path, format!("{payload}\n")).map_err(|error| {
        RtcContractError::Unavailable(format!(
            "failed to write {store_name} {}: {error}",
            file_path.display()
        ))
    })?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_domain_core::rtc::{RtcSession, RtcSessionState, RtcSignalEvent, RtcSignalSender};
    use std::panic::{self, AssertUnwindSafe};

    #[test]
    fn memory_store_merges_signals_and_rejects_stale_session_regression() {
        let store = MemoryImCallStateStore::default();
        store
            .save_state(rtc_state_record(
                RtcSessionState::Accepted,
                "2026-05-06T00:00:03.000Z",
                vec![rtc_signal_event(1), rtc_signal_event(2)],
            ))
            .expect("new IM call state save should succeed");
        store
            .save_state(rtc_state_record(
                RtcSessionState::Started,
                "2026-05-06T00:00:02.000Z",
                vec![rtc_signal_event(1)],
            ))
            .expect("stale IM call state save should not fail the caller");

        let state = store
            .state("t_demo", "rtc_demo")
            .expect("IM call state should be present");
        assert_eq!(state.session.state, RtcSessionState::Accepted);
        assert_eq!(
            state
                .signals
                .iter()
                .map(|signal| signal.signal_seq)
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
    }

    #[test]
    fn memory_store_load_recovers_from_poisoned_lock() {
        let store = MemoryImCallStateStore::default();
        poison_mutex(&store.states);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            store.load_state("t_demo", "rtc_poison_store")
        }));
        assert!(
            result.is_ok(),
            "memory IM call state store load should not panic when its mutex is poisoned"
        );
        let load_result = result.expect("panic status should be captured");
        assert!(
            load_result.is_ok(),
            "memory IM call state store load should recover from a poisoned lock"
        );
    }

    #[test]
    fn file_store_persists_across_reopen() {
        let file_path = std::env::temp_dir().join(format!(
            "sdkwork_im_call_state_store_{}.json",
            std::process::id()
        ));
        let _ = fs::remove_file(file_path.as_path());
        let store = FileImCallStateStore::new(&file_path);
        store
            .save_state(rtc_state_record(
                RtcSessionState::Accepted,
                "2026-05-06T00:00:03.000Z",
                vec![rtc_signal_event(1)],
            ))
            .expect("IM call state save should succeed");

        let reopened = FileImCallStateStore::new(&file_path);
        let restored = reopened
            .load_state("t_demo", "rtc_demo")
            .expect("IM call state load should succeed")
            .expect("IM call state should exist");

        assert_eq!(restored.session.state, RtcSessionState::Accepted);
        assert_eq!(restored.signals[0].signal_type, "rtc.offer");
        validate_im_call_state_store_file(&file_path).expect("IM call state file should validate");
        let _ = fs::remove_file(file_path);
    }

    fn rtc_state_record(
        state: RtcSessionState,
        updated_at: &str,
        signals: Vec<RtcSignalEvent>,
    ) -> RtcStateRecord {
        RtcStateRecord {
            tenant_id: "t_demo".into(),
            rtc_session_id: "rtc_demo".into(),
            session: RtcSession {
                tenant_id: "t_demo".into(),
                rtc_session_id: "rtc_demo".into(),
                conversation_id: Some("c_demo".into()),
                rtc_mode: "voice".into(),
                initiator_id: "u_demo".into(),
                initiator_kind: "user".into(),
                provider_plugin_id: Some("webrtc".into()),
                provider_session_id: Some("ps_demo".into()),
                access_endpoint: Some("wss://rtc.example.test/session/ps_demo".into()),
                provider_region: Some("cn-shanghai".into()),
                state,
                signaling_stream_id: Some("st_demo".into()),
                artifact_message_id: None,
                started_at: "2026-05-06T00:00:00.000Z".into(),
                ended_at: None,
            },
            signals,
            updated_at: updated_at.into(),
        }
    }

    fn rtc_signal_event(signal_seq: u64) -> RtcSignalEvent {
        RtcSignalEvent {
            tenant_id: "t_demo".into(),
            rtc_session_id: "rtc_demo".into(),
            signal_seq,
            conversation_id: Some("c_demo".into()),
            rtc_mode: "voice".into(),
            signal_type: "rtc.offer".into(),
            schema_ref: Some("webrtc.offer.v1".into()),
            payload: format!("{{\"seq\":{signal_seq}}}"),
            sender: RtcSignalSender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            signaling_stream_id: Some("st_demo".into()),
            occurred_at: format!("2026-05-06T00:00:0{signal_seq}.000Z"),
        }
    }

    fn poison_mutex<T>(mutex: &Mutex<T>) {
        let _ = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = mutex.lock().expect("test poison lock should succeed");
            panic!("intentional poison for regression coverage");
        }));
    }
}
