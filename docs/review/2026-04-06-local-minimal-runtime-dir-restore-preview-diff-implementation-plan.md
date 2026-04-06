# Local-Minimal Runtime-Dir Restore Preview Diff Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extend `preview-runtime-restore` with read-only field-level change summaries so operators can understand which JSON object keys would change before running a restore.

**Architecture:** Keep restore preview as a separate read-only operator seam. Enrich per-file preview actions with an optional structured diff summary produced only when both source and target payloads are parseable JSON objects; all other cases keep the existing byte-level classification. The preview command, JSON report, and text formatter must remain backward-compatible for existing aggregate counts and action semantics.

**Tech Stack:** Rust, serde, serde_json, cargo test, cargo fmt

---

### Task 1: Lock the New Contract with Red Tests

**Files:**
- Modify: `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`

- [ ] **Step 1: Add a failing test for JSON object key-level diff summary**

Cover a restore preview where `realtime-disconnect-fences.json` exists in both source and target, but one key is added, one removed, and one modified.

- [ ] **Step 2: Run the targeted test and verify it fails for the expected reason**

Run: `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
Expected: FAIL because the preview action view does not yet expose structured change summary fields.

- [ ] **Step 3: Add a failing formatter assertion**

Extend the same test module so formatted preview text must surface a concise summary for JSON-object diffs.

- [ ] **Step 4: Re-run the targeted test and verify it still fails for the missing diff summary**

Run: `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
Expected: FAIL with missing fields or missing formatted output.

### Task 2: Implement Structured Preview Diff Summary

**Files:**
- Modify: `services/local-minimal-node/src/lib.rs`

- [ ] **Step 1: Add new serializable view types for structured preview diff summaries**

Define optional summary payloads on `RuntimeDirRestorePreviewActionView` without changing existing aggregate counters or action labels.

- [ ] **Step 2: Implement minimal JSON-object diff logic**

Only when both source and target payloads are parseable top-level JSON objects:
- collect added keys
- collect removed keys
- collect modified keys
- derive unchanged count

For arrays, scalars, invalid JSON, or missing files, leave the structured summary absent.

- [ ] **Step 3: Attach summaries during restore preview generation**

Keep existing `would_restore` / `noop` / `skip` semantics untouched. The new summary is informational only.

- [ ] **Step 4: Extend text formatting**

Make `format_runtime_dir_restore_preview(...)` print a compact, deterministic line for files that carry structured change summary.

### Task 3: Document the New Standard

**Files:**
- Create: `docs/架构/108-local-minimal-runtime-dir-restore-preview-diff-standard-2026-04-06.md`
- Create: `docs/review/2026-04-06-local-minimal-runtime-dir-restore-preview-diff-review-cycle.md`
- Modify: `docs/架构/107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md`

- [ ] **Step 1: Write the architecture standard**

Document scope, read-only rule, summary semantics, formatter expectations, and non-goals.

- [ ] **Step 2: Write the review-cycle document**

Capture findings, root cause, implementation notes, regression coverage, and residual risks.

- [ ] **Step 3: Update Standard 107 composition notes**

Reference the new diff standard as a layered enhancement rather than a replacement.

### Task 4: Verify End to End

**Files:**
- Modify as required by earlier tasks only

- [ ] **Step 1: Run formatting**

Run: `cargo fmt --all`

- [ ] **Step 2: Verify formatting is clean**

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

- [ ] **Step 6: Run script-level restore preview verification**

Run: `powershell -ExecutionPolicy Bypass -File bin\\preview-runtime-restore-local.ps1 -RuntimeDir .runtime\\local-minimal -BackupDir .runtime\\local-minimal\\backups\\runtime-dir-restore-1775463595576629400 -Json`
Expected: PASS with `status = "noop"` and no mutation.
