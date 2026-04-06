# Local-Minimal Runtime-Dir Backup Restore Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a backup-snapshot restore workflow for managed `local-minimal` runtime directories so operators can restore state files from a chosen local backup snapshot with a restore-before-overwrite safety backup.

**Architecture:** Keep restore as a separate local operator seam, not part of inspection or public HTTP APIs. `local-minimal-node` will expose a restore function and CLI subcommand that validates the source backup directory, snapshots the current runtime state into a new restore backup, copies available managed state files from the selected backup snapshot into the runtime `state/` directory, then returns a structured restore report with before/after inspection views.

**Tech Stack:** Rust, std::fs, serde/serde_json, existing runtime-dir inspection and repair report patterns, existing local lifecycle scripts for PowerShell/bash/cmd, deployment-profile script coverage.

---

### Task 1: Freeze Restore Scope

**Files:**
- Create: `docs/架构/105-local-minimal-runtime-dir-backup-restore-standard-2026-04-06.md`
- Create: `docs/review/2026-04-06-local-minimal-runtime-dir-backup-restore-review-cycle.md`
- Modify: `docs/架构/104-local-minimal-runtime-dir-safe-repair-standard-2026-04-06.md`

- [ ] **Step 1: Define the restore boundary**

Lock this wave to:
- local operator entrypoints only
- explicit `--backup-dir` selection
- restore-before-overwrite backup creation
- file-level restore from a selected backup snapshot

- [ ] **Step 2: Record non-goals**

Keep out of scope:
- remote HTTP restore APIs
- automatic backup selection heuristics
- operator-guided merge/conflict resolution across snapshots


### Task 2: Add Red Tests For Restore

**Files:**
- Create: `services/local-minimal-node/tests/runtime_dir_restore_test.rs`
- Modify: `services/local-minimal-node/tests/deployment_profile_test.rs`

- [ ] **Step 1: Write a failing test for restoring a chosen snapshot over current state**

Expected:
- restore copies managed files from the selected backup snapshot
- current state is backed up before overwrite
- post-restore inspection reflects the restored snapshot

- [ ] **Step 2: Write a failing test for invalid backup input**

Expected:
- restoring from a missing backup dir fails with a controlled error
- current runtime-dir is not mutated

- [ ] **Step 3: Extend deployment asset coverage**

Expected:
- `bin/restore-runtime-local.ps1`
- `bin/restore-runtime-local.sh`
- `bin/restore-runtime-local.cmd`
- `_cmd-forward-powershell.cmd` forwards `--backup-dir`

- [ ] **Step 4: Run the focused red tests**

Run:
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`


### Task 3: Implement Restore Runtime And CLI

**Files:**
- Modify: `services/local-minimal-node/src/lib.rs`
- Modify: `services/local-minimal-node/src/main.rs`

- [ ] **Step 1: Add restore report types and helpers**

Implement a structured restore result containing:
- runtime dir
- source backup dir
- pre-restore backup dir
- before/after inspection
- per-file restore actions

- [ ] **Step 2: Add restore-before-overwrite backup logic**

Behavior:
- validate source backup dir exists
- create timestamped restore backup under `<runtime-dir>/backups/...`
- snapshot current runtime state before any overwrite

- [ ] **Step 3: Restore managed state files from the selected snapshot**

Behavior:
- copy available managed files from `<backup-dir>/state/`
- do not invent source files that do not exist in the selected snapshot
- re-run inspection after restore

- [ ] **Step 4: Add CLI command**

Add:
- `local-minimal-node restore-runtime-dir --runtime-dir <path> --backup-dir <path> [--json]`

- [ ] **Step 5: Run focused tests to turn green**

Run:
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_test -- --nocapture`


### Task 4: Add Local Lifecycle Entry Points

**Files:**
- Create: `bin/restore-runtime-local.ps1`
- Create: `bin/restore-runtime-local.sh`
- Create: `bin/restore-runtime-local.cmd`
- Modify: `bin/status-local.ps1`
- Modify: `bin/status-local.sh`
- Modify: `bin/_cmd-forward-powershell.cmd`

- [ ] **Step 1: Add direct restore scripts**

Scripts must:
- resolve runtime dir from config or explicit override
- require explicit backup-dir input
- invoke `restore-runtime-dir`
- support `--json` / `-Json`

- [ ] **Step 2: Wire cmd forwarding**

Ensure `.cmd` entrypoints support:
- `/backupDir`
- `--backup-dir`

- [ ] **Step 3: Update operator status guidance**

Status scripts may stay lightweight but should mention:
- inspect
- repair
- restore

- [ ] **Step 4: Re-run deployment asset coverage**

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
- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo test -p local-minimal-node --offline`

- [ ] **Step 4: Run script-level verification**

Run:
- `powershell -ExecutionPolicy Bypass -File bin\\restore-runtime-local.ps1 -RuntimeDir .runtime\\local-minimal -BackupDir <path> -Json`
- `powershell -ExecutionPolicy Bypass -File bin\\inspect-runtime-local.ps1 -RuntimeDir .runtime\\local-minimal -Json`

- [ ] **Step 5: Record remaining out-of-scope work**

Capture:
- guided corrupt-file remediation
- backup snapshot listing/indexing
- remote authenticated control-plane restore workflows
