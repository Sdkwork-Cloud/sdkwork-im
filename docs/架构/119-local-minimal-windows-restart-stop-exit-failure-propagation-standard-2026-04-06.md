# 119. local-minimal Windows restart stop exit failure propagation standard

## 1. Standard

For `bin/restart-local.ps1`, Windows restart semantics are:

- if `stop-local.ps1` throws, restart must fail immediately
- if `stop-local.ps1` exits non-zero, restart must fail immediately
- in either failure shape, `start-local.ps1` must not run
- when failure comes from a non-zero stop exit, restart must preserve that exit code

## 2. Why this matters

Restart is an operator recovery action. If restart can continue after stop failure, the system can:

- hide a real lifecycle fault
- launch a new process while the prior one is still in an unknown state
- produce false-positive automation results
- make incident debugging materially harder

## 3. PowerShell-specific rule

PowerShell restart wrappers must guard both failure channels:

1. terminating script failures
2. non-zero script exits surfaced through `$LASTEXITCODE`

Handling only `throw` is insufficient.

## 4. Regression requirement

The repository must keep behavior regressions for:

- terminating stop failure propagation
- non-zero stop exit propagation

These regressions must verify:

- restart exits non-zero
- start is not invoked
- stop failure details remain observable

## 5. Text contract requirement

`deployment_profile_test.rs` must assert that `restart-local.ps1` contains an explicit stop-exit guard, so future edits cannot quietly remove it while leaving behavior tests to catch it later.
