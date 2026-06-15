# local-minimal shell pid identity guard review cycle

## 1. Review scope

- [start-local.sh](<workspace-root>/sdkwork-im/bin/start-local.sh)
- [status-local.sh](<workspace-root>/sdkwork-im/bin/status-local.sh)
- [stop-local.sh](<workspace-root>/sdkwork-im/bin/stop-local.sh)
- [deployment_profile_test.rs](<workspace-root>/sdkwork-im/services/local-minimal-node/tests/deployment_profile_test.rs)

## 2. Findings

### 2.1 Real production bug: shell scripts trusted pid liveness without process identity

The Unix lifecycle scripts only relied on `kill -0`:

- `start-local.sh` blocked startup if any process occupied the pid from a stale pid file
- `status-local.sh` treated any live pid as a running managed service
- `stop-local.sh` would signal any live process found in the pid file

This is an operator-safety bug because pid reuse is a normal long-running host behavior.

### 2.2 Additional shell-specific risk: `set -euo pipefail` can turn `ps` races into hard script exits

After introducing process-name inspection, a second review pass found a shell failure mode:

- the scripts run with `set -euo pipefail`
- a short race between `kill -0` and `ps` can make `ps` exit non-zero
- without protection, command substitution would abort the whole script instead of treating the pid as unmanaged

That would convert a recoverable stale-pid edge into a lifecycle script crash.

## 3. Root cause

The shell lifecycle scripts lacked a managed-process identity boundary:

1. read pid file
2. check liveness with `kill -0`
3. assume the live process at that pid is `local-minimal-node`

They validated existence, but not ownership of the pid.

## 4. Fix

All three shell lifecycle scripts now apply the same safe pattern:

1. define `EXPECTED_PROCESS_NAME="local-minimal-node"`
2. validate a live pid with `pid_matches_expected_process`
3. inspect `ps -p "$pid" -o comm=`
4. remove the pid file and treat it as stale when the identity does not match
5. neutralize `ps` race failures with `|| true` so the scripts degrade to safe cleanup instead of aborting

The `start-local.sh` readiness loop and `stop-local.sh` wait loop now also use the identity guard, not raw pid liveness.

## 5. Regression coverage added

The deployment asset contract now asserts the shell safety boundary directly:

- `EXPECTED_PROCESS_NAME="local-minimal-node"`
- `pid_matches_expected_process`
- `ps -p "$pid" -o comm=`

This keeps Unix lifecycle parity with the already-hardened Windows scripts.

## 6. Verification evidence

Targeted red-green proof:

- added shell contract assertions first
- `cargo test -p local-minimal-node --offline test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints -- --nocapture` failed on missing shell identity guard
- after the shell script changes, the same command passed

Broader verification:

- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`: passed, `13 passed; 0 failed`
- `cargo fmt --all`: passed
- `cargo test -p local-minimal-node --offline`: passed

Fresh runtime evidence:

- sequential Windows smoke passed:
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin\stop-local.ps1`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin\restart-local.ps1`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin\status-local.ps1`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin\stop-local.ps1`
- result:
  - service started on `http://127.0.0.1:18124`
  - `status: running`
  - `health status: ok`
  - stop succeeded cleanly

Shell behavior verification status on this runner:

- `C:\Program Files\Git\bin\bash.exe --version`: failed with `couldn't create signal pipe, Win32 error 5`
- `C:\Program Files\Git\usr\bin\bash.exe --version`: failed with the same error
- `bash --version`: failed

So shell runtime behavior tests remain environment-blocked on this machine. Code and contract parity are complete; direct shell behavior replay still requires a usable bash environment.

## 7. Current stage

Lifecycle operator hardening is now stronger across both script families:

- Windows lifecycle scripts already enforce managed-process identity and restart safety
- Unix lifecycle scripts now enforce the same pid identity rule at the contract and implementation level
- local Windows runtime smoke still succeeds after the shell-side hardening

## 8. Next actions

1. Add shell behavior regressions mirroring the Windows stale-pid tests when a usable bash runtime is available.
2. Continue lifecycle review on stronger identity representations if future operator requirements need path-level validation, not only process-name validation.
3. Keep review-driven hardening before expanding broader deployment automation.
