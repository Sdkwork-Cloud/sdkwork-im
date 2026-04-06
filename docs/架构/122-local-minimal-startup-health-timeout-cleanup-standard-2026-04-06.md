# 122. local-minimal startup health-timeout cleanup standard

## 1. Standard

If a lifecycle startup wrapper launches `local-minimal-node` in background mode and readiness later fails, that startup attempt is considered unsuccessful and must be fully rolled back.

`start-local.ps1` and `start-local.sh` must therefore:

- stop the launched managed process
- remove the managed pid file
- return a non-zero exit status

## 2. Why this matters

Background launch followed by readiness validation is a two-phase operator action.

If phase 2 fails but phase 1 is left running, operators and automation receive contradictory signals:

- the command says startup failed
- the process table says something is still running
- the pid file suggests managed ownership still exists

That breaks restart safety, status accuracy, and later remediation steps.

## 3. Minimum rollback rule

The minimum accepted rollback behavior after readiness failure is:

- no live managed `local-minimal-node` process remains from the failed startup attempt
- no `local-minimal-node.pid` remains from the failed startup attempt

## 4. Identity safety rule

Startup cleanup must preserve the existing pid identity guard:

- only a managed `local-minimal-node` process may be stopped
- unrelated processes must never be killed just because they reused a pid

## 5. Regression requirement

The repository must keep:

- a Windows behavior regression proving that health-timeout startup cleanup stops the launched process and clears the pid file
- deployment asset contract assertions that freeze the cleanup helper presence in both PowerShell and shell startup scripts

## 6. Cross-platform note

The Windows lifecycle path is behavior-verified on this runner.

The Unix shell lifecycle path has implementation and contract parity, but direct behavior replay still requires a usable bash-capable environment.
