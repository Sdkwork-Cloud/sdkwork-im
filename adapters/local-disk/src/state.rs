use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use im_platform_contracts::{
    ContractError, PresenceStateRecord, PresenceStateStore, RtcStateRecord, RtcStateStore,
    StreamStateRecord, StreamStateStore,
};

use crate::shared::{
    read_json_records_or_default, rtc_scope_key, scope_key, stream_scope_key, write_json_records,
};

#[derive(Clone, Debug)]
pub struct FileStreamStateStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileStreamStateStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, StreamStateRecord>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "stream state store")
    }

    fn write_records(
        &self,
        records: &BTreeMap<String, StreamStateRecord>,
    ) -> Result<(), ContractError> {
        write_json_records(self.file_path.as_path(), records, "stream state store")
    }
}

impl StreamStateStore for FileStreamStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        stream_id: &str,
    ) -> Result<Option<StreamStateRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("stream state file store lock should lock");
        Ok(self
            .read_records()?
            .remove(stream_scope_key(tenant_id, stream_id).as_str()))
    }

    fn save_state(&self, record: StreamStateRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("stream state file store lock should lock");
        let mut records = self.read_records()?;
        records.insert(
            stream_scope_key(record.tenant_id.as_str(), record.stream_id.as_str()),
            record,
        );
        self.write_records(&records)
    }

    fn clear_state(&self, tenant_id: &str, stream_id: &str) -> Result<bool, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("stream state file store lock should lock");
        let mut records = self.read_records()?;
        let removed = records
            .remove(stream_scope_key(tenant_id, stream_id).as_str())
            .is_some();
        self.write_records(&records)?;
        Ok(removed)
    }
}

#[derive(Clone, Debug)]
pub struct FileRtcStateStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileRtcStateStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, RtcStateRecord>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "rtc state store")
    }

    fn write_records(
        &self,
        records: &BTreeMap<String, RtcStateRecord>,
    ) -> Result<(), ContractError> {
        write_json_records(self.file_path.as_path(), records, "rtc state store")
    }
}

impl RtcStateStore for FileRtcStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<Option<RtcStateRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("rtc state file store lock should lock");
        Ok(self
            .read_records()?
            .remove(rtc_scope_key(tenant_id, rtc_session_id).as_str()))
    }

    fn save_state(&self, record: RtcStateRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("rtc state file store lock should lock");
        let mut records = self.read_records()?;
        records.insert(
            rtc_scope_key(record.tenant_id.as_str(), record.rtc_session_id.as_str()),
            record,
        );
        self.write_records(&records)
    }

    fn clear_state(&self, tenant_id: &str, rtc_session_id: &str) -> Result<bool, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("rtc state file store lock should lock");
        let mut records = self.read_records()?;
        let removed = records
            .remove(rtc_scope_key(tenant_id, rtc_session_id).as_str())
            .is_some();
        self.write_records(&records)?;
        Ok(removed)
    }
}

#[derive(Clone, Debug)]
pub struct FilePresenceStateStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FilePresenceStateStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, PresenceStateRecord>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "presence state store")
    }

    fn write_records(
        &self,
        records: &BTreeMap<String, PresenceStateRecord>,
    ) -> Result<(), ContractError> {
        write_json_records(self.file_path.as_path(), records, "presence state store")
    }
}

impl PresenceStateStore for FilePresenceStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("presence state file store lock should lock");
        Ok(self
            .read_records()?
            .remove(scope_key(tenant_id, principal_id, device_id).as_str()))
    }

    fn save_state(&self, record: PresenceStateRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("presence state file store lock should lock");
        let mut records = self.read_records()?;
        records.insert(
            scope_key(
                record.tenant_id.as_str(),
                record.principal_id.as_str(),
                record.device_id.as_str(),
            ),
            record,
        );
        self.write_records(&records)
    }

    fn list_states_for_principal(
        &self,
        tenant_id: &str,
        principal_id: &str,
    ) -> Result<Vec<PresenceStateRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("presence state file store lock should lock");
        Ok(self
            .read_records()?
            .into_values()
            .filter(|record| record.tenant_id == tenant_id && record.principal_id == principal_id)
            .collect())
    }
}

pub fn validate_stream_state_store_file(file_path: impl AsRef<Path>) -> Result<(), ContractError> {
    let _: BTreeMap<String, StreamStateRecord> =
        read_json_records_or_default(file_path.as_ref(), "stream state store")?;
    Ok(())
}

pub fn validate_rtc_state_store_file(file_path: impl AsRef<Path>) -> Result<(), ContractError> {
    let _: BTreeMap<String, RtcStateRecord> =
        read_json_records_or_default(file_path.as_ref(), "rtc state store")?;
    Ok(())
}

pub fn validate_presence_state_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, PresenceStateRecord> =
        read_json_records_or_default(file_path.as_ref(), "presence state store")?;
    Ok(())
}
