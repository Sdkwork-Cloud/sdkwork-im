# 121. local-minimal shell pid identity guard standard

## 1. Standard

For Unix lifecycle scripts, pid existence alone is never sufficient to identify a managed `local-minimal-node` process.

`start-local.sh`, `status-local.sh`, and `stop-local.sh` must all validate:

- the pid exists
- the live process identity matches `local-minimal-node`

If the identity does not match:

- treat the pid file as stale metadata
- remove it
- do not report `running`
- do not block startup
- do not signal the unrelated process

## 2. Why this matters

Pid files are only hints. On long-lived hosts, pid reuse and stale runtime metadata are normal failure shapes.

Without identity validation, lifecycle tooling can:

- stop the wrong process
- produce false running or health signals
- block safe startup or restart
- make support automation trust incorrect local runtime state

## 3. Current shell identity rule

The current Unix operator rule validates:

- `kill -0 "$pid"` succeeds
- `ps -p "$pid" -o comm=` resolves to `local-minimal-node`

This is the minimum accepted safety bar for the local profile.

## 4. Shell robustness rule

Because the lifecycle scripts run with `set -euo pipefail`, process inspection must not abort the script on short-lived pid races.

The shell implementation must therefore treat failed process-name inspection as a safe mismatch condition, not as a fatal script error.

## 5. Regression requirement

The repository must keep deployment asset contract assertions that freeze the presence of:

- `EXPECTED_PROCESS_NAME="local-minimal-node"`
- `pid_matches_expected_process`
- `ps -p "$pid" -o comm=`

Direct shell behavior regressions for stale pid collisions are also required whenever the verification environment provides a usable bash runtime.

## 6. Cross-platform note

Windows and Unix lifecycle scripts now share the same operator intent:

- managed-process identity must be validated before start, status, or stop decisions
- stale unmanaged pid files must self-heal

Cross-platform lifecycle parity is part of the local deployment safety baseline.
