# Local-Minimal Runtime-Dir RTC Typed Diff Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a domain-aware typed diff summary for `rtc-state.json` so private-deployment operators can see RTC session lifecycle drift and persisted signal log changes during restore preview.

**Architecture:** Keep the existing restore preview composition unchanged: file-level action first, generic JSON object-key diff second, typed domain summary last. Extend the shared `RuntimeDirRestorePreviewDomainSummaryView` additively so `rtc-state.json` can expose RTC-session and signal-log semantics without changing restore behavior, preview status rules, or preview fingerprint confirmation.

**Tech Stack:** Rust, serde, serde_json, cargo test, cargo fmt, PowerShell, bash

---

### Task 1: Lock RTC Restore Semantics with Red Tests

**Files:**
- Modify: `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`

- [ ] **Step 1: Add an RTC snapshot helper for tests**

Create a focused helper that serializes `rtc-state.json` entries with `session`, `signals`, and `updated_at` fields using the real persisted wire contract.

- [ ] **Step 2: Add a failing typed-summary regression test**

Require restore preview to expose a `domainSummary` with `summaryKind = "rtc_state"` and the planned RTC semantic categories for a mixed change set.

- [ ] **Step 3: Add a failing formatter regression test**

Require `format_runtime_dir_restore_preview(...)` to emit an `rtc-diff` line that includes the new categories.

- [ ] **Step 4: Run the targeted test and verify failure**

Run:
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`

Expected: FAIL because RTC-specific typed summary fields and formatter support do not exist yet.

### Task 2: Implement RTC Typed Diff

**Files:**
- Modify: `services/local-minimal-node/src/lib.rs`

- [ ] **Step 1: Extend the shared domain summary view additively**

Add optional RTC-oriented fields without changing existing disconnect fence, checkpoint, subscription, and stream JSON contracts.

- [ ] **Step 2: Implement `rtc-state.json` summarization**

Add a typed summarizer that:
- parses both payloads as `Map<String, RtcStateRecord>`
- preserves record-level `addedKeys` and `removedKeys`
- compares shared records by RTC-session lifecycle semantics and signal-log semantics
- emits categories for session state drift, signaling-stream drift, artifact-message drift, signal additions/removals/modifications, updated-at-only changes, and fallback `otherModifiedKeys`

- [ ] **Step 3: Compose the new summarizer into restore preview**

Wire the RTC summarizer into the existing `or_else(...)` chain after stream support without changing earlier typed diff behavior.

- [ ] **Step 4: Extend text rendering**

Emit an `rtc-diff` line under the preview action that prints RTC-specific categories in deterministic order.

### Task 3: Document the RTC Standard and Review Outcome

**Files:**
- Create: `docs/架构/114-local-minimal-runtime-dir-rtc-typed-diff-standard-2026-04-06.md`
- Create: `docs/review/2026-04-06-local-minimal-runtime-dir-rtc-typed-diff-review-cycle.md`

- [ ] **Step 1: Write the architecture standard**

Document the typed summary contract, eligibility rules, semantic category meanings, formatter rules, and verification requirements for `rtc-state.json`.

- [ ] **Step 2: Write the review-cycle document**

Capture the operator gap left by generic object-key diff, the chosen additive design, and why RTC session plus signal semantics are required for private deployment restore review.

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
