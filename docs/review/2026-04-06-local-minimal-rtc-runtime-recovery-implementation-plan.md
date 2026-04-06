# Local-Minimal RTC Runtime Recovery Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make managed `local-minimal` rebuilds restore RTC session runtime state from the runtime dir so `invite` / `accept` / `reject` / `end` / `signals` flows continue after restart without recreating the same `rtcSessionId`.

**Architecture:** Introduce a pluggable `RtcStateStore` keyed by `tenant + rtc_session_id`, persist the full `RtcSession` plus ordered `RtcSignalEvent[]`, restore RTC state lazily inside `RtcRuntime`, and bind a file-backed store in managed `local-minimal` runtime-dir builders. Keep unmanaged/default builders memory-backed.

**Tech Stack:** Rust, Axum, serde/serde_json, local file persistence, `rtc-signaling-service`, `local-minimal-node`, existing `im-platform-contracts` adapter model, runtime-dir deployment profile.

---

### Task 1: Freeze The Restart Gap With Red Tests

**Files:**
- Create: `services/local-minimal-node/tests/rtc_runtime_persistence_test.rs`
- Modify: `services/rtc-signaling-service/tests/rtc_signal_flow_test.rs`

- [ ] Add a managed `local-minimal` runtime-dir restart test that creates a conversation-bound RTC session, rebuilds the app with the same runtime dir, and proves `accept` plus custom signal posting still work.
- [ ] Add an `rtc-signaling-service` rebuild test using a shared durable store seam once the contract exists.
- [ ] Run the focused tests and capture the red state before runtime changes.

### Task 2: Add RTC State Store Contract And Adapters

**Files:**
- Modify: `crates/im-platform-contracts/src/lib.rs`
- Modify: `adapters/local-memory/src/lib.rs`
- Modify: `adapters/local-disk/src/lib.rs`

- [ ] Add `RtcStateRecord` and `RtcStateStore`.
- [ ] Add `MemoryRtcStateStore`.
- [ ] Add `FileRtcStateStore`.
- [ ] Add a file adapter reopen regression test preserving session and signal history.

### Task 3: Recover RTC State In Runtime

**Files:**
- Modify: `services/rtc-signaling-service/src/lib.rs`
- Modify: `services/rtc-signaling-service/Cargo.toml`
- Modify: `services/rtc-signaling-service/tests/rtc_signal_flow_test.rs`

- [ ] Add `RtcRuntime::with_store(...)`.
- [ ] Lazily restore RTC session state on access paths.
- [ ] Persist RTC state after create, invite, accept, reject, end, and custom signal mutations.
- [ ] Surface store failures as controlled `rtc_store_*` errors.

### Task 4: Wire Managed Local-Minimal Runtime-Dir RTC Recovery

**Files:**
- Modify: `services/local-minimal-node/src/lib.rs`
- Modify: `services/local-minimal-node/tests/rtc_runtime_persistence_test.rs`

- [ ] Bind `FileRtcStateStore` under `<runtime-dir>/state/rtc-state.json`.
- [ ] Ensure managed builders compose RTC durability with existing domain, realtime, and stream recovery seams.
- [ ] Re-run the managed restart regression.

### Task 5: Close The Review Loop

**Files:**
- Create: `docs/review/2026-04-06-local-minimal-rtc-runtime-recovery-review-cycle.md`
- Create: `docs/架构/98-local-minimal-rtc-runtime-persistence-standard-2026-04-06.md`
- Modify: `docs/架构/95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md`
- Modify: `docs/架构/96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md`
- Modify: `docs/架构/97-local-minimal-stream-runtime-persistence-standard-2026-04-06.md`

- [ ] Record findings, root cause, implementation, and verification.
- [ ] Freeze the runtime-dir file path and composition rules.
- [ ] Document the next residual durability boundary after RTC recovery.
