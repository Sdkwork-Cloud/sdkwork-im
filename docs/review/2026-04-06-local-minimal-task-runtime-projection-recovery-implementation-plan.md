# Local-Minimal Task Runtime Projection Recovery Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make managed `local-minimal` rebuilds restore notification task projections and automation execution projections from the runtime dir so private deployment can continue querying notification and automation state after restart.

**Architecture:** Introduce pluggable stores for notification tasks and automation executions, persist their final projection state after each successful mutation, restore them lazily inside the corresponding runtimes, and bind file-backed stores in managed `local-minimal` runtime-dir builders. Keep unmanaged/default builders memory-backed.

**Tech Stack:** Rust, Axum, serde/serde_json, local file persistence, `notification-service`, `automation-service`, `local-minimal-node`, existing `im-platform-contracts` adapter model.

---

### Task 1: Freeze The Restart Gaps With Red Tests

**Files:**
- Create: `services/notification-service/tests/notification_runtime_persistence_test.rs`
- Create: `services/automation-service/tests/automation_runtime_persistence_test.rs`
- Create: `services/local-minimal-node/tests/task_runtime_projection_persistence_test.rs`

- [ ] Add a `notification-service` rebuild test using a shared durable store and verify `list_notifications(...)` still returns a pre-restart notification after rebuild.
- [ ] Add an `automation-service` rebuild test using a shared durable store and verify `get_execution(...)` still returns a pre-restart execution after rebuild.
- [ ] Add a managed `local-minimal` runtime-dir rebuild test proving both `/im/v3/api/notifications` and `/im/v3/api/automation/executions/{id}` continue working after restart.
- [ ] Run the focused tests and capture the red state before runtime changes.

### Task 2: Add Projection Store Contracts And Adapters

**Files:**
- Modify: `crates/im-platform-contracts/src/lib.rs`
- Modify: `adapters/local-memory/src/lib.rs`
- Modify: `adapters/local-disk/src/lib.rs`

- [ ] Add `NotificationTaskRecord` and `NotificationTaskStore`.
- [ ] Add `AutomationExecutionRecord` and `AutomationExecutionStore`.
- [ ] Add `MemoryNotificationTaskStore` and `MemoryAutomationExecutionStore`.
- [ ] Add `FileNotificationTaskStore` and `FileAutomationExecutionStore`.
- [ ] Add file adapter reopen regression tests for both stores.

### Task 3: Recover Notification And Automation Projections In Runtime

**Files:**
- Modify: `services/notification-service/src/lib.rs`
- Modify: `services/notification-service/Cargo.toml`
- Modify: `services/automation-service/src/lib.rs`
- Modify: `services/automation-service/Cargo.toml`

- [ ] Add constructors that accept both journal and projection store dependencies.
- [ ] Lazily restore a notification projection on direct lookup and recipient-scope list queries.
- [ ] Lazily restore an automation execution projection on direct lookup and idempotent request paths.
- [ ] Persist the final projected notification task after request dispatch succeeds.
- [ ] Persist the final projected automation execution after request completion succeeds.
- [ ] Surface projection-store failures as controlled `*_store_*` runtime errors.

### Task 4: Wire Managed Local-Minimal Runtime-Dir Projection Recovery

**Files:**
- Modify: `services/local-minimal-node/src/lib.rs`
- Modify: `services/local-minimal-node/tests/task_runtime_projection_persistence_test.rs`

- [ ] Bind `FileNotificationTaskStore` under `<runtime-dir>/state/notification-tasks.json`.
- [ ] Bind `FileAutomationExecutionStore` under `<runtime-dir>/state/automation-executions.json`.
- [ ] Ensure managed builders compose notification and automation durability with existing domain, realtime, stream, and RTC recovery seams.
- [ ] Re-run the managed restart regression.

### Task 5: Close The Review Loop

**Files:**
- Create: `docs/review/2026-04-06-local-minimal-task-runtime-projection-recovery-review-cycle.md`
- Create: `docs/架构/99-local-minimal-notification-runtime-persistence-standard-2026-04-06.md`
- Create: `docs/架构/100-local-minimal-automation-runtime-persistence-standard-2026-04-06.md`
- Modify: `docs/架构/95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md`
- Modify: `docs/架构/98-local-minimal-rtc-runtime-persistence-standard-2026-04-06.md`

- [ ] Record findings, root cause, implementation, and verification.
- [ ] Freeze runtime-dir file paths and composition rules.
- [ ] Document the next residual private-deployment durability boundary after notification and automation recovery.
