# Continuous Optimization: start-local.ps1 Health Timeout Window Recalibration

## Goal

- Stabilize the Windows health-timeout rollback regression test without weakening the shipped cleanup behavior.

## Scope

- Modify only `services/local-minimal-node/tests/deployment_profile_test.rs`.
- Do not change `bin/start-local.ps1`.

## Implementation

- Keep the failing exact test as the regression proof.
- Widen only the copied PowerShell readiness loop used by the test from `5 x 100ms` to `20 x 100ms`.
- Re-run the exact test, repeat it for stability, then re-run `deployment_profile_test` and the package test surface.

## Expected State

- The test still proves the same rollback contract:
  - startup fails after synthetic health timeout
  - the child process is stopped
  - the pid file is removed
- The test no longer depends on an unrealistically narrow Windows cold-start window.

## Boundary

- No production timeout changes.
- No new Bash/Linux/macOS claims beyond the tests that actually ran in this Windows session.
