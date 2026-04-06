# Local-Minimal Runtime-Dir Safe Repair Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a backup-first local repair workflow for managed `local-minimal` runtime directories that can recreate missing state files safely without mutating corrupt files.

**Architecture:** Keep inspection read-only and introduce a separate local repair path in `local-minimal-node` plus `bin/repair-runtime-local.*`. The repair workflow snapshots the current managed state into a timestamped backup directory, recreates only files currently classified as `missing`, leaves `corrupt` files untouched, and returns a structured repair report. No remote HTTP repair endpoint is added in this wave.

**Tech Stack:** Rust, std::fs, serde/serde_json, existing `inspect_runtime_dir(...)` classification logic, local lifecycle scripts for PowerShell/bash/cmd, existing deployment-profile tests.

---

### Task 1: Freeze Safe Repair Scope

**Files:**
- Create: `docs/架构/104-local-minimal-runtime-dir-safe-repair-standard-2026-04-06.md`
- Create: `docs/review/2026-04-06-local-minimal-runtime-dir-safe-repair-review-cycle.md`
- Modify: `docs/架构/103-local-minimal-runtime-dir-semantic-validation-standard-2026-04-06.md`

- [ ] **Step 1: Define the repair boundary**

Lock this wave to:
- local operator entrypoints only
- backup-first repair
- recreate missing managed files only
- no automatic mutation of `corrupt` files

- [ ] **Step 2: Record operator semantics**

Document:
- where backups are stored
- what report shape is returned
- why remote HTTP repair stays out of scope for this wave


### Task 2: Add Red Tests For Safe Repair

**Files:**
- Create: `services/local-minimal-node/tests/runtime_dir_repair_test.rs`
- Modify: `services/local-minimal-node/tests/deployment_profile_test.rs`

- [ ] **Step 1: Write a failing test for repairing a fully missing managed runtime-dir**

Expected:
- repair creates all 9 required files with typed-empty content
- post-repair inspection reports `ok`
- a backup directory and manifest/report are created

- [ ] **Step 2: Write a failing test proving corrupt files are not auto-rewritten**

Expected:
- if `rtc-state.json` is corrupt and `presence-state.json` is missing
- repair recreates only `presence-state.json`
- `rtc-state.json` remains `corrupt`
- final inspection remains `degraded`

- [ ] **Step 3: Extend deployment asset assertions**

Expected:
- `bin/repair-runtime-local.ps1`
- `bin/repair-runtime-local.sh`
- `bin/repair-runtime-local.cmd`
- lifecycle/status scripts reference the repair step when appropriate

- [ ] **Step 4: Run the focused red tests**

Run:
- `cargo test -p local-minimal-node --offline --test runtime_dir_repair_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`


### Task 3: Implement Repair Runtime And CLI

**Files:**
- Modify: `services/local-minimal-node/src/lib.rs`
- Modify: `services/local-minimal-node/src/main.rs`

- [ ] **Step 1: Add repair report types and helpers**

Implement a structured repair result that includes:
- runtime dir
- backup dir
- before/after inspection
- per-file repair actions

- [ ] **Step 2: Add backup-first repair logic**

Behavior:
- create timestamped backup root under `<runtime-dir>/backups/...`
- snapshot existing managed state files before any mutation
- write a manifest/report for operator traceability
- recreate only files classified as `missing`

- [ ] **Step 3: Preserve corrupt-file fail-closed behavior**

Behavior:
- do not rewrite files already classified as `corrupt`
- report them as skipped with a manual-repair recommendation

- [ ] **Step 4: Add CLI command**

Add:
- `local-minimal-node repair-runtime-dir --runtime-dir <path> [--json]`

- [ ] **Step 5: Run focused tests to turn green**

Run:
- `cargo test -p local-minimal-node --offline --test runtime_dir_repair_test -- --nocapture`


### Task 4: Add Local Lifecycle Entry Points

**Files:**
- Create: `bin/repair-runtime-local.ps1`
- Create: `bin/repair-runtime-local.sh`
- Create: `bin/repair-runtime-local.cmd`
- Modify: `bin/status-local.ps1`
- Modify: `bin/status-local.sh`
- Modify: `bin/_cmd-forward-powershell.cmd`

- [ ] **Step 1: Add direct repair scripts**

Scripts must:
- resolve runtime dir from config or explicit override
- invoke `repair-runtime-dir`
- support `--json` / `-Json`
- reuse existing debug/release binary lookup conventions

- [ ] **Step 2: Wire cmd forwarding**

Ensure `.cmd` entrypoints normalize the documented switches without hardcoding a separate PowerShell launch path.

- [ ] **Step 3: Point operator status output at repair**

Status scripts may stay lightweight but should tell operators how to run the explicit repair step after inspection.

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
- `cargo test -p local-minimal-node --offline --test runtime_dir_repair_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo test -p local-minimal-node --offline`

- [ ] **Step 4: Run script-level verification**

Run:
- `powershell -ExecutionPolicy Bypass -File bin\\repair-runtime-local.ps1 -RuntimeDir .runtime\\local-minimal -Json`
- `powershell -ExecutionPolicy Bypass -File bin\\inspect-runtime-local.ps1 -RuntimeDir .runtime\\local-minimal -Json`

- [ ] **Step 5: Record remaining out-of-scope work**

Capture:
- corrupt-file guided repair flows
- selective restore from backups
- remote authenticated ops repair APIs
