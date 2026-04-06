# Local-Minimal Runtime-Dir Disconnect Fence Typed Diff Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extend restore preview with a domain-aware typed diff summary for `realtime-disconnect-fences.json` so operators can distinguish added or removed fences from `owner_node_id` and `session_id` changes.

**Architecture:** Keep the generic JSON object key diff as the baseline explanation, then layer an optional disconnect-fence-specific summary only for `realtime-disconnect-fences.json`. The typed summary remains read-only and informational, and must not change preview action classification or actual restore behavior.

**Tech Stack:** Rust, serde, serde_json, cargo test, cargo fmt

---

### Task 1: Lock the Typed Summary Contract with Red Tests

**Files:**
- Modify: `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`

- [ ] **Step 1: Add a failing test for disconnect fence typed diff summary**

Cover one preview where:
- one fence key is added in source
- one fence key is removed from source
- one shared fence changes `owner_node_id`
- one shared fence changes `session_id`
- one shared fence changes neither owner nor session but still differs

- [ ] **Step 2: Run the targeted test and verify it fails**

Run: `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
Expected: FAIL because the preview action view does not yet expose a typed domain summary.

- [ ] **Step 3: Add formatter assertions**

Require text output to surface a compact `disconnect-fence-diff` line.

- [ ] **Step 4: Re-run the targeted test and verify it still fails for the missing typed summary**

Run: `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
Expected: FAIL for missing field or missing formatted output.

### Task 2: Implement Disconnect Fence Typed Summary

**Files:**
- Modify: `services/local-minimal-node/src/lib.rs`

- [ ] **Step 1: Add a serializable typed summary view**

Expose an optional domain summary on preview actions for disconnect fences.

- [ ] **Step 2: Implement disconnect-fence-specific diff logic**

For `realtime-disconnect-fences.json`, when both source and target payloads are parseable maps:
- collect added keys
- collect removed keys
- collect keys whose `owner_node_id` changes
- collect keys whose `session_id` changes
- collect other modified keys
- derive unchanged count

- [ ] **Step 3: Attach the typed summary without changing generic preview semantics**

The existing `changeSummary`, `action`, `detail`, and aggregate counts must remain valid.

- [ ] **Step 4: Extend text formatting**

Render a deterministic `disconnect-fence-diff` line under the corresponding action.

### Task 3: Document the New Operator Standard

**Files:**
- Create: `docs/架构/109-local-minimal-runtime-dir-disconnect-fence-typed-diff-standard-2026-04-06.md`
- Create: `docs/review/2026-04-06-local-minimal-runtime-dir-disconnect-fence-typed-diff-review-cycle.md`
- Modify: `docs/架构/108-local-minimal-runtime-dir-restore-preview-diff-standard-2026-04-06.md`

- [ ] **Step 1: Write the typed diff architecture standard**

Document scope, eligibility, typed fields, formatter expectations, and non-goals.

- [ ] **Step 2: Write the review-cycle document**

Capture why generic key-level diff was still insufficient and how the typed summary closes the operator gap.

- [ ] **Step 3: Update Standard 108 composition notes**

Reference the new typed disconnect-fence standard as a specialized layer on top of generic object-key diff.

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
Expected: PASS with a stable JSON payload and no runtime mutation.
