use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::json;

static NEXT_RUNTIME_DIR_ID: AtomicU64 = AtomicU64::new(0);

fn unique_path(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    let sequence = NEXT_RUNTIME_DIR_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("sdkwork_im_{prefix}_{unique}_{sequence}"))
}

fn state_file(root: &Path, file_name: &str) -> PathBuf {
    root.join("state").join(file_name)
}

fn write_state_file(root: &Path, file_name: &str, content: &str) {
    let state_dir = root.join("state");
    fs::create_dir_all(&state_dir).expect("state dir should be created");
    fs::write(state_dir.join(file_name), content).expect("state file should be written");
}

fn write_valid_backup_snapshot(root: &Path, owner_node_id: &str) {
    write_state_file(root, "commit-journal.json", "");
    for file_name in [
        "realtime-checkpoints.json",
        "realtime-subscriptions.json",
        "stream-state.json",
        "rtc-state.json",
        "automation-executions.json",
        "projection-metadata.json",
        "projection-timeline.json",
        "realtime-event-windows.json",
    ] {
        write_state_file(root, file_name, "{}");
    }
    write_state_file(
        root,
        "presence-state.json",
        "{\"by_device\":{},\"presence_by_principal\":{},\"online_by_seen_at\":{}}",
    );
    write_state_file(
        root,
        "notification-tasks.json",
        "{\"by_notification\":{},\"tasks_by_recipient\":{}}",
    );
    write_state_file(
        root,
        "realtime-disconnect-fences.json",
        serde_json::to_string_pretty(&json!({
            "t_demo:u_demo:d_demo": {
                "tenant_id": "t_demo",
                "principal_kind": "user",
                "principal_id": "u_demo",
                "device_id": "d_demo",
                "session_id": "s_demo",
                "owner_node_id": owner_node_id,
                "disconnected_at": "2026-04-06T00:00:00.000Z",
                "fence_token": format!("fence:t_demo:user:u_demo:d_demo:s_demo:{owner_node_id}:2026-04-06T00:00:00.000Z")
            }
        }))
        .expect("disconnect fence snapshot should serialize")
        .as_str(),
    );
}

#[test]
fn test_restore_runtime_dir_restores_selected_snapshot_and_creates_pre_restore_backup() {
    let runtime_dir = unique_path("runtime_dir_restore");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let _ = local_minimal_node::repair_runtime_dir(runtime_dir.as_path())
        .expect("repair should succeed");

    write_state_file(
        runtime_dir.as_path(),
        "realtime-disconnect-fences.json",
        serde_json::to_string_pretty(&json!({
            "t_demo:u_demo:d_demo": {
                "tenant_id": "t_demo",
                "principal_kind": "user",
                "principal_id": "u_demo",
                "device_id": "d_demo",
                "session_id": "s_demo",
                "owner_node_id": "node_current",
                "disconnected_at": "2026-04-06T00:00:00.000Z",
                "fence_token": "fence:t_demo:user:u_demo:d_demo:s_demo:node_current:2026-04-06T00:00:00.000Z"
            }
        }))
        .expect("current fence snapshot should serialize")
        .as_str(),
    );
    write_state_file(
        runtime_dir.as_path(),
        "projection-metadata.json",
        "{\"t_demo:c_demo:conversation-summary\":\"current\"}",
    );
    write_state_file(
        runtime_dir.as_path(),
        "projection-timeline.json",
        "{\"t_demo:c_demo\":{\"1\":\"current\"}}",
    );

    let backup_dir = unique_path("runtime_dir_restore_backup");
    write_valid_backup_snapshot(backup_dir.as_path(), "node_backup");
    write_state_file(
        backup_dir.as_path(),
        "projection-metadata.json",
        "{\"t_demo:c_demo:conversation-summary\":\"backup\"}",
    );
    write_state_file(
        backup_dir.as_path(),
        "projection-timeline.json",
        "{\"t_demo:c_demo\":{\"1\":\"backup\"}}",
    );
    let preview = local_minimal_node::preview_restore_runtime_dir(
        runtime_dir.as_path(),
        backup_dir.as_path(),
    )
    .expect("restore preview should succeed");

    let report = local_minimal_node::restore_runtime_dir_with_expected_preview_fingerprint(
        runtime_dir.as_path(),
        backup_dir.as_path(),
        Some(preview.preview_fingerprint.as_str()),
    )
    .expect("restore should succeed");

    assert_eq!(report.status, "restored");
    assert_eq!(report.before.status, "ok");
    assert_eq!(report.after.status, "ok");
    assert_eq!(report.restored_file_count, 12);
    assert_eq!(report.skipped_file_count, 0);
    assert_eq!(report.source_backup_dir, backup_dir.display().to_string());
    assert_eq!(
        report.confirmed_preview_fingerprint.as_deref(),
        Some(preview.preview_fingerprint.as_str())
    );

    let pre_restore_backup_dir = PathBuf::from(
        report
            .pre_restore_backup_dir
            .clone()
            .expect("pre-restore backup dir should exist"),
    );
    assert!(pre_restore_backup_dir.exists());
    assert!(
        fs::read_to_string(
            pre_restore_backup_dir
                .join("state")
                .join("realtime-disconnect-fences.json")
        )
        .expect("pre-restore backup should capture current state")
        .contains("node_current")
    );
    assert!(
        fs::read_to_string(state_file(
            runtime_dir.as_path(),
            "realtime-disconnect-fences.json"
        ))
        .expect("runtime state should be restored from backup snapshot")
        .contains("node_backup")
    );
    assert_eq!(
        fs::read_to_string(state_file(
            runtime_dir.as_path(),
            "projection-metadata.json"
        ))
        .expect("projection metadata should be restored from backup snapshot"),
        "{\"t_demo:c_demo:conversation-summary\":\"backup\"}"
    );
    assert_eq!(
        fs::read_to_string(state_file(
            runtime_dir.as_path(),
            "projection-timeline.json"
        ))
        .expect("projection timeline should be restored from backup snapshot"),
        "{\"t_demo:c_demo\":{\"1\":\"backup\"}}"
    );

    let _ = fs::remove_dir_all(runtime_dir);
    let _ = fs::remove_dir_all(backup_dir);
}

#[test]
fn test_restore_runtime_dir_rejects_missing_backup_dir_without_mutation() {
    let runtime_dir = unique_path("runtime_dir_restore_missing_backup");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let missing_backup_dir = unique_path("runtime_dir_missing_backup_input");
    let error = local_minimal_node::restore_runtime_dir(
        runtime_dir.as_path(),
        missing_backup_dir.as_path(),
    )
    .expect_err("missing backup dir should fail");

    assert!(error.contains("backup dir does not exist"));
    assert!(!runtime_dir.join("state").exists());

    let _ = fs::remove_dir_all(runtime_dir);
}

#[test]
fn test_restore_runtime_dir_rejects_mismatched_preview_fingerprint_without_mutation() {
    let runtime_dir = unique_path("runtime_dir_restore_mismatched_preview_fingerprint");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let _ = local_minimal_node::repair_runtime_dir(runtime_dir.as_path())
        .expect("repair should succeed");

    write_state_file(
        runtime_dir.as_path(),
        "realtime-disconnect-fences.json",
        serde_json::to_string_pretty(&json!({
            "t_demo:u_demo:d_demo": {
                "tenant_id": "t_demo",
                "principal_kind": "user",
                "principal_id": "u_demo",
                "device_id": "d_demo",
                "session_id": "s_demo",
                "owner_node_id": "node_current",
                "disconnected_at": "2026-04-06T00:00:00.000Z",
                "fence_token": "fence:t_demo:user:u_demo:d_demo:s_demo:node_current:2026-04-06T00:00:00.000Z"
            }
        }))
        .expect("current fence snapshot should serialize")
        .as_str(),
    );

    let original_payload = fs::read_to_string(state_file(
        runtime_dir.as_path(),
        "realtime-disconnect-fences.json",
    ))
    .expect("current runtime payload should be readable");
    let backup_count_before = if runtime_dir.join("backups").exists() {
        fs::read_dir(runtime_dir.join("backups"))
            .expect("existing backups dir should be readable")
            .count()
    } else {
        0
    };

    let backup_dir = unique_path("runtime_dir_restore_mismatched_preview_backup");
    write_valid_backup_snapshot(backup_dir.as_path(), "node_backup");

    let error = local_minimal_node::restore_runtime_dir_with_expected_preview_fingerprint(
        runtime_dir.as_path(),
        backup_dir.as_path(),
        Some("preview_fp_invalid"),
    )
    .expect_err("restore should fail when preview fingerprint mismatches");

    assert!(error.contains("preview fingerprint mismatch"));
    assert_eq!(
        fs::read_to_string(state_file(
            runtime_dir.as_path(),
            "realtime-disconnect-fences.json"
        ))
        .expect("runtime payload should remain unchanged"),
        original_payload
    );
    let backup_count_after = if runtime_dir.join("backups").exists() {
        fs::read_dir(runtime_dir.join("backups"))
            .expect("backups dir should remain readable")
            .count()
    } else {
        0
    };
    assert!(
        backup_count_after == backup_count_before,
        "mismatched fingerprint must fail before creating new pre-restore backups"
    );

    let _ = fs::remove_dir_all(runtime_dir);
    let _ = fs::remove_dir_all(backup_dir);
}
