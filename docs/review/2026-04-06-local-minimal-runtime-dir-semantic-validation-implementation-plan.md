# Local-Minimal Runtime-Dir Semantic Validation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Upgrade managed `local-minimal` runtime-dir inspection from generic JSON parseability to typed semantic validation so operator tooling can detect structurally valid but operationally invalid persistence files before restart or repair.

**Architecture:** Keep Standard 102's inspection surface stable while strengthening how each file is validated. The local-disk adapter layer provides typed validation for each file family, and `local-minimal-node` uses those validators when building `/backend/v3/api/ops/runtime_dir` and CLI inspection output. `commit-journal.json` receives a deeper validation path by loading envelopes and replay-checking them against the same runtime semantics that startup depends on.

**Tech Stack:** Rust, Axum, serde/serde_json, existing `im-adapters-local-disk` typed record formats, `conversation-runtime`, `projection-service`, existing runtime-dir inspection endpoint and script surfaces.

---

### Task 1: Freeze Semantic Validation Scope

**Files:**
- Create: `docs/架构/103-local-minimal-runtime-dir-semantic-validation-standard-2026-04-06.md`
- Create: `docs/review/2026-04-06-local-minimal-runtime-dir-semantic-validation-review-cycle.md`

- [ ] **Step 1: Define what “semantic validation” means in this wave**

Lock the boundary:
- typed file-format validation for every managed runtime-dir file
- journal replay validation for `commit-journal.json`
- no automatic mutation or auto-repair in this wave

- [ ] **Step 2: Record the operator-facing outcome**

Document that Standard 102’s contract remains stable while validation becomes stricter.


### Task 2: Add Red Tests For Semantic Validation

**Files:**
- Modify: `services/local-minimal-node/tests/runtime_dir_inspection_test.rs`

- [ ] **Step 1: Write a failing test for typed file-shape validation**

Example:
- `realtime-checkpoints.json` or `presence-state.json` contains a JSON array instead of the expected map
- inspection must mark that file as `corrupt`

- [ ] **Step 2: Write a failing test for journal replay validation**

Example:
- `commit-journal.json` contains a valid envelope array but invalid replay order such as `message.posted` before `conversation.created`
- inspection must mark `commit-journal.json` as `corrupt`

- [ ] **Step 3: Run the focused red test**

Run: `cargo test -p local-minimal-node --offline --test runtime_dir_inspection_test -- --nocapture`


### Task 3: Implement Typed Validators In Local-Disk Adapter Layer

**Files:**
- Modify: `adapters/local-disk/src/lib.rs`
- Modify: `services/local-minimal-node/src/lib.rs`

- [ ] **Step 1: Add public typed validation helpers in `im-adapters-local-disk`**

Each managed file must be validated against its real stored type:
- `Vec<CommitEnvelope>`
- `BTreeMap<String, RealtimeDisconnectFenceRecord>`
- `BTreeMap<String, RealtimeCheckpointRecord>`
- `BTreeMap<String, RealtimeSubscriptionRecord>`
- `BTreeMap<String, PresenceStateRecord>`
- `BTreeMap<String, StreamStateRecord>`
- `BTreeMap<String, RtcStateRecord>`
- `BTreeMap<String, NotificationTaskRecord>`
- `BTreeMap<String, AutomationExecutionRecord>`

- [ ] **Step 2: Add journal replay validation**

Use the same runtime assumptions as startup replay to reject:
- invalid envelope payload shape
- impossible event order
- references to missing conversations during replay

- [ ] **Step 3: Switch `local-minimal-node` inspection to typed validation**

Replace generic `serde_json::Value` validation with adapter-backed semantic validation results.

- [ ] **Step 4: Re-run the focused test**

Run: `cargo test -p local-minimal-node --offline --test runtime_dir_inspection_test -- --nocapture`


### Task 4: Extend Broader Regression Coverage

**Files:**
- Modify: `services/ops-service/tests/http_smoke_test.rs`
- Modify: `services/local-minimal-node/tests/deployment_profile_test.rs`

- [ ] **Step 1: Verify the endpoint contract remains stable**

Keep `/backend/v3/api/ops/runtime_dir` and lifecycle scripts backward-compatible.

- [ ] **Step 2: Run targeted regression commands**

Run:
- `cargo test -p ops-service --offline --test http_smoke_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`


### Task 5: Final Verification

**Files:**
- Modify: only files changed by Tasks 1-4

- [ ] **Step 1: Run formatting**

Run: `cargo fmt --all`

- [ ] **Step 2: Run format verification**

Run: `cargo fmt --all --check`

- [ ] **Step 3: Run broad crate verification**

Run:
- `cargo test -p ops-service --offline`
- `cargo test -p local-minimal-node --offline`

- [ ] **Step 4: Run script-level inspection verification**

Run:
- `powershell -ExecutionPolicy Bypass -File bin\\inspect-runtime-local.ps1 -RuntimeDir .runtime\\local-minimal -Json`

- [ ] **Step 5: Record residual risk and next wave**

Capture what still remains out of scope:
- safe repair workflows
- cross-node drift reconciliation
- durable route failover reconstruction
