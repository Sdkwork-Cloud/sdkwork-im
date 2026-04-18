use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_RUNTIME_DIR_ID: AtomicU64 = AtomicU64::new(0);

fn unique_runtime_dir(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    let sequence = NEXT_RUNTIME_DIR_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("craw_chat_{prefix}_{unique}_{sequence}"))
}

fn run_local_minimal_cli(args: &[&str]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_local-minimal-node"))
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("local-minimal-node binary should spawn");

    for _ in 0..20 {
        if child
            .try_wait()
            .expect("local-minimal-node process status should be readable")
            .is_some()
        {
            return child
                .wait_with_output()
                .expect("local-minimal-node output should collect");
        }
        thread::sleep(Duration::from_millis(100));
    }

    let _ = child.kill();
    let _ = child.wait();
    panic!("local-minimal-node cli did not exit within timeout");
}

#[test]
fn test_local_minimal_cli_rejects_missing_runtime_dir_option_value() {
    let output = run_local_minimal_cli(&["inspect-runtime-dir", "--runtime-dir"]);

    assert!(
        !output.status.success(),
        "inspect-runtime-dir must fail when --runtime-dir has no value. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--runtime-dir requires a value"),
        "cli stderr should explain missing --runtime-dir value. stderr: {stderr}"
    );
    assert!(
        !stderr.to_lowercase().contains("panicked"),
        "cli must return a controlled error instead of panic. stderr: {stderr}"
    );
}

#[test]
fn test_local_minimal_cli_rejects_non_numeric_retention_days() {
    let output = run_local_minimal_cli(&[
        "archive-runtime-backup",
        "--backup-dir",
        ".",
        "--retention-days",
        "abc",
    ]);

    assert!(
        !output.status.success(),
        "archive-runtime-backup must fail for non-numeric retention days. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--retention-days expects an integer number of days"),
        "cli stderr should explain invalid --retention-days value. stderr: {stderr}"
    );
    assert!(
        !stderr.to_lowercase().contains("panicked"),
        "cli must return a controlled error instead of panic. stderr: {stderr}"
    );
}

#[test]
fn test_local_minimal_cli_rejects_backups_path_that_is_not_a_directory() {
    let runtime_dir = unique_runtime_dir("runtime_cli_backups_file");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    fs::write(runtime_dir.join("backups"), "not-a-directory")
        .expect("backups path placeholder file should be created");

    let runtime_dir_text = runtime_dir.display().to_string();
    let output = run_local_minimal_cli(&[
        "list-runtime-backups",
        "--runtime-dir",
        runtime_dir_text.as_str(),
    ]);

    assert!(
        !output.status.success(),
        "list-runtime-backups must fail when backups path is not a directory. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("failed to read runtime-dir backups dir"),
        "cli stderr should explain invalid backups path. stderr: {stderr}"
    );
    assert!(
        !stderr.to_lowercase().contains("panicked"),
        "cli must return a controlled error instead of panic. stderr: {stderr}"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[test]
fn test_local_minimal_cli_repair_rejects_backups_path_that_is_not_a_directory() {
    let runtime_dir = unique_runtime_dir("runtime_cli_repair_backups_file");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    fs::write(runtime_dir.join("backups"), "not-a-directory")
        .expect("backups path placeholder file should be created");

    let runtime_dir_text = runtime_dir.display().to_string();
    let output = run_local_minimal_cli(&["repair-runtime-dir", "--runtime-dir", &runtime_dir_text]);

    assert!(
        !output.status.success(),
        "repair-runtime-dir must fail when backups path is not a directory. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("failed to create runtime-dir repair backup dir"),
        "cli stderr should explain invalid repair backup dir. stderr: {stderr}"
    );
    assert!(
        !stderr.to_lowercase().contains("panicked"),
        "cli must return a controlled error instead of panic. stderr: {stderr}"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}
