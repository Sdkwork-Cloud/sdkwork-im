use std::fs;
use std::path::Path;
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
    std::env::temp_dir().join(format!("sdkwork_im_{prefix}_{unique}_{sequence}"))
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn run_local_minimal_cli(args: &[&str]) -> Output {
    run_local_minimal_cli_with_env(args, &[])
}

fn run_local_minimal_cli_with_env(args: &[&str], envs: &[(&str, &str)]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_local-minimal-node"));
    command
        .args(args)
        .env_remove("SDKWORK_IM_RUNTIME_PROFILE")
        .env_remove("SDKWORK_IM_STORAGE_PROVIDER")
        .env_remove("SDKWORK_IM_DATABASE_URL")
        .env_remove("SDKWORK_IM_POSTGRES_CONFIG")
        .env_remove("SDKWORK_IM_COMMERCIAL_EVIDENCE_ROOT")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (key, value) in envs {
        command.env(key, value);
    }

    let mut child = command
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

fn copy_step11_evidence_bundle(target_root: &Path) {
    let source_root = repo_root();
    let catalog_source = source_root.join("tools/perf/step-11-scenario-catalog.json");
    let metrics_source =
        source_root.join("artifacts/perf/step-11/pre-release/im-websocket-e2e/metrics.json");
    let catalog_target = target_root.join("tools/perf/step-11-scenario-catalog.json");
    let metrics_target =
        target_root.join("artifacts/perf/step-11/pre-release/im-websocket-e2e/metrics.json");

    fs::create_dir_all(
        catalog_target
            .parent()
            .expect("catalog path should have parent"),
    )
    .expect("target catalog directory should be created");
    fs::create_dir_all(
        metrics_target
            .parent()
            .expect("metrics path should have parent"),
    )
    .expect("target metrics directory should be created");
    fs::copy(catalog_source, catalog_target).expect("Step 11 catalog should copy");
    fs::copy(metrics_source, metrics_target).expect("Step 11 websocket metrics should copy");
}

#[test]
fn test_commercial_readiness_cli_reports_blockers_as_json() {
    let output = run_local_minimal_cli_with_env(
        &["commercial-readiness", "--json"],
        &[
            ("SDKWORK_IM_RUNTIME_PROFILE", "production-postgres"),
            ("SDKWORK_IM_STORAGE_PROVIDER", "postgresql"),
            (
                "SDKWORK_IM_DATABASE_URL",
                "postgres://example.invalid/sdkwork_im",
            ),
        ],
    );

    assert!(
        !output.status.success(),
        "commercial-readiness must fail while commercial blockers remain. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"status\": \"blocked\""),
        "commercial-readiness --json should print blocked status. stdout: {stdout}"
    );
    assert!(
        stdout.contains("postgres_runtime_adapter_contract_only"),
        "commercial-readiness --json should include PostgreSQL contract-only blocker. stdout: {stdout}"
    );
    assert!(
        stdout.contains("step11_pre_release_gate_not_passed"),
        "commercial-readiness --json should include Step 11 pre-release blocker. stdout: {stdout}"
    );
}

#[test]
fn test_commercial_readiness_cli_uses_explicit_evidence_root() {
    let evidence_root = unique_runtime_dir("commercial_evidence_root_cli");
    copy_step11_evidence_bundle(&evidence_root);
    let evidence_root_text = evidence_root.display().to_string();

    let output = run_local_minimal_cli_with_env(
        &[
            "commercial-readiness",
            "--evidence-root",
            evidence_root_text.as_str(),
            "--json",
        ],
        &[
            ("SDKWORK_IM_RUNTIME_PROFILE", "production-postgres"),
            ("SDKWORK_IM_STORAGE_PROVIDER", "postgresql"),
            (
                "SDKWORK_IM_DATABASE_URL",
                "postgres://example.invalid/sdkwork_im",
            ),
        ],
    );

    assert!(
        !output.status.success(),
        "commercial-readiness should use explicit evidence root and report blockers. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("step11_websocket_e2e_gate_not_passed"),
        "explicit evidence root should be parsed for websocket evidence. stdout: {stdout}"
    );

    let _ = fs::remove_dir_all(evidence_root);
}

#[test]
fn test_server_startup_blocks_production_profile_before_binding_listener() {
    let evidence_root = unique_runtime_dir("commercial_evidence_root_startup");
    copy_step11_evidence_bundle(&evidence_root);
    let evidence_root_text = evidence_root.display().to_string();

    let output = run_local_minimal_cli_with_env(
        &[],
        &[
            ("SDKWORK_IM_RUNTIME_PROFILE", "production-postgres"),
            ("SDKWORK_IM_STORAGE_PROVIDER", "postgresql"),
            (
                "SDKWORK_IM_DATABASE_URL",
                "postgres://example.invalid/sdkwork_im",
            ),
            ("SDKWORK_IM_BIND_ADDR", "127.0.0.1:0"),
            (
                "SDKWORK_IM_COMMERCIAL_EVIDENCE_ROOT",
                evidence_root_text.as_str(),
            ),
        ],
    );

    assert!(
        !output.status.success(),
        "production startup must fail closed while commercial readiness blockers remain. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("commercial readiness blocked"),
        "startup error should identify the commercial readiness gate. stderr: {stderr}"
    );
    assert!(
        stderr.contains("postgres_runtime_adapter_contract_only"),
        "startup error should include the strongest runtime blocker. stderr: {stderr}"
    );
    assert!(
        !stderr.to_lowercase().contains("panicked"),
        "startup gate must return a controlled error instead of panic. stderr: {stderr}"
    );

    let _ = fs::remove_dir_all(evidence_root);
}

#[test]
fn test_server_startup_rejects_invalid_postgres_config_without_leaking_password() {
    let runtime_dir = unique_runtime_dir("postgres_invalid_config_runtime");
    let config_root = unique_runtime_dir("postgres_invalid_config");
    let storage_dir = config_root.join("storage");
    let secrets_dir = config_root.join("secrets");
    fs::create_dir_all(&storage_dir).expect("storage dir should be created");
    fs::create_dir_all(&secrets_dir).expect("secrets dir should be created");
    fs::write(
        secrets_dir.join("postgresql.password"),
        "super-secret-password\n",
    )
    .expect("password file should be written");
    let postgres_config_path = storage_dir.join("postgresql.yaml");
    fs::write(
        &postgres_config_path,
        r#"provider: postgresql
connection:
  host: " "
  database: sdkwork_im
  username: sdkwork_im_app
  passwordFile: ./secrets/postgresql.password
  sslmode: trust-me
pool:
  minConnections: 31
  maxConnections: 30
"#,
    )
    .expect("postgres config should be written");

    let runtime_dir_text = runtime_dir.display().to_string();
    let postgres_config_text = postgres_config_path.display().to_string();
    let output = run_local_minimal_cli_with_env(
        &[],
        &[
            ("SDKWORK_IM_RUNTIME_DIR", runtime_dir_text.as_str()),
            ("SDKWORK_IM_STORAGE_PROVIDER", "postgresql"),
            ("SDKWORK_IM_POSTGRES_CONFIG", postgres_config_text.as_str()),
            ("SDKWORK_IM_BIND_ADDR", "127.0.0.1:0"),
        ],
    );

    assert!(
        !output.status.success(),
        "startup must fail closed for invalid PostgreSQL config. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("connection.host")
            && stderr.contains("connection.sslmode")
            && stderr.contains("pool.minConnections"),
        "startup error should expose actionable config fields. stderr: {stderr}"
    );
    assert!(
        !stderr.contains("super-secret-password"),
        "startup error must not leak PostgreSQL password contents. stderr: {stderr}"
    );
    assert!(
        !stderr.to_lowercase().contains("panicked"),
        "startup error must remain controlled instead of panicking. stderr: {stderr}"
    );

    let _ = fs::remove_dir_all(runtime_dir);
    let _ = fs::remove_dir_all(config_root);
}

#[test]
fn test_local_minimal_cli_rejects_missing_runtime_dir_option_value() {
    let output = run_local_minimal_cli(&["inspect-runtime_dir", "--runtime_dir"]);

    assert!(
        !output.status.success(),
        "inspect-runtime_dir must fail when --runtime_dir has no value. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--runtime_dir requires a value"),
        "cli stderr should explain missing --runtime_dir value. stderr: {stderr}"
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
        "--runtime_dir",
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
        stderr.contains("failed to read runtime_dir backups dir"),
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
    let output = run_local_minimal_cli(&["repair-runtime_dir", "--runtime_dir", &runtime_dir_text]);

    assert!(
        !output.status.success(),
        "repair-runtime_dir must fail when backups path is not a directory. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("failed to create runtime_dir repair backup dir"),
        "cli stderr should explain invalid repair backup dir. stderr: {stderr}"
    );
    assert!(
        !stderr.to_lowercase().contains("panicked"),
        "cli must return a controlled error instead of panic. stderr: {stderr}"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}
