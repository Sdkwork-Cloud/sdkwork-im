use craw_chat_contract_core::ContractError;
use im_domain_core::notification::NotificationTask;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationTaskRecord {
    pub tenant_id: String,
    pub notification_id: String,
    pub task: NotificationTask,
    pub updated_at: String,
}

pub trait NotificationTaskStore: Send + Sync {
    fn load_task(
        &self,
        tenant_id: &str,
        notification_id: &str,
    ) -> Result<Option<NotificationTaskRecord>, ContractError>;

    fn save_task(&self, record: NotificationTaskRecord) -> Result<(), ContractError>;

    fn list_tasks_for_recipient(
        &self,
        tenant_id: &str,
        recipient_id: &str,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError>;
}
