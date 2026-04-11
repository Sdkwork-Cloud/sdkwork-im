use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use im_platform_contracts::{ContractError, TimelineProjectionStore};

use crate::shared::{read_json_records_or_default, write_json_records};

#[derive(Clone, Debug)]
pub struct FileTimelineProjectionStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileTimelineProjectionStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    pub fn entries(&self, conversation_id: &str) -> Vec<(u64, String)> {
        let _guard = self
            .io_lock
            .lock()
            .expect("timeline projection file store lock should lock");
        self.read_records()
            .expect("timeline projection store should parse")
            .get(conversation_id)
            .map(|items| {
                items
                    .iter()
                    .map(|(message_seq, payload)| (*message_seq, payload.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn read_records(&self) -> Result<BTreeMap<String, BTreeMap<u64, String>>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "timeline projection store")
    }

    fn write_records(
        &self,
        records: &BTreeMap<String, BTreeMap<u64, String>>,
    ) -> Result<(), ContractError> {
        write_json_records(
            self.file_path.as_path(),
            records,
            "timeline projection store",
        )
    }
}

impl TimelineProjectionStore for FileTimelineProjectionStore {
    fn upsert_timeline_entry(
        &self,
        conversation_id: &str,
        message_seq: u64,
        payload: &str,
    ) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("timeline projection file store lock should lock");
        let mut records = self.read_records()?;
        records
            .entry(conversation_id.to_string())
            .or_default()
            .insert(message_seq, payload.to_string());
        self.write_records(&records)
    }

    fn load_timeline(&self, conversation_id: &str) -> Result<Vec<(u64, String)>, ContractError> {
        Ok(self.entries(conversation_id))
    }
}

pub fn validate_timeline_projection_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, BTreeMap<u64, String>> =
        read_json_records_or_default(file_path.as_ref(), "timeline projection store")?;
    Ok(())
}
