# local-minimal startup health-timeout cleanup review cycle

## 1. Review scope

- [start-local.ps1](<workspace-root>/sdkwork-im/bin/start-local.ps1)
- [start-local.sh](<workspace-root>/sdkwork-im/bin/start-local.sh)
- [deployment_profile_test.rs](<workspace-root>/sdkwork-im/services/local-minimal-node/tests/deployment_profile_test.rs)

## 2. Findings

### 2.1 Real production bug: startup timeout could leave an orphaned managed process

Red regression:

- a long-lived fake `local-minimal-node.exe` was launched successfully
- readiness never became healthy
- `start-local.ps1` returned failure
- the managed child process continued running in the background

This means command failure and runtime state diverged.

### 2.2 Startup timeout could also leave stale managed pid metadata

Red regression:

- the same health-timeout path left `local-minimal-node.pid` in place
- operator tooling could then interpret the failed startup as an already running managed instance

That contaminates later `start`, `status`, and `stop` decisions.

## 3. Root cause

The lifecycle startup wrappers handled launch and readiness as two separate phases, but they did not bind failure cleanup to the second phase.

The old sequence was:

1. launch `local-minimal-node`
2. write pid file
3. wait for readiness
4. throw on timeout

There was no compensating cleanup for the already-launched process.

## 4. Fix

`start-local.ps1` now includes `Stop-ManagedProcessAndRemovePidFile` and uses it in the background startup failure path.

That cleanup:

- validates the process identity as `local-minimal-node`
- force-stops the launched process when still managed
- waits briefly for exit
- removes the pid file regardless of whether the process already exited

`start-local.sh` now mirrors the same operator intent through `stop_managed_process_and_remove_pid_file`.

## 5. Regression coverage added

Behavior regression:

- `test_start_local_ps1_stops_background_process_and_clears_pid_file_when_health_check_times_out`

Deployment asset contract additions:

- `bin/start-local.ps1` must contain `Stop-ManagedProcessAndRemovePidFile`
- `bin/start-local.sh` must contain `stop_managed_process_and_remove_pid_file`

## 6. Verification evidence

Targeted verification:

- `cargo test -p local-minimal-node --offline test_start_local_ps1_stops_background_process_and_clears_pid_file_when_health_check_times_out -- --nocapture`: passed
- `cargo test -p local-minimal-node --offline test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints -- --nocapture`: passed

Broader verification:

- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`: passed, `14 passed; 0 failed`
- `cargo fmt --all`: passed
- `cargo test -p local-minimal-node --offline`: passed

Fresh runtime evidence:

- Windows sequential smoke passed:
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin\stop-local.ps1`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin\restart-local.ps1`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin\status-local.ps1`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin\stop-local.ps1`
- result:
  - service started on `http://127.0.0.1:18124`
  - `status: running`
  - `health status: ok`
  - stop succeeded cleanly

Shell verification note:

- direct `.sh` behavior replay is still blocked on this runner because no usable bash runtime is available
- the shell-side cleanup change is covered here by source contract and implementation parity

## 7. Current stage

The local lifecycle startup path is now stricter in an important way:

- a failed readiness wait is treated as a failed launch, not as a partly successful background start
- managed process state and pid-file state are cleaned up together

## 8. Next actions

1. Add Unix behavior regressions for startup-timeout cleanup in a bash-capable environment.
2. Continue lifecycle review around restart sequencing and failure compensation so all command boundaries remain transactional from an operator perspective.
3. Keep reviewing deployment scripts for any remaining partial-failure state leaks.
