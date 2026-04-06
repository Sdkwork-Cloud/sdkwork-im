# 2026-04-06 Local-Minimal Runtime-Dir Disconnect Fence Typed Diff Review Cycle

## 1. Findings

### 1.1 High: generic key diff still hid the most important disconnect-fence semantics

After Standard 108, operators could see which disconnect fence keys were added, removed, or modified.

But that still did not answer the more operationally useful questions:

- did route ownership move to another node
- did the fence now require a different session
- was the change only a timestamp refresh

### 1.2 High: disconnect fences are recovery-critical state

`realtime-disconnect-fences.json` directly affects reconnect and takeover behavior.

That makes it a high-value candidate for a typed diff before broader domain-specific summaries are introduced elsewhere.

### 1.3 Medium: the enhancement still needed to stay read-only

The right next step was explanation, not behavior change.

This wave therefore adds:

- typed reporting

and explicitly does not add:

- restore confirmations
- restore policies
- semantic merge

## 2. Root Cause

The generic diff layer operates on key and value equality, but operators reason about disconnect fences in terms of:

- ownership
- session lineage
- stale versus fresh record shape

Without a typed layer, they still had to inspect raw JSON manually for the most important cases.

## 3. Implementation

This review wave adds a disconnect-fence-specific typed summary:

- added `RuntimeDirRestorePreviewDomainSummaryView`
- extended preview actions with optional `domain_summary`
- added `summarize_disconnect_fence_restore_preview_change(...)`
- kept generic `change_summary` intact
- extended text output with `disconnect-fence-diff`

The typed summary distinguishes:

- added keys
- removed keys
- `owner_node_id` changes
- `session_id` changes
- other modified keys
- unchanged shared key count

## 4. Safety Rule Preserved

This wave preserves all existing safety guarantees:

- preview remains read-only
- restore semantics are unchanged
- generic diff output is still emitted
- invalid or unsupported payloads simply omit typed summary

## 5. Regression Coverage

- `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`
  - `test_preview_restore_runtime_dir_reports_disconnect_fence_typed_summary`
  - existing preview tests remain unchanged and green

## 6. Verification

This wave must be verified with fresh command output:

- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo test -p local-minimal-node --offline`
- `powershell -ExecutionPolicy Bypass -File bin\\preview-runtime-restore-local.ps1 -RuntimeDir .runtime\\local-minimal -BackupDir .runtime\\local-minimal\\backups\\runtime-dir-restore-1775463595576629400 -Json`

## 7. Standardized Outcome

`local-minimal` restore preview can now explain disconnect fence impact in domain language instead of only raw JSON diff categories.

That makes failover and reconnect incident review faster and less error-prone.

## 8. Residual Risk

This wave intentionally does **not** yet provide:

- typed summaries for checkpoints, subscriptions, stream state, rtc state, notification tasks, or automation executions
- nested path summaries inside fence records
- confirmation or apply handoff
- remote orchestration

## 9. Next Wave

The next review wave should target one of these:

1. typed summaries for other high-risk state files
2. preview-to-restore confirmation handoff
3. remote authenticated orchestration for private clustered deployment
