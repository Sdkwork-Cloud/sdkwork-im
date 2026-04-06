# local-minimal Windows pid identity guard implementation plan

## 1. Objective

Prevent Windows lifecycle scripts from treating an unrelated process as `local-minimal-node` only because a stale pid file points at an occupied pid.

## 2. Problem statement

The PowerShell lifecycle scripts previously trusted pid-file identity too early:

- `start-local.ps1` could refuse startup because an unrelated process occupied the pid from a stale pid file
- `status-local.ps1` could report `running` for an unrelated process
- `stop-local.ps1` could kill an unrelated process

This is a serious operator safety issue because pid reuse or stale pid metadata can cross process boundaries.

## 3. Scope

- Modify:
  - `bin/start-local.ps1`
  - `bin/status-local.ps1`
  - `bin/stop-local.ps1`
- Add Windows behavior regressions in:
  - `services/local-minimal-node/tests/deployment_profile_test.rs`
- Record the standard in `/docs/review/` and `/docs/架构/`

## 4. Planned implementation

1. Add a failing behavior regression proving `stop-local.ps1` must not kill an unrelated `powershell` process referenced by a stale pid file.
2. Add a failing behavior regression proving `status-local.ps1` must treat the same situation as `stopped`.
3. Add a failing behavior regression proving `start-local.ps1` must ignore stale pid metadata that points to an unrelated process.
4. Apply the smallest PowerShell fix:
   - after resolving the pid to a live process
   - verify `ProcessName` is `local-minimal-node`
   - if not, delete the pid file and treat it as stale metadata
5. Freeze the script text contract in the deployment asset assertions.

## 5. Verification plan

- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_stop_local_ps1_does_not_kill_unmanaged_process_from_stale_pid_file -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_status_local_ps1_treats_unmanaged_process_from_stale_pid_file_as_stopped -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_start_local_ps1_ignores_unmanaged_process_from_stale_pid_file -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo test -p local-minimal-node --offline`

## 6. Exit criteria

- unrelated processes are never treated as managed `local-minimal-node` instances
- stale pid files are removed automatically
- Windows lifecycle behavior regressions cover start, status, and stop for this failure shape
- package verification remains green
