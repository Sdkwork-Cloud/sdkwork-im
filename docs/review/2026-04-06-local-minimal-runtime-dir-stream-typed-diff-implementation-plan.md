# Local-Minimal Runtime-Dir Stream Typed Diff Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a domain-aware typed diff summary for `stream-state.json` so private-deployment operators can see stream lifecycle drift, sequence movement, and frame-level changes during restore preview.

**Architecture:** Keep the existing restore preview composition unchanged: file-level action first, generic JSON object-key diff second, typed domain summary last. Extend the shared `RuntimeDirRestorePreviewDomainSummaryView` additively so `stream-state.json` can expose stream-session and frame semantics without changing restore behavior, preview status rules, or preview fingerprint confirmation.

**Tech Stack:** Rust, serde, serde_json, cargo test, cargo fmt, PowerShell, bash

---

### Task 1: Lock Stream Restore Semantics with Red Tests

**Files:**
- Modify: `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`

- [ ] **Step 1: Add a stream snapshot helper for tests**

Create a focused helper that serializes `stream-state.json` entries with `session`, `frames`, and `updatedAt` fields using the real persisted wire contract.

- [ ] **Step 2: Add a failing typed-summary regression test**

Require restore preview to expose a `domainSummary` with `summaryKind = "stream_state"` and the planned stream semantic categories for a mixed change set.

- [ ] **Step 3: Add a failing formatter regression test**

Require `format_runtime_dir_restore_preview(...)` to emit a `stream-diff` line that includes the new categories.

- [ ] **Step 4: Run the targeted test and verify failure**

Run:
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`

Expected: FAIL because stream-specific typed summary fields and formatter support do not exist yet.

### Task 2: Implement Stream Typed Diff

**Files:**
- Modify: `services/local-minimal-node/src/lib.rs`

- [ ] **Step 1: Extend the shared domain summary view additively**

Add optional stream-oriented fields without changing existing disconnect fence, checkpoint, and subscription JSON contracts.

- [ ] **Step 2: Implement `stream-state.json` summarization**

Add a typed summarizer that:
- parses both payloads as `Map<String, StreamStateRecord>`
- preserves record-level `addedKeys` and `removedKeys`
- compares shared records by session lifecycle semantics and frame-set semantics
- emits categories for session state drift, last-frame advance/rewind, checkpoint advance/rewind, result-message drift, frame additions/removals/modifications, updated-at-only changes, and fallback `otherModifiedKeys`

- [ ] **Step 3: Compose the new summarizer into restore preview**

Wire the stream summarizer into the existing `or_else(...)` chain after subscription support without changing earlier typed diff behavior.

- [ ] **Step 4: Extend text rendering**

Emit a `stream-diff` line under the preview action that prints stream-specific categories in deterministic order.

### Task 3: Document the Stream Standard and Review Outcome

**Files:**
- Create: `docs/架构/113-local-minimal-runtime-dir-stream-typed-diff-standard-2026-04-06.md`
- Create: `docs/review/2026-04-06-local-minimal-runtime-dir-stream-typed-diff-review-cycle.md`

- [ ] **Step 1: Write the architecture standard**

Document the typed summary contract, eligibility rules, semantic category meanings, formatter rules, and verification requirements for `stream-state.json`.

- [ ] **Step 2: Write the review-cycle document**

Capture the operator gap left by generic object-key diff, the chosen additive design, and why stream lifecycle plus frame semantics are required for private deployment restore review.

### Task 4: Verify End to End

**Files:**
- Modify as required by earlier tasks only

- [ ] **Step 1: Run formatting**

Run: `cargo fmt --all`

- [ ] **Step 2: Verify formatting**

Run: `cargo fmt --all --check`
Expected: PASS

- [ ] **Step 3: Run targeted restore preview tests**

Run:
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`

Expected: PASS

- [ ] **Step 4: Run package verification**

Run: `cargo test -p local-minimal-node --offline`
Expected: PASS
