use super::*;

pub fn format_runtime_dir_restore_preview(view: &RuntimeDirRestorePreviewView) -> String {
    fn format_change_keys(keys: &[String]) -> String {
        if keys.is_empty() {
            "[]".into()
        } else {
            format!("[{}]", keys.join(", "))
        }
    }

    fn format_optional_change_keys(keys: Option<&Vec<String>>) -> String {
        match keys {
            Some(keys) => format_change_keys(keys.as_slice()),
            None => "[]".into(),
        }
    }

    fn format_optional_count(value: Option<usize>) -> String {
        value
            .map(|value| value.to_string())
            .unwrap_or_else(|| "0".into())
    }

    let mut lines = vec![format!(
        "runtime_dir restore preview status: {}",
        view.status
    )];
    lines.push(format!("runtime_dir: {}", view.runtime_dir));
    lines.push(format!("source-backup-dir: {}", view.source_backup_dir));
    lines.push(format!("preview-fingerprint: {}", view.preview_fingerprint));
    lines.push(format!(
        "source-snapshot-quality: {}",
        view.source_snapshot_quality
    ));
    lines.push(format!(
        "source managed files: {}",
        view.source_managed_file_count
    ));
    lines.push(format!(
        "source missing files: {}",
        view.source_missing_file_count
    ));
    if let Some(report_type) = view.source_report_type.as_deref() {
        lines.push(format!("source report type: {report_type}"));
    }
    if let Some(report_status) = view.source_report_status.as_deref() {
        lines.push(format!("source report status: {report_status}"));
    }
    lines.push(format!(
        "would restore files: {}",
        view.would_restore_file_count
    ));
    lines.push(format!("unchanged files: {}", view.unchanged_file_count));
    lines.push(format!("skipped files: {}", view.skipped_file_count));
    lines.push(format!("before status: {}", view.before.status));

    if view.actions.is_empty() {
        lines.push("actions: none".into());
    } else {
        lines.push("actions:".into());
        for action in &view.actions {
            lines.push(format!(
                "- {} {} ({})",
                action.action, action.file_name, action.detail
            ));
            if let Some(change_summary) = action.change_summary.as_ref() {
                lines.push(format!(
                    "  json-object-diff: +{} -{} ~{} unchanged={} source_keys={} target_keys={}",
                    format_change_keys(change_summary.added_keys.as_slice()),
                    format_change_keys(change_summary.removed_keys.as_slice()),
                    format_change_keys(change_summary.modified_keys.as_slice()),
                    change_summary.unchanged_key_count,
                    change_summary.source_key_count,
                    change_summary.target_key_count
                ));
            }
            if let Some(domain_summary) = action.domain_summary.as_ref() {
                if domain_summary.summary_kind == "disconnect_fences" {
                    lines.push(format!(
                        "  disconnect-fence-diff: +{} -{} owner_changed={} session_changed={} other_modified={} unchanged={}",
                        format_change_keys(domain_summary.added_keys.as_slice()),
                        format_change_keys(domain_summary.removed_keys.as_slice()),
                        format_change_keys(domain_summary.owner_node_changed_keys.as_slice()),
                        format_change_keys(domain_summary.session_changed_keys.as_slice()),
                        format_change_keys(domain_summary.other_modified_keys.as_slice()),
                        domain_summary.unchanged_key_count
                    ));
                } else if domain_summary.summary_kind == "realtime_checkpoints" {
                    lines.push(format!(
                        "  checkpoint-diff: +{} -{} latest_advanced={} latest_rewound={} acked_advanced={} acked_rewound={} trimmed_advanced={} trimmed_rewound={} timestamp_only={} other_modified={} unchanged={}",
                        format_change_keys(domain_summary.added_keys.as_slice()),
                        format_change_keys(domain_summary.removed_keys.as_slice()),
                        format_optional_change_keys(domain_summary.latest_advanced_keys.as_ref()),
                        format_optional_change_keys(domain_summary.latest_rewound_keys.as_ref()),
                        format_optional_change_keys(domain_summary.acked_advanced_keys.as_ref()),
                        format_optional_change_keys(domain_summary.acked_rewound_keys.as_ref()),
                        format_optional_change_keys(domain_summary.trimmed_advanced_keys.as_ref()),
                        format_optional_change_keys(domain_summary.trimmed_rewound_keys.as_ref()),
                        format_optional_change_keys(domain_summary.timestamp_only_changed_keys.as_ref()),
                        format_change_keys(domain_summary.other_modified_keys.as_slice()),
                        domain_summary.unchanged_key_count
                    ));
                } else if domain_summary.summary_kind == "realtime_event_windows" {
                    lines.push(format!(
                        "  event-window-diff: +{} -{} trimmed_advanced={} trimmed_rewound={} capacity_trimmed_advanced={} capacity_trimmed_rewound={} capacity_trimmed_timestamp_only={} updated_at_only={} other_modified={} unchanged={}",
                        format_change_keys(domain_summary.added_keys.as_slice()),
                        format_change_keys(domain_summary.removed_keys.as_slice()),
                        format_optional_change_keys(domain_summary.trimmed_advanced_keys.as_ref()),
                        format_optional_change_keys(domain_summary.trimmed_rewound_keys.as_ref()),
                        format_optional_change_keys(domain_summary.capacity_trimmed_advanced_keys.as_ref()),
                        format_optional_change_keys(domain_summary.capacity_trimmed_rewound_keys.as_ref()),
                        format_optional_change_keys(
                            domain_summary
                                .capacity_trimmed_timestamp_changed_keys
                                .as_ref()
                        ),
                        format_optional_change_keys(domain_summary.timestamp_only_changed_keys.as_ref()),
                        format_change_keys(domain_summary.other_modified_keys.as_slice()),
                        domain_summary.unchanged_key_count
                    ));
                } else if domain_summary.summary_kind == "realtime_subscriptions" {
                    lines.push(format!(
                        "  subscription-diff: +{} -{} scope_added={} scope_removed={} event_types_added={} event_types_removed={} subscribed_at_only={} synced_timestamp_only={} other_modified={} unchanged={} unchanged_scopes={}",
                        format_change_keys(domain_summary.added_keys.as_slice()),
                        format_change_keys(domain_summary.removed_keys.as_slice()),
                        format_optional_change_keys(domain_summary.added_scope_keys.as_ref()),
                        format_optional_change_keys(domain_summary.removed_scope_keys.as_ref()),
                        format_optional_change_keys(domain_summary.event_types_added_scope_keys.as_ref()),
                        format_optional_change_keys(domain_summary.event_types_removed_scope_keys.as_ref()),
                        format_optional_change_keys(
                            domain_summary
                                .subscribed_at_only_changed_scope_keys
                                .as_ref()
                        ),
                        format_optional_change_keys(domain_summary.timestamp_only_changed_keys.as_ref()),
                        format_change_keys(domain_summary.other_modified_keys.as_slice()),
                        domain_summary.unchanged_key_count,
                        format_optional_count(domain_summary.unchanged_scope_count)
                    ));
                } else if domain_summary.summary_kind == "stream_state" {
                    lines.push(format!(
                        "  stream-diff: +{} -{} state_changed={} last_frame_advanced={} last_frame_rewound={} checkpoint_advanced={} checkpoint_rewound={} result_message_changed={} frame_added={} frame_removed={} frame_modified={} updated_at_only={} other_modified={} unchanged={} unchanged_frames={}",
                        format_change_keys(domain_summary.added_keys.as_slice()),
                        format_change_keys(domain_summary.removed_keys.as_slice()),
                        format_optional_change_keys(domain_summary.stream_state_changed_keys.as_ref()),
                        format_optional_change_keys(domain_summary.stream_last_frame_advanced_keys.as_ref()),
                        format_optional_change_keys(domain_summary.stream_last_frame_rewound_keys.as_ref()),
                        format_optional_change_keys(domain_summary.stream_checkpoint_advanced_keys.as_ref()),
                        format_optional_change_keys(domain_summary.stream_checkpoint_rewound_keys.as_ref()),
                        format_optional_change_keys(domain_summary.stream_result_message_changed_keys.as_ref()),
                        format_optional_change_keys(domain_summary.added_frame_keys.as_ref()),
                        format_optional_change_keys(domain_summary.removed_frame_keys.as_ref()),
                        format_optional_change_keys(domain_summary.modified_frame_keys.as_ref()),
                        format_optional_change_keys(domain_summary.timestamp_only_changed_keys.as_ref()),
                        format_change_keys(domain_summary.other_modified_keys.as_slice()),
                        domain_summary.unchanged_key_count,
                        format_optional_count(domain_summary.unchanged_frame_count)
                    ));
                } else if domain_summary.summary_kind == "rtc_state" {
                    lines.push(format!(
                        "  rtc-diff: +{} -{} state_changed={} signaling_stream_changed={} artifact_message_changed={} signal_added={} signal_removed={} signal_modified={} updated_at_only={} other_modified={} unchanged={} unchanged_signals={}",
                        format_change_keys(domain_summary.added_keys.as_slice()),
                        format_change_keys(domain_summary.removed_keys.as_slice()),
                        format_optional_change_keys(domain_summary.rtc_state_changed_keys.as_ref()),
                        format_optional_change_keys(domain_summary.rtc_signaling_stream_changed_keys.as_ref()),
                        format_optional_change_keys(domain_summary.rtc_artifact_message_changed_keys.as_ref()),
                        format_optional_change_keys(domain_summary.added_signal_keys.as_ref()),
                        format_optional_change_keys(domain_summary.removed_signal_keys.as_ref()),
                        format_optional_change_keys(domain_summary.modified_signal_keys.as_ref()),
                        format_optional_change_keys(domain_summary.timestamp_only_changed_keys.as_ref()),
                        format_change_keys(domain_summary.other_modified_keys.as_slice()),
                        domain_summary.unchanged_key_count,
                        format_optional_count(domain_summary.unchanged_signal_count)
                    ));
                }
            }
        }
    }

    lines.join("\n")
}
