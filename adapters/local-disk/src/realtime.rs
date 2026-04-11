use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use im_platform_contracts::{
    ContractError, RealtimeCheckpointRecord, RealtimeCheckpointStore,
    RealtimeDisconnectFenceRecord, RealtimeDisconnectFenceStore, RealtimeSubscriptionRecord,
    RealtimeSubscriptionStore,
};

use crate::shared::{read_json_records_or_default, scope_key, write_json_records};

#[derive(Clone, Debug)]
pub struct FileRealtimeCheckpointStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileRealtimeCheckpointStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, RealtimeCheckpointRecord>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "realtime checkpoint store")
    }

    fn write_records(
        &self,
        records: &BTreeMap<String, RealtimeCheckpointRecord>,
    ) -> Result<(), ContractError> {
        write_json_records(
            self.file_path.as_path(),
            records,
            "realtime checkpoint store",
        )
    }
}

impl RealtimeCheckpointStore for FileRealtimeCheckpointStore {
    fn load_checkpoint(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("checkpoint file store lock should lock");
        Ok(self
            .read_records()?
            .remove(scope_key(tenant_id, principal_id, device_id).as_str()))
    }

    fn save_checkpoint(&self, record: RealtimeCheckpointRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("checkpoint file store lock should lock");
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
}

#[derive(Clone, Debug)]
pub struct FileRealtimeDisconnectFenceStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileRealtimeDisconnectFenceStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(
        &self,
    ) -> Result<BTreeMap<String, RealtimeDisconnectFenceRecord>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "disconnect fence store")
    }

    fn write_records(
        &self,
        records: &BTreeMap<String, RealtimeDisconnectFenceRecord>,
    ) -> Result<(), ContractError> {
        write_json_records(self.file_path.as_path(), records, "disconnect fence store")
    }
}

impl RealtimeDisconnectFenceStore for FileRealtimeDisconnectFenceStore {
    fn load_fence(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeDisconnectFenceRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("disconnect fence file store lock should lock");
        Ok(self
            .read_records()?
            .remove(scope_key(tenant_id, principal_id, device_id).as_str()))
    }

    fn save_fence(&self, record: RealtimeDisconnectFenceRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("disconnect fence file store lock should lock");
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

    fn clear_fence(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("disconnect fence file store lock should lock");
        let mut records = self.read_records()?;
        let removed = records
            .remove(scope_key(tenant_id, principal_id, device_id).as_str())
            .is_some();
        self.write_records(&records)?;
        Ok(removed)
    }
}

#[derive(Clone, Debug)]
pub struct FileRealtimeSubscriptionStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileRealtimeSubscriptionStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, RealtimeSubscriptionRecord>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "realtime subscription store")
    }

    fn write_records(
        &self,
        records: &BTreeMap<String, RealtimeSubscriptionRecord>,
    ) -> Result<(), ContractError> {
        write_json_records(
            self.file_path.as_path(),
            records,
            "realtime subscription store",
        )
    }
}

impl RealtimeSubscriptionStore for FileRealtimeSubscriptionStore {
    fn load_subscriptions(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeSubscriptionRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("subscription file store lock should lock");
        Ok(self
            .read_records()?
            .remove(scope_key(tenant_id, principal_id, device_id).as_str()))
    }

    fn save_subscriptions(&self, record: RealtimeSubscriptionRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("subscription file store lock should lock");
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

    fn clear_subscriptions(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("subscription file store lock should lock");
        let mut records = self.read_records()?;
        let removed = records
            .remove(scope_key(tenant_id, principal_id, device_id).as_str())
            .is_some();
        self.write_records(&records)?;
        Ok(removed)
    }
}

pub fn validate_realtime_checkpoint_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, RealtimeCheckpointRecord> =
        read_json_records_or_default(file_path.as_ref(), "realtime checkpoint store")?;
    Ok(())
}

pub fn validate_realtime_disconnect_fence_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, RealtimeDisconnectFenceRecord> =
        read_json_records_or_default(file_path.as_ref(), "disconnect fence store")?;
    Ok(())
}

pub fn validate_realtime_subscription_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, RealtimeSubscriptionRecord> =
        read_json_records_or_default(file_path.as_ref(), "realtime subscription store")?;
    Ok(())
}
