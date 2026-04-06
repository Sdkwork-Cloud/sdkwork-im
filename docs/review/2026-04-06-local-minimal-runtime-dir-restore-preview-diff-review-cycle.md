# 2026-04-06 Local-Minimal Runtime-Dir Restore Preview Diff Review Cycle

## 1. Findings

### 1.1 High: restore preview still forced operators to mentally inspect payload-level differences

Standard 107 closed the dry-run gap by showing which managed files would be restored.

But when a file-level action was `would_restore`, operators still had to manually open source and target JSON files to understand the actual impact.

### 1.2 High: byte-level difference alone was not enough for incident response

A `content_differs` result answers:

- that a restore would happen

but not:

- whether the restore adds new records
- whether it removes current runtime records
- whether it only changes a single existing record

That missing explanation increases operator hesitation and recovery risk.

### 1.3 Medium: the next step needed to be additive, not semantic restore logic

The correct next wave was to enrich preview output, not to redesign restore behavior.

That means:

- no partial-file apply
- no semantic merge
- no mutation
- only structured explanation layered onto existing preview actions

## 2. Root Cause

The operator seam was still missing one level of detail:

1. backup listing explained source snapshot quality
2. restore preview explained file-level impact
3. but no first-class summary explained field-level impact for JSON-object state files

Without that layer, operators still had to perform manual file diff analysis during recovery.

## 3. Implementation

This review wave adds an optional structured diff summary to restore preview actions:

- added `RuntimeDirRestorePreviewChangeSummaryView`
- extended `RuntimeDirRestorePreviewActionView` with optional `change_summary`
- added `summarize_runtime_restore_preview_change(...)`
- kept existing `would_restore` / `noop` / `skip` action classification unchanged
- extended text formatting with an indented `json-object-diff` line

Current summary behavior is deliberately narrow:

- only when source and target both exist
- only when payloads differ
- only when both payloads are parseable top-level JSON objects

The implementation uses stable key ordering so preview output is deterministic across runs.

## 4. Safety Rule Preserved

This wave preserves the read-only preview contract:

- no restore execution
- no runtime mutation
- no backup creation
- no semantic merge
- no rewrite of persisted JSON payloads

The new summary is informational only.

## 5. Regression Coverage

- `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`
  - `test_preview_restore_runtime_dir_reports_ready_without_mutation_for_full_snapshot`
  - `test_preview_restore_runtime_dir_reports_partial_for_sparse_snapshot`
  - `test_preview_restore_runtime_dir_reports_json_object_key_change_summary`
  - `test_preview_restore_runtime_dir_rejects_missing_backup_dir_without_mutation`

## 6. Verification

This wave must be verified with fresh command output:

- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo test -p local-minimal-node --offline`
- `powershell -ExecutionPolicy Bypass -File bin\\preview-runtime-restore-local.ps1 -RuntimeDir .runtime\\local-minimal -BackupDir .runtime\\local-minimal\\backups\\runtime-dir-restore-1775463595576629400 -Json`

## 7. Standardized Outcome

Managed `local-minimal` restore preview now provides:

1. snapshot-level source quality
2. file-level restore actions
3. optional field-level JSON object key summaries

This reduces incident-response ambiguity while preserving explicit restore execution as a separate auditable step.

## 8. Residual Risk

This wave intentionally does **not** yet provide:

- nested JSON path diffs
- array element diffs for commit journals
- summaries for `target_missing` or `skip` cases
- operator confirm/apply workflows
- remote authenticated orchestration APIs

## 9. Next Wave

The next review wave should target one of these:

1. typed domain-aware summaries for high-risk state files
2. preview-to-restore confirmation handoff
3. authenticated remote orchestration for clustered private deployment
