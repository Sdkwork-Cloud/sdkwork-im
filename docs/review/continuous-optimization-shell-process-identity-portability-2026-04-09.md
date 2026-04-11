# Continuous Optimization: Shell Process Identity Portability

## Context

- `bin/start-local.sh`, `bin/status-local.sh`, and `bin/stop-local.sh` used `ps -p "$pid" -o comm=` and exact `local-minimal-node` matching.
- On BSD/macOS, `comm` may truncate long command names, so a live managed process can be misread as stale or foreign.

## Confirmed Bug

- `status-local.sh` could report `stopped` for a live managed process.
- `start-local.sh` could ignore a live managed process and proceed with a conflicting restart path.
- `stop-local.sh` could refuse to stop a live managed process because the truncated name no longer matched.

## Root Cause

- The scripts matched a truncation-prone process field instead of full argv.
- They also assumed the returned field already was the executable basename.

## Fix

- Switch the three Bash lifecycle scripts from `ps -o comm=` to `ps -o args=`.
- Trim leading spaces, isolate argv[0], then compare only its basename with `local-minimal-node`.
- Add regression test `test_shell_lifecycle_scripts_use_args_based_process_identity_for_portability`.

## Verification

Red:

```powershell
cargo test -p local-minimal-node --offline --test deployment_profile_test test_shell_lifecycle_scripts_use_args_based_process_identity_for_portability -- --exact --nocapture
```

Green:

```powershell
cargo test -p local-minimal-node --offline --test deployment_profile_test test_shell_lifecycle_scripts_use_args_based_process_identity_for_portability -- --exact --nocapture
cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture
cargo fmt --all --check
cargo test -p local-minimal-node --offline -- --nocapture
```

## Result

- Bash lifecycle identity matching now follows the executable basename derived from full argv and is safer across Bash environments.
- Same-root-cause scan found no remaining lifecycle `ps -o comm=` check in repo scripts.

## Next Gap

- Run native Bash lifecycle smoke on Linux/macOS or Git Bash to prove runtime behavior beyond the content-level contract verified in this Windows session.
