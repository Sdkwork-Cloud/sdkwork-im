use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_RUNTIME_DIR_ID: AtomicU64 = AtomicU64::new(0);

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    let sequence = NEXT_RUNTIME_DIR_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("craw_chat_runtime_dir_repair_{unique}_{sequence}"))
}

fn state_file(runtime_dir: &Path, file_name: &str) -> PathBuf {
    runtime_dir.join("state").join(file_name)
}

fn write_runtime_state_file(runtime_dir: &Path, file_name: &str, content: &str) {
    let state_dir = runtime_dir.join("state");
    fs::create_dir_all(&state_dir).expect("state dir should be created");
    fs::write(state_dir.join(file_name), content).expect("state file should be written");
}

#[test]
fn test_repair_runtime_dir_recreates_missing_files_with_backup_first_flow() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let report = local_minimal_node::repair_runtime_dir(runtime_dir.as_path());

    assert_eq!(report.status, "repaired");
    assert_eq!(report.before.status, "degraded");
    assert_eq!(report.before.missing_file_count, 12);
    assert_eq!(report.after.status, "ok");
    assert_eq!(report.after.missing_file_count, 0);
    assert_eq!(report.after.corrupt_file_count, 0);
    assert_eq!(report.repaired_file_count, 12);
    assert_eq!(report.skipped_file_count, 0);

    let backup_dir = PathBuf::from(
        report
            .backup_dir
            .clone()
            .expect("backup dir should be created for repair traceability"),
    );
    assert!(backup_dir.exists());
    assert!(backup_dir.join("repair-report.json").exists());
    assert_eq!(
        fs::read_to_string(state_file(runtime_dir.as_path(), "commit-journal.json"))
            .expect("commit journal should be recreated")
            .trim(),
        "[]"
    );
    assert_eq!(
        fs::read_to_string(state_file(runtime_dir.as_path(), "presence-state.json"))
            .expect("presence state should be recreated")
            .trim(),
        "{}"
    );
    assert_eq!(
        fs::read_to_string(state_file(
            runtime_dir.as_path(),
            "projection-metadata.json"
        ))
        .expect("projection metadata should be recreated")
        .trim(),
        "{}"
    );
    assert_eq!(
        fs::read_to_string(state_file(
            runtime_dir.as_path(),
            "projection-timeline.json"
        ))
        .expect("projection timeline should be recreated")
        .trim(),
        "{}"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[test]
fn test_repair_runtime_dir_leaves_corrupt_files_untouched_while_fixing_missing_files() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    write_runtime_state_file(runtime_dir.as_path(), "rtc-state.json", "{not-valid-json");

    let report = local_minimal_node::repair_runtime_dir(runtime_dir.as_path());

    assert_eq!(report.status, "partial");
    assert_eq!(report.before.missing_file_count, 11);
    assert_eq!(report.before.corrupt_file_count, 1);
    assert_eq!(report.after.status, "degraded");
    assert_eq!(report.after.missing_file_count, 0);
    assert_eq!(report.after.corrupt_file_count, 1);
    assert_eq!(report.repaired_file_count, 11);
    assert_eq!(report.skipped_file_count, 1);
    assert_eq!(
        fs::read_to_string(state_file(runtime_dir.as_path(), "rtc-state.json"))
            .expect("corrupt file should remain on disk"),
        "{not-valid-json"
    );
    assert_eq!(
        fs::read_to_string(state_file(runtime_dir.as_path(), "presence-state.json"))
            .expect("missing presence file should be recreated")
            .trim(),
        "{}"
    );
    assert_eq!(
        fs::read_to_string(state_file(
            runtime_dir.as_path(),
            "projection-metadata.json"
        ))
        .expect("missing projection metadata file should be recreated")
        .trim(),
        "{}"
    );

    let backup_dir = PathBuf::from(
        report
            .backup_dir
            .clone()
            .expect("backup dir should be created for repair traceability"),
    );
    assert_eq!(
        fs::read_to_string(backup_dir.join("state").join("rtc-state.json"))
            .expect("backup should capture pre-repair corrupt file"),
        "{not-valid-json"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}
