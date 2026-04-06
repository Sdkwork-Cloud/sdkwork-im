# Local-Minimal Runtime-Dir Checkpoint Typed Diff Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extend restore preview with a domain-aware typed diff summary for `realtime-checkpoints.json` so operators can distinguish sequence advancement, rewind, and timestamp-only changes before running a restore.

**Architecture:** Reuse the existing restore preview and generic diff seam, then layer an optional checkpoint-specific typed summary only for `realtime-checkpoints.json`. The summary remains informational and read-only, and must not change action classification, aggregate counts, or actual restore semantics.

**Tech Stack:** Rust, serde, serde_json, cargo test, cargo fmt

---

### Task 1: Lock the Checkpoint Typed Summary Contract with Red Tests

**Files:**
- Modify: `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`

- [ ] **Step 1: Add a failing test for checkpoint typed diff summary**

Cover one preview where:
- one checkpoint key is added in source
- one checkpoint key is removed from source
- one shared checkpoint advances `latest_realtime_seq`
- one shared checkpoint rewinds `acked_through_seq`
- one shared checkpoint advances `trimmed_through_seq`
- one shared checkpoint changes only `updated_at`
- one shared checkpoint changes other fields without sequence movement

- [ ] **Step 2: Run the targeted test and verify it fails**

Run: `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
Expected: FAIL because the preview action view does not yet expose checkpoint typed summary fields.

- [ ] **Step 3: Add formatter assertions**

Require text output to surface a compact `checkpoint-diff` line.

- [ ] **Step 4: Re-run the targeted test and verify it still fails for the missing typed summary**

Run: `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
Expected: FAIL because the typed summary and formatter line do not exist yet.

### Task 2: Implement Checkpoint Typed Summary

**Files:**
- Modify: `services/local-minimal-node/src/lib.rs`

- [ ] **Step 1: Extend the serializable domain summary view**

Add checkpoint-specific optional fields without breaking existing disconnect-fence behavior.

- [ ] **Step 2: Implement checkpoint-specific diff logic**

For `realtime-checkpoints.json`, when both source and target payloads are parseable maps:
- collect added and removed keys
- collect `latest_realtime_seq` advanced and rewound keys
- collect `acked_through_seq` advanced and rewound keys
- collect `trimmed_through_seq` advanced and rewound keys
- collect timestamp-only changed keys
- collect other modified keys
- derive unchanged count

- [ ] **Step 3: Attach the typed summary without changing generic preview semantics**

The preview must continue to emit generic `changeSummary` and unchanged aggregate counters.

- [ ] **Step 4: Extend text formatting**

Render a deterministic `checkpoint-diff` line under the corresponding action.

### Task 3: Document the New Operator Standard

**Files:**
- Create: `docs/架构/110-local-minimal-runtime-dir-checkpoint-typed-diff-standard-2026-04-06.md`
- Create: `docs/review/2026-04-06-local-minimal-runtime-dir-checkpoint-typed-diff-review-cycle.md`
- Modify: `docs/架构/108-local-minimal-runtime-dir-restore-preview-diff-standard-2026-04-06.md`

- [ ] **Step 1: Write the typed diff architecture standard**

Document scope, sequence semantics, formatter expectations, and non-goals.

- [ ] **Step 2: Write the review-cycle document**

Capture why generic key-level diff was still insufficient for checkpoint recovery review.

- [ ] **Step 3: Update Standard 108 composition notes**

Reference the new checkpoint typed diff standard as another specialized layer on top of generic object-key diff.

### Task 4: Verify End to End

**Files:**
- Modify as required by earlier tasks only

- [ ] **Step 1: Run formatting**

Run: `cargo fmt --all`

- [ ] **Step 2: Verify formatting**

Run: `cargo fmt --all --check`
Expected: PASS

- [ ] **Step 3: Run targeted preview tests**

Run: `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
Expected: PASS

- [ ] **Step 4: Run deployment profile assertions**

Run: `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
Expected: PASS

- [ ] **Step 5: Run package verification**

Run: `cargo test -p local-minimal-node --offline`
Expected: PASS

- [ ] **Step 6: Run script-level preview verification**

Run: `powershell -ExecutionPolicy Bypass -File bin\\preview-runtime-restore-local.ps1 -RuntimeDir .runtime\\local-minimal -BackupDir .runtime\\local-minimal\\backups\\runtime-dir-restore-1775463595576629400 -Json`
Expected: PASS with stable JSON output and no runtime mutation.
