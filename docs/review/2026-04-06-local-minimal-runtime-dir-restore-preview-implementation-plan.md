# Local-Minimal Runtime-Dir Restore Preview Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a read-only restore preview workflow so operators can dry-run a chosen backup snapshot and see which managed files would be restored, skipped, or left unchanged before executing an actual restore.

**Architecture:** Keep restore preview as a separate local operator seam. `local-minimal-node` will expose a `preview-runtime-restore` command that validates the selected backup snapshot, compares its managed `state/` files against the current runtime `state/` directory, returns a structured per-file preview, and performs no mutation. Lifecycle wrappers under `bin/` will expose the same capability for PowerShell, bash, and cmd operators.

**Tech Stack:** Rust, `std::fs`, `serde`/`serde_json`, existing runtime-dir inspection and backup summary helpers, local lifecycle scripts, deployment asset tests.

---

### Task 1: Freeze Restore Preview Scope

**Files:**
- Create: `docs/架构/107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md`
- Create: `docs/review/2026-04-06-local-minimal-runtime-dir-restore-preview-review-cycle.md`
- Modify: `docs/架构/105-local-minimal-runtime-dir-backup-restore-standard-2026-04-06.md`
- Modify: `docs/架构/106-local-minimal-runtime-dir-backup-catalog-standard-2026-04-06.md`

- [ ] **Step 1: Define the dry-run operator boundary**

Lock this wave to:
- local preview only
- explicit `--backup-dir` selection
- no filesystem mutation
- no restore execution side effects

- [ ] **Step 2: Freeze preview action semantics**

Preview must distinguish:
- `would_restore`
- `noop`
- `skip`


### Task 2: Add Red Tests For Restore Preview

**Files:**
- Create: `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`
- Modify: `services/local-minimal-node/tests/deployment_profile_test.rs`

- [ ] **Step 1: Write a failing test for a full snapshot preview**

Expected:
- preview reports `ready`
- differing files are marked `would_restore`
- identical files are marked `noop`
- runtime-dir state is not mutated

- [ ] **Step 2: Write a failing test for a sparse snapshot preview**

Expected:
- preview reports `partial`
- missing source files are marked `skip`
- source snapshot quality is surfaced

- [ ] **Step 3: Write a failing test for invalid backup input**

Expected:
- missing backup dir fails with a controlled error
- runtime-dir remains untouched

- [ ] **Step 4: Extend deployment asset coverage**

Expected:
- `bin/preview-runtime-restore-local.ps1`
- `bin/preview-runtime-restore-local.sh`
- `bin/preview-runtime-restore-local.cmd`
- status scripts reference preview guidance

- [ ] **Step 5: Run focused red tests**

Run:
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`


### Task 3: Implement Restore Preview Runtime And CLI

**Files:**
- Modify: `services/local-minimal-node/src/lib.rs`
- Modify: `services/local-minimal-node/src/main.rs`

- [ ] **Step 1: Add preview view types**

Include:
- overall preview status
- runtime dir
- source backup dir
- source snapshot summary
- before inspection view
- per-file preview actions

- [ ] **Step 2: Reuse backup validation and summary helpers**

Behavior:
- validate source backup dir exists
- validate source `state/` dir exists
- reuse backup summary metadata where possible

- [ ] **Step 3: Compare source and target managed files without mutation**

Behavior:
- `would_restore` when source exists and target differs or is missing
- `noop` when source and target content match
- `skip` when source file is absent in the selected snapshot

- [ ] **Step 4: Add CLI command**

Add:
- `local-minimal-node preview-runtime-restore --backup-dir <path> [--runtime-dir <path>] [--json]`

- [ ] **Step 5: Run focused tests to turn green**

Run:
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`


### Task 4: Add Local Lifecycle Entry Points

**Files:**
- Create: `bin/preview-runtime-restore-local.ps1`
- Create: `bin/preview-runtime-restore-local.sh`
- Create: `bin/preview-runtime-restore-local.cmd`
- Modify: `bin/status-local.ps1`
- Modify: `bin/status-local.sh`

- [ ] **Step 1: Add direct preview scripts**

Scripts must:
- resolve runtime dir from config or explicit override
- require explicit backup-dir input
- invoke `preview-runtime-restore`
- support `--json` / `-Json`

- [ ] **Step 2: Update operator status guidance**

Status scripts should mention:
- inspect
- repair
- list backups
- preview restore
- restore

- [ ] **Step 3: Re-run deployment asset coverage**

Run:
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`


### Task 5: Final Verification

**Files:**
- Modify: only files changed by Tasks 1-4

- [ ] **Step 1: Run formatting**

Run: `cargo fmt --all`

- [ ] **Step 2: Run format verification**

Run: `cargo fmt --all --check`

- [ ] **Step 3: Run focused and broad verification**

Run:
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo test -p local-minimal-node --offline`

- [ ] **Step 4: Run script-level verification**

Run:
- `powershell -ExecutionPolicy Bypass -File bin\\preview-runtime-restore-local.ps1 -RuntimeDir .runtime\\local-minimal -BackupDir <path> -Json`

- [ ] **Step 5: Record remaining out-of-scope work**

Capture:
- file-level diff bodies
- operator confirm/apply workflows
- remote authenticated preview APIs
