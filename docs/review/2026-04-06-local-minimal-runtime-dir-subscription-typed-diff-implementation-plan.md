# Local-Minimal Runtime-Dir Subscription Typed Diff Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a domain-aware typed diff summary for `realtime-subscriptions.json` so private-deployment operators can see subscription scope and event-type drift during restore preview instead of only raw JSON object-key changes.

**Architecture:** Keep the existing restore preview composition unchanged: file-level action first, generic JSON object-key diff second, typed domain summary last. Extend the shared `RuntimeDirRestorePreviewDomainSummaryView` additively so `realtime-subscriptions.json` can surface nested scope semantics without changing restore behavior, preview status rules, or fingerprint confirmation flow.

**Tech Stack:** Rust, serde, serde_json, cargo test, cargo fmt, PowerShell, bash

---

### Task 1: Lock Subscription Restore Semantics with Red Tests

**Files:**
- Modify: `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`

- [ ] **Step 1: Add a subscription snapshot helper for tests**

Create a focused helper that serializes `realtime-subscriptions.json` entries with nested `items`, `event_types`, `subscribed_at`, and `synced_at` fields.

- [ ] **Step 2: Add a failing typed-summary regression test**

Require restore preview to expose a `domainSummary` with `summaryKind = "realtime_subscriptions"` and all planned semantic categories for a mixed change set.

- [ ] **Step 3: Add a failing formatter regression test**

Require `format_runtime_dir_restore_preview(...)` to emit a `subscription-diff` line that includes the new typed categories.

- [ ] **Step 4: Run the targeted test and verify failure**

Run:
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`

Expected: FAIL because subscription-specific typed summary fields and formatter support do not exist yet.

### Task 2: Implement Subscription Typed Diff

**Files:**
- Modify: `services/local-minimal-node/src/lib.rs`

- [ ] **Step 1: Extend the shared domain summary view additively**

Add optional subscription-oriented fields without changing existing disconnect fence and checkpoint JSON contracts.

- [ ] **Step 2: Implement `realtime-subscriptions.json` summarization**

Add a typed summarizer that:
- parses both payloads as `Map<String, RealtimeSubscriptionRecord>`
- preserves device-level `addedKeys` and `removedKeys`
- compares shared records by nested scope identity
- emits scoped categories for added scopes, removed scopes, event-type expansion, event-type reduction, scope-level subscribed-at-only changes, and record-level synced-at-only changes
- falls back to `otherModifiedKeys` for shared records with changes outside the tracked categories

- [ ] **Step 3: Compose the new summarizer into restore preview**

Wire the subscription summarizer into the existing `or_else(...)` chain after checkpoint support without changing earlier typed diff behavior.

- [ ] **Step 4: Extend text rendering**

Emit a `subscription-diff` line under the preview action that prints the subscription-specific categories in deterministic order.

### Task 3: Document the Subscription Standard and Review Outcome

**Files:**
- Create: `docs/架构/112-local-minimal-runtime-dir-subscription-typed-diff-standard-2026-04-06.md`
- Create: `docs/review/2026-04-06-local-minimal-runtime-dir-subscription-typed-diff-review-cycle.md`

- [ ] **Step 1: Write the architecture standard**

Document the typed summary contract, eligibility rules, semantic category meanings, formatter rules, and verification requirements for `realtime-subscriptions.json`.

- [ ] **Step 2: Write the review-cycle document**

Capture the operator gap left by generic object-key diff, the chosen additive design, and why nested scope semantics are required for private deployment restore review.

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
