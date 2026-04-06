# 2026-04-06 Local-Minimal Runtime-Dir Restore Preview Confirmation Review Cycle

## 1. Findings

### 1.1 High: richer preview still did not guarantee action matched review

After the earlier preview waves, operators could inspect:

- file-level restore actions
- generic object-key diffs
- disconnect fence typed diffs
- checkpoint typed diffs

But restore still accepted only `backup-dir`, which meant the operator could review one preview and later restore a changed runtime state without any explicit handoff.

### 1.2 High: the remaining gap was decision-to-action integrity

The missing capability was not more explanation. It was proof that the restore being executed still matched the preview that had been reviewed.

### 1.3 Medium: the fix needed to stay explicit and non-interactive

The correct next seam was therefore:

- a deterministic preview fingerprint
- an optional expected fingerprint parameter on restore
- fail-fast mismatch behavior before any mutation

## 2. Root Cause

The operator lifecycle had strong preview semantics but no explicit confirmation boundary between:

1. reviewing preview output
2. executing restore

Without that boundary, stale operator context remained possible.

## 3. Implementation

This review wave adds preview-to-restore confirmation:

- added `previewFingerprint` to restore preview output
- added deterministic preview fingerprint generation
- added restore helper with optional expected fingerprint validation
- added `confirmedPreviewFingerprint` to restore report output
- extended CLI parsing for `--expected-preview-fingerprint`
- extended local restore scripts and status guidance

On mismatch, restore now fails before any pre-restore backup or runtime mutation.

## 4. Safety Rule Preserved

This wave preserves earlier rules:

- preview remains read-only
- restore remains explicit
- confirmation is optional, not silently enforced
- mismatch failure is zero-side-effect

## 5. Regression Coverage

- `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`
  - stable preview fingerprint exposure
- `services/local-minimal-node/tests/runtime_dir_restore_test.rs`
  - matching fingerprint success
  - mismatched fingerprint zero-side-effect failure
- `services/local-minimal-node/tests/deployment_profile_test.rs`
  - restore wrapper flag exposure
  - status guidance exposure

## 6. Verification

This wave must be verified with fresh command output:

- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo test -p local-minimal-node --offline`
- `powershell -ExecutionPolicy Bypass -File bin\\preview-runtime-restore-local.ps1 -RuntimeDir .runtime\\local-minimal -BackupDir .runtime\\local-minimal\\backups\\runtime-dir-restore-1775463595576629400 -Json`

## 7. Standardized Outcome

`local-minimal` restore now supports a safer operator loop:

1. preview
2. review
3. capture fingerprint
4. restore only if the live preview still matches

That tightens operator safety without introducing interactive blockers or control-plane dependencies.

## 8. Residual Risk

This wave intentionally does **not** yet provide:

- mandatory confirmation for all restores
- signed or cryptographic approvals
- multi-operator approval workflows
- remote authenticated orchestration

## 9. Next Wave

The next review wave should target one of these:

1. typed summaries for realtime subscriptions or stream state
2. mandatory private-deployment restore policy modes
3. authenticated remote orchestration for clustered private deployment
