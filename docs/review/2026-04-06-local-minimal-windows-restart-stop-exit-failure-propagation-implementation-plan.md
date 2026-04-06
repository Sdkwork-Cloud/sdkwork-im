# local-minimal Windows restart stop exit failure propagation implementation plan

## 1. Objective

Harden `bin/restart-local.ps1` so it treats a non-zero `stop-local.ps1` exit as a real restart failure and never starts a new instance after that failure.

## 2. Problem statement

The Windows restart script already stopped on terminating `throw` failures from `stop-local.ps1`, but it did not stop on non-zero script exits such as `exit 9`.

Observed bad behavior:

- `stop-local.ps1` printed output and exited non-zero
- `restart-local.ps1` still invoked `start-local.ps1`
- overall process exited with code `0`

That breaks the operator contract because restart can silently mask stop failures and launch a new instance against an unknown prior state.

## 3. Scope

- Modify only `bin/restart-local.ps1`
- Add regression coverage in `services/local-minimal-node/tests/deployment_profile_test.rs`
- Freeze the standard in `/docs/review/` and `/docs/架构/`

## 4. Planned implementation

1. Add a Windows behavior regression for terminating stop failures.
2. Add a Windows behavior regression for non-zero stop exits.
3. Run the new non-zero exit regression and confirm it fails against the current script.
4. Apply the smallest production fix:
   - invoke `stop-local.ps1`
   - read `$LASTEXITCODE`
   - exit immediately when it is non-zero
5. Freeze the script text contract with deployment asset assertions.
6. Re-run the targeted regressions and then the broader package tests.

## 5. Verification plan

- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_restart_local_ps1_propagates_non_zero_stop_exit_before_starting_new_instance -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_restart_local_ps1_propagates_terminating_stop_failure_before_starting_new_instance -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo test -p local-minimal-node --offline`
- real Windows lifecycle smoke:
  - `powershell -ExecutionPolicy Bypass -File bin\\stop-local.ps1`
  - `powershell -ExecutionPolicy Bypass -File bin\\restart-local.ps1`
  - `powershell -ExecutionPolicy Bypass -File bin\\status-local.ps1`
  - `powershell -ExecutionPolicy Bypass -File bin\\stop-local.ps1`

## 6. Exit criteria

- non-zero stop exit now aborts restart
- start script is not invoked after stop exit failure
- restart preserves the stop exit code
- terminating stop failures still abort restart
- broader package tests remain green
