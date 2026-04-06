# local-minimal Windows restart stop exit failure propagation review cycle

## 1. Review scope

- [restart-local.ps1](D:/javasource/spring-ai-plus/spring-ai-plus-business/apps/craw-chat/bin/restart-local.ps1)
- [deployment_profile_test.rs](D:/javasource/spring-ai-plus/spring-ai-plus-business/apps/craw-chat/services/local-minimal-node/tests/deployment_profile_test.rs)

## 2. Finding

### 2.1 Real production bug: non-zero stop exit was swallowed by restart-local.ps1

`restart-local.ps1` invoked:

```powershell
& $stopScript | Out-Host
```

but never checked `$LASTEXITCODE` afterward.

Observed runtime evidence in the temp reproduction:

- `stop-local.ps1` printed `stub stop`
- `stop-local.ps1` executed `exit 9`
- `restart-local.ps1` still invoked `start-local.ps1`
- marker file confirmed start execution
- overall restart exit code became `0`

Impact:

- restart can report success while stop actually failed
- a second instance can be launched against an uncertain prior process state
- operator diagnostics become misleading

## 3. Root cause

PowerShell distinguishes between:

- terminating script failures such as `throw`
- non-zero script exits surfaced through `$LASTEXITCODE`

The original script handled the first case implicitly but ignored the second one completely.

## 4. Fix

After invoking `stop-local.ps1`, capture the stop exit code and exit immediately when it is non-zero:

- preserve the stop failure
- prevent `start-local.ps1` execution
- preserve the operator-visible exit code

## 5. Regression coverage added

- `test_restart_local_ps1_propagates_terminating_stop_failure_before_starting_new_instance`
- `test_restart_local_ps1_propagates_non_zero_stop_exit_before_starting_new_instance`

The deployment asset contract also now asserts the presence of the stop-exit guard in the script text.

## 6. Verification evidence

Targeted regressions:

- non-zero stop exit regression: passed after fix
- terminating stop failure regression: passed
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
- status reported `running` with `health status: ok`
- stop succeeded and removed the process cleanly

Environment note:

- when background lifecycle commands are split across separate tool invocations in this runner, the observed process lifetime is not authoritative
- the reliable smoke evidence for this wave is the single-shell sequential lifecycle run above

## 7. Next actions

1. Execute a real Windows lifecycle smoke after the package tests.
2. Continue operator review on `status-local.ps1` and `stop-local.ps1` for similar PowerShell-specific failure-shape gaps.
3. Keep Linux lifecycle behavior verification on the backlog until a usable Bash runtime exists on at least one execution lane.
