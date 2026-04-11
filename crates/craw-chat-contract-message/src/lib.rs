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

pub trait TimelineProjectionStore {
    fn upsert_timeline_entry(
        &self,
        conversation_id: &str,
        message_seq: u64,
        payload: &str,
    ) -> Result<(), ContractError>;

    fn load_timeline(&self, conversation_id: &str) -> Result<Vec<(u64, String)>, ContractError>;
}
