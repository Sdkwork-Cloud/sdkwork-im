use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use im_platform_contracts::{
    AutomationExecutionRecord, AutomationExecutionStore, ContractError, NotificationTaskRecord,
    NotificationTaskStore,
};

use crate::shared::{
    execution_scope_key, legacy_execution_scope_key, notification_scope_key,
    read_json_records_or_default, write_json_records,
};

#[derive(Clone, Debug)]
pub struct FileNotificationTaskStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileNotificationTaskStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, NotificationTaskRecord>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "notification task store")
    }

    fn write_records(
        &self,
        records: &BTreeMap<String, NotificationTaskRecord>,
    ) -> Result<(), ContractError> {
        write_json_records(self.file_path.as_path(), records, "notification task store")
    }
}

impl NotificationTaskStore for FileNotificationTaskStore {
    fn load_task(
        &self,
        tenant_id: &str,
        notification_id: &str,
    ) -> Result<Option<NotificationTaskRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("notification task file store lock should lock");
        Ok(self
            .read_records()?
            .remove(notification_scope_key(tenant_id, notification_id).as_str()))
    }

    fn save_task(&self, record: NotificationTaskRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("notification task file store lock should lock");
        let mut records = self.read_records()?;
        records.insert(
            notification_scope_key(record.tenant_id.as_str(), record.notification_id.as_str()),
            record,
        );
        self.write_records(&records)
    }

    fn list_tasks_for_recipient(
        &self,
        tenant_id: &str,
        recipient_id: &str,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("notification task file store lock should lock");
        Ok(self
            .read_records()?
            .into_values()
            .filter(|record| {
                record.tenant_id == tenant_id && record.task.recipient_id == recipient_id
            })
            .collect())
    }
}

#[derive(Clone, Debug)]
pub struct FileAutomationExecutionStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileAutomationExecutionStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, AutomationExecutionRecord>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "automation execution store")
    }

    fn write_records(
        &self,
        records: &BTreeMap<String, AutomationExecutionRecord>,
    ) -> Result<(), ContractError> {
        write_json_records(
            self.file_path.as_path(),
            records,
            "automation execution store",
        )
    }
}

impl AutomationExecutionStore for FileAutomationExecutionStore {
    fn load_execution(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Result<Option<AutomationExecutionRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("automation execution file store lock should lock");
        let mut records = self.read_records()?;
        if let Some(record) = records.remove(
            execution_scope_key(tenant_id, principal_kind, principal_id, execution_id).as_str(),
        ) {
            return Ok(Some(record));
        }

        Ok(records
            .remove(legacy_execution_scope_key(tenant_id, principal_id, execution_id).as_str())
            .filter(|record| record.execution.principal_kind == principal_kind))
    }

    fn save_execution(&self, record: AutomationExecutionRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("automation execution file store lock should lock");
        let mut records = self.read_records()?;
        records.insert(
            execution_scope_key(
                record.tenant_id.as_str(),
                record.execution.principal_kind.as_str(),
                record.principal_id.as_str(),
                record.execution_id.as_str(),
            ),
            record,
        );
        self.write_records(&records)
    }
}

pub fn validate_notification_task_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, NotificationTaskRecord> =
        read_json_records_or_default(file_path.as_ref(), "notification task store")?;
    Ok(())
}

pub fn validate_automation_execution_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, AutomationExecutionRecord> =
        read_json_records_or_default(file_path.as_ref(), "automation execution store")?;
    Ok(())
}
