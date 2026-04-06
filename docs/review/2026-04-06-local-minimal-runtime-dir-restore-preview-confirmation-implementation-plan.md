# Local-Minimal Runtime-Dir Restore Preview Confirmation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add an explicit preview-to-restore confirmation seam so operators can require restore to match a previously reviewed preview fingerprint before any mutation begins.

**Architecture:** Keep restore preview and restore as separate operator seams, but connect them with a deterministic `previewFingerprint`. Preview emits the fingerprint in JSON and text output. Restore gains an optional expected fingerprint parameter and fails before creating pre-restore backups or mutating runtime state when the current preview fingerprint does not match the operator-provided value.

**Tech Stack:** Rust, serde, serde_json, cargo test, cargo fmt, PowerShell, bash

---

### Task 1: Lock the Confirmation Contract with Red Tests

**Files:**
- Modify: `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`
- Modify: `services/local-minimal-node/tests/runtime_dir_restore_test.rs`

- [ ] **Step 1: Add a failing preview test for stable fingerprint exposure**

Require `preview_restore_runtime_dir(...)` to return a non-empty fingerprint and keep it stable across repeated calls against unchanged inputs.

- [ ] **Step 2: Add a failing restore test for matching fingerprint success**

Require a restore helper with expected fingerprint to succeed and surface the confirmed fingerprint in the restore report.

- [ ] **Step 3: Add a failing restore test for mismatched fingerprint zero-side-effect failure**

Require restore to fail before creating pre-restore backups or mutating runtime state when the supplied fingerprint does not match the current preview.

- [ ] **Step 4: Run targeted tests and verify failure**

Run:
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_test -- --nocapture`

Expected: FAIL because preview fingerprints and restore confirmation support do not exist yet.

### Task 2: Implement Preview Fingerprint and Restore Confirmation

**Files:**
- Modify: `services/local-minimal-node/src/lib.rs`
- Modify: `services/local-minimal-node/src/main.rs`

- [ ] **Step 1: Add preview fingerprint to the preview view**

Expose a deterministic `previewFingerprint` field on restore preview output.

- [ ] **Step 2: Implement deterministic fingerprint material and hashing**

Use deterministic serialized preview material, excluding the fingerprint field itself, to derive a stable preview fingerprint.

- [ ] **Step 3: Add restore helper with optional expected fingerprint**

Restore must:
- recompute current preview
- compare fingerprint when supplied
- fail before mutation on mismatch
- surface the confirmed fingerprint in the restore report on success

- [ ] **Step 4: Extend CLI parsing**

Add `--expected-preview-fingerprint <value>` to `restore-runtime-dir`.

- [ ] **Step 5: Extend text formatters**

Preview text must print the fingerprint.
Restore text must print the confirmed fingerprint when present.

### Task 3: Wire Scripts and Operator Guidance

**Files:**
- Modify: `bin/restore-runtime-local.ps1`
- Modify: `bin/restore-runtime-local.sh`
- Modify: `bin/status-local.ps1`
- Modify: `bin/status-local.sh`
- Modify: `services/local-minimal-node/tests/deployment_profile_test.rs`

- [ ] **Step 1: Add restore script parameter passthrough**

Wire PowerShell and bash restore wrappers to accept an optional expected preview fingerprint.

- [ ] **Step 2: Update status guidance**

Status scripts must explain the preferred operator loop:
- preview
- capture fingerprint
- restore with expected fingerprint

- [ ] **Step 3: Lock script contract in tests**

Extend deployment profile assertions for the new flag and guidance text.

### Task 4: Document the Standard

**Files:**
- Create: `docs/架构/111-local-minimal-runtime-dir-restore-preview-confirmation-standard-2026-04-06.md`
- Create: `docs/review/2026-04-06-local-minimal-runtime-dir-restore-preview-confirmation-review-cycle.md`
- Modify: `docs/架构/107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md`

- [ ] **Step 1: Write the architecture standard**

Document the confirmation seam, fingerprint semantics, non-goals, and zero-side-effect mismatch rule.

- [ ] **Step 2: Write the review-cycle document**

Capture why richer preview still needed an explicit restore handoff to avoid stale or mismatched operator decisions.

- [ ] **Step 3: Update Standard 107 composition notes**

Reference the new confirmation standard as the safety layer above preview.

### Task 5: Verify End to End

**Files:**
- Modify as required by earlier tasks only

- [ ] **Step 1: Run formatting**

Run: `cargo fmt --all`

- [ ] **Step 2: Verify formatting**

Run: `cargo fmt --all --check`
Expected: PASS

- [ ] **Step 3: Run targeted preview and restore tests**

Run:
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`

Expected: PASS

- [ ] **Step 4: Run package verification**

Run: `cargo test -p local-minimal-node --offline`
Expected: PASS

- [ ] **Step 5: Run script-level preview verification**

Run: `powershell -ExecutionPolicy Bypass -File bin\\preview-runtime-restore-local.ps1 -RuntimeDir .runtime\\local-minimal -BackupDir .runtime\\local-minimal\\backups\\runtime-dir-restore-1775463595576629400 -Json`
Expected: PASS with a stable `previewFingerprint`.
