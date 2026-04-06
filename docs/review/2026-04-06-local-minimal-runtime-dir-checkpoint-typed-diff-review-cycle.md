# 2026-04-06 Local-Minimal Runtime-Dir Checkpoint Typed Diff Review Cycle

## 1. Findings

### 1.1 High: generic diff still hid whether checkpoint progress was moving forward or backward

After Standard 108, operators could see that a checkpoint record would change.

But they still could not answer whether:

- `latest_realtime_seq` advanced or rewound
- `acked_through_seq` advanced or rewound
- `trimmed_through_seq` advanced or rewound
- the record only refreshed `updated_at`

That made checkpoint recovery review slower than it needed to be.

### 1.2 High: checkpoint drift is operationally meaningful

`realtime-checkpoints.json` influences replay windows, redelivery expectations, and trim progress.

That makes directional sequence change more important than generic object-key diff alone.

### 1.3 Medium: the enhancement still had to remain informational

The correct next step was still read-only explanation:

- no checkpoint reconciliation logic
- no restore confirmation
- no state rewrite

## 2. Root Cause

The generic preview layer reports value difference but not domain meaning.

Operators reason about realtime checkpoints in terms of:

- progression
- rollback
- timestamp refresh
- inconsistent record identity

Without typed summary, they still had to read raw JSON manually for sequence semantics.

## 3. Implementation

This review wave adds a checkpoint-specific typed summary:

- extended `RuntimeDirRestorePreviewDomainSummaryView`
- added `summarize_realtime_checkpoint_restore_preview_change(...)`
- layered checkpoint summary selection after disconnect-fence summary selection
- extended text rendering with `checkpoint-diff`

The typed summary distinguishes:

- added and removed checkpoint records
- latest seq advance and rewind
- acked seq advance and rewind
- trimmed seq advance and rewind
- timestamp-only changes
- other modified records
- unchanged shared keys

## 4. Safety Rule Preserved

This wave preserves all existing safety guarantees:

- preview remains read-only
- restore semantics are unchanged
- generic diff output is still emitted
- unsupported payloads simply omit checkpoint typed summary

## 5. Regression Coverage

- `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`
  - `test_preview_restore_runtime_dir_reports_checkpoint_typed_summary`
  - all existing preview tests remain green

## 6. Verification

This wave must be verified with fresh command output:

- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo test -p local-minimal-node --offline`
- `powershell -ExecutionPolicy Bypass -File bin\\preview-runtime-restore-local.ps1 -RuntimeDir .runtime\\local-minimal -BackupDir .runtime\\local-minimal\\backups\\runtime-dir-restore-1775463595576629400 -Json`

## 7. Standardized Outcome

`local-minimal` restore preview now explains checkpoint impact in sequence-oriented operator language instead of only raw JSON difference categories.

That reduces ambiguity during replay and recovery review.

## 8. Residual Risk

This wave intentionally does **not** yet provide:

- typed summaries for subscriptions, stream state, rtc state, notification tasks, or automation executions
- nested path diff for checkpoint payloads
- preview-to-restore confirmation handoff
- remote orchestration

## 9. Next Wave

The next review wave should target one of these:

1. typed summaries for realtime subscriptions or stream state
2. preview-to-restore confirmation handoff
3. authenticated remote orchestration for private clustered deployment
