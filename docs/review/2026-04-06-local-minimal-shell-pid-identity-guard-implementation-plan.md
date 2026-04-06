# local-minimal shell pid identity guard implementation plan

## 1. Objective

Prevent Unix lifecycle scripts from treating an unrelated process as `local-minimal-node` when a stale pid file points to a reused live pid.

## 2. Problem statement

The shell lifecycle scripts previously validated only pid liveness:

- `start-local.sh` could reject startup because an unrelated process occupied a stale pid
- `status-local.sh` could report `running` for an unrelated process
- `stop-local.sh` could send a signal to an unrelated process

This is the same operator-safety class of defect that was already fixed on Windows.

## 3. Scope

- Modify:
  - `bin/start-local.sh`
  - `bin/status-local.sh`
  - `bin/stop-local.sh`
- Extend deployment asset contract assertions in:
  - `services/local-minimal-node/tests/deployment_profile_test.rs`
- Record the review result and cross-platform operator standard in:
  - `/docs/review/`
  - `/docs/架构/`

## 4. Planned implementation

1. Add a failing deployment asset contract asserting that the three shell scripts define an expected process identity guard, not only `kill -0`.
2. Introduce a shared shell pattern in each lifecycle script:
   - `EXPECTED_PROCESS_NAME="local-minimal-node"`
   - `pid_matches_expected_process`
   - `get_running_pid_from_pid_file`
3. Make stale or unmanaged pid-file targets self-heal by deleting the pid file and treating the service as not running.
4. Reuse the same identity guard in readiness and stop wait loops so pid reuse does not extend false running state.
5. Verify the full `local-minimal-node` package still passes and the Windows lifecycle chain still starts, reports healthy, and stops cleanly.

## 5. Verification plan

- `cargo test -p local-minimal-node --offline test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo fmt --all`
- `cargo test -p local-minimal-node --offline`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin\stop-local.ps1`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin\restart-local.ps1`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin\status-local.ps1`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin\stop-local.ps1`

## 6. Exit criteria

- shell lifecycle scripts never trust pid existence alone
- unmanaged live pids are normalized into stale metadata and removed
- deployment asset assertions freeze the shell identity guard contract
- package tests remain green
- local Windows smoke still starts, reports `health status: ok`, and stops cleanly
