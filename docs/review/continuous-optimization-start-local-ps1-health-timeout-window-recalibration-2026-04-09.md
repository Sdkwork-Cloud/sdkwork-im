# Continuous Optimization: start-local.ps1 Health Timeout Window Recalibration

## Context

- Follow-up to the earlier Step 10 stability pass for `test_start_local_ps1_stops_background_process_and_clears_pid_file_when_health_check_times_out`.
- The exact Windows test failed again with `probe should record its pid before the wrapper returns`.

## Confirmed Bug

- The copied `start-local.ps1` inside the test still compressed the synthetic health loop to `5 x 100ms`.
- On this Windows host, `Start-Process` plus redirected-log startup could still consume enough cold-start time that the probe was force-stopped before it wrote `.runtime/local-minimal/state/health-timeout-probe.pid`.
- The failure was in the regression test window, not in the production cleanup contract.

## Evidence

- Red:
  - `cargo test -p local-minimal-node --offline --test deployment_profile_test test_start_local_ps1_stops_background_process_and_clears_pid_file_when_health_check_times_out -- --exact --nocapture`
- Residue after the failing run:
  - marker file absent
  - pid file already cleared
  - launching the same probe binary directly wrote the marker, confirming the probe itself was valid

## Fix

- Recalibrate only the PowerShell test copy of `start-local.ps1`.
- Expand the synthetic readiness window from `5` to `20` attempts while keeping `100ms` sleeps.
- Keep production `bin/start-local.ps1` unchanged.
- Keep the same cleanup assertions: startup must fail, the child must be stopped, and the pid file must be removed.

## Verification

- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_start_local_ps1_stops_background_process_and_clears_pid_file_when_health_check_times_out -- --exact --nocapture`
- Repeat the exact test 5 times: all green
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo fmt --all --check`
- `cargo test -p local-minimal-node --offline -- --nocapture`

## Next

- Continue the review loop from the next concrete docs/runtime mismatch.
- Candidate next pass: remaining Windows runtime-ops `.cmd --help` surfaces that still lack direct proof in tests.
