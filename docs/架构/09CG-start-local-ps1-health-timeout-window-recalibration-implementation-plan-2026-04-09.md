# start-local.ps1 Health Timeout Window Recalibration Implementation Plan

## Goal

- Re-stabilize the Windows Step 10 rollback regression proof after the earlier `5 x 100ms` test window remained too small.

## Steps

- Reproduce the failing exact test and confirm the missing probe marker.
- Change only the copied PowerShell readiness loop in `deployment_profile_test.rs` from `5` to `20` attempts.
- Keep the `100ms` sleep interval and all cleanup assertions unchanged.
- Re-run the exact test, repeated exact runs, the full `deployment_profile_test` suite, formatting, and the package tests.

## Boundary

- No edits to `bin/start-local.ps1`.
- No broad refactor of the lifecycle scripts.
- No cross-platform completion claim without matching runtime proof.
