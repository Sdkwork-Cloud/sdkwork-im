# Local-Minimal Runtime-Dir Backup Catalog Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a local backup catalog and preview workflow so operators can list managed runtime-dir snapshots and quickly tell whether a backup is empty, partial, or fully restorable before running restore.

**Architecture:** Keep backup cataloging as a read-only local operator seam. `local-minimal-node` will expose a `list-runtime-backups` command that scans `<runtime-dir>/backups`, summarizes each snapshot directory, extracts lightweight preview metadata from `repair-report.json` or `restore-report.json`, and reports restore readiness based on how many managed state files are present. Shell wrappers under `bin/` will expose the same capability for PowerShell, bash, and cmd operators.

**Tech Stack:** Rust, `std::fs`, `serde`/`serde_json`, existing runtime-dir view/report patterns, local lifecycle scripts, deployment asset tests.

---

### Task 1: Freeze Backup Catalog Scope

**Files:**
- Create: `docs/架构/106-local-minimal-runtime-dir-backup-catalog-standard-2026-04-06.md`
- Create: `docs/review/2026-04-06-local-minimal-runtime-dir-backup-catalog-review-cycle.md`
- Modify: `docs/架构/105-local-minimal-runtime-dir-backup-restore-standard-2026-04-06.md`

- [ ] **Step 1: Define the read-only operator boundary**

Lock this wave to:
- local listing and preview only
- no mutation
- no automatic restore selection
- no remote API exposure

- [ ] **Step 2: Freeze preview semantics**

Catalog must distinguish:
- `empty_snapshot`
- `partial_snapshot`
- `full_snapshot`


### Task 2: Add Red Tests For Backup Catalog

**Files:**
- Create: `services/local-minimal-node/tests/runtime_dir_backup_catalog_test.rs`
- Modify: `services/local-minimal-node/tests/deployment_profile_test.rs`

- [ ] **Step 1: Write a failing catalog test for mixed snapshot quality**

Expected:
- empty, partial, and full snapshots are all surfaced
- report metadata is previewed
- items are sorted newest-first

- [ ] **Step 2: Write a failing catalog test for a missing backups directory**

Expected:
- missing `<runtime-dir>/backups` is returned as an empty catalog
- no error or mutation occurs

- [ ] **Step 3: Extend deployment asset coverage**

Expected:
- `bin/list-runtime-backups-local.ps1`
- `bin/list-runtime-backups-local.sh`
- `bin/list-runtime-backups-local.cmd`
- status scripts reference listing guidance

- [ ] **Step 4: Run focused red tests**

Run:
- `cargo test -p local-minimal-node --offline --test runtime_dir_backup_catalog_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`


### Task 3: Implement Backup Catalog Runtime And CLI

**Files:**
- Modify: `services/local-minimal-node/src/lib.rs`
- Modify: `services/local-minimal-node/src/main.rs`

- [ ] **Step 1: Add catalog view types**

Include:
- overall catalog status
- runtime dir
- backups dir
- backup count
- per-backup preview items

- [ ] **Step 2: Implement read-only backup directory scanning**

Behavior:
- scan directory entries under `<runtime-dir>/backups`
- ignore non-directories
- classify operation from directory name
- count managed state files present

- [ ] **Step 3: Add preview metadata extraction**

Behavior:
- read `repair-report.json` or `restore-report.json` when present
- expose report type and report status
- compute snapshot quality from managed file count

- [ ] **Step 4: Add CLI command**

Add:
- `local-minimal-node list-runtime-backups [--runtime-dir <path>] [--json]`

- [ ] **Step 5: Run focused tests to turn green**

Run:
- `cargo test -p local-minimal-node --offline --test runtime_dir_backup_catalog_test -- --nocapture`


### Task 4: Add Local Lifecycle Entry Points

**Files:**
- Create: `bin/list-runtime-backups-local.ps1`
- Create: `bin/list-runtime-backups-local.sh`
- Create: `bin/list-runtime-backups-local.cmd`
- Modify: `bin/status-local.ps1`
- Modify: `bin/status-local.sh`

- [ ] **Step 1: Add direct listing scripts**

Scripts must:
- resolve runtime dir from config or explicit override
- invoke `list-runtime-backups`
- support `--json` / `-Json`

- [ ] **Step 2: Update operator status guidance**

Status scripts should mention:
- inspect
- repair
- list backups
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
- `cargo test -p local-minimal-node --offline --test runtime_dir_backup_catalog_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo test -p local-minimal-node --offline`

- [ ] **Step 4: Run script-level verification**

Run:
- `powershell -ExecutionPolicy Bypass -File bin\\list-runtime-backups-local.ps1 -RuntimeDir .runtime\\local-minimal -Json`

- [ ] **Step 5: Record remaining out-of-scope work**

Capture:
- backup diff/preview details beyond summary metadata
- operator-guided restore confirmations
- remote authenticated backup catalog APIs
