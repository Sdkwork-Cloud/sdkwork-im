# local-minimal Windows pid identity guard review cycle

## 1. Review scope

- [start-local.ps1](<workspace-root>/sdkwork-im/bin/start-local.ps1)
- [status-local.ps1](<workspace-root>/sdkwork-im/bin/status-local.ps1)
- [stop-local.ps1](<workspace-root>/sdkwork-im/bin/stop-local.ps1)
- [deployment_profile_test.rs](<workspace-root>/sdkwork-im/services/local-minimal-node/tests/deployment_profile_test.rs)

## 2. Findings

### 2.1 Real production bug: stop-local.ps1 could kill an unrelated process

Red regression:

- stale pid file pointed to a sleeping `powershell` process
- `stop-local.ps1` treated that process as managed
- the unrelated process was terminated

This is a direct operator-safety defect.

### 2.2 status-local.ps1 could report running for an unrelated process

Red regression:

- stale pid file pointed to a sleeping `powershell` process
- `status-local.ps1` reported the lifecycle state based only on pid existence
- pid file remained in place

This produces false runtime diagnostics.

### 2.3 start-local.ps1 could block startup because of stale unmanaged pid metadata

Red regression:

- stale pid file pointed to a sleeping `powershell` process
- `start-local.ps1` treated that pid as an already-running managed instance
- startup was blocked for the wrong reason

## 3. Root cause

All three PowerShell lifecycle scripts shared the same weakness:

- read pid file
- resolve live process by pid
- assume that any live process at that pid is `local-minimal-node`

They validated process existence, but not process identity.

## 4. Fix

The shared `Get-RunningProcessFromPidFile` logic in each PowerShell script now:

1. reads the pid
2. resolves the live process
3. verifies `ProcessName -ieq "local-minimal-node"`
4. deletes the pid file and returns `$null` when the process identity does not match

This converts stale pid collisions into safe, self-healing metadata cleanup.

## 5. Regression coverage added

- `test_stop_local_ps1_does_not_kill_unmanaged_process_from_stale_pid_file`
- `test_status_local_ps1_treats_unmanaged_process_from_stale_pid_file_as_stopped`
- `test_start_local_ps1_ignores_unmanaged_process_from_stale_pid_file`

The deployment asset contract also now asserts the presence of the process identity guard in the three PowerShell lifecycle scripts.

## 6. Verification evidence

Targeted regressions:

- stop unmanaged-pid regression: passed
- status unmanaged-pid regression: passed
- start unmanaged-pid regression: passed
- deployment asset contract: passed

Broader verification:

- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo test -p local-minimal-node --offline`
- sequential Windows lifecycle smoke in one shell session:
  - `powershell -ExecutionPolicy Bypass -File bin\stop-local.ps1`
  - `powershell -ExecutionPolicy Bypass -File bin\restart-local.ps1`
  - `Start-Sleep -Seconds 2`
  - `powershell -ExecutionPolicy Bypass -File bin\status-local.ps1`
  - `powershell -ExecutionPolicy Bypass -File bin\stop-local.ps1`

Smoke result:

- restart succeeded
- status reported `running`
- `health status: ok`
- stop succeeded and removed the managed process cleanly

## 7. Current stage

Windows lifecycle safety is stronger in two important ways now:

- stale pid metadata no longer maps arbitrary live processes into managed local service state
- start, status, and stop now self-heal stale unmanaged pid files

## 8. Next actions

1. Mirror the same pid identity guard into `start-local.sh`, `status-local.sh`, and `stop-local.sh` with shell-capable verification.
2. Continue lifecycle review on path-level identity, not only process name, if a stronger cross-platform representation is needed.
3. Keep extending operator regressions before broadening feature work.
