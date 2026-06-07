# local-minimal shell startup force-kill cleanup review cycle

## 1. Review scope

- [start-local.sh](<workspace-root>/craw-chat/bin/start-local.sh)
- [deployment_profile_test.rs](<workspace-root>/craw-chat/services/local-minimal-node/tests/deployment_profile_test.rs)

## 2. Findings

### 2.1 Real operator correctness gap: shell rollback could remove pid metadata before the process was actually gone

Code review showed that `stop_managed_process_and_remove_pid_file` in `start-local.sh` only attempted a soft stop.

Old behavior:

- send `SIGTERM`
- wait briefly
- remove pid file unconditionally

If the process ignored `SIGTERM`, the wrapper could leave a live process behind with no pid file.

### 2.2 This is worse than a normal failed startup

After a plain startup failure, operators at least expect either:

- the process is gone, or
- the pid file still reflects a managed failure state

The old shell rollback could produce the most misleading combination:

- process alive
- pid metadata gone

That breaks lifecycle observability.

## 3. Root cause

The shell startup rollback logic treated process termination as best effort, but pid-file removal as unconditional.

So cleanup was not transactionally bound to the real process state.

## 4. Fix

`start-local.sh` now hardens `stop_managed_process_and_remove_pid_file`:

1. attempt `SIGTERM`
2. wait
3. if still running, send `SIGKILL`
4. wait again
5. if still running after escalation, return failure and do not remove the pid file
6. only remove the pid file after the managed process is confirmed gone

The timeout caller now also surfaces a dedicated rollback-failure error if the process remains alive after escalation.

## 5. Regression coverage added

Deployment asset contract:

- `bin/start-local.sh` must contain `kill -9 "$pid"`
- `bin/start-local.sh` must contain a helper failure `return 1`

Behavior regression:

- `test_start_local_sh_force_kills_background_process_and_clears_pid_file_when_health_check_times_out`

That regression compiles a fake `local-minimal-node` which ignores `SIGTERM` and verifies the startup wrapper performs a hard kill before finishing rollback.

## 6. Verification evidence

Targeted verification:

- `cargo test -p local-minimal-node --offline test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints -- --nocapture`: passed
- `cargo test -p local-minimal-node --offline test_start_local_sh_force_kills_background_process_and_clears_pid_file_when_health_check_times_out -- --nocapture`: passed on the current runner as an environment-aware skip because no usable bash runtime is available

Broader verification:

- `cargo fmt --all`: passed
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`: passed, `15 passed; 0 failed`
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

Shell behavior verification limitation on this machine:

- `C:\Program Files\Git\bin\bash.exe --version`: fails with `couldn't create signal pipe, Win32 error 5`
- `C:\Program Files\Git\usr\bin\bash.exe --version`: fails with the same error
- `bash --version`: fails

So the new shell behavior regression is implemented and will execute on a bash-capable host, but it can only skip on this runner.

## 7. Current stage

Local lifecycle rollback semantics are now more coherent:

- Windows startup rollback already force-cleans managed failures
- Unix shell startup rollback now has the same escalation intent and stronger pid-file discipline

## 8. Next actions

1. Run the new shell force-kill regression on a real Linux/macOS or usable Git Bash environment.
2. Continue lifecycle review for any remaining script path where rollback removes metadata before proving the process is gone.
3. Keep hardening lifecycle tooling as transactional operator flows, not as best-effort glue.
