# 2026-04-06 Local-Minimal Runtime-Dir Backup Catalog Review Cycle

## 1. Findings

### 1.1 High: explicit restore existed, but operators still had poor snapshot visibility

- the restore workflow already required explicit `--backup-dir`
- but operator tooling still gave no supported way to assess snapshot quality before restore

The result was a practical usability gap: a backup path could be valid while still being an empty or sparse snapshot.

### 1.2 High: the prior restore verification exposed a real operator blind spot

In the previous cycle, script-level restore verification against an existing repair backup returned:

- `status = partial`
- `restoredFileCount = 0`
- `skippedFileCount = 9`

That behavior was correct, but it also proved operators needed a preview layer that would have surfaced that snapshot as effectively empty before restore was attempted.

### 1.3 Medium: listing must stay read-only

The missing capability was catalog visibility, not another mutation workflow.

That means the correct next seam was:

- list backup snapshots
- preview lightweight metadata
- keep all mutation inside the already explicit repair and restore commands

## 2. Root Cause

The root cause was an incomplete operator lifecycle:

1. backup-first repair existed
2. explicit restore existed
3. but there was still no first-class catalog for backup discovery and readiness preview

Without that seam, operators were forced to inspect backup directories manually.

## 3. Implementation

This review cycle adds a read-only backup catalog workflow:

- added `list_runtime_backups(...)` in `local-minimal-node`
- added `RuntimeDirBackupCatalogView`
- added `RuntimeDirBackupCatalogItemView`
- added local CLI entrypoint:
  - `local-minimal-node list-runtime-backups [--runtime-dir <path>] [--json]`
- added lifecycle wrappers:
  - `bin/list-runtime-backups-local.ps1`
  - `bin/list-runtime-backups-local.sh`
  - `bin/list-runtime-backups-local.cmd`
- updated status scripts to reference:
  - inspection
  - repair
  - backup listing
  - restore

The catalog logic is intentionally lightweight:

1. scan `<runtime-dir>/backups/`
2. ignore non-directory entries
3. classify operation from backup directory name
4. count managed state files present in `state/`
5. classify snapshot quality as empty, partial, or full
6. preview report type and report status from `repair-report.json` or `restore-report.json`
7. return items newest-first

## 4. Safety Rule

This wave preserves the explicit-mutation model:

- listing is read-only
- listing does not create backup metadata
- listing does not rank or auto-select a snapshot
- listing does not alter restore semantics

This avoids smuggling policy or mutation into what should remain a safe inspection surface.

## 5. Regression Coverage

- `services/local-minimal-node/tests/runtime_dir_backup_catalog_test.rs`
  - `test_list_runtime_backups_classifies_snapshot_quality_and_previews_report_metadata`
  - `test_list_runtime_backups_returns_empty_catalog_when_backups_dir_is_missing`
- `services/local-minimal-node/tests/deployment_profile_test.rs`
  - list script asset assertions
  - status guidance assertions

## 6. Verification

Verified in this cycle with fresh command output:

- `cargo test -p local-minimal-node --offline --test runtime_dir_backup_catalog_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`

Integrated verification for this wave must also include:

- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p local-minimal-node --offline`
- script-level verification through `bin/list-runtime-backups-local.ps1`

## 7. Standardized Outcome

Managed `local-minimal` private deployment now has a clearer operator loop:

1. inspect
2. validate
3. repair
4. list and preview backups
5. restore

This reduces accidental use of empty or sparse snapshots while keeping restore explicit.

## 8. Residual Risk

This wave intentionally does **not** yet provide:

- file-level backup diffs
- restore dry-run simulation
- operator confirmations for destructive restore
- remote authenticated backup catalog APIs

## 9. Next Wave

The next review wave should target one of these:

1. restore dry-run and diff preview tooling
2. guided corrupt-file remediation with explicit confirmations
3. remote authenticated backup catalog and restore orchestration for clustered private deployment
