# Local-Minimal Live Subscription Recovery Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make managed `local-minimal` rebuilds restore live realtime subscription intent for the same device after a fresh resume so private deployment no longer requires an extra `subscriptions.sync` after restart.

**Architecture:** Introduce a pluggable durable subscription store keyed by `tenant + principal + device`, bind it under the runtime dir for managed `local-minimal`, and lazily restore subscriptions during client route bootstrap paths such as `session.resume` or equivalent client route registration. Keep unmanaged/default builders memory-backed.

**Tech Stack:** Rust, Axum, serde/serde_json, local file persistence, existing realtime runtime and runtime-dir builder seams, existing device-scope checkpoint/disconnect-fence patterns.

---

### Task 1: Freeze Recovery Boundary And Red Tests

**Files:**
- Create: `docs/review/2026-04-06-local-minimal-live-subscription-recovery-review-cycle.md`
- Create: `docs/架构/96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md`
- Test: `services/session-gateway/tests/realtime_runtime_test.rs`
- Test: `services/local-minimal-node/tests/live_subscription_recovery_persistence_test.rs`

- [ ] **Step 1: Write the runtime red test**

Add a session-gateway test that:
- persists device subscriptions in a durable store
- rebuilds a fresh runtime with the same stores
- does not call `sync_subscriptions(...)` again
- publishes a matching event
- verifies delivery still occurs because subscriptions were restored automatically

- [ ] **Step 2: Run the runtime red test**

Run: `cargo test -p session-gateway --offline test_runtime_restores_persisted_subscriptions_on_rebuild_without_resync -- --nocapture`
Expected: FAIL because subscription persistence/recovery is not implemented yet.

- [ ] **Step 3: Write the local-minimal red test**

Add an end-to-end managed runtime-dir test that:
- creates a conversation
- syncs a device subscription once
- rebuilds the app with the same runtime dir
- performs a fresh `session.resume`
- posts a matching event without a new `subscriptions.sync`
- verifies the resumed device receives the live realtime event

- [ ] **Step 4: Run the local-minimal red test**

Run: `cargo test -p local-minimal-node --offline --test live_subscription_recovery_persistence_test -- --nocapture`
Expected: FAIL because live subscriptions are lost across rebuild.

### Task 2: Add A Pluggable Subscription Store Contract

**Files:**
- Modify: `crates/im-platform-contracts/Cargo.toml`
- Modify: `crates/im-platform-contracts/src/lib.rs`
- Modify: `adapters/local-memory/src/lib.rs`
- Modify: `adapters/local-disk/src/lib.rs`
- Test: `adapters/local-disk/src/lib.rs`

- [ ] **Step 1: Add the contract types**

Introduce a durable device-scope realtime subscription record and store trait.

- [ ] **Step 2: Add memory and file-backed adapters**

Requirements:
- device-scope load/save/clear
- file path under runtime state dir
- temp-file replace semantics for the file adapter

- [ ] **Step 3: Add file adapter regression coverage**

Run: `cargo test -p im-adapters-local-disk --offline test_file_subscription_store_persists_across_reopen -- --nocapture`
Expected: PASS after implementation.

### Task 3: Recover Subscriptions In Realtime Runtime

**Files:**
- Modify: `services/session-gateway/src/realtime.rs`
- Modify: `services/session-gateway/tests/realtime_runtime_test.rs`

- [ ] **Step 1: Extend runtime construction**

Support:
- checkpoint store injection
- subscription store injection
- default memory-backed fallback for unmanaged/test paths

- [ ] **Step 2: Restore subscriptions lazily**

Restore device-scope subscriptions when the runtime initializes client route state for an active client route bootstrap path.

Constraints:
- do not silently resurrect subscriptions after explicit disconnect if the store was cleared
- keep route/session fencing rules unchanged
- keep publish-path matching logic unchanged except for restored subscription availability

- [ ] **Step 3: Persist subscription mutations**

Persist on:
- `sync_subscriptions(...)`
- `clear_device_subscriptions(...)`
- `restore_client_route_state(...)` when route migration snapshots are restored

- [ ] **Step 4: Run the runtime test**

Run: `cargo test -p session-gateway --offline test_runtime_restores_persisted_subscriptions_on_rebuild_without_resync -- --nocapture`
Expected: PASS.

### Task 4: Wire Managed Local-Minimal Runtime-Dir Recovery

**Files:**
- Modify: `services/local-minimal-node/src/lib.rs`
- Modify: `services/local-minimal-node/tests/live_subscription_recovery_persistence_test.rs`

- [ ] **Step 1: Bind the runtime-dir store**

Managed runtime-dir builders must bind:
- `FileRealtimeCheckpointStore`
- `FileRealtimeDisconnectFenceStore`
- `FileRealtimeSubscriptionStore`

- [ ] **Step 2: Recover subscriptions through a fresh client route bootstrap**

Use the same managed runtime-dir and verify that a fresh resumed device gets restored live subscription matching without a second `subscriptions.sync`.

- [ ] **Step 3: Run the local-minimal test**

Run: `cargo test -p local-minimal-node --offline --test live_subscription_recovery_persistence_test -- --nocapture`
Expected: PASS.

### Task 5: Close The Review Loop

**Files:**
- Modify: `docs/review/2026-04-06-local-minimal-live-subscription-recovery-review-cycle.md`
- Modify: `docs/架构/96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md`
- Modify: `docs/架构/94-local-minimal-runtime-checkpoint-persistence-standard-2026-04-06.md`
- Modify: `docs/架构/95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md`

- [ ] **Step 1: Run focused verification**

Run:
- `cargo test -p session-gateway --offline`
- `cargo test -p im-adapters-local-disk --offline`
- `cargo test -p local-minimal-node --offline --test live_subscription_recovery_persistence_test -- --nocapture`

Expected: PASS.

- [ ] **Step 2: Run broader verification**

Run:
- `cargo fmt --all --check`
- `cargo test -p local-minimal-node --offline`

Expected: PASS.

- [ ] **Step 3: Record residual risk**

Document:
- what is now automatically restored
- what still requires explicit client/session re-bootstrap
- which runtime families remain memory-first
