use craw_chat_contract_core::ContractError;

pub use im_domain_events::CommitEnvelope;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitPosition {
    pub partition: String,
    pub offset: u64,
}

impl CommitPosition {
    pub fn new(partition: impl Into<String>, offset: u64) -> Self {
        Self {
            partition: partition.into(),
            offset,
        }
    }

    pub fn cursor(&self) -> String {
        format!("{}:{}", self.partition, self.offset)
    }
}

pub trait CommitJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimelineProjectionRecord {
    pub message_seq: u64,
    pub payload: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimelineProjectionBatch {
    pub conversation_id: String,
    pub records: Vec<TimelineProjectionRecord>,
}

pub trait TimelineProjectionStore {
    fn upsert_timeline_entry(
        &self,
        conversation_id: &str,
        message_seq: u64,
        payload: &str,
    ) -> Result<(), ContractError>;

    fn load_timeline(&self, conversation_id: &str) -> Result<Vec<(u64, String)>, ContractError>;

    fn upsert_timeline_entries(
        &self,
        conversation_id: &str,
        records: &[TimelineProjectionRecord],
    ) -> Result<(), ContractError> {
        for record in records {
            self.upsert_timeline_entry(
                conversation_id,
                record.message_seq,
                record.payload.as_str(),
            )?;
        }
        Ok(())
    }

    fn upsert_timeline_batches(
        &self,
        batches: &[TimelineProjectionBatch],
    ) -> Result<(), ContractError> {
        for batch in batches {
            self.upsert_timeline_entries(batch.conversation_id.as_str(), &batch.records)?;
        }
        Ok(())
    }
}
