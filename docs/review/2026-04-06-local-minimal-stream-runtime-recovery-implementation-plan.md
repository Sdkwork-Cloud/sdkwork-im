# Local-Minimal Stream Runtime Recovery Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make managed `local-minimal` rebuilds restore stream runtime state from the runtime dir so stream list / checkpoint / complete / abort flows continue after restart without reopening the stream.

**Architecture:** Introduce a pluggable `StreamStateStore` keyed by `tenant + stream_id`, provide memory and local-file adapters, restore stream state lazily inside `StreamingRuntime`, and bind a file-backed store in managed `local-minimal` runtime-dir builders. Keep unmanaged builders memory-backed.

**Tech Stack:** Rust, Axum, serde/serde_json, local file persistence, `streaming-service`, runtime-dir builder seams, existing `im-platform-contracts` adapter model.

---

### Task 1: Freeze Red Tests

**Files:**
- Create: `services/local-minimal-node/tests/stream_runtime_persistence_test.rs`
- Modify: `services/streaming-service/tests/stream_lifecycle_test.rs`

- [ ] Add a `streaming-service` rebuild test using a shared in-memory store.
- [ ] Add a managed `local-minimal` runtime-dir restart test proving stream list/complete works without reopening the stream.
- [ ] Run the focused tests and capture the red state.

### Task 2: Add Stream State Store Contract And Adapters

**Files:**
- Modify: `crates/im-platform-contracts/src/lib.rs`
- Modify: `adapters/local-memory/src/lib.rs`
- Modify: `adapters/local-disk/src/lib.rs`

- [ ] Add `StreamStateRecord` and `StreamStateStore`.
- [ ] Add `MemoryStreamStateStore`.
- [ ] Add `FileStreamStateStore`.
- [ ] Add a file adapter reopen regression test.

### Task 3: Recover Stream State In Runtime

**Files:**
- Modify: `services/streaming-service/src/lib.rs`
- Modify: `services/streaming-service/Cargo.toml`

- [ ] Add `StreamingRuntime::with_store(...)`.
- [ ] Lazily restore stream state on access paths.
- [ ] Persist stream state after every mutating operation.
- [ ] Surface store failures as controlled `stream_store_*` errors.

### Task 4: Wire Managed Local-Minimal Runtime-Dir Stream Recovery

**Files:**
- Modify: `services/local-minimal-node/src/lib.rs`
- Modify: `services/local-minimal-node/tests/stream_runtime_persistence_test.rs`

- [ ] Bind `FileStreamStateStore` under `<runtime-dir>/state/stream-state.json`.
- [ ] Ensure managed builders compose the durable stream runtime with existing domain + realtime recovery.
- [ ] Re-run the managed restart regression.

### Task 5: Close The Review Loop

**Files:**
- Create: `docs/review/2026-04-06-local-minimal-stream-runtime-recovery-review-cycle.md`
- Create: `docs/架构/97-local-minimal-stream-runtime-persistence-standard-2026-04-06.md`
- Modify: `docs/架构/94-local-minimal-runtime-checkpoint-persistence-standard-2026-04-06.md`
- Modify: `docs/架构/95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md`
- Modify: `docs/架构/96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md`

- [ ] Record findings, root cause, implementation, and verification.
- [ ] Freeze the runtime-dir file path and composition rules.
- [ ] Document the next residual gap after stream recovery.
