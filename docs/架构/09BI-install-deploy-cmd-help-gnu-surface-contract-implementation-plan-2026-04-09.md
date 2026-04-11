# 09BI Install/Deploy CMD Help GNU-Surface Contract Implementation Plan

## Goal

Make `bin/install-local.cmd --help` and `bin/deploy-local.cmd --help` surface the GNU-style Windows named flags that operators are expected to use.

## Implementation

1. Freeze both Windows help surfaces in `services/local-minimal-node/tests/deployment_profile_test.rs`
2. Patch the `-Help` branches in `bin/install-local.ps1` and `bin/deploy-local.ps1`
3. Keep forwarding and runtime execution behavior unchanged for this loop
4. Backwrite review, step, and architecture indexes

## Non-Goals

- No launcher refactor
- No `_cmd-forward-powershell.cmd` redesign
- No install/deploy runtime behavior change
- No unrelated Windows wrapper edits
