use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("service dir should have parent")
        .parent()
        .expect("workspace root should exist")
        .to_path_buf()
}

fn first_non_empty_line(content: &str) -> &str {
    content
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .expect("content should contain at least one non-empty line")
}

fn unique_temp_root(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_{prefix}_{unique}"))
}

fn resolve_usable_bash() -> Option<PathBuf> {
    let mut candidates = Vec::new();
    #[cfg(windows)]
    {
        candidates.push(PathBuf::from(r"C:\Program Files\Git\bin\bash.exe"));
        candidates.push(PathBuf::from(r"C:\Program Files\Git\usr\bin\bash.exe"));
    }
    candidates.push(PathBuf::from("bash"));

    candidates.into_iter().find(|candidate| {
        Command::new(candidate)
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    })
}

#[cfg(windows)]
fn windows_system_executable(file_name: &str) -> PathBuf {
    let windows_dir = std::env::var_os("WINDIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"C:\Windows"));
    windows_dir.join("System32").join(file_name)
}

#[test]
fn test_restart_local_sh_propagates_stop_failure_before_starting_new_instance() {
    let root = workspace_root();
    let temp_root = unique_temp_root("restart_sh_stop_failure_behavior");
    let bin_dir = temp_root.join("bin");
    let start_marker = temp_root.join("start-invoked.marker");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    fs::copy(
        root.join("bin").join("restart-local.sh"),
        bin_dir.join("restart-local.sh"),
    )
    .expect("restart-local.sh should be copied into temp workspace");

    fs::write(
        bin_dir.join("stop-local.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\necho \"simulated stop failure\" >&2\nexit 17\n",
    )
    .expect("stub stop-local.sh should be written");
    fs::write(
        bin_dir.join("start-local.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nROOT_DIR=\"$(cd \"$(dirname \"${BASH_SOURCE[0]}\")/..\" && pwd)\"\ntouch \"${ROOT_DIR}/start-invoked.marker\"\n",
    )
    .expect("stub start-local.sh should be written");

    let Some(bash_path) = resolve_usable_bash() else {
        eprintln!(
            "skipping restart-local.sh behavior regression because no usable bash runtime is available"
        );
        let _ = fs::remove_dir_all(&temp_root);
        return;
    };

    let output = Command::new(&bash_path)
        .current_dir(&temp_root)
        .arg("bin/restart-local.sh")
        .output()
        .expect("restart-local.sh should execute in temp workspace");

    assert!(
        !output.status.success(),
        "restart-local.sh should fail when stop-local.sh exits non-zero"
    );
    assert!(
        !start_marker.exists(),
        "restart-local.sh must not invoke start-local.sh after stop-local.sh fails"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("simulated stop failure"),
        "restart-local.sh should surface stop-local.sh stderr on failure. actual stderr: {stderr}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_restart_local_ps1_propagates_terminating_stop_failure_before_starting_new_instance() {
    let root = workspace_root();
    let temp_root = unique_temp_root("restart_ps1_stop_throw");
    let bin_dir = temp_root.join("bin");
    let start_marker = temp_root.join("start-invoked.marker");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    fs::copy(
        root.join("bin").join("restart-local.ps1"),
        bin_dir.join("restart-local.ps1"),
    )
    .expect("restart-local.ps1 should be copied into temp workspace");

    fs::write(
        bin_dir.join("stop-local.ps1"),
        "Write-Host 'stub stop'\r\nthrow 'simulated terminating stop failure'\r\n",
    )
    .expect("stub stop-local.ps1 should be written");
    fs::write(
        bin_dir.join("start-local.ps1"),
        "$root = Split-Path -Parent $PSScriptRoot\r\nNew-Item -ItemType File -Force -Path (Join-Path $root 'start-invoked.marker') | Out-Null\r\nWrite-Host 'stub start'\r\n",
    )
    .expect("stub start-local.ps1 should be written");

    let output = Command::new("powershell")
        .current_dir(&temp_root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\restart-local.ps1",
        ])
        .output()
        .expect("restart-local.ps1 should execute in temp workspace");

    assert!(
        !output.status.success(),
        "restart-local.ps1 should fail when stop-local.ps1 throws"
    );
    assert!(
        !start_marker.exists(),
        "restart-local.ps1 must not invoke start-local.ps1 after a terminating stop-local.ps1 failure"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}\n{stderr}");
    assert!(
        combined.contains("simulated terminating stop failure"),
        "restart-local.ps1 should surface stop-local.ps1 terminating failure details. actual output: {combined}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_restart_local_ps1_propagates_non_zero_stop_exit_before_starting_new_instance() {
    let root = workspace_root();
    let temp_root = unique_temp_root("restart_ps1_stop_exit");
    let bin_dir = temp_root.join("bin");
    let start_marker = temp_root.join("start-invoked.marker");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    fs::copy(
        root.join("bin").join("restart-local.ps1"),
        bin_dir.join("restart-local.ps1"),
    )
    .expect("restart-local.ps1 should be copied into temp workspace");

    fs::write(
        bin_dir.join("stop-local.ps1"),
        "Write-Host 'stub stop'\r\nexit 9\r\n",
    )
    .expect("stub stop-local.ps1 should be written");
    fs::write(
        bin_dir.join("start-local.ps1"),
        "$root = Split-Path -Parent $PSScriptRoot\r\nNew-Item -ItemType File -Force -Path (Join-Path $root 'start-invoked.marker') | Out-Null\r\nWrite-Host 'stub start'\r\n",
    )
    .expect("stub start-local.ps1 should be written");

    let output = Command::new("powershell")
        .current_dir(&temp_root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\restart-local.ps1",
        ])
        .output()
        .expect("restart-local.ps1 should execute in temp workspace");

    assert!(
        !output.status.success(),
        "restart-local.ps1 should fail when stop-local.ps1 exits non-zero"
    );
    assert!(
        !start_marker.exists(),
        "restart-local.ps1 must not invoke start-local.ps1 after a non-zero stop-local.ps1 exit"
    );
    assert_eq!(
        output.status.code(),
        Some(9),
        "restart-local.ps1 should preserve the stop-local.ps1 exit code"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("stub stop"),
        "restart-local.ps1 should surface stop-local.ps1 output before exiting. actual stdout: {stdout}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_stop_local_ps1_does_not_kill_unmanaged_process_from_stale_pid_file() {
    let root = workspace_root();
    let temp_root = unique_temp_root("stop_ps1_unmanaged_pid");
    let bin_dir = temp_root.join("bin");
    let pid_dir = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("pids");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&pid_dir).expect("temp pid dir should be created");

    fs::copy(
        root.join("bin").join("stop-local.ps1"),
        bin_dir.join("stop-local.ps1"),
    )
    .expect("stop-local.ps1 should be copied into temp workspace");

    let mut unrelated_process = Command::new("powershell")
        .args(["-NoProfile", "-Command", "Start-Sleep -Seconds 30"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("unrelated powershell process should start");

    fs::write(
        pid_dir.join("local-minimal-node.pid"),
        unrelated_process.id().to_string(),
    )
    .expect("pid file should be written");

    let output = Command::new("powershell")
        .current_dir(&temp_root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\stop-local.ps1",
        ])
        .output()
        .expect("stop-local.ps1 should execute in temp workspace");

    assert!(
        output.status.success(),
        "stop-local.ps1 should treat an unmanaged process PID as stale metadata"
    );
    assert!(
        unrelated_process
            .try_wait()
            .expect("unrelated process wait state should be readable")
            .is_none(),
        "stop-local.ps1 must not kill an unrelated process that reused or occupied the pid file"
    );
    assert!(
        !pid_dir.join("local-minimal-node.pid").exists(),
        "stop-local.ps1 should remove a stale pid file that points to an unmanaged process"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("local-minimal-node is not running."),
        "stop-local.ps1 should normalize unmanaged pid-file targets as not running. actual stdout: {stdout}"
    );

    let _ = unrelated_process.kill();
    let _ = unrelated_process.wait();
    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_status_local_ps1_treats_unmanaged_process_from_stale_pid_file_as_stopped() {
    let root = workspace_root();
    let temp_root = unique_temp_root("status_ps1_unmanaged_pid");
    let bin_dir = temp_root.join("bin");
    let pid_dir = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("pids");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&pid_dir).expect("temp pid dir should be created");

    fs::copy(
        root.join("bin").join("status-local.ps1"),
        bin_dir.join("status-local.ps1"),
    )
    .expect("status-local.ps1 should be copied into temp workspace");

    let mut unrelated_process = Command::new("powershell")
        .args(["-NoProfile", "-Command", "Start-Sleep -Seconds 30"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("unrelated powershell process should start");

    fs::write(
        pid_dir.join("local-minimal-node.pid"),
        unrelated_process.id().to_string(),
    )
    .expect("pid file should be written");

    let output = Command::new("powershell")
        .current_dir(&temp_root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\status-local.ps1",
        ])
        .output()
        .expect("status-local.ps1 should execute in temp workspace");

    assert!(output.status.success(), "status-local.ps1 should succeed");
    assert!(
        unrelated_process
            .try_wait()
            .expect("unrelated process wait state should be readable")
            .is_none(),
        "status-local.ps1 must not disturb an unrelated process from a stale pid file"
    );
    assert!(
        !pid_dir.join("local-minimal-node.pid").exists(),
        "status-local.ps1 should remove a stale pid file that points to an unmanaged process"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("status: stopped"),
        "status-local.ps1 should treat an unmanaged pid-file target as stopped. actual stdout: {stdout}"
    );
    assert!(
        !stdout.contains("status: running"),
        "status-local.ps1 must not report an unmanaged pid-file target as running. actual stdout: {stdout}"
    );

    let _ = unrelated_process.kill();
    let _ = unrelated_process.wait();
    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_start_local_ps1_ignores_unmanaged_process_from_stale_pid_file() {
    let root = workspace_root();
    let temp_root = unique_temp_root("start_ps1_unmanaged_pid");
    let bin_dir = temp_root.join("bin");
    let fake_tools_dir = temp_root.join("fake-tools");
    let target_debug_dir = temp_root.join("target").join("debug");
    let pid_dir = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("pids");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&fake_tools_dir).expect("temp fake-tools dir should be created");
    fs::create_dir_all(&target_debug_dir).expect("temp debug target dir should be created");
    fs::create_dir_all(&pid_dir).expect("temp pid dir should be created");

    for file_name in [
        "start-local.ps1",
        "install-local.ps1",
        "init-config-local.ps1",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        fake_tools_dir.join("cargo.cmd"),
        "@echo off\r\nexit /b 0\r\n",
    )
    .expect("fake cargo.cmd should be written");

    let mut unrelated_process = Command::new("powershell")
        .args(["-NoProfile", "-Command", "Start-Sleep -Seconds 30"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("unrelated powershell process should start");

    fs::write(
        pid_dir.join("local-minimal-node.pid"),
        unrelated_process.id().to_string(),
    )
    .expect("pid file should be written");

    let whoami_path = windows_system_executable("whoami.exe");
    fs::copy(
        &whoami_path,
        target_debug_dir.join("local-minimal-node.exe"),
    )
    .unwrap_or_else(|_| {
        panic!(
            "failed to copy fake node binary from {}",
            whoami_path.display()
        )
    });

    let original_path =
        std::env::var_os("PATH").expect("PATH must be available to run lifecycle scripts");
    let temp_path = format!(
        "{};{}",
        fake_tools_dir.display(),
        PathBuf::from(original_path).display()
    );

    let output = Command::new("powershell")
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\start-local.ps1",
        ])
        .output()
        .expect("start-local.ps1 should execute against temp workspace");

    assert!(
        !output.status.success(),
        "fake binary should still fail readiness after stale unmanaged pid metadata is ignored"
    );
    assert!(
        unrelated_process
            .try_wait()
            .expect("unrelated process wait state should be readable")
            .is_none(),
        "start-local.ps1 must not treat an unrelated process from the pid file as a managed running instance"
    );
    assert!(
        !pid_dir.join("local-minimal-node.pid").exists(),
        "start-local.ps1 should clear stale unmanaged pid metadata during failed startup"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}\n{stderr}");
    assert!(
        !combined.contains("already running"),
        "start-local.ps1 must not reject startup because of an unrelated process in the stale pid file. actual output: {combined}"
    );
    assert!(
        combined.contains("Starting local-minimal-node in background"),
        "start-local.ps1 should proceed to launch after clearing the unmanaged pid file. actual output: {combined}"
    );

    let _ = unrelated_process.kill();
    let _ = unrelated_process.wait();
    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints() {
    let root = workspace_root();
    let dockerfile_path = root
        .join("deployments")
        .join("docker")
        .join("local-minimal.Dockerfile");
    let compose_path = root
        .join("deployments")
        .join("docker-compose")
        .join("local-minimal.yml");
    let bootstrap_path = root
        .join("deployments")
        .join("scripts")
        .join("bootstrap-local.ps1");
    let bin_install_ps1_path = root.join("bin").join("install-local.ps1");
    let bin_install_sh_path = root.join("bin").join("install-local.sh");
    let bin_install_cmd_path = root.join("bin").join("install-local.cmd");
    let bin_deploy_ps1_path = root.join("bin").join("deploy-local.ps1");
    let bin_deploy_sh_path = root.join("bin").join("deploy-local.sh");
    let bin_deploy_cmd_path = root.join("bin").join("deploy-local.cmd");
    let bin_start_ps1_path = root.join("bin").join("start-local.ps1");
    let bin_start_sh_path = root.join("bin").join("start-local.sh");
    let bin_start_cmd_path = root.join("bin").join("start-local.cmd");
    let bin_stop_ps1_path = root.join("bin").join("stop-local.ps1");
    let bin_stop_sh_path = root.join("bin").join("stop-local.sh");
    let bin_stop_cmd_path = root.join("bin").join("stop-local.cmd");
    let bin_cmd_forwarder_path = root.join("bin").join("_cmd-forward-powershell.cmd");
    let bin_restart_ps1_path = root.join("bin").join("restart-local.ps1");
    let bin_restart_sh_path = root.join("bin").join("restart-local.sh");
    let bin_restart_cmd_path = root.join("bin").join("restart-local.cmd");
    let bin_status_ps1_path = root.join("bin").join("status-local.ps1");
    let bin_status_sh_path = root.join("bin").join("status-local.sh");
    let bin_status_cmd_path = root.join("bin").join("status-local.cmd");
    let bin_inspect_runtime_ps1_path = root.join("bin").join("inspect-runtime-local.ps1");
    let bin_inspect_runtime_sh_path = root.join("bin").join("inspect-runtime-local.sh");
    let bin_inspect_runtime_cmd_path = root.join("bin").join("inspect-runtime-local.cmd");
    let bin_repair_runtime_ps1_path = root.join("bin").join("repair-runtime-local.ps1");
    let bin_repair_runtime_sh_path = root.join("bin").join("repair-runtime-local.sh");
    let bin_repair_runtime_cmd_path = root.join("bin").join("repair-runtime-local.cmd");
    let bin_restore_runtime_ps1_path = root.join("bin").join("restore-runtime-local.ps1");
    let bin_restore_runtime_sh_path = root.join("bin").join("restore-runtime-local.sh");
    let bin_restore_runtime_cmd_path = root.join("bin").join("restore-runtime-local.cmd");
    let bin_preview_restore_runtime_ps1_path =
        root.join("bin").join("preview-runtime-restore-local.ps1");
    let bin_preview_restore_runtime_sh_path =
        root.join("bin").join("preview-runtime-restore-local.sh");
    let bin_preview_restore_runtime_cmd_path =
        root.join("bin").join("preview-runtime-restore-local.cmd");
    let bin_list_runtime_backups_ps1_path = root.join("bin").join("list-runtime-backups-local.ps1");
    let bin_list_runtime_backups_sh_path = root.join("bin").join("list-runtime-backups-local.sh");
    let bin_list_runtime_backups_cmd_path = root.join("bin").join("list-runtime-backups-local.cmd");
    let bin_init_config_ps1_path = root.join("bin").join("init-config-local.ps1");
    let bin_init_config_sh_path = root.join("bin").join("init-config-local.sh");
    let bin_init_config_cmd_path = root.join("bin").join("init-config-local.cmd");
    let smoke_path = root
        .join("tools")
        .join("smoke")
        .join("local_stack_smoke.ps1");
    let smoke_sh_path = root
        .join("tools")
        .join("smoke")
        .join("local_stack_smoke.sh");
    let local_memory_adapter_path = root
        .join("adapters")
        .join("local-memory")
        .join("Cargo.toml");
    let redpanda_readme_path = root
        .join("adapters")
        .join("journal-redpanda")
        .join("README.md");
    let cockroach_readme_path = root
        .join("adapters")
        .join("meta-cockroach")
        .join("README.md");
    let scylla_readme_path = root
        .join("adapters")
        .join("timeline-scylla")
        .join("README.md");

    let dockerfile = fs::read_to_string(&dockerfile_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment dockerfile: {}",
            dockerfile_path.display()
        )
    });
    let compose = fs::read_to_string(&compose_path)
        .unwrap_or_else(|_| panic!("missing compose profile: {}", compose_path.display()));
    let bootstrap = fs::read_to_string(&bootstrap_path)
        .unwrap_or_else(|_| panic!("missing bootstrap script: {}", bootstrap_path.display()));
    let bin_install_ps1 = fs::read_to_string(&bin_install_ps1_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_install_ps1_path.display()));
    let bin_install_sh = fs::read_to_string(&bin_install_sh_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_install_sh_path.display()));
    let bin_install_cmd = fs::read_to_string(&bin_install_cmd_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_install_cmd_path.display()));
    let bin_deploy_ps1 = fs::read_to_string(&bin_deploy_ps1_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_deploy_ps1_path.display()));
    let bin_deploy_sh = fs::read_to_string(&bin_deploy_sh_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_deploy_sh_path.display()));
    let bin_deploy_cmd = fs::read_to_string(&bin_deploy_cmd_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_deploy_cmd_path.display()));
    let bin_start_ps1 = fs::read_to_string(&bin_start_ps1_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_start_ps1_path.display()));
    let bin_start_sh = fs::read_to_string(&bin_start_sh_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_start_sh_path.display()));
    let bin_start_cmd = fs::read_to_string(&bin_start_cmd_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_start_cmd_path.display()));
    let bin_stop_ps1 = fs::read_to_string(&bin_stop_ps1_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_stop_ps1_path.display()));
    let bin_stop_sh = fs::read_to_string(&bin_stop_sh_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_stop_sh_path.display()));
    let bin_stop_cmd = fs::read_to_string(&bin_stop_cmd_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_stop_cmd_path.display()));
    let bin_cmd_forwarder = fs::read_to_string(&bin_cmd_forwarder_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_cmd_forwarder_path.display()));
    let bin_restart_ps1 = fs::read_to_string(&bin_restart_ps1_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_restart_ps1_path.display()));
    let bin_restart_sh = fs::read_to_string(&bin_restart_sh_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_restart_sh_path.display()));
    let bin_restart_cmd = fs::read_to_string(&bin_restart_cmd_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_restart_cmd_path.display()));
    let bin_status_ps1 = fs::read_to_string(&bin_status_ps1_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_status_ps1_path.display()));
    let bin_status_sh = fs::read_to_string(&bin_status_sh_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_status_sh_path.display()));
    let bin_status_cmd = fs::read_to_string(&bin_status_cmd_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_status_cmd_path.display()));
    let bin_inspect_runtime_ps1 =
        fs::read_to_string(&bin_inspect_runtime_ps1_path).unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_inspect_runtime_ps1_path.display()
            )
        });
    let bin_inspect_runtime_sh =
        fs::read_to_string(&bin_inspect_runtime_sh_path).unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_inspect_runtime_sh_path.display()
            )
        });
    let bin_inspect_runtime_cmd =
        fs::read_to_string(&bin_inspect_runtime_cmd_path).unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_inspect_runtime_cmd_path.display()
            )
        });
    let bin_repair_runtime_ps1 =
        fs::read_to_string(&bin_repair_runtime_ps1_path).unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_repair_runtime_ps1_path.display()
            )
        });
    let bin_repair_runtime_sh =
        fs::read_to_string(&bin_repair_runtime_sh_path).unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_repair_runtime_sh_path.display()
            )
        });
    let bin_repair_runtime_cmd =
        fs::read_to_string(&bin_repair_runtime_cmd_path).unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_repair_runtime_cmd_path.display()
            )
        });
    let bin_restore_runtime_ps1 =
        fs::read_to_string(&bin_restore_runtime_ps1_path).unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_restore_runtime_ps1_path.display()
            )
        });
    let bin_restore_runtime_sh =
        fs::read_to_string(&bin_restore_runtime_sh_path).unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_restore_runtime_sh_path.display()
            )
        });
    let bin_restore_runtime_cmd =
        fs::read_to_string(&bin_restore_runtime_cmd_path).unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_restore_runtime_cmd_path.display()
            )
        });
    let bin_preview_restore_runtime_ps1 = fs::read_to_string(&bin_preview_restore_runtime_ps1_path)
        .unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_preview_restore_runtime_ps1_path.display()
            )
        });
    let bin_preview_restore_runtime_sh = fs::read_to_string(&bin_preview_restore_runtime_sh_path)
        .unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_preview_restore_runtime_sh_path.display()
            )
        });
    let bin_preview_restore_runtime_cmd = fs::read_to_string(&bin_preview_restore_runtime_cmd_path)
        .unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_preview_restore_runtime_cmd_path.display()
            )
        });
    let bin_list_runtime_backups_ps1 = fs::read_to_string(&bin_list_runtime_backups_ps1_path)
        .unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_list_runtime_backups_ps1_path.display()
            )
        });
    let bin_list_runtime_backups_sh = fs::read_to_string(&bin_list_runtime_backups_sh_path)
        .unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_list_runtime_backups_sh_path.display()
            )
        });
    let bin_list_runtime_backups_cmd = fs::read_to_string(&bin_list_runtime_backups_cmd_path)
        .unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_list_runtime_backups_cmd_path.display()
            )
        });
    let bin_init_config_ps1 = fs::read_to_string(&bin_init_config_ps1_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_init_config_ps1_path.display()));
    let bin_init_config_sh = fs::read_to_string(&bin_init_config_sh_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_init_config_sh_path.display()));
    let bin_init_config_cmd = fs::read_to_string(&bin_init_config_cmd_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_init_config_cmd_path.display()));
    let smoke = fs::read_to_string(&smoke_path)
        .unwrap_or_else(|_| panic!("missing smoke script: {}", smoke_path.display()));
    let smoke_sh = fs::read_to_string(&smoke_sh_path)
        .unwrap_or_else(|_| panic!("missing smoke script: {}", smoke_sh_path.display()));
    let local_memory_adapter =
        fs::read_to_string(&local_memory_adapter_path).unwrap_or_else(|_| {
            panic!(
                "missing local memory adapter cargo manifest: {}",
                local_memory_adapter_path.display()
            )
        });
    let redpanda_readme = fs::read_to_string(&redpanda_readme_path)
        .unwrap_or_else(|_| panic!("missing adapter README: {}", redpanda_readme_path.display()));
    let cockroach_readme = fs::read_to_string(&cockroach_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing adapter README: {}",
            cockroach_readme_path.display()
        )
    });
    let scylla_readme = fs::read_to_string(&scylla_readme_path)
        .unwrap_or_else(|_| panic!("missing adapter README: {}", scylla_readme_path.display()));

    assert!(dockerfile.contains("local-minimal-node"));
    assert!(dockerfile.contains("cargo build --release -p local-minimal-node"));
    assert!(dockerfile.contains("EXPOSE 18090"));

    assert!(compose.contains("local-minimal-node"));
    assert!(compose.contains("18090:18090"));
    assert!(compose.contains("healthz"));

    assert!(bootstrap.contains("deployments/docker-compose/local-minimal.yml"));
    assert!(bootstrap.contains("docker compose"));
    assert!(bootstrap.contains("docker compose version"));
    assert!(bootstrap.contains("docker --version"));
    assert!(bootstrap.contains("docker info"));
    assert!(bootstrap.contains("Docker daemon"));
    assert!(bootstrap.contains("Docker compose failed"));
    assert!(bootstrap.contains("docker compose -f $composeFile ps"));
    assert!(bootstrap.contains("logs --tail 200"));
    assert!(bootstrap.contains("Smoke verification failed"));
    assert!(bootstrap.contains("without smoke verification"));
    assert!(bootstrap.contains("local_stack_smoke.ps1"));
    assert_eq!(first_non_empty_line(&bootstrap), "param(");

    assert!(bin_install_ps1.contains("cargo build -p local-minimal-node --offline"));
    assert!(bin_install_ps1.contains(".runtime"));
    assert_eq!(first_non_empty_line(&bin_install_ps1), "param(");
    assert!(bin_install_ps1.contains("$PSBoundParameters.ContainsKey('BindAddress')"));
    assert!(bin_install_ps1.contains("-Force:$bindAddressProvided"));
    assert!(bin_install_sh.contains("cargo build -p local-minimal-node --offline"));
    assert!(bin_install_sh.contains(".runtime"));
    assert_eq!(first_non_empty_line(&bin_install_sh), "#!/usr/bin/env bash");
    assert!(bin_install_sh.contains("bind_addr_provided=0"));
    assert!(bin_install_sh.contains("--force"));
    assert!(bin_install_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(!bin_install_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File"));

    assert!(bin_deploy_ps1.contains("bootstrap-local.ps1"));
    assert!(bin_deploy_ps1.contains("docker compose"));
    assert_eq!(first_non_empty_line(&bin_deploy_ps1), "param(");
    assert!(bin_deploy_sh.contains("deployments/docker-compose/local-minimal.yml"));
    assert!(bin_deploy_sh.contains("docker compose"));
    assert!(bin_deploy_sh.contains("docker compose version"));
    assert!(bin_deploy_sh.contains("tools/smoke/local_stack_smoke.sh"));
    assert!(bin_deploy_sh.contains("docker compose -f \"$COMPOSE_FILE\" ps"));
    assert!(bin_deploy_sh.contains("logs --tail 200"));
    assert!(bin_deploy_sh.contains("Smoke verification failed"));
    assert_eq!(first_non_empty_line(&bin_deploy_sh), "#!/usr/bin/env bash");
    assert!(bin_deploy_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(!bin_deploy_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File"));

    assert!(bin_start_ps1.contains("local-minimal-node"));
    assert!(bin_start_ps1.contains("Start-Process"));
    assert!(
        !bin_start_ps1.contains("$host ="),
        "start-local.ps1 must not assign the automatic $Host variable"
    );
    assert!(
        bin_start_ps1.contains("-RedirectStandardOutput"),
        "start-local.ps1 must redirect Start-Process stdout to the documented log file on Windows"
    );
    assert!(
        bin_start_ps1.contains("-RedirectStandardError"),
        "start-local.ps1 must redirect Start-Process stderr to the documented log file on Windows"
    );
    assert!(bin_start_ps1.contains("UseBasicParsing"));
    assert_eq!(first_non_empty_line(&bin_start_ps1), "param(");
    assert!(bin_start_ps1.contains("$PSBoundParameters.ContainsKey('BindAddress')"));
    assert!(!bin_start_ps1.contains("$installBindAddress ="));
    assert!(bin_start_ps1.contains("ExpectedProcessName = \"local-minimal-node\""));
    assert!(bin_start_ps1.contains("$process.ProcessName -ieq $ExpectedProcessName"));
    assert!(bin_start_ps1.contains("Stop-ManagedProcessAndRemovePidFile"));
    assert!(bin_start_ps1.contains("CRAW_CHAT_RUNTIME_DIR"));
    assert!(bin_start_ps1.contains("CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET"));
    assert!(bin_start_sh.contains("local-minimal-node"));
    assert!(bin_start_sh.contains("nohup"));
    assert_eq!(first_non_empty_line(&bin_start_sh), "#!/usr/bin/env bash");
    assert!(bin_start_sh.contains("bind_addr_provided=0"));
    assert!(bin_start_sh.contains("if [[ \"$bind_addr_provided\" -eq 1 ]]; then"));
    assert!(bin_start_sh.contains("command -v wget"));
    assert!(bin_start_sh.contains("wget -q -O /dev/null"));
    assert!(bin_start_sh.contains("Neither curl nor wget is available for health verification."));
    assert!(bin_start_sh.contains("CRAW_CHAT_RUNTIME_DIR"));
    assert!(bin_start_sh.contains("CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET"));
    assert!(bin_start_sh.contains("EXPECTED_PROCESS_NAME=\"local-minimal-node\""));
    assert!(bin_start_sh.contains("pid_matches_expected_process"));
    assert!(bin_start_sh.contains("stop_managed_process_and_remove_pid_file"));
    assert!(bin_start_sh.contains("kill -9 \"$pid\""));
    assert!(bin_start_sh.contains("return 1"));
    assert!(bin_start_sh.contains("ps -p \"$pid\" -o comm="));
    assert!(bin_start_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(!bin_start_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File"));

    assert!(bin_stop_ps1.contains("local-minimal-node.pid"));
    assert!(bin_stop_ps1.contains("Stop-Process"));
    assert!(bin_stop_ps1.contains("Wait-Process"));
    assert!(bin_stop_ps1.contains("did not exit within"));
    assert!(bin_stop_ps1.contains("ExpectedProcessName = \"local-minimal-node\""));
    assert!(bin_stop_ps1.contains("$process.ProcessName -ieq $ExpectedProcessName"));
    assert_eq!(first_non_empty_line(&bin_stop_ps1), "param(");
    assert!(bin_stop_sh.contains("local-minimal-node.pid"));
    assert!(bin_stop_sh.contains("kill"));
    assert!(bin_stop_sh.contains("for _ in $(seq 1 30)"));
    assert!(bin_stop_sh.contains("kill -0 \"$pid\""));
    assert!(bin_stop_sh.contains("did not exit within 30 seconds"));
    assert!(bin_stop_sh.contains("EXPECTED_PROCESS_NAME=\"local-minimal-node\""));
    assert!(bin_stop_sh.contains("pid_matches_expected_process"));
    assert!(bin_stop_sh.contains("ps -p \"$pid\" -o comm="));
    assert_eq!(first_non_empty_line(&bin_stop_sh), "#!/usr/bin/env bash");
    assert!(bin_stop_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(!bin_stop_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File"));

    assert!(bin_restart_ps1.contains("stop-local.ps1"));
    assert!(bin_restart_ps1.contains("start-local.ps1"));
    assert!(bin_restart_ps1.contains("$stopExitCode"));
    assert!(bin_restart_ps1.contains("exit $stopExitCode"));
    assert_eq!(first_non_empty_line(&bin_restart_ps1), "param(");
    assert!(bin_restart_sh.contains("stop-local.sh"));
    assert!(bin_restart_sh.contains("start-local.sh"));
    assert!(
        !bin_restart_sh.contains("|| true"),
        "restart-local.sh must not swallow stop-local.sh failures before starting a new instance"
    );
    assert_eq!(first_non_empty_line(&bin_restart_sh), "#!/usr/bin/env bash");
    assert!(bin_restart_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(!bin_restart_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File"));

    assert!(bin_status_ps1.contains("local-minimal-node.pid"));
    assert!(bin_status_ps1.contains("stdout"));
    assert!(bin_status_ps1.contains("health status:"));
    assert!(bin_status_ps1.contains("Invoke-WebRequest"));
    assert!(bin_status_ps1.contains("inspect-runtime-local.ps1"));
    assert!(bin_status_ps1.contains("repair-runtime-local.ps1"));
    assert!(bin_status_ps1.contains("list-runtime-backups-local.ps1"));
    assert!(bin_status_ps1.contains("preview-runtime-restore-local.ps1"));
    assert!(bin_status_ps1.contains("restore-runtime-local.ps1"));
    assert!(bin_status_ps1.contains("ExpectedPreviewFingerprint"));
    assert!(bin_status_ps1.contains("ExpectedProcessName = \"local-minimal-node\""));
    assert!(bin_status_ps1.contains("$process.ProcessName -ieq $ExpectedProcessName"));
    assert_eq!(first_non_empty_line(&bin_status_ps1), "param(");
    assert!(bin_status_sh.contains("local-minimal-node.pid"));
    assert!(bin_status_sh.contains("stdout"));
    assert!(bin_status_sh.contains("health status:"));
    assert!(bin_status_sh.contains("command -v wget"));
    assert!(bin_status_sh.contains("inspect-runtime-local.sh"));
    assert!(bin_status_sh.contains("repair-runtime-local.sh"));
    assert!(bin_status_sh.contains("list-runtime-backups-local.sh"));
    assert!(bin_status_sh.contains("preview-runtime-restore-local.sh"));
    assert!(bin_status_sh.contains("restore-runtime-local.sh"));
    assert!(bin_status_sh.contains("--expected-preview-fingerprint"));
    assert!(bin_status_sh.contains("EXPECTED_PROCESS_NAME=\"local-minimal-node\""));
    assert!(bin_status_sh.contains("pid_matches_expected_process"));
    assert!(bin_status_sh.contains("ps -p \"$pid\" -o comm="));
    assert_eq!(first_non_empty_line(&bin_status_sh), "#!/usr/bin/env bash");
    assert!(bin_status_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(!bin_status_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File"));

    assert!(bin_inspect_runtime_ps1.contains("inspect-runtime-dir"));
    assert!(bin_inspect_runtime_ps1.contains("CRAW_CHAT_RUNTIME_DIR"));
    assert!(bin_inspect_runtime_ps1.contains("target\\release\\local-minimal-node.exe"));
    assert!(bin_inspect_runtime_ps1.contains("target\\debug\\local-minimal-node.exe"));
    assert_eq!(first_non_empty_line(&bin_inspect_runtime_ps1), "param(");
    assert!(bin_inspect_runtime_sh.contains("inspect-runtime-dir"));
    assert!(bin_inspect_runtime_sh.contains("CRAW_CHAT_RUNTIME_DIR"));
    assert!(bin_inspect_runtime_sh.contains("target/release/local-minimal-node"));
    assert!(bin_inspect_runtime_sh.contains("target/debug/local-minimal-node"));
    assert_eq!(
        first_non_empty_line(&bin_inspect_runtime_sh),
        "#!/usr/bin/env bash"
    );
    assert!(bin_inspect_runtime_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(
        !bin_inspect_runtime_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File")
    );

    assert!(bin_repair_runtime_ps1.contains("repair-runtime-dir"));
    assert!(bin_repair_runtime_ps1.contains("CRAW_CHAT_RUNTIME_DIR"));
    assert!(bin_repair_runtime_ps1.contains("target\\release\\local-minimal-node.exe"));
    assert!(bin_repair_runtime_ps1.contains("target\\debug\\local-minimal-node.exe"));
    assert_eq!(first_non_empty_line(&bin_repair_runtime_ps1), "param(");
    assert!(bin_repair_runtime_sh.contains("repair-runtime-dir"));
    assert!(bin_repair_runtime_sh.contains("CRAW_CHAT_RUNTIME_DIR"));
    assert!(bin_repair_runtime_sh.contains("target/release/local-minimal-node"));
    assert!(bin_repair_runtime_sh.contains("target/debug/local-minimal-node"));
    assert_eq!(
        first_non_empty_line(&bin_repair_runtime_sh),
        "#!/usr/bin/env bash"
    );
    assert!(bin_repair_runtime_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(
        !bin_repair_runtime_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File")
    );

    assert!(bin_restore_runtime_ps1.contains("restore-runtime-dir"));
    assert!(bin_restore_runtime_ps1.contains("ExpectedPreviewFingerprint"));
    assert!(bin_restore_runtime_ps1.contains("--expected-preview-fingerprint"));
    assert!(bin_restore_runtime_ps1.contains("CRAW_CHAT_RUNTIME_DIR"));
    assert!(bin_restore_runtime_ps1.contains("target\\release\\local-minimal-node.exe"));
    assert!(bin_restore_runtime_ps1.contains("target\\debug\\local-minimal-node.exe"));
    assert_eq!(first_non_empty_line(&bin_restore_runtime_ps1), "param(");
    assert!(bin_restore_runtime_sh.contains("restore-runtime-dir"));
    assert!(bin_restore_runtime_sh.contains("expected_preview_fingerprint"));
    assert!(bin_restore_runtime_sh.contains("--expected-preview-fingerprint"));
    assert!(bin_restore_runtime_sh.contains("CRAW_CHAT_RUNTIME_DIR"));
    assert!(bin_restore_runtime_sh.contains("target/release/local-minimal-node"));
    assert!(bin_restore_runtime_sh.contains("target/debug/local-minimal-node"));
    assert_eq!(
        first_non_empty_line(&bin_restore_runtime_sh),
        "#!/usr/bin/env bash"
    );
    assert!(bin_restore_runtime_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(
        !bin_restore_runtime_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File")
    );

    assert!(bin_preview_restore_runtime_ps1.contains("preview-runtime-restore"));
    assert!(bin_preview_restore_runtime_ps1.contains("CRAW_CHAT_RUNTIME_DIR"));
    assert!(bin_preview_restore_runtime_ps1.contains("target\\release\\local-minimal-node.exe"));
    assert!(bin_preview_restore_runtime_ps1.contains("target\\debug\\local-minimal-node.exe"));
    assert_eq!(
        first_non_empty_line(&bin_preview_restore_runtime_ps1),
        "param("
    );
    assert!(bin_preview_restore_runtime_sh.contains("preview-runtime-restore"));
    assert!(bin_preview_restore_runtime_sh.contains("CRAW_CHAT_RUNTIME_DIR"));
    assert!(bin_preview_restore_runtime_sh.contains("target/release/local-minimal-node"));
    assert!(bin_preview_restore_runtime_sh.contains("target/debug/local-minimal-node"));
    assert_eq!(
        first_non_empty_line(&bin_preview_restore_runtime_sh),
        "#!/usr/bin/env bash"
    );
    assert!(bin_preview_restore_runtime_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(
        !bin_preview_restore_runtime_cmd
            .contains("powershell -NoProfile -ExecutionPolicy Bypass -File")
    );

    assert!(bin_list_runtime_backups_ps1.contains("list-runtime-backups"));
    assert!(bin_list_runtime_backups_ps1.contains("CRAW_CHAT_RUNTIME_DIR"));
    assert!(bin_list_runtime_backups_ps1.contains("target\\release\\local-minimal-node.exe"));
    assert!(bin_list_runtime_backups_ps1.contains("target\\debug\\local-minimal-node.exe"));
    assert_eq!(
        first_non_empty_line(&bin_list_runtime_backups_ps1),
        "param("
    );
    assert!(bin_list_runtime_backups_sh.contains("list-runtime-backups"));
    assert!(bin_list_runtime_backups_sh.contains("CRAW_CHAT_RUNTIME_DIR"));
    assert!(bin_list_runtime_backups_sh.contains("target/release/local-minimal-node"));
    assert!(bin_list_runtime_backups_sh.contains("target/debug/local-minimal-node"));
    assert_eq!(
        first_non_empty_line(&bin_list_runtime_backups_sh),
        "#!/usr/bin/env bash"
    );
    assert!(bin_list_runtime_backups_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(
        !bin_list_runtime_backups_cmd
            .contains("powershell -NoProfile -ExecutionPolicy Bypass -File")
    );

    assert!(bin_init_config_ps1.contains("CRAW_CHAT_BIND_ADDR"));
    assert!(bin_init_config_ps1.contains("CRAW_CHAT_RUNTIME_DIR"));
    assert!(bin_init_config_ps1.contains("CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET"));
    assert!(bin_init_config_ps1.contains("local-minimal.env"));
    assert!(bin_init_config_ps1.contains("state"));
    assert_eq!(first_non_empty_line(&bin_init_config_ps1), "param(");
    assert!(bin_init_config_sh.contains("CRAW_CHAT_BIND_ADDR"));
    assert!(bin_init_config_sh.contains("CRAW_CHAT_RUNTIME_DIR"));
    assert!(bin_init_config_sh.contains("CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET"));
    assert!(bin_init_config_sh.contains("local-minimal.env"));
    assert!(bin_init_config_sh.contains("state"));
    assert_eq!(
        first_non_empty_line(&bin_init_config_sh),
        "#!/usr/bin/env bash"
    );
    assert!(bin_init_config_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(!bin_init_config_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File"));

    assert!(bin_cmd_forwarder.contains("/force"));
    assert!(bin_cmd_forwarder.contains("-Force"));
    assert!(bin_cmd_forwarder.contains("/skipSmoke"));
    assert!(bin_cmd_forwarder.contains("--skip-smoke"));
    assert!(bin_cmd_forwarder.contains("-SkipSmoke"));
    assert!(bin_cmd_forwarder.contains("/bindAddress"));
    assert!(bin_cmd_forwarder.contains("-BindAddress"));
    assert!(bin_cmd_forwarder.contains("/runtimeDir"));
    assert!(bin_cmd_forwarder.contains("-RuntimeDir"));
    assert!(bin_cmd_forwarder.contains("--runtime-dir"));
    assert!(bin_cmd_forwarder.contains("/json"));
    assert!(bin_cmd_forwarder.contains("-Json"));
    assert!(bin_cmd_forwarder.contains("--json"));
    assert!(bin_cmd_forwarder.contains("/backupDir"));
    assert!(bin_cmd_forwarder.contains("-BackupDir"));
    assert!(bin_cmd_forwarder.contains("--backup-dir"));
    assert_eq!(first_non_empty_line(&bin_cmd_forwarder), "@echo off");

    assert!(smoke.contains("http://127.0.0.1:18090/healthz"));
    assert!(smoke.contains("Authorization"));
    assert_eq!(first_non_empty_line(&smoke), "param(");
    assert!(smoke_sh.contains("http://127.0.0.1:18090/healthz"));
    assert!(smoke_sh.contains("Authorization: Bearer"));
    assert!(smoke_sh.contains("command -v wget"));
    assert!(smoke_sh.contains("/api/v1/conversations"));
    assert_eq!(first_non_empty_line(&smoke_sh), "#!/usr/bin/env bash");

    assert!(local_memory_adapter.contains("name = \"im-adapters-local-memory\""));
    assert!(redpanda_readme.contains("# journal-redpanda"));
    assert!(cockroach_readme.contains("# meta-cockroach"));
    assert!(scylla_readme.contains("# timeline-scylla"));
}

#[cfg(windows)]
#[test]
fn test_init_config_cmd_normalizes_cmd_style_switches() {
    let root = workspace_root();
    let temp_root = unique_temp_root("cmd_wrapper");
    let bin_dir = temp_root.join("bin");
    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    for file_name in [
        "init-config-local.ps1",
        "init-config-local.cmd",
        "_cmd-forward-powershell.cmd",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    let seed_status = Command::new("cmd")
        .current_dir(&temp_root)
        .args([
            "/c",
            "bin\\init-config-local.cmd",
            "/bindAddress",
            "127.0.0.1:18090",
        ])
        .status()
        .expect("seed init-config-local.cmd should execute");
    assert!(seed_status.success());

    let overwrite_status = Command::new("cmd")
        .current_dir(&temp_root)
        .args([
            "/c",
            "bin\\init-config-local.cmd",
            "/force",
            "/bindAddress",
            "127.0.0.1:18101",
        ])
        .status()
        .expect("overwrite init-config-local.cmd should execute");
    assert!(overwrite_status.success());

    let config_file = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("config")
        .join("local-minimal.env");
    let config = fs::read_to_string(&config_file)
        .unwrap_or_else(|_| panic!("missing temp config file: {}", config_file.display()));
    assert!(config.contains("CRAW_CHAT_BIND_ADDR=127.0.0.1:18101"));
    assert!(config.contains("CRAW_CHAT_RUNTIME_DIR="));
    assert!(config.contains("CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET="));

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_install_local_cmd_rewrites_existing_config_when_bind_address_is_explicitly_provided() {
    let root = workspace_root();
    let temp_root = unique_temp_root("install_cmd_bind_override");
    let bin_dir = temp_root.join("bin");
    let fake_tools_dir = temp_root.join("fake-tools");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&fake_tools_dir).expect("temp fake-tools dir should be created");

    for file_name in [
        "init-config-local.ps1",
        "init-config-local.cmd",
        "install-local.ps1",
        "install-local.cmd",
        "_cmd-forward-powershell.cmd",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        fake_tools_dir.join("cargo.cmd"),
        "@echo off\r\nexit /b 0\r\n",
    )
    .expect("fake cargo.cmd should be written");

    let original_path =
        std::env::var_os("PATH").expect("PATH must be available to run lifecycle scripts");
    let temp_path = format!(
        "{};{}",
        fake_tools_dir.display(),
        PathBuf::from(original_path).display()
    );

    let seed_status = Command::new("cmd")
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .args([
            "/c",
            "bin\\init-config-local.cmd",
            "/bindAddress",
            "127.0.0.1:18090",
        ])
        .status()
        .expect("seed init-config-local.cmd should execute");
    assert!(seed_status.success());

    let install_status = Command::new("cmd")
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .args([
            "/c",
            "bin\\install-local.cmd",
            "/bindAddress",
            "127.0.0.1:18111",
        ])
        .status()
        .expect("install-local.cmd should execute");
    assert!(install_status.success());

    let config_file = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("config")
        .join("local-minimal.env");
    let config = fs::read_to_string(&config_file)
        .unwrap_or_else(|_| panic!("missing temp config file: {}", config_file.display()));
    assert!(config.contains("CRAW_CHAT_BIND_ADDR=127.0.0.1:18111"));
    assert!(config.contains("CRAW_CHAT_RUNTIME_DIR="));
    assert!(config.contains("CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET="));

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_deploy_local_cmd_normalizes_skip_smoke_switches() {
    let root = workspace_root();
    let temp_root = unique_temp_root("deploy_cmd_skip_smoke");
    let bin_dir = temp_root.join("bin");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    for file_name in ["deploy-local.cmd", "_cmd-forward-powershell.cmd"] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        bin_dir.join("deploy-local.ps1"),
        "param([switch]$SkipSmoke)\r\nif (-not $SkipSmoke) { throw 'SkipSmoke switch was not forwarded.' }\r\nWrite-Host 'skip smoke forwarded'\r\n",
    )
    .expect("stub deploy-local.ps1 should be written");

    let status = Command::new("cmd")
        .current_dir(&temp_root)
        .args(["/c", "bin\\deploy-local.cmd", "--skip-smoke"])
        .status()
        .expect("deploy-local.cmd should execute");
    assert!(status.success());

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_start_local_cmd_normalizes_documented_long_switches() {
    let root = workspace_root();
    let temp_root = unique_temp_root("start_cmd_long_switches");
    let bin_dir = temp_root.join("bin");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    for file_name in ["start-local.cmd", "_cmd-forward-powershell.cmd"] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        bin_dir.join("start-local.ps1"),
        "param([switch]$Release, [switch]$Foreground, [string]$BindAddress)\r\nif (-not $Release) { throw 'Release switch was not forwarded.' }\r\nif (-not $Foreground) { throw 'Foreground switch was not forwarded.' }\r\nif ($BindAddress -ne '127.0.0.1:19090') { throw \"BindAddress was not forwarded: $BindAddress\" }\r\nWrite-Host 'documented switches forwarded'\r\n",
    )
    .expect("stub start-local.ps1 should be written");

    let status = Command::new("cmd")
        .current_dir(&temp_root)
        .args([
            "/c",
            "bin\\start-local.cmd",
            "--release",
            "--foreground",
            "--bind-addr",
            "127.0.0.1:19090",
        ])
        .status()
        .expect("start-local.cmd should execute");
    assert!(status.success());

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_open_chat_test_ps1_uses_detached_gui_launcher_for_default_windows_mode() {
    let root = workspace_root();
    let script_path = root.join("bin").join("open-chat-test.ps1");
    let script = fs::read_to_string(&script_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", script_path.display()));

    assert!(
        script.contains("Invoke-CimMethod -ClassName Win32_Process -MethodName Create"),
        "open-chat-test.ps1 must launch default GUI chat windows through Win32_Process.Create so windows opened by automation survive the parent host lifecycle"
    );
    assert!(
        script.contains("wscript.exe"),
        "open-chat-test.ps1 must keep a detached GUI launch fallback for Windows environments where CIM create is unavailable"
    );
}

#[cfg(windows)]
#[test]
fn test_chat_window_gui_ps1_uses_polling_runtime_instead_of_async_child_stdio_bridge() {
    let root = workspace_root();
    let script_path = root.join("bin").join("chat-window-gui.ps1");
    let script = fs::read_to_string(&script_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", script_path.display()));

    assert!(
        script.contains("System.Windows.Forms.Timer"),
        "chat-window-gui.ps1 must use a timer-based polling loop for stable transcript refresh"
    );
    assert!(
        script.contains("timeline"),
        "chat-window-gui.ps1 must poll conversation timeline through chat-cli for transcript refresh"
    );
    assert!(
        script.contains("send-message"),
        "chat-window-gui.ps1 must send messages through chat-cli commands"
    );
    assert!(
        !script.contains("BeginOutputReadLine"),
        "chat-window-gui.ps1 must not depend on redirected child stdout readers because that bridge is unstable in the Windows PowerShell GUI host"
    );
}

#[cfg(windows)]
#[test]
fn test_chat_window_gui_ps1_reads_cli_json_via_utf8_process_io() {
    let root = workspace_root();
    let script_path = root.join("bin").join("chat-window-gui.ps1");
    let script = fs::read_to_string(&script_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", script_path.display()));

    assert!(
        script.contains("StandardOutputEncoding = [System.Text.Encoding]::UTF8"),
        "chat-window-gui.ps1 must force UTF-8 when reading craw-chat-cli stdout in detached Windows PowerShell hosts"
    );
    assert!(
        script.contains("StandardErrorEncoding = [System.Text.Encoding]::UTF8"),
        "chat-window-gui.ps1 must force UTF-8 when reading craw-chat-cli stderr in detached Windows PowerShell hosts"
    );
    assert!(
        script.contains("craw-chat-cli.exe"),
        "chat-window-gui.ps1 must invoke the built craw-chat-cli.exe directly instead of capturing chat-cli.ps1 text output through the PowerShell pipeline"
    );
}

#[cfg(windows)]
#[test]
fn test_start_local_ps1_captures_background_process_stdout_into_documented_log_file() {
    let root = workspace_root();
    let temp_root = unique_temp_root("start_ps1_log_capture");
    let bin_dir = temp_root.join("bin");
    let fake_tools_dir = temp_root.join("fake-tools");
    let target_debug_dir = temp_root.join("target").join("debug");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&fake_tools_dir).expect("temp fake-tools dir should be created");
    fs::create_dir_all(&target_debug_dir).expect("temp debug target dir should be created");

    for file_name in [
        "start-local.ps1",
        "install-local.ps1",
        "init-config-local.ps1",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        fake_tools_dir.join("cargo.cmd"),
        "@echo off\r\nexit /b 0\r\n",
    )
    .expect("fake cargo.cmd should be written");

    let whoami_path = windows_system_executable("whoami.exe");
    fs::copy(
        &whoami_path,
        target_debug_dir.join("local-minimal-node.exe"),
    )
    .unwrap_or_else(|_| {
        panic!(
            "failed to copy fake node binary from {}",
            whoami_path.display()
        )
    });

    let expected_stdout = Command::new(&whoami_path)
        .output()
        .expect("whoami.exe should run for expected stdout capture");
    assert!(
        expected_stdout.status.success(),
        "whoami.exe should succeed when collecting expected stdout"
    );
    let expected_stdout = String::from_utf8_lossy(&expected_stdout.stdout)
        .trim()
        .to_string();
    assert!(
        !expected_stdout.is_empty(),
        "whoami.exe output should not be empty"
    );

    let original_path =
        std::env::var_os("PATH").expect("PATH must be available to run lifecycle scripts");
    let temp_path = format!(
        "{};{}",
        fake_tools_dir.display(),
        PathBuf::from(original_path).display()
    );

    let status = Command::new("powershell")
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\start-local.ps1",
        ])
        .status()
        .expect("start-local.ps1 should execute against temp workspace");
    assert!(
        !status.success(),
        "fake binary should exit before readiness so the wrapper returns failure"
    );

    let stdout_log = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("logs")
        .join("local-minimal-node.out.log");
    let stderr_log = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("logs")
        .join("local-minimal-node.err.log");

    let stdout = fs::read_to_string(&stdout_log)
        .unwrap_or_else(|_| panic!("missing stdout log file: {}", stdout_log.display()));
    let stderr = fs::read_to_string(&stderr_log)
        .unwrap_or_else(|_| panic!("missing stderr log file: {}", stderr_log.display()));

    assert!(
        stdout.contains(&expected_stdout),
        "stdout log should capture child process stdout. expected fragment: {expected_stdout}, actual: {stdout}"
    );
    assert!(
        stderr.is_empty(),
        "stderr log should stay empty when fake process only writes stdout"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_start_local_ps1_captures_background_process_stderr_into_documented_log_file() {
    let root = workspace_root();
    let temp_root = unique_temp_root("start_ps1_stderr_capture");
    let bin_dir = temp_root.join("bin");
    let fake_tools_dir = temp_root.join("fake-tools");
    let target_debug_dir = temp_root.join("target").join("debug");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&fake_tools_dir).expect("temp fake-tools dir should be created");
    fs::create_dir_all(&target_debug_dir).expect("temp debug target dir should be created");

    for file_name in [
        "start-local.ps1",
        "install-local.ps1",
        "init-config-local.ps1",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        fake_tools_dir.join("cargo.cmd"),
        "@echo off\r\nexit /b 0\r\n",
    )
    .expect("fake cargo.cmd should be written");

    let expected_stderr_line = "synthetic stderr from local-minimal-node probe";
    let probe_source = temp_root.join("stderr-probe.rs");
    fs::write(
        &probe_source,
        format!("fn main() {{ eprintln!(\"{expected_stderr_line}\"); std::process::exit(1); }}\n"),
    )
    .expect("stderr probe source should be written");

    let rustc = std::env::var_os("RUSTC")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("rustc"));
    let rustc_status = Command::new(&rustc)
        .current_dir(&temp_root)
        .args([
            probe_source
                .to_str()
                .expect("probe source path should be valid"),
            "-o",
            target_debug_dir
                .join("local-minimal-node.exe")
                .to_str()
                .expect("probe exe path should be valid"),
        ])
        .status()
        .expect("rustc should compile the stderr probe executable");
    assert!(
        rustc_status.success(),
        "rustc should successfully compile the stderr probe executable"
    );

    let original_path =
        std::env::var_os("PATH").expect("PATH must be available to run lifecycle scripts");
    let temp_path = format!(
        "{};{}",
        fake_tools_dir.display(),
        PathBuf::from(original_path).display()
    );

    let status = Command::new("powershell")
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\start-local.ps1",
        ])
        .status()
        .expect("start-local.ps1 should execute against temp workspace");
    assert!(
        !status.success(),
        "fake binary should exit before readiness so the wrapper returns failure"
    );

    let stdout_log = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("logs")
        .join("local-minimal-node.out.log");
    let stderr_log = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("logs")
        .join("local-minimal-node.err.log");

    let stdout = fs::read_to_string(&stdout_log)
        .unwrap_or_else(|_| panic!("missing stdout log file: {}", stdout_log.display()));
    let stderr = fs::read_to_string(&stderr_log)
        .unwrap_or_else(|_| panic!("missing stderr log file: {}", stderr_log.display()));

    assert!(
        stdout.trim().is_empty(),
        "stdout log should stay empty when fake process only writes stderr. actual stdout: {stdout}"
    );
    assert!(
        stderr.contains(&expected_stderr_line),
        "stderr log should capture child process stderr. expected fragment: {expected_stderr_line}, actual: {stderr}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_start_local_ps1_stops_background_process_and_clears_pid_file_when_health_check_times_out() {
    let root = workspace_root();
    let temp_root = unique_temp_root("start_ps1_health_timeout_cleanup");
    let bin_dir = temp_root.join("bin");
    let fake_tools_dir = temp_root.join("fake-tools");
    let target_debug_dir = temp_root.join("target").join("debug");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&fake_tools_dir).expect("temp fake-tools dir should be created");
    fs::create_dir_all(&target_debug_dir).expect("temp debug target dir should be created");

    for file_name in [
        "start-local.ps1",
        "install-local.ps1",
        "init-config-local.ps1",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    let start_script_path = bin_dir.join("start-local.ps1");
    let start_script = fs::read_to_string(&start_script_path)
        .expect("copied start-local.ps1 should be readable for test acceleration");
    let accelerated_start_script = start_script
        .replacen(
            "$ErrorActionPreference = 'Stop'",
            "$ErrorActionPreference = 'Stop'\r\nfunction Invoke-WebRequest { throw 'synthetic health probe failure' }\r\n",
            1,
        )
        .replacen(
            "for ($attempt = 0; $attempt -lt 30; $attempt++) {",
            "for ($attempt = 0; $attempt -lt 2; $attempt++) {",
            1,
        )
        .replacen("Start-Sleep -Seconds 1", "Start-Sleep -Milliseconds 100", 1);
    assert_ne!(
        start_script, accelerated_start_script,
        "test acceleration should rewrite the copied start-local.ps1 readiness loop"
    );
    fs::write(&start_script_path, accelerated_start_script)
        .expect("accelerated start-local.ps1 should be written");

    fs::write(
        fake_tools_dir.join("cargo.cmd"),
        "@echo off\r\nexit /b 0\r\n",
    )
    .expect("fake cargo.cmd should be written");

    let probe_source = temp_root.join("health-timeout-probe.rs");
    fs::write(
        &probe_source,
        r#"
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    let runtime_dir = env::var("CRAW_CHAT_RUNTIME_DIR").expect("runtime dir env should be present");
    let marker = PathBuf::from(runtime_dir).join("state").join("health-timeout-probe.pid");
    if let Some(parent) = marker.parent() {
        fs::create_dir_all(parent).expect("marker parent dir should exist");
    }
    fs::write(&marker, process::id().to_string()).expect("marker should be written");
    thread::sleep(Duration::from_secs(300));
}
"#,
    )
    .expect("health-timeout probe source should be written");

    let rustc = std::env::var_os("RUSTC")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("rustc"));
    let rustc_status = Command::new(&rustc)
        .current_dir(&temp_root)
        .args([
            probe_source
                .to_str()
                .expect("probe source path should be valid"),
            "-o",
            target_debug_dir
                .join("local-minimal-node.exe")
                .to_str()
                .expect("probe exe path should be valid"),
        ])
        .status()
        .expect("rustc should compile the health-timeout probe executable");
    assert!(
        rustc_status.success(),
        "rustc should successfully compile the health-timeout probe executable"
    );

    let original_path =
        std::env::var_os("PATH").expect("PATH must be available to run lifecycle scripts");
    let temp_path = format!(
        "{};{}",
        fake_tools_dir.display(),
        PathBuf::from(original_path).display()
    );

    let status = Command::new("powershell")
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\start-local.ps1",
        ])
        .status()
        .expect("start-local.ps1 should execute against temp workspace");

    assert!(
        !status.success(),
        "health-timeout probe should cause the wrapper to fail startup"
    );

    let marker_file = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("state")
        .join("health-timeout-probe.pid");
    assert!(
        marker_file.exists(),
        "probe should record its pid before the wrapper returns"
    );

    let probe_pid = fs::read_to_string(&marker_file)
        .expect("probe pid marker should be readable")
        .trim()
        .parse::<u32>()
        .expect("probe pid marker should contain a numeric pid");
    let pid_file = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("pids")
        .join("local-minimal-node.pid");

    let probe_running_status = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            &format!(
                "if ($null -eq (Get-Process -Id {probe_pid} -ErrorAction SilentlyContinue)) {{ exit 0 }} else {{ exit 1 }}"
            ),
        ])
        .status()
        .expect("probe running-state query should execute");
    let probe_still_running = !probe_running_status.success();

    let _ = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            &format!(
                "$p = Get-Process -Id {probe_pid} -ErrorAction SilentlyContinue; if ($null -ne $p) {{ Stop-Process -Id {probe_pid} -Force -ErrorAction SilentlyContinue }}",
            ),
        ])
        .status();

    assert!(
        !probe_still_running,
        "start-local.ps1 must stop the launched local-minimal-node process when health readiness times out"
    );
    assert!(
        !pid_file.exists(),
        "start-local.ps1 must clear the pid file when startup fails after launching the process"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_start_local_sh_force_kills_background_process_and_clears_pid_file_when_health_check_times_out()
 {
    let root = workspace_root();
    let temp_root = unique_temp_root("start_sh_force_kill_cleanup");
    let bin_dir = temp_root.join("bin");
    let fake_tools_dir = temp_root.join("fake-tools");
    let target_debug_dir = temp_root.join("target").join("debug");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&fake_tools_dir).expect("temp fake-tools dir should be created");
    fs::create_dir_all(&target_debug_dir).expect("temp debug target dir should be created");

    for file_name in ["start-local.sh", "install-local.sh", "init-config-local.sh"] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    let start_script_path = bin_dir.join("start-local.sh");
    let start_script = fs::read_to_string(&start_script_path)
        .expect("copied start-local.sh should be readable for test acceleration");
    let accelerated_start_script = start_script
        .replacen("for _ in $(seq 1 30); do", "for _ in $(seq 1 2); do", 1)
        .replacen("for _ in $(seq 1 5); do", "for _ in $(seq 1 1); do", 1)
        .replace("sleep 1", "sleep 0.1");
    assert_ne!(
        start_script, accelerated_start_script,
        "test acceleration should rewrite the copied start-local.sh loops"
    );
    fs::write(&start_script_path, accelerated_start_script)
        .expect("accelerated start-local.sh should be written");

    fs::write(
        fake_tools_dir.join("cargo"),
        "#!/usr/bin/env bash\nexit 0\n",
    )
    .expect("fake cargo should be written");
    fs::write(fake_tools_dir.join("curl"), "#!/usr/bin/env bash\nexit 1\n")
        .expect("fake curl should be written");
    fs::write(fake_tools_dir.join("wget"), "#!/usr/bin/env bash\nexit 1\n")
        .expect("fake wget should be written");

    let Some(bash_path) = resolve_usable_bash() else {
        eprintln!(
            "skipping start-local.sh force-kill cleanup regression because no usable bash runtime is available"
        );
        let _ = fs::remove_dir_all(&temp_root);
        return;
    };

    for tool in ["cargo", "curl", "wget"] {
        let chmod_status = Command::new(&bash_path)
            .current_dir(&temp_root)
            .arg("-lc")
            .arg(format!(
                "chmod +x \"{}\"",
                fake_tools_dir.join(tool).display()
            ))
            .status()
            .expect("chmod should execute for fake shell tools");
        assert!(
            chmod_status.success(),
            "fake shell tool should become executable: {tool}"
        );
    }

    let probe_source = temp_root.join("health-timeout-force-kill-probe.rs");
    fs::write(
        &probe_source,
        r#"
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::thread;
use std::time::Duration;

#[cfg(unix)]
unsafe extern "C" {
    fn signal(signum: i32, handler: usize) -> usize;
}

#[cfg(unix)]
const SIGTERM: i32 = 15;
#[cfg(unix)]
const SIG_IGN: usize = 1usize;

fn main() {
    let runtime_dir = env::var("CRAW_CHAT_RUNTIME_DIR").expect("runtime dir env should be present");
    let marker = PathBuf::from(runtime_dir).join("state").join("health-timeout-force-kill-probe.pid");
    if let Some(parent) = marker.parent() {
        fs::create_dir_all(parent).expect("marker parent dir should exist");
    }
    #[cfg(unix)]
    unsafe {
        signal(SIGTERM, SIG_IGN);
    }
    fs::write(&marker, process::id().to_string()).expect("marker should be written");
    thread::sleep(Duration::from_secs(300));
}
"#,
    )
    .expect("health-timeout force-kill probe source should be written");

    let rustc = std::env::var_os("RUSTC")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("rustc"));
    let rustc_status = Command::new(&rustc)
        .current_dir(&temp_root)
        .args([
            probe_source
                .to_str()
                .expect("probe source path should be valid"),
            "-o",
            target_debug_dir
                .join("local-minimal-node")
                .to_str()
                .expect("probe path should be valid"),
        ])
        .status()
        .expect("rustc should compile the shell force-kill probe executable");
    assert!(
        rustc_status.success(),
        "rustc should successfully compile the shell force-kill probe executable"
    );

    let chmod_probe_status = Command::new(&bash_path)
        .current_dir(&temp_root)
        .arg("-lc")
        .arg(format!(
            "chmod +x \"{}\"",
            target_debug_dir.join("local-minimal-node").display()
        ))
        .status()
        .expect("chmod should execute for the shell probe binary");
    assert!(
        chmod_probe_status.success(),
        "compiled shell probe should become executable"
    );

    let original_path =
        std::env::var_os("PATH").expect("PATH must be available to run lifecycle scripts");
    let temp_path = format!(
        "{}:{}",
        fake_tools_dir.display(),
        PathBuf::from(original_path).display()
    );

    let output = Command::new(&bash_path)
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .arg("bin/start-local.sh")
        .output()
        .expect("start-local.sh should execute against temp workspace");
    assert!(
        !output.status.success(),
        "health-timeout shell probe should cause the wrapper to fail startup"
    );

    let marker_file = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("state")
        .join("health-timeout-force-kill-probe.pid");
    assert!(
        marker_file.exists(),
        "shell probe should record its pid before the wrapper returns"
    );

    let probe_pid = fs::read_to_string(&marker_file)
        .expect("probe pid marker should be readable")
        .trim()
        .parse::<u32>()
        .expect("probe pid marker should contain a numeric pid");
    let pid_file = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("pids")
        .join("local-minimal-node.pid");

    let probe_running_status = Command::new(&bash_path)
        .current_dir(&temp_root)
        .arg("-lc")
        .arg(format!("kill -0 {probe_pid}"))
        .status()
        .expect("probe running-state query should execute");
    let probe_still_running = probe_running_status.success();

    let _ = Command::new(&bash_path)
        .current_dir(&temp_root)
        .arg("-lc")
        .arg(format!("kill -9 {probe_pid} >/dev/null 2>&1 || true"))
        .status();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("local-minimal-node did not become healthy within 30 seconds"),
        "start-local.sh should surface the health-timeout failure. actual stderr: {stderr}"
    );
    assert!(
        !probe_still_running,
        "start-local.sh must force-kill the launched process when it ignores SIGTERM during startup rollback"
    );
    assert!(
        !pid_file.exists(),
        "start-local.sh must clear the pid file once startup rollback finishes"
    );

    let _ = fs::remove_dir_all(&temp_root);
}
