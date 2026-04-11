use super::*;

mod diff;
mod format;

use diff::{
    stable_runtime_dir_restore_preview_fingerprint,
    summarize_disconnect_fence_restore_preview_change,
    summarize_realtime_checkpoint_restore_preview_change,
    summarize_realtime_subscription_restore_preview_change,
    summarize_rtc_state_restore_preview_change, summarize_runtime_restore_preview_change,
    summarize_stream_state_restore_preview_change,
};
pub use format::format_runtime_dir_restore_preview;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirRestorePreviewChangeSummaryView {
    pub summary_kind: String,
    pub source_key_count: usize,
    pub target_key_count: usize,
    pub added_keys: Vec<String>,
    pub removed_keys: Vec<String>,
    pub modified_keys: Vec<String>,
    pub unchanged_key_count: usize,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirRestorePreviewDomainSummaryView {
    pub summary_kind: String,
    pub added_keys: Vec<String>,
    pub removed_keys: Vec<String>,
    pub owner_node_changed_keys: Vec<String>,
    pub session_changed_keys: Vec<String>,
    pub other_modified_keys: Vec<String>,
    pub unchanged_key_count: usize,
    pub latest_advanced_keys: Option<Vec<String>>,
    pub latest_rewound_keys: Option<Vec<String>>,
    pub acked_advanced_keys: Option<Vec<String>>,
    pub acked_rewound_keys: Option<Vec<String>>,
    pub trimmed_advanced_keys: Option<Vec<String>>,
    pub trimmed_rewound_keys: Option<Vec<String>>,
    pub timestamp_only_changed_keys: Option<Vec<String>>,
    pub added_scope_keys: Option<Vec<String>>,
    pub removed_scope_keys: Option<Vec<String>>,
    pub event_types_added_scope_keys: Option<Vec<String>>,
    pub event_types_removed_scope_keys: Option<Vec<String>>,
    pub subscribed_at_only_changed_scope_keys: Option<Vec<String>>,
    pub unchanged_scope_count: Option<usize>,
    pub stream_state_changed_keys: Option<Vec<String>>,
    pub stream_last_frame_advanced_keys: Option<Vec<String>>,
    pub stream_last_frame_rewound_keys: Option<Vec<String>>,
    pub stream_checkpoint_advanced_keys: Option<Vec<String>>,
    pub stream_checkpoint_rewound_keys: Option<Vec<String>>,
    pub stream_result_message_changed_keys: Option<Vec<String>>,
    pub added_frame_keys: Option<Vec<String>>,
    pub removed_frame_keys: Option<Vec<String>>,
    pub modified_frame_keys: Option<Vec<String>>,
    pub unchanged_frame_count: Option<usize>,
    pub rtc_state_changed_keys: Option<Vec<String>>,
    pub rtc_signaling_stream_changed_keys: Option<Vec<String>>,
    pub rtc_artifact_message_changed_keys: Option<Vec<String>>,
    pub added_signal_keys: Option<Vec<String>>,
    pub removed_signal_keys: Option<Vec<String>>,
    pub modified_signal_keys: Option<Vec<String>>,
    pub unchanged_signal_count: Option<usize>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirRestorePreviewActionView {
    pub file_name: String,
    pub source_path: String,
    pub target_path: String,
    pub source_exists: bool,
    pub target_exists: bool,
    pub action: String,
    pub detail: String,
    pub change_summary: Option<RuntimeDirRestorePreviewChangeSummaryView>,
    pub domain_summary: Option<RuntimeDirRestorePreviewDomainSummaryView>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirRestorePreviewView {
    pub status: String,
    pub runtime_dir: String,
    pub source_backup_dir: String,
    pub preview_fingerprint: String,
    pub source_snapshot_quality: String,
    pub source_managed_file_count: usize,
    pub source_missing_file_count: usize,
    pub source_report_type: Option<String>,
    pub source_report_status: Option<String>,
    pub would_restore_file_count: usize,
    pub unchanged_file_count: usize,
    pub skipped_file_count: usize,
    pub before: RuntimeDirInspectionView,
    pub actions: Vec<RuntimeDirRestorePreviewActionView>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RuntimeDirRestorePreviewFingerprintMaterial<'a> {
    status: &'a str,
    runtime_dir: &'a str,
    source_backup_dir: &'a str,
    source_snapshot_quality: &'a str,
    source_managed_file_count: usize,
    source_missing_file_count: usize,
    source_report_type: Option<&'a str>,
    source_report_status: Option<&'a str>,
    would_restore_file_count: usize,
    unchanged_file_count: usize,
    skipped_file_count: usize,
    before: &'a RuntimeDirInspectionView,
    actions: &'a [RuntimeDirRestorePreviewActionView],
}

pub fn preview_restore_runtime_dir(
    runtime_dir: impl AsRef<StdPath>,
    backup_dir: impl AsRef<StdPath>,
) -> Result<RuntimeDirRestorePreviewView, String> {
    let runtime_dir = runtime_dir.as_ref();
    let backup_dir = backup_dir.as_ref();
    let (source_state_dir, source_summary) = validate_runtime_backup_source(backup_dir)?;

    let state_dir = runtime_dir.join("state");
    let before = inspect_runtime_dir(runtime_dir);
    let mut actions = Vec::new();
    let mut would_restore_file_count = 0usize;
    let mut unchanged_file_count = 0usize;
    let mut skipped_file_count = 0usize;

    for file_name in EXPECTED_RUNTIME_STATE_FILES {
        let source_path = source_state_dir.join(file_name);
        let target_path = state_dir.join(file_name);
        let source_exists = source_path.exists();
        let target_exists = target_path.exists();
        let mut change_summary = None;
        let mut domain_summary = None;

        let (action, detail) = if !source_exists {
            skipped_file_count += 1;
            ("skip", "missing_in_source_backup_snapshot")
        } else if target_exists {
            let source_payload = fs::read(&source_path).map_err(|error| {
                format!(
                    "failed to read source backup file {} during restore preview: {error}",
                    source_path.display()
                )
            })?;
            let target_payload = fs::read(&target_path).map_err(|error| {
                format!(
                    "failed to read runtime state file {} during restore preview: {error}",
                    target_path.display()
                )
            })?;
            if source_payload == target_payload {
                unchanged_file_count += 1;
                ("noop", "source_matches_target")
            } else {
                would_restore_file_count += 1;
                change_summary =
                    summarize_runtime_restore_preview_change(&source_payload, &target_payload);
                domain_summary = summarize_disconnect_fence_restore_preview_change(
                    file_name,
                    &source_payload,
                    &target_payload,
                )
                .or_else(|| {
                    summarize_realtime_checkpoint_restore_preview_change(
                        file_name,
                        &source_payload,
                        &target_payload,
                    )
                })
                .or_else(|| {
                    summarize_realtime_subscription_restore_preview_change(
                        file_name,
                        &source_payload,
                        &target_payload,
                    )
                })
                .or_else(|| {
                    summarize_stream_state_restore_preview_change(
                        file_name,
                        &source_payload,
                        &target_payload,
                    )
                })
                .or_else(|| {
                    summarize_rtc_state_restore_preview_change(
                        file_name,
                        &source_payload,
                        &target_payload,
                    )
                });
                ("would_restore", "content_differs")
            }
        } else {
            would_restore_file_count += 1;
            ("would_restore", "target_missing")
        };

        actions.push(RuntimeDirRestorePreviewActionView {
            file_name: file_name.into(),
            source_path: source_path.display().to_string(),
            target_path: target_path.display().to_string(),
            source_exists,
            target_exists,
            action: action.into(),
            detail: detail.into(),
            change_summary,
            domain_summary,
        });
    }

    let status = if would_restore_file_count == 0 && skipped_file_count == 0 {
        "noop"
    } else if skipped_file_count == 0 {
        "ready"
    } else {
        "partial"
    };

    let runtime_dir_text = runtime_dir.display().to_string();
    let source_backup_dir_text = backup_dir.display().to_string();
    let fingerprint_material = RuntimeDirRestorePreviewFingerprintMaterial {
        status,
        runtime_dir: runtime_dir_text.as_str(),
        source_backup_dir: source_backup_dir_text.as_str(),
        source_snapshot_quality: source_summary.snapshot_quality.as_str(),
        source_managed_file_count: source_summary.managed_file_count,
        source_missing_file_count: source_summary.missing_file_count,
        source_report_type: source_summary.report_type.as_deref(),
        source_report_status: source_summary.report_status.as_deref(),
        would_restore_file_count,
        unchanged_file_count,
        skipped_file_count,
        before: &before,
        actions: actions.as_slice(),
    };
    let preview_fingerprint = stable_runtime_dir_restore_preview_fingerprint(&fingerprint_material);

    Ok(RuntimeDirRestorePreviewView {
        status: status.into(),
        runtime_dir: runtime_dir_text,
        source_backup_dir: source_backup_dir_text,
        preview_fingerprint,
        source_snapshot_quality: source_summary.snapshot_quality,
        source_managed_file_count: source_summary.managed_file_count,
        source_missing_file_count: source_summary.missing_file_count,
        source_report_type: source_summary.report_type,
        source_report_status: source_summary.report_status,
        would_restore_file_count,
        unchanged_file_count,
        skipped_file_count,
        before,
        actions,
    })
}
