# 2026-04-06 Local-Minimal Runtime-Dir Restore Preview Review Cycle

## 1. Findings

### 1.1 High: backup discovery and restore still left an operator decision gap

- Standard 106 added backup listing and snapshot quality preview
- Standard 105 already required explicit restore selection

But operators still could not see how a specific snapshot would interact with the current runtime-dir before invoking a destructive restore.

### 1.2 High: snapshot quality alone was not enough

A backup can be:

- structurally full
- but still different from current state in only one file

or:

- sparse
- but still useful for restoring a missing target file

That means snapshot quality and restore impact are related but not identical concerns.

### 1.3 Medium: restore needed a dry-run seam, not another hidden heuristic

The missing capability was a read-only preview of restore actions.

The correct next seam was therefore:

- explicit backup selection
- no mutation
- exact per-file restore action preview

## 2. Root Cause

The root cause was an incomplete operator lifecycle:

1. backups could be listed
2. restore could be executed explicitly
3. but there was no first-class dry-run bridge between those steps

Without that seam, operators had to infer restore impact mentally from raw backup and runtime state.

## 3. Implementation

This review cycle adds a read-only restore preview workflow:

- added `preview_restore_runtime_dir(...)` in `local-minimal-node`
- added `RuntimeDirRestorePreviewView`
- added `RuntimeDirRestorePreviewActionView`
- added local CLI entrypoint:
  - `local-minimal-node preview-runtime-restore --backup-dir <path> [--runtime-dir <path>] [--json]`
- added lifecycle wrappers:
  - `bin/preview-runtime-restore-local.ps1`
  - `bin/preview-runtime-restore-local.sh`
  - `bin/preview-runtime-restore-local.cmd`
- updated status scripts to reference:
  - inspection
  - repair
  - backup listing
  - restore preview
  - restore

The preview logic is intentionally deterministic:

1. validate the selected backup dir exists
2. validate `<backup-dir>/state/` exists
3. inspect current runtime-dir state
4. summarize source snapshot quality and report metadata
5. compare each managed source file to the current target
6. classify each file as `would_restore`, `noop`, or `skip`
7. return aggregate preview counts without performing any mutation

## 4. Safety Rule

This wave preserves the explicit-mutation model:

- preview is read-only
- preview does not create pre-restore backups
- preview does not execute restore
- preview does not normalize or rewrite state

That keeps decision support separate from action execution.

## 5. Regression Coverage

- `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`
  - `test_preview_restore_runtime_dir_reports_ready_without_mutation_for_full_snapshot`
  - `test_preview_restore_runtime_dir_reports_partial_for_sparse_snapshot`
  - `test_preview_restore_runtime_dir_rejects_missing_backup_dir_without_mutation`
- `services/local-minimal-node/tests/deployment_profile_test.rs`
  - preview script asset assertions
  - status guidance assertions

## 6. Verification

Verified in this cycle with fresh command output:

- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`

Integrated verification for this wave must also include:

- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p local-minimal-node --offline`
- script-level verification through `bin/preview-runtime-restore-local.ps1`

## 7. Standardized Outcome

Managed `local-minimal` private deployment now has a tighter operator loop:

1. inspect
2. validate
3. repair
4. list backups
5. preview restore
6. restore

This reduces accidental destructive restore operations and makes rollback decisions auditable.

## 8. Residual Risk

This wave intentionally does **not** yet provide:

- field-level JSON diffs
- operator confirm/apply prompts
- restore dry-run snapshots for post-review handoff
- remote authenticated preview APIs

## 9. Next Wave

The next review wave should target one of these:

1. field-level diff preview for managed state files
2. explicit operator confirm/apply workflows layered on preview
3. remote authenticated preview and restore orchestration for clustered private deployment
