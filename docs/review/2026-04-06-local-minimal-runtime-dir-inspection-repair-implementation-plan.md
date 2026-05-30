# Local-Minimal Runtime-Dir Inspection Repair Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a restart-safe operator inspection surface for managed `local-minimal` runtime-dir deployments so missing or malformed persisted state files are visible through APIs, local scripts, and documentation before they become silent private-deployment failures.

**Architecture:** Keep runtime-dir durability pluggable and read-only in this wave. `ops-service` defines the inspection response contract, `local-minimal-node` computes inspection state for managed runtime-dir deployments, and lifecycle scripts expose the same capability for local operators. Do not implement destructive or speculative data repair in this wave; only classify file health and recommended next action.

**Tech Stack:** Rust, Axum, serde/serde_json, existing `ops-service` / `local-minimal-node` composition seams, existing lifecycle scripts under `bin/`, Markdown standards under `docs/review` and `docs/架构`.

---

### Task 1: Freeze Inspection Contract And Failure Model

**Files:**
- Create: `docs/架构/102-local-minimal-runtime-dir-inspection-repair-standard-2026-04-06.md`
- Modify: `services/ops-service/src/lib.rs`
- Modify: `services/local-minimal-node/src/lib.rs`
- Test: `services/ops-service/tests/http_smoke_test.rs`

- [ ] **Step 1: Define the inspection response contract**

Lock a single operator-facing response shape covering:
- runtime dir path
- state dir path
- overall status
- per-file inspection items
- counts for healthy / missing / corrupt files
- recommended action per item

- [ ] **Step 2: Add the failing API contract test**

Add a red test proving `/backend/v3/api/ops/runtime_dir` exists and requires `ops.read`.

- [ ] **Step 3: Implement the minimal contract in `ops-service`**

Add the new response/view structs and route wiring without coupling `ops-service` to filesystem logic.

- [ ] **Step 4: Verify the new ops contract is green**

Run: `cargo test -p ops-service --offline --test http_smoke_test -- --nocapture`


### Task 2: Add Managed Runtime-Dir Inspection Logic

**Files:**
- Create: `services/local-minimal-node/tests/runtime_dir_inspection_test.rs`
- Modify: `services/local-minimal-node/src/lib.rs`
- Modify: `services/local-minimal-node/src/main.rs`

- [ ] **Step 1: Write the failing managed runtime-dir inspection test**

Cover at least:
- healthy managed runtime-dir with all known state files present and parseable
- missing file reported as degraded
- malformed JSON file reported as corrupt

- [ ] **Step 2: Run the red test and verify expected failure**

Run: `cargo test -p local-minimal-node --offline --test runtime_dir_inspection_test -- --nocapture`

- [ ] **Step 3: Implement shared inspection logic**

Add a read-only runtime-dir inspector that checks:
- `<runtime-dir>/state/commit-journal.json`
- `<runtime-dir>/state/realtime-disconnect-fences.json`
- `<runtime-dir>/state/realtime-checkpoints.json`
- `<runtime-dir>/state/realtime-subscriptions.json`
- `<runtime-dir>/state/presence-state.json`
- `<runtime-dir>/state/stream-state.json`
- `<runtime-dir>/state/rtc-state.json`
- `<runtime-dir>/state/notification-tasks.json`
- `<runtime-dir>/state/automation-executions.json`

The inspector must classify each file as `ok`, `missing`, or `corrupt`.

- [ ] **Step 4: Surface the inspection result through both runtime and CLI**

Expose:
- `/backend/v3/api/ops/runtime_dir`
- a binary entry path for local script use

- [ ] **Step 5: Re-run the managed runtime-dir test**

Run: `cargo test -p local-minimal-node --offline --test runtime_dir_inspection_test -- --nocapture`


### Task 3: Wire Cross-Platform Local Operator Scripts

**Files:**
- Create: `bin/inspect-runtime-local.ps1`
- Create: `bin/inspect-runtime-local.sh`
- Create: `bin/inspect-runtime-local.cmd`
- Modify: `bin/_cmd-forward-powershell.cmd`
- Modify: `bin/status-local.ps1`
- Modify: `bin/status-local.sh`
- Modify: `services/local-minimal-node/tests/deployment_profile_test.rs`

- [ ] **Step 1: Add the failing deployment asset test**

Extend the deployment asset contract so the new inspection scripts and documented switches are required.

- [ ] **Step 2: Run the deployment contract red test**

Run: `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`

- [ ] **Step 3: Implement inspection scripts**

Scripts must:
- resolve configured runtime dir
- locate an existing binary or fall back to `cargo run`
- call the runtime-dir inspection entrypoint
- work on Windows, PowerShell, and POSIX shells

- [ ] **Step 4: Update status scripts to point operators at the new inspection surface**

Keep `status-local.*` lightweight, but ensure they document or reference the deeper runtime-dir inspection step.

- [ ] **Step 5: Re-run the deployment contract**

Run: `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`


### Task 4: Write Review Record And Standard

**Files:**
- Create: `docs/review/2026-04-06-local-minimal-runtime-dir-inspection-repair-review-cycle.md`
- Create: `docs/架构/102-local-minimal-runtime-dir-inspection-repair-standard-2026-04-06.md`
- Modify: `docs/架构/95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md`
- Modify: `docs/架构/99-local-minimal-notification-runtime-persistence-standard-2026-04-06.md`
- Modify: `docs/架构/100-local-minimal-automation-runtime-persistence-standard-2026-04-06.md`
- Modify: `docs/架构/101-local-minimal-presence-runtime-persistence-standard-2026-04-06.md`

- [ ] **Step 1: Record findings, root cause, and repair scope**

Explain why the previous persistence waves still left operators blind to runtime-dir drift.

- [ ] **Step 2: Freeze the inspection standard**

Document:
- required managed files
- status model
- endpoint contract
- script entrypoints
- repair boundary for this wave

- [ ] **Step 3: Update prior standards’ residual-risk sections**

Replace the old generic “future inspection tooling” references with the new standard where appropriate.


### Task 5: Final Verification

**Files:**
- Modify: only files changed by Tasks 1-4

- [ ] **Step 1: Run formatting**

Run: `cargo fmt --all --check`

- [ ] **Step 2: Run focused crate tests**

Run:
- `cargo test -p ops-service --offline`
- `cargo test -p local-minimal-node --offline --test runtime_dir_inspection_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`

- [ ] **Step 3: Run broader local-minimal verification**

Run: `cargo test -p local-minimal-node --offline`

- [ ] **Step 4: Write the review-cycle outcome**

Capture:
- issues fixed
- residual risks
- next recommended wave
