# local-minimal shell startup force-kill cleanup implementation plan

## 1. Objective

Ensure `start-local.sh` does not leave a managed `local-minimal-node` process alive after startup rollback when the child process ignores `SIGTERM`.

## 2. Problem statement

The shell startup rollback helper previously:

1. sent `SIGTERM`
2. waited a short period
3. removed the pid file

If the launched process ignored `SIGTERM`, the helper could delete the pid file while the process was still alive.

That creates a more dangerous state than a plain timeout:

- the managed process still runs
- the lifecycle metadata says it does not
- later `start` or `status` commands lose accurate ownership information

## 3. Scope

- Modify:
  - `bin/start-local.sh`
- Extend deployment and lifecycle regressions in:
  - `services/local-minimal-node/tests/deployment_profile_test.rs`
- Record the review result and operator standard in:
  - `/docs/review/`
  - `/docs/架构/`

## 4. Planned implementation

1. Add a failing deployment asset contract requiring a `kill -9 "$pid"` fallback in `start-local.sh`.
2. Add a shell behavior regression that launches a fake `local-minimal-node` which ignores `SIGTERM` and only dies to `SIGKILL`.
3. Upgrade `stop_managed_process_and_remove_pid_file` to:
   - try `SIGTERM`
   - wait briefly
   - escalate to `SIGKILL`
   - return failure and preserve the pid file if the process is still alive even after escalation
4. Keep startup timeout reporting and rollback semantics intact.

## 5. Verification plan

- `cargo test -p local-minimal-node --offline test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints -- --nocapture`
- `cargo test -p local-minimal-node --offline test_start_local_sh_force_kills_background_process_and_clears_pid_file_when_health_check_times_out -- --nocapture`
- `cargo fmt --all`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo test -p local-minimal-node --offline`
- Windows sequential smoke:
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin\stop-local.ps1`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin\restart-local.ps1`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin\status-local.ps1`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin\stop-local.ps1`

## 6. Exit criteria

- shell startup rollback can escalate from `SIGTERM` to `SIGKILL`
- startup rollback does not delete pid metadata if the managed process cannot be removed
- deployment asset assertions freeze the stronger rollback contract
- deployment profile tests and full package verification remain green
