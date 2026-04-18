use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use im_platform_contracts::{ContractError, DeviceTwinRecord, DeviceTwinStore};

use crate::shared::{device_twin_scope_key, read_json_records_or_default, update_json_records};

#[derive(Clone, Debug)]
pub struct FileDeviceTwinStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileDeviceTwinStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, DeviceTwinRecord>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "device twin store")
    }
}

impl DeviceTwinStore for FileDeviceTwinStore {
    fn load_twin(
        &self,
        tenant_id: &str,
        device_id: &str,
    ) -> Result<Option<DeviceTwinRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("device twin file store lock should lock");
        Ok(self
            .read_records()?
            .remove(device_twin_scope_key(tenant_id, device_id).as_str()))
    }

    fn save_twin(&self, record: DeviceTwinRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("device twin file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "device twin store",
            |records: &mut BTreeMap<String, DeviceTwinRecord>| {
                records.insert(
                    device_twin_scope_key(record.tenant_id.as_str(), record.device_id.as_str()),
                    record,
                );
            },
        )
    }
}

pub fn validate_device_twin_store_file(file_path: impl AsRef<Path>) -> Result<(), ContractError> {
    let _: BTreeMap<String, DeviceTwinRecord> =
        read_json_records_or_default(file_path.as_ref(), "device twin store")?;
    Ok(())
}
