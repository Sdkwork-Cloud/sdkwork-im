use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::Duration;

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
