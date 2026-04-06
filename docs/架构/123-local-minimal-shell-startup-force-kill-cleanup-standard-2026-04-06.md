# 123. local-minimal shell startup force-kill cleanup standard

## 1. Standard

For shell startup rollback, `SIGTERM` is not sufficient as the terminal safety boundary.

If a managed `local-minimal-node` process remains alive after soft termination during startup rollback, `start-local.sh` must escalate to `SIGKILL`.

## 2. Required rollback semantics

When background startup fails after launch:

- attempt graceful termination first
- escalate to forceful termination if still running
- remove the pid file only after the managed process is confirmed gone
- if the process still survives, return failure and keep pid metadata intact

## 3. Why this matters

The most dangerous local operator state is:

- process alive
- pid file removed
- command reported failure

That combination breaks lifecycle observability and can cause later operator commands to make incorrect assumptions about ownership and liveness.

## 4. Regression requirement

The repository must keep:

- a deployment asset contract asserting `kill -9 "$pid"` fallback in `start-local.sh`
- a shell behavior regression that simulates a `SIGTERM`-ignoring `local-minimal-node` process and verifies rollback escalation

## 5. Cross-platform note

This runner behavior-verifies the Windows lifecycle path and source-verifies the Unix shell path.

Direct execution of the shell rollback regression still requires a usable bash-capable environment, but the code and test contract are now in place for that verification.
