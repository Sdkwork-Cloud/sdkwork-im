# 120. local-minimal Windows pid identity guard standard

## 1. Standard

For Windows local lifecycle scripts, pid existence alone is never sufficient to identify a managed `local-minimal-node` process.

`start-local.ps1`, `status-local.ps1`, and `stop-local.ps1` must all validate:

- the pid exists
- the live process identity matches `local-minimal-node`

If the identity does not match:

- treat the pid file as stale metadata
- remove it
- do not report `running`
- do not block startup
- do not stop the unrelated process

## 2. Why this matters

Pid files are only a hint. On long-running developer or operator machines, stale pid metadata and pid reuse are normal failure shapes.

Without identity validation, lifecycle tooling can:

- kill the wrong process
- misreport service health
- block safe restart or startup flows
- create false support and automation signals

## 3. Current identity rule

The current Windows operator rule validates `ProcessName -ieq "local-minimal-node"`.

This is the minimum accepted safety bar for the local profile.

## 4. Regression requirement

The repository must keep behavior regressions for:

- `stop-local.ps1` refusing to kill an unrelated process from stale pid metadata
- `status-local.ps1` reporting `stopped` for that same stale pid case
- `start-local.ps1` ignoring stale unmanaged pid metadata and proceeding with startup

## 5. Cross-platform note

The same operator intent should be mirrored into the Unix shell lifecycle scripts. If a shell-capable environment is unavailable on the current runner, the Windows-verified standard still stands and Unix parity remains required backlog work.
