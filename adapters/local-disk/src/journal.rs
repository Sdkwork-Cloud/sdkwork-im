use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use im_platform_contracts::{CommitEnvelope, CommitJournal, CommitPosition, ContractError};

use crate::shared::{
    read_json_records_or_default, read_json_records_or_default_unlocked, with_store_file_lock,
    write_json_records_unlocked,
};

#[derive(Clone, Debug)]
pub struct FileCommitJournal {
    partition: Arc<String>,
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileCommitJournal {
    pub fn new(partition: impl Into<String>, file_path: impl Into<PathBuf>) -> Self {
        Self {
            partition: Arc::new(partition.into()),
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    pub fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("commit journal file store lock should lock");
        self.read_events()
    }

    fn read_events(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "commit journal")
    }

    fn read_events_unlocked(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        read_json_records_or_default_unlocked(self.file_path.as_path(), "commit journal")
    }

    fn write_events_unlocked(&self, events: &[CommitEnvelope]) -> Result<(), ContractError> {
        write_json_records_unlocked(self.file_path.as_path(), events, "commit journal")
    }
}

impl CommitJournal for FileCommitJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("commit journal file store lock should lock");
        with_store_file_lock(self.file_path.as_path(), "commit journal", || {
            let mut events = self.read_events_unlocked()?;
            events.push(envelope);
            self.write_events_unlocked(&events)?;
            Ok(CommitPosition::new(
                self.partition.as_str(),
                events.len() as u64,
            ))
        })
    }

    fn append_batch(
        &self,
        envelopes: Vec<CommitEnvelope>,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("commit journal file store lock should lock");
        with_store_file_lock(self.file_path.as_path(), "commit journal", || {
            let mut events = self.read_events_unlocked()?;
            let start_offset = events.len() as u64 + 1;
            let batch_len = envelopes.len() as u64;
            events.extend(envelopes);
            self.write_events_unlocked(&events)?;
            Ok((0..batch_len)
                .map(|index| CommitPosition::new(self.partition.as_str(), start_offset + index))
                .collect())
        })
    }
}

pub fn read_commit_journal_file(
    file_path: impl AsRef<Path>,
) -> Result<Vec<CommitEnvelope>, ContractError> {
    read_json_records_or_default(file_path.as_ref(), "commit journal")
}

pub fn validate_commit_journal_file(file_path: impl AsRef<Path>) -> Result<(), ContractError> {
    let _: Vec<CommitEnvelope> = read_commit_journal_file(file_path)?;
    Ok(())
}
