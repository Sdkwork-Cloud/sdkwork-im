use super::*;
use im_time::rfc3339_lt;
use std::io::Write;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum MessageSideEffectOutboxStatus {
    Pending,
    Delivered,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct MessageSideEffectOutboxRecord {
    pub outbox_id: String,
    pub tenant_id: String,
    pub actor_id: String,
    pub actor_kind: String,
    pub actor_session_id: Option<String>,
    pub actor_device_id: Option<String>,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub side_effect: String,
    pub scope_type: String,
    pub scope_id: String,
    pub event_type: String,
    pub payload: String,
    pub status: MessageSideEffectOutboxStatus,
    pub attempt_count: u64,
    pub last_error_code: Option<String>,
    pub last_error_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl MessageSideEffectOutboxRecord {
    pub(super) fn realtime_message_posted(
        auth: &AppContext,
        conversation_id: &str,
        message_id: &str,
        message_seq: u64,
        event_type: &str,
        payload: String,
    ) -> Self {
        let now = im_time::utc_now_rfc3339_millis();
        Self {
            outbox_id: message_side_effect_outbox_id(message_id, "realtime_delivery"),
            tenant_id: auth.tenant_id.clone(),
            actor_id: auth.actor_id.clone(),
            actor_kind: auth.actor_kind.clone(),
            actor_session_id: auth.session_id.clone(),
            actor_device_id: auth.device_id.clone(),
            conversation_id: conversation_id.into(),
            message_id: message_id.into(),
            message_seq,
            side_effect: "realtime_delivery".into(),
            scope_type: "conversation".into(),
            scope_id: conversation_id.into(),
            event_type: event_type.into(),
            payload,
            status: MessageSideEffectOutboxStatus::Pending,
            attempt_count: 0,
            last_error_code: None,
            last_error_message: None,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub(super) fn notification_message_posted(
        auth: &AppContext,
        request: notification_service::RequestMessagePostedNotifications,
    ) -> Self {
        let now = im_time::utc_now_rfc3339_millis();
        let message_id = request.message_id.clone();
        let message_seq = request.message_seq;
        let conversation_id = request.conversation_id.clone();
        Self {
            outbox_id: message_side_effect_outbox_id(&message_id, "notification_delivery"),
            tenant_id: auth.tenant_id.clone(),
            actor_id: auth.actor_id.clone(),
            actor_kind: auth.actor_kind.clone(),
            actor_session_id: auth.session_id.clone(),
            actor_device_id: auth.device_id.clone(),
            conversation_id,
            message_id,
            message_seq,
            side_effect: "notification_delivery".into(),
            scope_type: "notification".into(),
            scope_id: request.source_event_id.clone(),
            event_type: "message.posted".into(),
            payload: serde_json::to_string(&MessageNotificationSideEffectPayload::from(request))
                .expect("message notification side-effect payload should serialize"),
            status: MessageSideEffectOutboxStatus::Pending,
            attempt_count: 0,
            last_error_code: None,
            last_error_message: None,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    fn auth_context(&self) -> AppContext {
        AppContext {
            tenant_id: self.tenant_id.clone(),
            organization_id: String::new(),
            user_id: self.actor_id.clone(),
            actor_id: self.actor_id.clone(),
            actor_kind: self.actor_kind.clone(),
            session_id: self.actor_session_id.clone(),
            app_id: Some("sdkwork-im".into()),
            environment: None,
            deployment_mode: None,
            auth_level: None,
            data_scope: BTreeSet::new(),
            permission_scope: BTreeSet::new(),
            device_id: self.actor_device_id.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageNotificationSideEffectPayload {
    source_event_id: String,
    conversation_id: String,
    message_id: String,
    message_seq: u64,
    message_type: String,
    summary: Option<String>,
}

impl From<notification_service::RequestMessagePostedNotifications>
    for MessageNotificationSideEffectPayload
{
    fn from(value: notification_service::RequestMessagePostedNotifications) -> Self {
        Self {
            source_event_id: value.source_event_id,
            conversation_id: value.conversation_id,
            message_id: value.message_id,
            message_seq: value.message_seq,
            message_type: value.message_type,
            summary: value.summary,
        }
    }
}

impl From<MessageNotificationSideEffectPayload>
    for notification_service::RequestMessagePostedNotifications
{
    fn from(value: MessageNotificationSideEffectPayload) -> Self {
        Self {
            source_event_id: value.source_event_id,
            conversation_id: value.conversation_id,
            message_id: value.message_id,
            message_seq: value.message_seq,
            message_type: value.message_type,
            summary: value.summary,
        }
    }
}

pub(super) trait MessageSideEffectOutboxStore: Send + Sync {
    fn upsert_pending(
        &self,
        record: MessageSideEffectOutboxRecord,
    ) -> Result<MessageSideEffectOutboxRecord, ContractError>;

    fn list_pending(&self) -> Result<Vec<MessageSideEffectOutboxRecord>, ContractError>;

    fn diagnostics_snapshot(
        &self,
    ) -> Result<Vec<ops_service::SideEffectOutboxDiagnosticsView>, ContractError>;

    fn mark_delivered(&self, outbox_id: &str) -> Result<(), ContractError>;

    fn mark_failed(
        &self,
        outbox_id: &str,
        error_code: &str,
        error_message: &str,
    ) -> Result<(), ContractError>;
}

#[derive(Default)]
pub(super) struct MemoryMessageSideEffectOutboxStore {
    records: std::sync::Mutex<BTreeMap<String, MessageSideEffectOutboxRecord>>,
}

impl MessageSideEffectOutboxStore for MemoryMessageSideEffectOutboxStore {
    fn upsert_pending(
        &self,
        record: MessageSideEffectOutboxRecord,
    ) -> Result<MessageSideEffectOutboxRecord, ContractError> {
        let mut records = lock_outbox_records(&self.records);
        let outbox_id = record.outbox_id.clone();
        let next = match records.remove(outbox_id.as_str()) {
            Some(existing)
                if matches!(existing.status, MessageSideEffectOutboxStatus::Delivered) =>
            {
                existing
            }
            Some(mut existing) => {
                existing.payload = record.payload;
                existing.event_type = record.event_type;
                existing.scope_type = record.scope_type;
                existing.scope_id = record.scope_id;
                existing.status = MessageSideEffectOutboxStatus::Pending;
                existing.updated_at = im_time::utc_now_rfc3339_millis();
                existing
            }
            None => record,
        };
        records.insert(outbox_id, next.clone());
        Ok(next)
    }

    fn list_pending(&self) -> Result<Vec<MessageSideEffectOutboxRecord>, ContractError> {
        let records = lock_outbox_records(&self.records);
        Ok(records
            .values()
            .filter(|record| matches!(record.status, MessageSideEffectOutboxStatus::Pending))
            .cloned()
            .collect())
    }

    fn diagnostics_snapshot(
        &self,
    ) -> Result<Vec<ops_service::SideEffectOutboxDiagnosticsView>, ContractError> {
        let records = lock_outbox_records(&self.records);
        Ok(build_message_side_effect_outbox_diagnostics(
            records.values(),
        ))
    }

    fn mark_delivered(&self, outbox_id: &str) -> Result<(), ContractError> {
        let mut records = lock_outbox_records(&self.records);
        if let Some(record) = records.get_mut(outbox_id) {
            record.status = MessageSideEffectOutboxStatus::Delivered;
            record.updated_at = im_time::utc_now_rfc3339_millis();
            record.last_error_code = None;
            record.last_error_message = None;
        }
        Ok(())
    }

    fn mark_failed(
        &self,
        outbox_id: &str,
        error_code: &str,
        error_message: &str,
    ) -> Result<(), ContractError> {
        let mut records = lock_outbox_records(&self.records);
        if let Some(record) = records.get_mut(outbox_id) {
            record.status = MessageSideEffectOutboxStatus::Pending;
            record.attempt_count = record.attempt_count.saturating_add(1);
            record.updated_at = im_time::utc_now_rfc3339_millis();
            record.last_error_code = Some(error_code.into());
            record.last_error_message = Some(error_message.into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(super) struct FileMessageSideEffectOutboxStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<std::sync::Mutex<()>>,
}

impl FileMessageSideEffectOutboxStore {
    pub(super) fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(std::sync::Mutex::new(())),
        }
    }

    fn read_records(
        &self,
    ) -> Result<BTreeMap<String, MessageSideEffectOutboxRecord>, ContractError> {
        read_outbox_records_or_default(self.file_path.as_path(), "message side-effect outbox")
    }
}

impl MessageSideEffectOutboxStore for FileMessageSideEffectOutboxStore {
    fn upsert_pending(
        &self,
        record: MessageSideEffectOutboxRecord,
    ) -> Result<MessageSideEffectOutboxRecord, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("message side-effect outbox file lock should lock");
        let outbox_id = record.outbox_id.clone();
        update_outbox_records(
            self.file_path.as_path(),
            "message side-effect outbox",
            move |records: &mut BTreeMap<String, MessageSideEffectOutboxRecord>| {
                let next = match records.remove(outbox_id.as_str()) {
                    Some(existing)
                        if matches!(existing.status, MessageSideEffectOutboxStatus::Delivered) =>
                    {
                        existing
                    }
                    Some(mut existing) => {
                        existing.payload = record.payload;
                        existing.event_type = record.event_type;
                        existing.scope_type = record.scope_type;
                        existing.scope_id = record.scope_id;
                        existing.status = MessageSideEffectOutboxStatus::Pending;
                        existing.updated_at = im_time::utc_now_rfc3339_millis();
                        existing
                    }
                    None => record,
                };
                records.insert(outbox_id, next.clone());
                next
            },
        )
    }

    fn list_pending(&self) -> Result<Vec<MessageSideEffectOutboxRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("message side-effect outbox file lock should lock");
        Ok(self
            .read_records()?
            .into_values()
            .filter(|record| matches!(record.status, MessageSideEffectOutboxStatus::Pending))
            .collect())
    }

    fn diagnostics_snapshot(
        &self,
    ) -> Result<Vec<ops_service::SideEffectOutboxDiagnosticsView>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("message side-effect outbox file lock should lock");
        let records = self.read_records()?;
        Ok(build_message_side_effect_outbox_diagnostics(
            records.values(),
        ))
    }

    fn mark_delivered(&self, outbox_id: &str) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("message side-effect outbox file lock should lock");
        let outbox_id = outbox_id.to_owned();
        update_outbox_records(
            self.file_path.as_path(),
            "message side-effect outbox",
            move |records: &mut BTreeMap<String, MessageSideEffectOutboxRecord>| {
                if let Some(record) = records.get_mut(outbox_id.as_str()) {
                    record.status = MessageSideEffectOutboxStatus::Delivered;
                    record.updated_at = im_time::utc_now_rfc3339_millis();
                    record.last_error_code = None;
                    record.last_error_message = None;
                }
            },
        )
    }

    fn mark_failed(
        &self,
        outbox_id: &str,
        error_code: &str,
        error_message: &str,
    ) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("message side-effect outbox file lock should lock");
        let outbox_id = outbox_id.to_owned();
        let error_code = error_code.to_owned();
        let error_message = error_message.to_owned();
        update_outbox_records(
            self.file_path.as_path(),
            "message side-effect outbox",
            move |records: &mut BTreeMap<String, MessageSideEffectOutboxRecord>| {
                if let Some(record) = records.get_mut(outbox_id.as_str()) {
                    record.status = MessageSideEffectOutboxStatus::Pending;
                    record.attempt_count = record.attempt_count.saturating_add(1);
                    record.updated_at = im_time::utc_now_rfc3339_millis();
                    record.last_error_code = Some(error_code);
                    record.last_error_message = Some(error_message);
                }
            },
        )
    }
}

pub(super) fn drain_pending_message_side_effect_outbox(
    state: &AppState,
) -> Result<(), ContractError> {
    let pending = state.message_side_effect_outbox.list_pending()?;
    for record in pending {
        let auth = record.auth_context();
        match record.side_effect.as_str() {
            "realtime_delivery" => match replay_message_realtime_side_effect(state, &auth, &record)
            {
                Ok(()) => state
                    .message_side_effect_outbox
                    .mark_delivered(record.outbox_id.as_str())?,
                Err(error) => state.message_side_effect_outbox.mark_failed(
                    record.outbox_id.as_str(),
                    error.code,
                    error.message.as_str(),
                )?,
            },
            "notification_delivery" => {
                match replay_message_notification_side_effect(state, &auth, record.payload.as_str())
                {
                    Ok(()) => state
                        .message_side_effect_outbox
                        .mark_delivered(record.outbox_id.as_str())?,
                    Err(error) => state.message_side_effect_outbox.mark_failed(
                        record.outbox_id.as_str(),
                        error.code(),
                        error.message(),
                    )?,
                }
            }
            _ => continue,
        }
    }
    Ok(())
}

fn replay_message_realtime_side_effect(
    state: &AppState,
    auth: &AppContext,
    record: &MessageSideEffectOutboxRecord,
) -> Result<(), ApiError> {
    if record.scope_type == "conversation" && record.event_type == "message.posted" {
        return effects::publish_realtime_conversation_message_event(
            state,
            auth,
            record.scope_id.as_str(),
            record.event_type.as_str(),
            record.payload.clone(),
        );
    }

    effects::publish_realtime_event_to_scope(
        state,
        auth,
        record.scope_type.as_str(),
        record.scope_id.as_str(),
        record.event_type.as_str(),
        record.payload.clone(),
    )
}

pub(super) fn record_pending_message_realtime_side_effect(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
    message_id: &str,
    message_seq: u64,
    event_type: &str,
    payload: String,
) -> Result<MessageSideEffectOutboxRecord, ContractError> {
    state.message_side_effect_outbox.upsert_pending(
        MessageSideEffectOutboxRecord::realtime_message_posted(
            auth,
            conversation_id,
            message_id,
            message_seq,
            event_type,
            payload,
        ),
    )
}

pub(super) fn record_pending_message_notification_side_effect(
    state: &AppState,
    auth: &AppContext,
    request: notification_service::RequestMessagePostedNotifications,
) -> Result<MessageSideEffectOutboxRecord, ContractError> {
    state.message_side_effect_outbox.upsert_pending(
        MessageSideEffectOutboxRecord::notification_message_posted(auth, request),
    )
}

pub(super) fn mark_message_side_effect_delivered(
    state: &AppState,
    outbox_id: &str,
) -> Result<(), ContractError> {
    state.message_side_effect_outbox.mark_delivered(outbox_id)
}

pub(super) fn mark_message_side_effect_failed(
    state: &AppState,
    outbox_id: &str,
    error: &ApiError,
) -> Result<(), ContractError> {
    state
        .message_side_effect_outbox
        .mark_failed(outbox_id, error.code, error.message.as_str())
}

pub(super) fn mark_message_notification_side_effect_failed(
    state: &AppState,
    outbox_id: &str,
    error: &notification_service::NotificationError,
) -> Result<(), ContractError> {
    state
        .message_side_effect_outbox
        .mark_failed(outbox_id, error.code(), error.message())
}

pub(super) fn message_side_effect_outbox_diagnostics(
    state: &AppState,
) -> Result<Vec<ops_service::SideEffectOutboxDiagnosticsView>, ContractError> {
    state.message_side_effect_outbox.diagnostics_snapshot()
}

fn replay_message_notification_side_effect(
    state: &AppState,
    auth: &AppContext,
    payload: &str,
) -> Result<(), notification_service::NotificationError> {
    let payload: MessageNotificationSideEffectPayload =
        serde_json::from_str(payload).map_err(|error| {
            notification_service::NotificationError::internal(
                "message_notification_outbox_payload_invalid",
                format!("failed to parse message notification outbox payload: {error}"),
            )
        })?;
    state
        .notification_runtime
        .request_message_posted_notifications(auth, payload.into())
        .map(|_| ())
}

fn message_side_effect_outbox_id(message_id: &str, side_effect: &str) -> String {
    stable_local_audit_record_id(
        "outbox_message_side_effect_",
        format!("{message_id}:{side_effect}").as_str(),
    )
}

fn build_message_side_effect_outbox_diagnostics<'a>(
    records: impl Iterator<Item = &'a MessageSideEffectOutboxRecord>,
) -> Vec<ops_service::SideEffectOutboxDiagnosticsView> {
    let mut records_by_side_effect = BTreeMap::<String, Vec<&MessageSideEffectOutboxRecord>>::new();
    for record in records {
        records_by_side_effect
            .entry(record.side_effect.clone())
            .or_default()
            .push(record);
    }
    for side_effect in ["notification_delivery", "realtime_delivery"] {
        records_by_side_effect
            .entry(side_effect.into())
            .or_default();
    }

    records_by_side_effect
        .into_iter()
        .map(|(side_effect, records)| build_side_effect_outbox_diagnostics(&side_effect, records))
        .collect()
}

fn build_side_effect_outbox_diagnostics(
    side_effect: &str,
    records: Vec<&MessageSideEffectOutboxRecord>,
) -> ops_service::SideEffectOutboxDiagnosticsView {
    let mut pending_count = 0_u64;
    let mut delivered_count = 0_u64;
    let mut failed_attempt_count = 0_u64;
    let mut oldest_pending_created_at: Option<String> = None;
    for record in records {
        failed_attempt_count = failed_attempt_count.saturating_add(record.attempt_count);
        match record.status {
            MessageSideEffectOutboxStatus::Pending => {
                pending_count = pending_count.saturating_add(1);
                if oldest_pending_created_at
                    .as_deref()
                    .is_none_or(|oldest| rfc3339_lt(record.created_at.as_str(), oldest))
                {
                    oldest_pending_created_at = Some(record.created_at.clone());
                }
            }
            MessageSideEffectOutboxStatus::Delivered => {
                delivered_count = delivered_count.saturating_add(1);
            }
        }
    }
    let status = if pending_count > 0 { "degraded" } else { "ok" };
    ops_service::SideEffectOutboxDiagnosticsView {
        name: format!("message_{side_effect}"),
        status: status.into(),
        pending_count,
        delivered_count,
        failed_attempt_count,
        oldest_pending_created_at,
    }
}

fn lock_outbox_records(
    records: &std::sync::Mutex<BTreeMap<String, MessageSideEffectOutboxRecord>>,
) -> std::sync::MutexGuard<'_, BTreeMap<String, MessageSideEffectOutboxRecord>> {
    match records.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn read_outbox_records_or_default<T>(
    file_path: &StdPath,
    store_name: &str,
) -> Result<T, ContractError>
where
    T: serde::de::DeserializeOwned + Default,
{
    recover_pending_outbox_temp_file(file_path, store_name)?;

    if !file_path.exists() {
        return Ok(T::default());
    }
    let bytes = fs::read(file_path).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to read {store_name} {}: {error}",
            file_path.display()
        ))
    })?;
    if bytes.is_empty() {
        return Ok(T::default());
    }
    serde_json::from_slice(&bytes).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to parse {store_name} {}: {error}",
            file_path.display()
        ))
    })
}

fn write_outbox_records<T: serde::Serialize + ?Sized>(
    file_path: &StdPath,
    records: &T,
    store_name: &str,
) -> Result<(), ContractError> {
    let parent = file_path.parent().ok_or_else(|| {
        ContractError::Unavailable(format!(
            "{store_name} path has no parent: {}",
            file_path.display()
        ))
    })?;
    fs::create_dir_all(parent).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to create {store_name} dir {}: {error}",
            parent.display()
        ))
    })?;
    let payload = serde_json::to_vec_pretty(records).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to serialize {store_name} {}: {error}",
            file_path.display()
        ))
    })?;

    let temp_path = outbox_temp_json_path(file_path);
    if temp_path.exists() {
        fs::remove_file(&temp_path).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to clear stale {store_name} temp file {}: {error}",
                temp_path.display()
            ))
        })?;
    }

    let mut temp_file = fs::File::create(&temp_path).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to create {store_name} temp file {}: {error}",
            temp_path.display()
        ))
    })?;
    temp_file.write_all(payload.as_slice()).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to write {store_name} temp file {}: {error}",
            temp_path.display()
        ))
    })?;
    temp_file.sync_all().map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to sync {store_name} temp file {}: {error}",
            temp_path.display()
        ))
    })?;
    drop(temp_file);

    fs::rename(&temp_path, file_path).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to finalize {store_name} {} from temp file {}: {error}",
            file_path.display(),
            temp_path.display()
        ))
    })
}

fn recover_pending_outbox_temp_file(
    file_path: &StdPath,
    store_name: &str,
) -> Result<(), ContractError> {
    let temp_path = outbox_temp_json_path(file_path);
    if !temp_path.exists() {
        return Ok(());
    }

    if file_path.exists() {
        return fs::remove_file(&temp_path).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to discard stale {store_name} temp file {}: {error}",
                temp_path.display()
            ))
        });
    }

    fs::rename(&temp_path, file_path).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to recover {store_name} from temp file {} to {}: {error}",
            temp_path.display(),
            file_path.display()
        ))
    })
}

fn outbox_temp_json_path(file_path: &StdPath) -> PathBuf {
    file_path.with_extension("json.tmp")
}

fn update_outbox_records<T, R>(
    file_path: &StdPath,
    store_name: &str,
    apply: impl FnOnce(&mut T) -> R,
) -> Result<R, ContractError>
where
    T: serde::de::DeserializeOwned + Default + serde::Serialize,
{
    let mut records = read_outbox_records_or_default(file_path, store_name)?;
    let result = apply(&mut records);
    write_outbox_records(file_path, &records, store_name)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_OUTBOX_TEST_RUNTIME_DIR_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    fn unique_outbox_test_dir(prefix: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let sequence = NEXT_OUTBOX_TEST_RUNTIME_DIR_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "sdkwork_im_message_side_effect_outbox_{prefix}_{unique}_{sequence}"
        ))
    }

    fn test_auth_context() -> AppContext {
        AppContext {
            tenant_id: "t_demo".into(),
            organization_id: String::new(),
            user_id: "u_demo".into(),
            actor_id: "u_demo".into(),
            actor_kind: "user".into(),
            session_id: Some("s_demo".into()),
            app_id: Some("sdkwork-im".into()),
            environment: None,
            deployment_mode: None,
            auth_level: None,
            data_scope: BTreeSet::new(),
            permission_scope: BTreeSet::new(),
            device_id: Some("d_demo".into()),
        }
    }

    fn pending_record(message_id: &str) -> MessageSideEffectOutboxRecord {
        MessageSideEffectOutboxRecord::realtime_message_posted(
            &test_auth_context(),
            "c_demo",
            message_id,
            1,
            "message.posted",
            serde_json::json!({
                "conversationId": "c_demo",
                "messageId": message_id,
                "messageSeq": 1
            })
            .to_string(),
        )
    }

    fn pending_record_created_at(
        message_id: &str,
        created_at: &str,
    ) -> MessageSideEffectOutboxRecord {
        let mut record = pending_record(message_id);
        record.created_at = created_at.into();
        record.updated_at = created_at.into();
        record
    }

    fn write_record_map(path: &StdPath, record: MessageSideEffectOutboxRecord) {
        let mut records = BTreeMap::new();
        records.insert(record.outbox_id.clone(), record);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("outbox test dir should be created");
        }
        fs::write(
            path,
            serde_json::to_vec_pretty(&records).expect("outbox records should serialize"),
        )
        .expect("outbox record map should be writable");
    }

    #[test]
    fn test_file_message_side_effect_outbox_recovers_temp_file_when_primary_missing() {
        let runtime_dir = unique_outbox_test_dir("temp_recovery");
        let file_path = runtime_dir
            .join("state")
            .join("message-side-effect-outbox.json");
        let temp_path = file_path.with_extension("json.tmp");
        write_record_map(temp_path.as_path(), pending_record("m_temp_only"));

        let store = FileMessageSideEffectOutboxStore::new(file_path.clone());
        let pending = store
            .list_pending()
            .expect("outbox should recover pending temp file");

        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].message_id, "m_temp_only");
        assert!(
            file_path.exists(),
            "recovered outbox primary file should be materialized"
        );
        assert!(
            !temp_path.exists(),
            "pending outbox temp file should be consumed after recovery"
        );

        let _ = fs::remove_dir_all(runtime_dir);
    }

    #[test]
    fn test_file_message_side_effect_outbox_discards_temp_file_when_primary_exists() {
        let runtime_dir = unique_outbox_test_dir("stale_temp_discard");
        let file_path = runtime_dir
            .join("state")
            .join("message-side-effect-outbox.json");
        let temp_path = file_path.with_extension("json.tmp");
        write_record_map(file_path.as_path(), pending_record("m_primary"));
        write_record_map(temp_path.as_path(), pending_record("m_stale_temp"));

        let store = FileMessageSideEffectOutboxStore::new(file_path.clone());
        let pending = store
            .list_pending()
            .expect("outbox should read committed primary file");

        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].message_id, "m_primary");
        assert!(
            !temp_path.exists(),
            "stale outbox temp file should be discarded when primary exists"
        );

        let _ = fs::remove_dir_all(runtime_dir);
    }

    #[test]
    fn test_side_effect_outbox_diagnostics_compares_oldest_pending_by_rfc3339_instant() {
        let later_fraction =
            pending_record_created_at("m_later_fraction", "2026-05-06T00:00:00.100Z");
        let whole_second = pending_record_created_at("m_whole_second", "2026-05-06T00:00:00Z");

        let diagnostics = build_side_effect_outbox_diagnostics(
            "realtime_delivery",
            vec![&later_fraction, &whole_second],
        );

        assert_eq!(
            diagnostics.oldest_pending_created_at.as_deref(),
            Some("2026-05-06T00:00:00Z")
        );
    }
}
