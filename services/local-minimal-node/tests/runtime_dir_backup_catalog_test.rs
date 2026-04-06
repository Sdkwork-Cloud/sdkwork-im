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
        "stream-state.json",
        "rtc-state.json",
        "notification-tasks.json",
        "automation-executions.json",
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

    let catalog = local_minimal_node::list_runtime_backups(runtime_dir.as_path());

    assert_eq!(catalog.status, "ok");
    assert_eq!(catalog.backup_count, 3);
    assert_eq!(catalog.items.len(), 3);
    assert_eq!(catalog.items[0].backup_name, "runtime-dir-restore-300");
    assert_eq!(catalog.items[0].operation, "restore");
    assert_eq!(catalog.items[0].snapshot_quality, "full_snapshot");
    assert_eq!(catalog.items[0].managed_file_count, 9);
    assert_eq!(catalog.items[0].missing_file_count, 0);
    assert_eq!(catalog.items[0].report_type.as_deref(), Some("restore"));
    assert_eq!(catalog.items[0].report_status.as_deref(), Some("restored"));

    assert_eq!(catalog.items[1].backup_name, "runtime-dir-restore-200");
    assert_eq!(catalog.items[1].snapshot_quality, "partial_snapshot");
    assert_eq!(catalog.items[1].managed_file_count, 2);
    assert_eq!(catalog.items[1].missing_file_count, 7);
    assert_eq!(catalog.items[1].report_status.as_deref(), Some("partial"));

    assert_eq!(catalog.items[2].backup_name, "runtime-dir-repair-100");
    assert_eq!(catalog.items[2].operation, "repair");
    assert_eq!(catalog.items[2].snapshot_quality, "empty_snapshot");
    assert_eq!(catalog.items[2].managed_file_count, 0);
    assert_eq!(catalog.items[2].missing_file_count, 9);
    assert_eq!(catalog.items[2].report_type.as_deref(), Some("repair"));
    assert_eq!(catalog.items[2].report_status.as_deref(), Some("repaired"));

    let _ = fs::remove_dir_all(runtime_dir);
}

#[test]
fn test_list_runtime_backups_returns_empty_catalog_when_backups_dir_is_missing() {
    let runtime_dir = unique_runtime_dir("runtime_backup_catalog_missing");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let catalog = local_minimal_node::list_runtime_backups(runtime_dir.as_path());

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
