use super::*;
use im_adapters_local_disk::{
    validate_metadata_store_file, validate_timeline_projection_store_file,
};
use im_time::utc_now_rfc3339_millis;

mod preview;

const EXPECTED_RUNTIME_STATE_FILES: [&str; 13] = [
    "commit-journal.json",
    "realtime-disconnect-fences.json",
    "realtime-checkpoints.json",
    "realtime-event-windows.json",
    "realtime-subscriptions.json",
    "presence-state.json",
    "device-twin-state.json",
    "stream-state.json",
    "rtc-state.json",
    "notification-tasks.json",
    "automation-executions.json",
    "projection-metadata.json",
    "projection-timeline.json",
];
const DEFAULT_ARCHIVE_RETENTION_DAYS: u64 = 30;

struct RuntimeStateValidationFailure {
    parseable: bool,
    error: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirRepairActionView {
    pub file_name: String,
    pub path: String,
    pub status: String,
    pub detail: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirRepairView {
    pub status: String,
    pub runtime_dir: String,
    pub backup_dir: Option<String>,
    pub repaired_file_count: usize,
    pub skipped_file_count: usize,
    pub before: RuntimeDirInspectionView,
    pub after: RuntimeDirInspectionView,
    pub actions: Vec<RuntimeDirRepairActionView>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirRestoreView {
    pub status: String,
    pub runtime_dir: String,
    pub source_backup_dir: String,
    pub confirmed_preview_fingerprint: Option<String>,
    pub pre_restore_backup_dir: Option<String>,
    pub restored_file_count: usize,
    pub skipped_file_count: usize,
    pub before: RuntimeDirInspectionView,
    pub after: RuntimeDirInspectionView,
    pub actions: Vec<RuntimeDirRepairActionView>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirArchiveView {
    pub status: String,
    pub runtime_dir: String,
    pub source_backup_dir: String,
    pub archived_backup_dir: String,
    pub archived_backup_name: String,
    pub operation: String,
    pub snapshot_quality: String,
    pub managed_file_count: usize,
    pub missing_file_count: usize,
    pub storage_class: String,
    pub retention_policy: String,
    pub retention_days: u64,
    pub restore_status: String,
    pub legal_hold: bool,
    pub archived_at: String,
    pub restore_from_backup_dir: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct RuntimeDirArchiveMetadata {
    storage_class: String,
    retention_policy: String,
    retention_days: u64,
    restore_status: String,
    legal_hold: bool,
    archived_at: String,
    archived_at_unix_seconds: u64,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirArchivePruneActionView {
    pub backup_name: String,
    pub backup_dir: String,
    pub status: String,
    pub detail: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirArchivePruneView {
    pub status: String,
    pub runtime_dir: String,
    pub backups_dir: String,
    pub inspected_backup_count: usize,
    pub pruned_backup_count: usize,
    pub skipped_backup_count: usize,
    pub actions: Vec<RuntimeDirArchivePruneActionView>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirBackupCatalogItemView {
    pub backup_name: String,
    pub backup_dir: String,
    pub operation: String,
    pub lifecycle_stage: String,
    pub has_state_dir: bool,
    pub snapshot_quality: String,
    pub managed_file_count: usize,
    pub missing_file_count: usize,
    pub report_type: Option<String>,
    pub report_status: Option<String>,
    pub storage_class: Option<String>,
    pub retention_policy: Option<String>,
    pub retention_days: Option<u64>,
    pub restore_status: Option<String>,
    pub legal_hold: bool,
    pub archived_at: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirBackupCatalogView {
    pub status: String,
    pub runtime_dir: String,
    pub backups_dir: String,
    pub backup_count: usize,
    pub items: Vec<RuntimeDirBackupCatalogItemView>,
}

pub use preview::{
    RuntimeDirRestorePreviewActionView, RuntimeDirRestorePreviewChangeSummaryView,
    RuntimeDirRestorePreviewDomainSummaryView, RuntimeDirRestorePreviewView,
    format_runtime_dir_restore_preview, preview_restore_runtime_dir,
};

#[derive(Clone, Debug)]
struct RuntimeBackupSnapshotSummary {
    backup_name: String,
    backup_dir: String,
    operation: String,
    lifecycle_stage: String,
    has_state_dir: bool,
    snapshot_quality: String,
    managed_file_count: usize,
    missing_file_count: usize,
    report_type: Option<String>,
    report_status: Option<String>,
    storage_class: Option<String>,
    retention_policy: Option<String>,
    retention_days: Option<u64>,
    restore_status: Option<String>,
    legal_hold: bool,
    archived_at: Option<String>,
}

fn contract_error_message(error: ContractError) -> String {
    match error {
        ContractError::UnsupportedCapability(message)
        | ContractError::Conflict(message)
        | ContractError::Unavailable(message) => message,
    }
}

fn runtime_state_parse_failure(error: ContractError) -> RuntimeStateValidationFailure {
    RuntimeStateValidationFailure {
        parseable: false,
        error: contract_error_message(error),
    }
}

pub(crate) fn apply_projection_journal_envelopes(
    recorded: &[CommitEnvelope],
    projection_service: &TimelineProjectionService,
    conversation_runtime: &ConversationRuntime<ProjectionJournal>,
    surface: &str,
) -> Result<(), String> {
    for envelope in recorded {
        projection_service.apply(envelope).map_err(|error| {
            format!(
                "failed to replay projection event {} during {surface}: {error:?}",
                envelope.event_id
            )
        })?;
        conversation_runtime
            .apply_recovered_envelope(envelope)
            .map_err(|error| {
                format!(
                    "failed to replay conversation event {} during {surface}: {error:?}",
                    envelope.event_id
                )
            })?;
    }

    Ok(())
}

fn validate_projection_journal_file(
    file_path: &StdPath,
) -> Result<(), RuntimeStateValidationFailure> {
    validate_commit_journal_file(file_path).map_err(runtime_state_parse_failure)?;
    let recorded = read_commit_journal_file(file_path).map_err(runtime_state_parse_failure)?;
    let projection_service = Arc::new(TimelineProjectionService::default());
    let conversation_runtime =
        ConversationRuntime::new(ProjectionJournal::new_memory(projection_service.clone()));

    apply_projection_journal_envelopes(
        recorded.as_slice(),
        projection_service.as_ref(),
        &conversation_runtime,
        "runtime_dir inspection",
    )
    .map_err(|error| RuntimeStateValidationFailure {
        parseable: true,
        error,
    })
}

fn validate_runtime_state_file(
    file_name: &str,
    file_path: &StdPath,
) -> Result<(), RuntimeStateValidationFailure> {
    match file_name {
        "commit-journal.json" => validate_projection_journal_file(file_path),
        "realtime-disconnect-fences.json" => {
            validate_realtime_disconnect_fence_store_file(file_path)
                .map_err(runtime_state_parse_failure)
        }
        "realtime-checkpoints.json" => {
            validate_realtime_checkpoint_store_file(file_path).map_err(runtime_state_parse_failure)
        }
        "realtime-event-windows.json" => validate_realtime_event_window_store_file(file_path)
            .map_err(runtime_state_parse_failure),
        "realtime-subscriptions.json" => validate_realtime_subscription_store_file(file_path)
            .map_err(runtime_state_parse_failure),
        "presence-state.json" => {
            validate_presence_state_store_file(file_path).map_err(runtime_state_parse_failure)
        }
        "device-twin-state.json" => {
            validate_device_twin_store_file(file_path).map_err(runtime_state_parse_failure)
        }
        "stream-state.json" => {
            validate_stream_state_store_file(file_path).map_err(runtime_state_parse_failure)
        }
        "rtc-state.json" => {
            validate_rtc_state_store_file(file_path).map_err(runtime_state_parse_failure)
        }
        "notification-tasks.json" => {
            validate_notification_task_store_file(file_path).map_err(runtime_state_parse_failure)
        }
        "automation-executions.json" => {
            validate_automation_execution_store_file(file_path).map_err(runtime_state_parse_failure)
        }
        "projection-metadata.json" => {
            validate_metadata_store_file(file_path).map_err(runtime_state_parse_failure)
        }
        "projection-timeline.json" => {
            validate_timeline_projection_store_file(file_path).map_err(runtime_state_parse_failure)
        }
        _ => Ok(()),
    }
}

fn empty_runtime_state_file_content(file_name: &str) -> &'static str {
    match file_name {
        "commit-journal.json" => "",
        "presence-state.json" => {
            "{\"by_device\":{},\"presence_by_principal\":{},\"online_by_seen_at\":{}}\n"
        }
        "notification-tasks.json" => "{\"by_notification\":{},\"tasks_by_recipient\":{}}\n",
        _ => "{}\n",
    }
}

fn runtime_dir_operation_backup_dir(runtime_dir: &StdPath, operation: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    runtime_dir
        .join("backups")
        .join(format!("runtime_dir-{operation}-{timestamp}"))
}

fn runtime_dir_repair_backup_dir(runtime_dir: &StdPath) -> PathBuf {
    runtime_dir_operation_backup_dir(runtime_dir, "repair")
}

fn runtime_dir_restore_backup_dir(runtime_dir: &StdPath) -> PathBuf {
    runtime_dir_operation_backup_dir(runtime_dir, "restore")
}

fn runtime_dir_archive_backup_dir(runtime_dir: &StdPath, backup_name: &str) -> PathBuf {
    runtime_dir
        .join("backups")
        .join(format!("archived-{backup_name}"))
}

fn runtime_backup_lifecycle_stage(backup_name: &str) -> &'static str {
    if backup_name.starts_with("archived-") {
        "archived"
    } else {
        "active"
    }
}

fn runtime_backup_operation(backup_name: &str) -> &'static str {
    let base_name = backup_name.strip_prefix("archived-").unwrap_or(backup_name);
    if base_name.starts_with("runtime_dir-repair-") {
        "repair"
    } else if base_name.starts_with("runtime_dir-restore-") {
        "restore"
    } else {
        "unknown"
    }
}

fn runtime_backup_snapshot_quality(managed_file_count: usize) -> &'static str {
    if managed_file_count == 0 {
        "empty_snapshot"
    } else if managed_file_count == EXPECTED_RUNTIME_STATE_FILES.len() {
        "full_snapshot"
    } else {
        "partial_snapshot"
    }
}

fn runtime_backup_report_preview(backup_dir: &StdPath) -> (Option<String>, Option<String>) {
    for (report_type, report_file_name) in [
        ("archive", "archive-report.json"),
        ("restore", "restore-report.json"),
        ("repair", "repair-report.json"),
    ] {
        let report_path = backup_dir.join(report_file_name);
        if !report_path.exists() {
            continue;
        }

        let report_status = fs::read(&report_path)
            .ok()
            .and_then(|payload| serde_json::from_slice::<serde_json::Value>(&payload).ok())
            .and_then(|value| {
                value
                    .get("status")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_owned)
            });
        return (Some(report_type.into()), report_status);
    }

    (None, None)
}

fn runtime_backup_archive_metadata_path(backup_dir: &StdPath) -> PathBuf {
    backup_dir.join("archive-metadata.json")
}

fn read_runtime_backup_archive_metadata(backup_dir: &StdPath) -> Option<RuntimeDirArchiveMetadata> {
    let metadata_path = runtime_backup_archive_metadata_path(backup_dir);
    fs::read(&metadata_path)
        .ok()
        .and_then(|payload| serde_json::from_slice::<RuntimeDirArchiveMetadata>(&payload).ok())
}

fn describe_runtime_backup_snapshot(
    backup_dir: &StdPath,
    backup_name: impl Into<String>,
) -> RuntimeBackupSnapshotSummary {
    let backup_name = backup_name.into();
    let state_dir = backup_dir.join("state");
    let has_state_dir = state_dir.exists();
    let managed_file_count = EXPECTED_RUNTIME_STATE_FILES
        .iter()
        .filter(|file_name| state_dir.join(file_name).exists())
        .count();
    let missing_file_count = EXPECTED_RUNTIME_STATE_FILES.len() - managed_file_count;
    let (report_type, report_status) = runtime_backup_report_preview(backup_dir);
    let archive_metadata = read_runtime_backup_archive_metadata(backup_dir);

    RuntimeBackupSnapshotSummary {
        backup_name: backup_name.clone(),
        backup_dir: backup_dir.display().to_string(),
        operation: runtime_backup_operation(backup_name.as_str()).into(),
        lifecycle_stage: runtime_backup_lifecycle_stage(backup_name.as_str()).into(),
        has_state_dir,
        snapshot_quality: runtime_backup_snapshot_quality(managed_file_count).into(),
        managed_file_count,
        missing_file_count,
        report_type,
        report_status,
        storage_class: archive_metadata
            .as_ref()
            .map(|metadata| metadata.storage_class.clone()),
        retention_policy: archive_metadata
            .as_ref()
            .map(|metadata| metadata.retention_policy.clone()),
        retention_days: archive_metadata
            .as_ref()
            .map(|metadata| metadata.retention_days),
        restore_status: archive_metadata
            .as_ref()
            .map(|metadata| metadata.restore_status.clone()),
        legal_hold: archive_metadata
            .as_ref()
            .map(|metadata| metadata.legal_hold)
            .unwrap_or(false),
        archived_at: archive_metadata
            .as_ref()
            .map(|metadata| metadata.archived_at.clone()),
    }
}

fn validate_runtime_backup_source(
    backup_dir: &StdPath,
) -> Result<(PathBuf, RuntimeBackupSnapshotSummary), String> {
    if !backup_dir.exists() {
        return Err(format!(
            "backup dir does not exist: {}",
            backup_dir.display()
        ));
    }

    let source_state_dir = backup_dir.join("state");
    if !source_state_dir.exists() {
        return Err(format!(
            "backup state dir does not exist: {}",
            source_state_dir.display()
        ));
    }

    let backup_name = backup_dir
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| backup_dir.display().to_string());
    let summary = describe_runtime_backup_snapshot(backup_dir, backup_name);
    Ok((source_state_dir, summary))
}

fn snapshot_runtime_state_files(
    state_dir: &StdPath,
    backup_dir: &StdPath,
    operation: &str,
) -> Result<(), String> {
    let backup_state_dir = backup_dir.join("state");
    fs::create_dir_all(&backup_state_dir).map_err(|error| {
        format!(
            "failed to create runtime_dir {operation} backup state dir {}: {error}",
            backup_state_dir.display()
        )
    })?;

    for file_name in EXPECTED_RUNTIME_STATE_FILES {
        let source = state_dir.join(file_name);
        if !source.exists() {
            continue;
        }

        let target = backup_state_dir.join(file_name);
        fs::copy(&source, &target).map_err(|error| {
            format!(
                "failed to snapshot runtime_dir state file {} to {} during {operation}: {error}",
                source.display(),
                target.display()
            )
        })?;
    }

    Ok(())
}

fn write_runtime_dir_repair_report(
    backup_dir: &StdPath,
    report: &RuntimeDirRepairView,
) -> Result<(), String> {
    let report_path = backup_dir.join("repair-report.json");
    let payload = serde_json::to_vec_pretty(report)
        .map_err(|error| format!("runtime_dir repair report should serialize to json: {error}"))?;
    fs::write(&report_path, payload).map_err(|error| {
        format!(
            "failed to write runtime_dir repair report {}: {error}",
            report_path.display()
        )
    })?;
    Ok(())
}

fn write_runtime_dir_restore_report(
    backup_dir: &StdPath,
    report: &RuntimeDirRestoreView,
) -> Result<(), String> {
    let report_path = backup_dir.join("restore-report.json");
    let payload = serde_json::to_vec_pretty(report)
        .map_err(|error| format!("runtime_dir restore report should serialize to json: {error}"))?;
    fs::write(&report_path, payload).map_err(|error| {
        format!(
            "failed to write runtime_dir restore report {}: {error}",
            report_path.display()
        )
    })?;
    Ok(())
}

fn write_runtime_dir_archive_report(
    backup_dir: &StdPath,
    report: &RuntimeDirArchiveView,
) -> Result<(), String> {
    let report_path = backup_dir.join("archive-report.json");
    let payload = serde_json::to_vec_pretty(report)
        .map_err(|error| format!("runtime_dir archive report should serialize to json: {error}"))?;
    fs::write(&report_path, payload).map_err(|error| {
        format!(
            "failed to write runtime_dir archive report {}: {error}",
            report_path.display()
        )
    })?;
    Ok(())
}

fn write_runtime_dir_archive_metadata(
    backup_dir: &StdPath,
    metadata: &RuntimeDirArchiveMetadata,
) -> Result<(), String> {
    let metadata_path = runtime_backup_archive_metadata_path(backup_dir);
    let payload = serde_json::to_vec_pretty(metadata).map_err(|error| {
        format!("runtime_dir archive metadata should serialize to json: {error}")
    })?;
    fs::write(&metadata_path, payload).map_err(|error| {
        format!(
            "failed to write runtime_dir archive metadata {}: {error}",
            metadata_path.display()
        )
    })?;
    Ok(())
}

fn current_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn archive_retention_elapsed(metadata: &RuntimeDirArchiveMetadata) -> bool {
    let retention_seconds = metadata.retention_days.saturating_mul(86_400);
    current_unix_seconds()
        >= metadata
            .archived_at_unix_seconds
            .saturating_add(retention_seconds)
}

pub fn repair_runtime_dir(
    runtime_dir: impl AsRef<StdPath>,
) -> Result<RuntimeDirRepairView, String> {
    let runtime_dir = runtime_dir.as_ref();
    let state_dir = runtime_dir.join("state");
    let before = inspect_runtime_dir(runtime_dir);
    let backup_dir = runtime_dir_repair_backup_dir(runtime_dir);

    fs::create_dir_all(&state_dir).map_err(|error| {
        format!(
            "failed to create runtime_dir state dir {} before repair: {error}",
            state_dir.display()
        )
    })?;
    fs::create_dir_all(&backup_dir).map_err(|error| {
        format!(
            "failed to create runtime_dir repair backup dir {}: {error}",
            backup_dir.display()
        )
    })?;
    snapshot_runtime_state_files(state_dir.as_path(), backup_dir.as_path(), "repair")?;

    let mut actions = Vec::new();
    let mut repaired_file_count = 0usize;
    let mut skipped_file_count = 0usize;

    for file in &before.files {
        let target_path = state_dir.join(file.file_name.as_str());
        match file.status.as_str() {
            "missing" => {
                fs::write(
                    &target_path,
                    empty_runtime_state_file_content(file.file_name.as_str()),
                )
                .map_err(|error| {
                    format!(
                        "failed to recreate missing runtime_dir file {}: {error}",
                        target_path.display()
                    )
                })?;
                actions.push(RuntimeDirRepairActionView {
                    file_name: file.file_name.clone(),
                    path: target_path.display().to_string(),
                    status: "repaired".into(),
                    detail: "recreated_missing_file".into(),
                });
                repaired_file_count += 1;
            }
            "corrupt" => {
                actions.push(RuntimeDirRepairActionView {
                    file_name: file.file_name.clone(),
                    path: target_path.display().to_string(),
                    status: "skipped".into(),
                    detail: "left_corrupt_file_untouched".into(),
                });
                skipped_file_count += 1;
            }
            _ => {}
        }
    }

    let after = inspect_runtime_dir(runtime_dir);
    let status = if repaired_file_count > 0 && after.status == "ok" {
        "repaired"
    } else if repaired_file_count > 0 || skipped_file_count > 0 {
        "partial"
    } else {
        "noop"
    };

    let report = RuntimeDirRepairView {
        status: status.into(),
        runtime_dir: runtime_dir.display().to_string(),
        backup_dir: Some(backup_dir.display().to_string()),
        repaired_file_count,
        skipped_file_count,
        before,
        after,
        actions,
    };
    write_runtime_dir_repair_report(backup_dir.as_path(), &report)?;
    Ok(report)
}

pub fn restore_runtime_dir(
    runtime_dir: impl AsRef<StdPath>,
    backup_dir: impl AsRef<StdPath>,
) -> Result<RuntimeDirRestoreView, String> {
    restore_runtime_dir_with_expected_preview_fingerprint(runtime_dir, backup_dir, None)
}

pub fn restore_runtime_dir_with_expected_preview_fingerprint(
    runtime_dir: impl AsRef<StdPath>,
    backup_dir: impl AsRef<StdPath>,
    expected_preview_fingerprint: Option<&str>,
) -> Result<RuntimeDirRestoreView, String> {
    let runtime_dir = runtime_dir.as_ref();
    let backup_dir = backup_dir.as_ref();
    let confirmed_preview_fingerprint = if let Some(expected_preview_fingerprint) =
        expected_preview_fingerprint
    {
        let preview = preview_restore_runtime_dir(runtime_dir, backup_dir)?;
        if preview.preview_fingerprint != expected_preview_fingerprint {
            return Err(format!(
                "preview fingerprint mismatch: expected {expected_preview_fingerprint}, actual {}",
                preview.preview_fingerprint
            ));
        }
        Some(preview.preview_fingerprint)
    } else {
        None
    };
    let (source_state_dir, _) = validate_runtime_backup_source(backup_dir)?;

    let state_dir = runtime_dir.join("state");
    let before = inspect_runtime_dir(runtime_dir);
    let pre_restore_backup_dir = runtime_dir_restore_backup_dir(runtime_dir);

    fs::create_dir_all(&state_dir).map_err(|error| {
        format!(
            "failed to create runtime_dir state dir {} before restore: {error}",
            state_dir.display()
        )
    })?;
    fs::create_dir_all(&pre_restore_backup_dir).map_err(|error| {
        format!(
            "failed to create runtime_dir restore backup dir {}: {error}",
            pre_restore_backup_dir.display()
        )
    })?;
    snapshot_runtime_state_files(
        state_dir.as_path(),
        pre_restore_backup_dir.as_path(),
        "restore",
    )?;

    let mut actions = Vec::new();
    let mut restored_file_count = 0usize;
    let mut skipped_file_count = 0usize;

    for file_name in EXPECTED_RUNTIME_STATE_FILES {
        let source = source_state_dir.join(file_name);
        let target = state_dir.join(file_name);
        if source.exists() {
            fs::copy(&source, &target).map_err(|error| {
                format!(
                    "failed to restore runtime_dir state file {} to {}: {error}",
                    source.display(),
                    target.display()
                )
            })?;
            actions.push(RuntimeDirRepairActionView {
                file_name: file_name.into(),
                path: target.display().to_string(),
                status: "restored".into(),
                detail: "copied_from_backup_snapshot".into(),
            });
            restored_file_count += 1;
        } else {
            actions.push(RuntimeDirRepairActionView {
                file_name: file_name.into(),
                path: target.display().to_string(),
                status: "skipped".into(),
                detail: "missing_in_source_backup_snapshot".into(),
            });
            skipped_file_count += 1;
        }
    }

    let after = inspect_runtime_dir(runtime_dir);
    let status = if restored_file_count > 0 && after.status == "ok" {
        "restored"
    } else if restored_file_count > 0 || skipped_file_count > 0 {
        "partial"
    } else {
        "noop"
    };

    let report = RuntimeDirRestoreView {
        status: status.into(),
        runtime_dir: runtime_dir.display().to_string(),
        source_backup_dir: backup_dir.display().to_string(),
        confirmed_preview_fingerprint,
        pre_restore_backup_dir: Some(pre_restore_backup_dir.display().to_string()),
        restored_file_count,
        skipped_file_count,
        before,
        after,
        actions,
    };
    write_runtime_dir_restore_report(pre_restore_backup_dir.as_path(), &report)?;
    Ok(report)
}

pub fn archive_runtime_backup(
    runtime_dir: impl AsRef<StdPath>,
    backup_dir: impl AsRef<StdPath>,
) -> Result<RuntimeDirArchiveView, String> {
    archive_runtime_backup_with_policy(
        runtime_dir.as_ref(),
        backup_dir.as_ref(),
        DEFAULT_ARCHIVE_RETENTION_DAYS,
        false,
    )
}

pub fn archive_runtime_backup_with_policy(
    runtime_dir: impl AsRef<StdPath>,
    backup_dir: impl AsRef<StdPath>,
    retention_days: u64,
    legal_hold: bool,
) -> Result<RuntimeDirArchiveView, String> {
    let runtime_dir = runtime_dir.as_ref();
    let backup_dir = backup_dir.as_ref();
    let backups_dir = runtime_dir.join("backups");
    let source_backup_dir_text = backup_dir.display().to_string();
    let canonical_backups_dir = fs::canonicalize(&backups_dir).map_err(|error| {
        format!(
            "failed to resolve runtime_dir backups dir {}: {error}",
            backups_dir.display()
        )
    })?;
    let canonical_source_backup_dir = fs::canonicalize(backup_dir).map_err(|error| {
        format!(
            "failed to resolve backup dir {}: {error}",
            backup_dir.display()
        )
    })?;
    if canonical_source_backup_dir.parent() != Some(canonical_backups_dir.as_path()) {
        return Err(format!(
            "backup dir must be a direct child of runtime_dir backups dir {}",
            canonical_backups_dir.display()
        ));
    }

    let (_, source_summary) = validate_runtime_backup_source(backup_dir)?;
    if source_summary.lifecycle_stage == "archived" {
        return Err(format!(
            "backup dir is already archived: {}",
            backup_dir.display()
        ));
    }
    if source_summary.operation == "unknown" {
        return Err(format!(
            "backup dir is not a managed runtime_dir backup snapshot: {}",
            backup_dir.display()
        ));
    }

    let archived_backup_dir =
        runtime_dir_archive_backup_dir(runtime_dir, source_summary.backup_name.as_str());
    if archived_backup_dir.exists() {
        return Err(format!(
            "archived backup dir already exists: {}",
            archived_backup_dir.display()
        ));
    }

    fs::rename(
        canonical_source_backup_dir.as_path(),
        archived_backup_dir.as_path(),
    )
    .map_err(|error| {
        format!(
            "failed to archive runtime backup {} to {}: {error}",
            backup_dir.display(),
            archived_backup_dir.display()
        )
    })?;

    let archived_backup_name = archived_backup_dir
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| archived_backup_dir.display().to_string());
    let archived_backup_dir_text = archived_backup_dir.display().to_string();
    let archive_metadata = RuntimeDirArchiveMetadata {
        storage_class: "archive".into(),
        retention_policy: format!("retain_for_days:{retention_days}"),
        retention_days,
        restore_status: "available".into(),
        legal_hold,
        archived_at: utc_now_rfc3339_millis(),
        archived_at_unix_seconds: current_unix_seconds(),
    };
    write_runtime_dir_archive_metadata(archived_backup_dir.as_path(), &archive_metadata)?;
    let report = RuntimeDirArchiveView {
        status: "archived".into(),
        runtime_dir: runtime_dir.display().to_string(),
        source_backup_dir: source_backup_dir_text,
        archived_backup_dir: archived_backup_dir_text.clone(),
        archived_backup_name,
        operation: source_summary.operation,
        snapshot_quality: source_summary.snapshot_quality,
        managed_file_count: source_summary.managed_file_count,
        missing_file_count: source_summary.missing_file_count,
        storage_class: archive_metadata.storage_class.clone(),
        retention_policy: archive_metadata.retention_policy.clone(),
        retention_days: archive_metadata.retention_days,
        restore_status: archive_metadata.restore_status.clone(),
        legal_hold: archive_metadata.legal_hold,
        archived_at: archive_metadata.archived_at.clone(),
        restore_from_backup_dir: archived_backup_dir_text,
    };
    write_runtime_dir_archive_report(archived_backup_dir.as_path(), &report)?;
    Ok(report)
}

pub fn prune_archived_runtime_backups(
    runtime_dir: impl AsRef<StdPath>,
) -> Result<RuntimeDirArchivePruneView, String> {
    let runtime_dir = runtime_dir.as_ref();
    let backups_dir = runtime_dir.join("backups");
    let mut inspected_backup_count = 0usize;
    let mut pruned_backup_count = 0usize;
    let mut skipped_backup_count = 0usize;
    let mut actions = Vec::new();

    if backups_dir.exists() {
        let entries = fs::read_dir(&backups_dir).map_err(|error| {
            format!(
                "failed to read runtime_dir backups dir {}: {error}",
                backups_dir.display()
            )
        })?;

        for entry in entries {
            let entry = entry.map_err(|error| {
                format!(
                    "failed to read runtime_dir backup directory entry under {}: {error}",
                    backups_dir.display()
                )
            })?;
            let backup_dir = entry.path();
            if !backup_dir.is_dir() {
                continue;
            }

            let backup_name = entry.file_name().to_string_lossy().into_owned();
            if runtime_backup_lifecycle_stage(backup_name.as_str()) != "archived" {
                continue;
            }

            inspected_backup_count += 1;

            let Some(metadata) = read_runtime_backup_archive_metadata(backup_dir.as_path()) else {
                skipped_backup_count += 1;
                actions.push(RuntimeDirArchivePruneActionView {
                    backup_name,
                    backup_dir: backup_dir.display().to_string(),
                    status: "skipped".into(),
                    detail: "missing_archive_metadata".into(),
                });
                continue;
            };

            if metadata.legal_hold {
                skipped_backup_count += 1;
                actions.push(RuntimeDirArchivePruneActionView {
                    backup_name,
                    backup_dir: backup_dir.display().to_string(),
                    status: "skipped".into(),
                    detail: "legal_hold".into(),
                });
                continue;
            }

            if !archive_retention_elapsed(&metadata) {
                skipped_backup_count += 1;
                actions.push(RuntimeDirArchivePruneActionView {
                    backup_name,
                    backup_dir: backup_dir.display().to_string(),
                    status: "skipped".into(),
                    detail: "retention_not_elapsed".into(),
                });
                continue;
            }

            fs::remove_dir_all(backup_dir.as_path()).map_err(|error| {
                format!(
                    "failed to prune archived runtime backup {}: {error}",
                    backup_dir.display()
                )
            })?;
            pruned_backup_count += 1;
            actions.push(RuntimeDirArchivePruneActionView {
                backup_name,
                backup_dir: backup_dir.display().to_string(),
                status: "pruned".into(),
                detail: "retention_elapsed".into(),
            });
        }
    }

    actions.sort_by(|left, right| left.backup_name.cmp(&right.backup_name));

    Ok(RuntimeDirArchivePruneView {
        status: "pruned".into(),
        runtime_dir: runtime_dir.display().to_string(),
        backups_dir: backups_dir.display().to_string(),
        inspected_backup_count,
        pruned_backup_count,
        skipped_backup_count,
        actions,
    })
}

pub fn list_runtime_backups(
    runtime_dir: impl AsRef<StdPath>,
) -> Result<RuntimeDirBackupCatalogView, String> {
    let runtime_dir = runtime_dir.as_ref();
    let backups_dir = runtime_dir.join("backups");
    let mut items = Vec::new();

    if backups_dir.exists() {
        let entries = fs::read_dir(&backups_dir).map_err(|error| {
            format!(
                "failed to read runtime_dir backups dir {}: {error}",
                backups_dir.display()
            )
        })?;

        for entry in entries {
            let entry = entry.map_err(|error| {
                format!(
                    "failed to read runtime_dir backup directory entry under {}: {error}",
                    backups_dir.display()
                )
            })?;
            let backup_dir = entry.path();
            if !backup_dir.is_dir() {
                continue;
            }

            let summary = describe_runtime_backup_snapshot(
                backup_dir.as_path(),
                entry.file_name().to_string_lossy(),
            );

            items.push(RuntimeDirBackupCatalogItemView {
                backup_name: summary.backup_name,
                backup_dir: summary.backup_dir,
                operation: summary.operation,
                lifecycle_stage: summary.lifecycle_stage,
                has_state_dir: summary.has_state_dir,
                snapshot_quality: summary.snapshot_quality,
                managed_file_count: summary.managed_file_count,
                missing_file_count: summary.missing_file_count,
                report_type: summary.report_type,
                report_status: summary.report_status,
                storage_class: summary.storage_class,
                retention_policy: summary.retention_policy,
                retention_days: summary.retention_days,
                restore_status: summary.restore_status,
                legal_hold: summary.legal_hold,
                archived_at: summary.archived_at,
            });
        }
    }

    items.sort_by(|left, right| right.backup_name.cmp(&left.backup_name));

    Ok(RuntimeDirBackupCatalogView {
        status: if items.is_empty() { "empty" } else { "ok" }.into(),
        runtime_dir: runtime_dir.display().to_string(),
        backups_dir: backups_dir.display().to_string(),
        backup_count: items.len(),
        items,
    })
}

pub fn inspect_runtime_dir(runtime_dir: impl AsRef<StdPath>) -> RuntimeDirInspectionView {
    let runtime_dir = runtime_dir.as_ref();
    let state_dir = runtime_dir.join("state");
    let mut files = Vec::new();

    for file_name in EXPECTED_RUNTIME_STATE_FILES {
        let path = state_dir.join(file_name);
        if !path.exists() {
            files.push(RuntimeDirInspectionItem {
                file_name: file_name.into(),
                path: path.display().to_string(),
                required: true,
                exists: false,
                parseable: false,
                status: "missing".into(),
                size_bytes: None,
                parse_error: None,
                recommended_action: "recreate_on_next_managed_start_or_write".into(),
            });
            continue;
        }

        let size_bytes = fs::metadata(&path).ok().map(|metadata| metadata.len());
        match validate_runtime_state_file(file_name, path.as_path()) {
            Ok(()) => files.push(RuntimeDirInspectionItem {
                file_name: file_name.into(),
                path: path.display().to_string(),
                required: true,
                exists: true,
                parseable: true,
                status: "ok".into(),
                size_bytes,
                parse_error: None,
                recommended_action: "none".into(),
            }),
            Err(validation) => files.push(RuntimeDirInspectionItem {
                file_name: file_name.into(),
                path: path.display().to_string(),
                required: true,
                exists: true,
                parseable: validation.parseable,
                status: "corrupt".into(),
                size_bytes,
                parse_error: Some(validation.error),
                recommended_action: "manual_json_repair_or_restore".into(),
            }),
        }
    }

    let healthy_file_count = files.iter().filter(|file| file.status == "ok").count();
    let missing_file_count = files.iter().filter(|file| file.status == "missing").count();
    let corrupt_file_count = files.iter().filter(|file| file.status == "corrupt").count();
    let status = if missing_file_count == 0 && corrupt_file_count == 0 {
        "ok"
    } else {
        "degraded"
    };

    RuntimeDirInspectionView {
        status: status.into(),
        runtime_dir: Some(runtime_dir.display().to_string()),
        state_dir: Some(state_dir.display().to_string()),
        healthy_file_count,
        missing_file_count,
        corrupt_file_count,
        files,
    }
}

pub fn format_runtime_dir_repair(view: &RuntimeDirRepairView) -> String {
    let mut lines = vec![format!("runtime_dir repair status: {}", view.status)];
    lines.push(format!("runtime_dir: {}", view.runtime_dir));
    if let Some(backup_dir) = view.backup_dir.as_deref() {
        lines.push(format!("backup-dir: {backup_dir}"));
    }
    lines.push(format!("repaired files: {}", view.repaired_file_count));
    lines.push(format!("skipped files: {}", view.skipped_file_count));
    lines.push(format!("before status: {}", view.before.status));
    lines.push(format!("after status: {}", view.after.status));

    if view.actions.is_empty() {
        lines.push("actions: none".into());
    } else {
        lines.push("actions:".into());
        for action in &view.actions {
            lines.push(format!(
                "- {} {} ({})",
                action.status, action.file_name, action.detail
            ));
        }
    }

    lines.join("\n")
}

pub fn format_runtime_dir_restore(view: &RuntimeDirRestoreView) -> String {
    let mut lines = vec![format!("runtime_dir restore status: {}", view.status)];
    lines.push(format!("runtime_dir: {}", view.runtime_dir));
    lines.push(format!("source-backup-dir: {}", view.source_backup_dir));
    if let Some(confirmed_preview_fingerprint) = view.confirmed_preview_fingerprint.as_deref() {
        lines.push(format!(
            "confirmed-preview-fingerprint: {confirmed_preview_fingerprint}"
        ));
    }
    if let Some(pre_restore_backup_dir) = view.pre_restore_backup_dir.as_deref() {
        lines.push(format!("pre-restore-backup-dir: {pre_restore_backup_dir}"));
    }
    lines.push(format!("restored files: {}", view.restored_file_count));
    lines.push(format!("skipped files: {}", view.skipped_file_count));
    lines.push(format!("before status: {}", view.before.status));
    lines.push(format!("after status: {}", view.after.status));

    if view.actions.is_empty() {
        lines.push("actions: none".into());
    } else {
        lines.push("actions:".into());
        for action in &view.actions {
            lines.push(format!(
                "- {} {} ({})",
                action.status, action.file_name, action.detail
            ));
        }
    }

    lines.join("\n")
}

pub fn format_runtime_backup_catalog(view: &RuntimeDirBackupCatalogView) -> String {
    let mut lines = vec![format!(
        "runtime_dir backup catalog status: {}",
        view.status
    )];
    lines.push(format!("runtime_dir: {}", view.runtime_dir));
    lines.push(format!("backups-dir: {}", view.backups_dir));
    lines.push(format!("backup count: {}", view.backup_count));

    if view.items.is_empty() {
        lines.push("backups: none".into());
    } else {
        lines.push("backups:".into());
        for item in &view.items {
            let mut details = vec![
                item.operation.clone(),
                format!("stage={}", item.lifecycle_stage),
                item.snapshot_quality.clone(),
                format!("managed={}", item.managed_file_count),
                format!("missing={}", item.missing_file_count),
            ];
            if let Some(report_type) = item.report_type.as_deref() {
                details.push(format!("report={report_type}"));
            }
            if let Some(report_status) = item.report_status.as_deref() {
                details.push(format!("status={report_status}"));
            }
            if let Some(storage_class) = item.storage_class.as_deref() {
                details.push(format!("storage={storage_class}"));
            }
            if let Some(retention_policy) = item.retention_policy.as_deref() {
                details.push(format!("retention={retention_policy}"));
            }
            if let Some(restore_status) = item.restore_status.as_deref() {
                details.push(format!("restore={restore_status}"));
            }
            if item.legal_hold {
                details.push("legal_hold=true".into());
            }
            if let Some(archived_at) = item.archived_at.as_deref() {
                details.push(format!("archived_at={archived_at}"));
            }
            lines.push(format!("- {} ({})", item.backup_name, details.join(", ")));
        }
    }

    lines.join("\n")
}

pub fn format_runtime_dir_archive(view: &RuntimeDirArchiveView) -> String {
    let mut lines = vec![format!("runtime_dir archive status: {}", view.status)];
    lines.push(format!("runtime_dir: {}", view.runtime_dir));
    lines.push(format!("source-backup-dir: {}", view.source_backup_dir));
    lines.push(format!("archived-backup-dir: {}", view.archived_backup_dir));
    lines.push(format!("operation: {}", view.operation));
    lines.push(format!("snapshot-quality: {}", view.snapshot_quality));
    lines.push(format!("managed files: {}", view.managed_file_count));
    lines.push(format!("missing files: {}", view.missing_file_count));
    lines.push(format!("storage-class: {}", view.storage_class));
    lines.push(format!("retention-policy: {}", view.retention_policy));
    lines.push(format!("retention-days: {}", view.retention_days));
    lines.push(format!("restore-status: {}", view.restore_status));
    lines.push(format!("legal-hold: {}", view.legal_hold));
    lines.push(format!("archived-at: {}", view.archived_at));
    lines.push(format!(
        "restore-from-backup-dir: {}",
        view.restore_from_backup_dir
    ));
    lines.join("\n")
}

pub fn format_runtime_dir_archive_prune(view: &RuntimeDirArchivePruneView) -> String {
    let mut lines = vec![format!("runtime_dir archive prune status: {}", view.status)];
    lines.push(format!("runtime_dir: {}", view.runtime_dir));
    lines.push(format!("backups-dir: {}", view.backups_dir));
    lines.push(format!(
        "inspected backups: {}",
        view.inspected_backup_count
    ));
    lines.push(format!("pruned backups: {}", view.pruned_backup_count));
    lines.push(format!("skipped backups: {}", view.skipped_backup_count));

    if view.actions.is_empty() {
        lines.push("actions: none".into());
    } else {
        lines.push("actions:".into());
        for action in &view.actions {
            lines.push(format!(
                "- {} {} ({})",
                action.status, action.backup_name, action.detail
            ));
        }
    }

    lines.join("\n")
}

pub fn format_runtime_dir_inspection(view: &RuntimeDirInspectionView) -> String {
    let mut lines = vec![format!("runtime_dir status: {}", view.status)];

    if let Some(runtime_dir) = view.runtime_dir.as_deref() {
        lines.push(format!("runtime_dir: {runtime_dir}"));
    }
    if let Some(state_dir) = view.state_dir.as_deref() {
        lines.push(format!("state-dir: {state_dir}"));
    }

    lines.push(format!("healthy files: {}", view.healthy_file_count));
    lines.push(format!("missing files: {}", view.missing_file_count));
    lines.push(format!("corrupt files: {}", view.corrupt_file_count));

    if view.files.is_empty() {
        lines.push("files: none".into());
    } else {
        lines.push("files:".into());
        for file in &view.files {
            let mut line = format!(
                "- {} {} ({})",
                file.status, file.file_name, file.recommended_action
            );
            if let Some(size_bytes) = file.size_bytes {
                line.push_str(format!(", {} bytes", size_bytes).as_str());
            }
            if let Some(parse_error) = file.parse_error.as_deref() {
                line.push_str(format!(", parse error: {parse_error}").as_str());
            }
            lines.push(line);
        }
    }

    lines.join("\n")
}
