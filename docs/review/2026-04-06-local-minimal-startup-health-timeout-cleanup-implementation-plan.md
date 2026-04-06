# local-minimal startup health-timeout cleanup implementation plan

## 1. Objective

Ensure lifecycle startup wrappers never leave a managed `local-minimal-node` process or pid file behind when background startup fails after launch because readiness never becomes healthy.

## 2. Problem statement

`start-local.ps1` and `start-local.sh` previously launched the process, wrote the pid file, waited for health, and then simply returned failure on timeout.

That meant a failed startup could still leave:

- a live `local-minimal-node` process running in the background
- a pid file that incorrectly suggested startup ownership still succeeded

This is an operator correctness defect because a failed command must not leave partially managed runtime state behind.

## 3. Scope

- Modify:
  - `bin/start-local.ps1`
  - `bin/start-local.sh`
- Add a Windows behavior regression in:
  - `services/local-minimal-node/tests/deployment_profile_test.rs`
- Freeze the cleanup contract in deployment asset assertions
- Record the review and standard in:
  - `/docs/review/`
  - `/docs/架构/`

## 4. Planned implementation

1. Add a failing Windows regression that launches a long-lived fake `local-minimal-node.exe` that never becomes healthy.
2. Prove that the current script exits with failure but leaves the child process and pid file behind.
3. Introduce explicit startup-failure cleanup in `start-local.ps1`:
   - stop the launched managed process
   - wait briefly for exit
   - always remove the pid file
4. Mirror the same startup cleanup semantics in `start-local.sh`.
5. Freeze the new cleanup helper presence in the deployment asset contract.

## 5. Verification plan

- `cargo test -p local-minimal-node --offline test_start_local_ps1_stops_background_process_and_clears_pid_file_when_health_check_times_out -- --nocapture`
- `cargo test -p local-minimal-node --offline test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo fmt --all`
- `cargo test -p local-minimal-node --offline`
- Windows sequential smoke:
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin\stop-local.ps1`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin\restart-local.ps1`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin\status-local.ps1`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin\stop-local.ps1`

## 6. Exit criteria

- startup timeout does not leave a live managed process behind
- startup timeout does not leave a pid file behind
- deployment asset assertions freeze the startup cleanup contract
- deployment profile tests remain green
- Windows smoke still starts, reports `health status: ok`, and stops cleanly
