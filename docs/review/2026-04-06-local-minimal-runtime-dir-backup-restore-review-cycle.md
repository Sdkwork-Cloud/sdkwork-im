# 2026-04-06 Local-Minimal Runtime-Dir Backup Restore Review Cycle

## 1. Findings

### 1.1 High: inspection and safe repair existed, but operators still lacked a supported rollback path

- Standards 102-104 gave operators:
  - read-only inspection
  - semantic validation
  - backup-first recreation for missing files
- but they still could not explicitly revert the managed runtime-dir to a previously captured snapshot

That left a recovery gap for cases where current state was valid JSON but operationally wrong.

### 1.2 High: repair and restore are different safety classes and must remain separate

- repair is safe because it only recreates known-missing files with typed-empty content
- restore is different because it overwrites current state from a historical snapshot

Merging the two flows would blur operator intent and weaken auditability.

### 1.3 Medium: overwrite workflows need a second safety boundary

Once restore is introduced, the system needs a restore-before-overwrite backup of the current runtime-dir state so the restore itself can be reversed if the wrong snapshot is chosen.

## 2. Root Cause

The root cause was that the runtime-dir lifecycle had standardized detection and low-risk repair, but had not yet standardized operator-controlled rollback:

1. durable state existed
2. inspection identified missing/corrupt/unhealthy state
3. repair handled the missing-file case
4. no supported snapshot restore workflow existed for targeted rollback

## 3. Implementation

This review cycle adds a separate local restore seam without weakening the earlier standards:

- added `restore_runtime_dir(...)` in `local-minimal-node`
- added `RuntimeDirRestoreView`
- added local CLI entrypoint:
  - `local-minimal-node restore-runtime-dir --backup-dir <path> [--runtime-dir <path>] [--json]`
- added lifecycle wrappers:
  - `bin/restore-runtime-local.ps1`
  - `bin/restore-runtime-local.sh`
  - `bin/restore-runtime-local.cmd`
- updated operator status scripts to reference:
  - inspection
  - repair
  - restore
- updated cmd argument normalization to forward:
  - `/backupDir`
  - `-BackupDir`
  - `--backup-dir`

The restore logic is intentionally explicit:

1. validate the selected backup directory exists
2. validate `<backup-dir>/state/` exists
3. inspect current runtime-dir state
4. create a pre-restore backup under `<runtime-dir>/backups/...`
5. snapshot existing managed state files into that pre-restore backup
6. copy managed files that exist in the selected backup snapshot
7. mark missing source files as skipped
8. emit a structured restore report and post-restore inspection result

## 4. Safety Rule

This wave keeps the operator posture explicit and fail-closed:

- restore is local-only
- restore requires explicit snapshot selection
- current state is backed up before overwrite
- no automatic backup selection occurs
- no implicit merge or rewrite of missing source files occurs

That avoids silent rollback behavior and keeps destructive state replacement operator-directed.

## 5. Regression Coverage

- `services/local-minimal-node/tests/runtime_dir_restore_test.rs`
  - `test_restore_runtime_dir_restores_selected_snapshot_and_creates_pre_restore_backup`
  - `test_restore_runtime_dir_rejects_missing_backup_dir_without_mutation`
- `services/local-minimal-node/tests/deployment_profile_test.rs`
  - restore script asset assertions
  - cmd forwarding assertions for `--backup-dir`
  - status script references for restore guidance

## 6. Verification

Verified in this cycle with fresh command output:

- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`

Integrated verification for this wave must also include:

- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p local-minimal-node --offline`
- script-level restore verification against a real local backup snapshot

## 7. Standardized Outcome

Managed `local-minimal` private deployment now has a fuller operator loop:

1. inspect
2. validate
3. repair missing files when safe
4. restore an explicitly chosen snapshot when rollback is required

This closes the gap between detection and controlled rollback without exposing restore as a public API.

## 8. Residual Risk

This wave intentionally does **not** yet provide:

- automatic ranking/listing of backup snapshots
- guided diff/preview before restore
- selective per-file restore filters
- guided corrupt-file remediation
- remote authenticated control-plane restore workflows

## 9. Next Wave

The next review wave should target one of these:

1. backup snapshot listing/indexing and operator preview tooling
2. guided corrupt-file remediation with explicit confirmations
3. remote authenticated restore orchestration for private clustered deployments
