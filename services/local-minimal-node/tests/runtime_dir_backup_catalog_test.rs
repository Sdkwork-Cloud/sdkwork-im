use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::json;

static NEXT_RUNTIME_DIR_ID: AtomicU64 = AtomicU64::new(0);

fn unique_runtime_dir(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    let sequence = NEXT_RUNTIME_DIR_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("craw_chat_{prefix}_{unique}_{sequence}"))
}

fn write_backup_state_file(root: &Path, file_name: &str, content: &str) {
    let state_dir = root.join("state");
    fs::create_dir_all(&state_dir).expect("backup state dir should be created");
    fs::write(state_dir.join(file_name), content).expect("backup state file should be written");
}

fn write_full_snapshot(root: &Path) {
    write_backup_state_file(root, "commit-journal.json", "[]");
    for file_name in [
        "realtime-disconnect-fences.json",
        "realtime-checkpoints.json",
        "realtime-subscriptions.json",
        "presence-state.json",
        "device-twin-state.json",
        "stream-state.json",
        "rtc-state.json",
        "notification-tasks.json",
        "automation-executions.json",
        "projection-metadata.json",
        "projection-timeline.json",
    ] {
        write_backup_state_file(root, file_name, "{}");
    }
}

#[test]
fn test_list_runtime_backups_classifies_snapshot_quality_and_previews_report_metadata() {
    let runtime_dir = unique_runtime_dir("runtime_backup_catalog");
    let backups_dir = runtime_dir.join("backups");
    fs::create_dir_all(&backups_dir).expect("backups dir should be created");

    let empty_backup = backups_dir.join("runtime-dir-repair-100");
    fs::create_dir_all(empty_backup.join("state")).expect("empty backup state dir should exist");
    fs::write(
        empty_backup.join("repair-report.json"),
        serde_json::to_vec_pretty(&json!({ "status": "repaired" }))
            .expect("repair report should serialize"),
    )
    .expect("repair report should be written");

    let partial_backup = backups_dir.join("runtime-dir-restore-200");
    write_backup_state_file(partial_backup.as_path(), "commit-journal.json", "[]");
    write_backup_state_file(partial_backup.as_path(), "presence-state.json", "{}");
    fs::write(
        partial_backup.join("restore-report.json"),
        serde_json::to_vec_pretty(&json!({ "status": "partial" }))
            .expect("restore report should serialize"),
    )
    .expect("restore report should be written");

    let full_backup = backups_dir.join("runtime-dir-restore-300");
    write_full_snapshot(full_backup.as_path());
    fs::write(
        full_backup.join("restore-report.json"),
        serde_json::to_vec_pretty(&json!({ "status": "restored" }))
            .expect("restore report should serialize"),
    )
    .expect("restore report should be written");

    let catalog = local_minimal_node::list_runtime_backups(runtime_dir.as_path())
        .expect("catalog should load successfully");

    assert_eq!(catalog.status, "ok");
    assert_eq!(catalog.backup_count, 3);
    assert_eq!(catalog.items.len(), 3);
    assert_eq!(catalog.items[0].backup_name, "runtime-dir-restore-300");
    assert_eq!(catalog.items[0].operation, "restore");
    assert_eq!(catalog.items[0].lifecycle_stage, "active");
    assert_eq!(catalog.items[0].snapshot_quality, "full_snapshot");
    assert_eq!(catalog.items[0].managed_file_count, 12);
    assert_eq!(catalog.items[0].missing_file_count, 0);
    assert_eq!(catalog.items[0].report_type.as_deref(), Some("restore"));
    assert_eq!(catalog.items[0].report_status.as_deref(), Some("restored"));

    assert_eq!(catalog.items[1].backup_name, "runtime-dir-restore-200");
    assert_eq!(catalog.items[1].lifecycle_stage, "active");
    assert_eq!(catalog.items[1].snapshot_quality, "partial_snapshot");
    assert_eq!(catalog.items[1].managed_file_count, 2);
    assert_eq!(catalog.items[1].missing_file_count, 10);
    assert_eq!(catalog.items[1].report_status.as_deref(), Some("partial"));

    assert_eq!(catalog.items[2].backup_name, "runtime-dir-repair-100");
    assert_eq!(catalog.items[2].operation, "repair");
    assert_eq!(catalog.items[2].lifecycle_stage, "active");
    assert_eq!(catalog.items[2].snapshot_quality, "empty_snapshot");
    assert_eq!(catalog.items[2].managed_file_count, 0);
    assert_eq!(catalog.items[2].missing_file_count, 12);
    assert_eq!(catalog.items[2].report_type.as_deref(), Some("repair"));
    assert_eq!(catalog.items[2].report_status.as_deref(), Some("repaired"));

    let _ = fs::remove_dir_all(runtime_dir);
}

#[test]
fn test_archive_runtime_backup_moves_snapshot_and_preserves_restore_path() {
    let runtime_dir = unique_runtime_dir("runtime_backup_archive");
    let backups_dir = runtime_dir.join("backups");
    fs::create_dir_all(&backups_dir).expect("backups dir should be created");

    let active_backup = backups_dir.join("runtime-dir-restore-400");
    write_full_snapshot(active_backup.as_path());
    fs::write(
        active_backup.join("restore-report.json"),
        serde_json::to_vec_pretty(&json!({ "status": "restored" }))
            .expect("restore report should serialize"),
    )
    .expect("restore report should be written");

    let archive = local_minimal_node::archive_runtime_backup_with_policy(
        runtime_dir.as_path(),
        active_backup.as_path(),
        30,
        false,
    )
    .expect("runtime backup should archive successfully");

    assert_eq!(archive.status, "archived");
    assert_eq!(archive.operation, "restore");
    assert_eq!(archive.snapshot_quality, "full_snapshot");
    assert_eq!(archive.managed_file_count, 12);
    assert_eq!(archive.missing_file_count, 0);
    assert_eq!(archive.storage_class, "archive");
    assert_eq!(archive.retention_policy, "retain_for_days:30");
    assert_eq!(archive.retention_days, 30);
    assert_eq!(archive.restore_status, "available");
    assert!(!archive.legal_hold);
    assert!(
        !archive.archived_at.is_empty(),
        "archive report should include archived timestamp"
    );
    assert_eq!(
        archive.source_backup_dir,
        active_backup.display().to_string()
    );
    assert_eq!(archive.restore_from_backup_dir, archive.archived_backup_dir);
    assert!(
        !active_backup.exists(),
        "active backup should be moved away"
    );

    let archived_backup = PathBuf::from(&archive.archived_backup_dir);
    assert!(archived_backup.exists(), "archived backup should exist");
    assert!(
        archived_backup.join("archive-report.json").exists(),
        "archived backup should record archive report"
    );
    assert!(
        archived_backup.join("archive-metadata.json").exists(),
        "archived backup should record archive metadata"
    );

    let catalog = local_minimal_node::list_runtime_backups(runtime_dir.as_path())
        .expect("catalog should load successfully");
    assert_eq!(catalog.status, "ok");
    assert_eq!(catalog.backup_count, 1);
    assert_eq!(
        catalog.items[0].backup_name,
        "archived-runtime-dir-restore-400"
    );
    assert_eq!(catalog.items[0].operation, "restore");
    assert_eq!(catalog.items[0].lifecycle_stage, "archived");
    assert_eq!(catalog.items[0].report_type.as_deref(), Some("archive"));
    assert_eq!(catalog.items[0].report_status.as_deref(), Some("archived"));
    assert_eq!(catalog.items[0].storage_class.as_deref(), Some("archive"));
    assert_eq!(
        catalog.items[0].retention_policy.as_deref(),
        Some("retain_for_days:30")
    );
    assert_eq!(catalog.items[0].retention_days, Some(30));
    assert_eq!(
        catalog.items[0].restore_status.as_deref(),
        Some("available")
    );
    assert!(!catalog.items[0].legal_hold);
    assert!(
        catalog.items[0].archived_at.is_some(),
        "catalog should expose archived timestamp"
    );

    let preview = local_minimal_node::preview_restore_runtime_dir(
        runtime_dir.as_path(),
        archived_backup.as_path(),
    )
    .expect("restore preview should accept archived backup snapshots");
    assert_eq!(
        preview.source_backup_dir,
        archived_backup.display().to_string()
    );
    assert!(
        preview.would_restore_file_count >= 1,
        "archived snapshot should still have a restore path"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[test]
fn test_prune_archived_runtime_backups_respects_retention_and_legal_hold() {
    let runtime_dir = unique_runtime_dir("runtime_backup_prune");
    let backups_dir = runtime_dir.join("backups");
    fs::create_dir_all(&backups_dir).expect("backups dir should be created");

    let prunable_backup = backups_dir.join("runtime-dir-restore-500");
    write_full_snapshot(prunable_backup.as_path());
    fs::write(
        prunable_backup.join("restore-report.json"),
        serde_json::to_vec_pretty(&json!({ "status": "restored" }))
            .expect("restore report should serialize"),
    )
    .expect("restore report should be written");
    let prunable_archive = local_minimal_node::archive_runtime_backup_with_policy(
        runtime_dir.as_path(),
        prunable_backup.as_path(),
        0,
        false,
    )
    .expect("prunable backup should archive successfully");

    let held_backup = backups_dir.join("runtime-dir-restore-600");
    write_full_snapshot(held_backup.as_path());
    fs::write(
        held_backup.join("restore-report.json"),
        serde_json::to_vec_pretty(&json!({ "status": "restored" }))
            .expect("restore report should serialize"),
    )
    .expect("restore report should be written");
    let held_archive = local_minimal_node::archive_runtime_backup_with_policy(
        runtime_dir.as_path(),
        held_backup.as_path(),
        0,
        true,
    )
    .expect("held backup should archive successfully");

    let prune = local_minimal_node::prune_archived_runtime_backups(runtime_dir.as_path())
        .expect("prune should complete successfully");

    assert_eq!(prune.status, "pruned");
    assert_eq!(prune.pruned_backup_count, 1);
    assert_eq!(prune.skipped_backup_count, 1);
    assert!(
        prune.actions.iter().any(
            |item| item.backup_name == "archived-runtime-dir-restore-500"
                && item.status == "pruned"
                && item.detail == "retention_elapsed"
        ),
        "prune report should include pruned backup"
    );
    assert!(
        prune.actions.iter().any(
            |item| item.backup_name == "archived-runtime-dir-restore-600"
                && item.status == "skipped"
                && item.detail == "legal_hold"
        ),
        "prune report should keep legal-hold backup"
    );

    assert!(
        !PathBuf::from(prunable_archive.archived_backup_dir).exists(),
        "retention-elapsed archive should be deleted"
    );
    assert!(
        PathBuf::from(&held_archive.archived_backup_dir).exists(),
        "legal-hold archive should remain"
    );

    let catalog = local_minimal_node::list_runtime_backups(runtime_dir.as_path())
        .expect("catalog should load successfully");
    assert_eq!(catalog.status, "ok");
    assert_eq!(catalog.backup_count, 1);
    assert_eq!(
        catalog.items[0].backup_name,
        "archived-runtime-dir-restore-600"
    );
    assert_eq!(catalog.items[0].retention_days, Some(0));
    assert!(catalog.items[0].legal_hold);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[test]
fn test_list_runtime_backups_returns_empty_catalog_when_backups_dir_is_missing() {
    let runtime_dir = unique_runtime_dir("runtime_backup_catalog_missing");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let catalog = local_minimal_node::list_runtime_backups(runtime_dir.as_path())
        .expect("catalog should load successfully");

    assert_eq!(catalog.status, "empty");
    assert_eq!(catalog.backup_count, 0);
    assert!(catalog.items.is_empty());
    assert_eq!(
        catalog.backups_dir,
        runtime_dir.join("backups").display().to_string()
    );
    assert!(!runtime_dir.join("backups").exists());

    let _ = fs::remove_dir_all(runtime_dir);
}
