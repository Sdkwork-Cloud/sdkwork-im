# Local-Minimal Presence Runtime Recovery Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make managed `local-minimal` rebuilds restore presence device inventory and last-observed timestamps from the runtime dir while forcing a fresh `session.resume` before stale pre-restart device traffic can become online again.

**Architecture:** Introduce a pluggable presence state store, persist per-device presence records behind `SessionPresenceRuntime`, lazily restore principal-scoped device state, normalize previously `online` entries to `offline` after restart, and require a fresh resume before non-resume device-bound traffic may reactivate a restored device. Keep unmanaged/default builders memory-backed and bind a file-backed store only for managed runtime-dir builders.

**Tech Stack:** Rust, Axum, serde/serde_json, local file persistence, `session-gateway`, `local-minimal-node`, existing `im-platform-contracts` adapter model.

---

### Task 1: Freeze The Restart Gap With Red Tests

**Files:**
- Create: `services/session-gateway/tests/presence_runtime_persistence_test.rs`
- Create: `services/local-minimal-node/tests/presence_runtime_persistence_test.rs`

- [ ] Add a `session-gateway` runtime rebuild test using a shared presence store and verify a previously online device restores as `offline` with preserved timestamps and requires a fresh resume before heartbeat.
- [ ] Add a managed `local-minimal` runtime-dir rebuild test proving `/api/v1/presence/me` restores device inventory across restart, stale heartbeat is rejected, and fresh resume reactivates the device.
- [ ] Run the focused tests and capture the red state before runtime changes.

### Task 2: Add Presence Store Contracts And Adapters

**Files:**
- Modify: `crates/im-platform-contracts/src/lib.rs`
- Modify: `adapters/local-memory/src/lib.rs`
- Modify: `adapters/local-disk/src/lib.rs`

- [ ] Add `PresenceStateRecord` and `PresenceStateStore`.
- [ ] Support both point lookup and principal-scope listing so restart can rebuild multi-device presence without relying on in-memory registration caches.
- [ ] Add `MemoryPresenceStateStore` and `FilePresenceStateStore`.
- [ ] Add file adapter reopen regression coverage for the presence store.

### Task 3: Recover Presence Runtime And Resume Semantics

**Files:**
- Modify: `services/session-gateway/src/lib.rs`
- Modify: `services/session-gateway/Cargo.toml`

- [ ] Add a `SessionPresenceRuntime::with_store(...)` constructor plus runtime-memory default store.
- [ ] Persist registered-device placeholders so restart can reconstruct device inventory even if projection/device caches are empty.
- [ ] Lazily restore principal-scoped presence state from the store.
- [ ] Normalize restored `online` records to `offline` with cleared `session_id`, preserved `last_resume_at` / `last_seen_at`, and an internal `resume required` marker.
- [ ] Reject non-resume device-bound traffic with `reconnect_required` while a restored device still requires a fresh resume.
- [ ] Clear the `resume required` marker on successful `session.resume`.
- [ ] Surface presence-store failures as controlled runtime/API errors.

### Task 4: Wire Managed Local-Minimal Runtime-Dir Presence Recovery

**Files:**
- Modify: `services/local-minimal-node/src/lib.rs`
- Modify: `services/local-minimal-node/tests/presence_runtime_persistence_test.rs`

- [ ] Bind `FilePresenceStateStore` under `<runtime-dir>/state/presence-state.json`.
- [ ] Ensure managed builders compose presence durability with existing disconnect-fence, checkpoint, domain, subscription, stream, RTC, notification, and automation recovery seams.
- [ ] Re-run the managed restart regression.

### Task 5: Close The Review Loop

**Files:**
- Create: `docs/review/2026-04-06-local-minimal-presence-runtime-recovery-review-cycle.md`
- Create: `docs/架构/101-local-minimal-presence-runtime-persistence-standard-2026-04-06.md`
- Modify: `docs/架构/95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md`
- Modify: `docs/架构/99-local-minimal-notification-runtime-persistence-standard-2026-04-06.md`
- Modify: `docs/架构/100-local-minimal-automation-runtime-persistence-standard-2026-04-06.md`

- [ ] Record findings, root cause, implementation, and verification.
- [ ] Freeze restart normalization semantics for `online -> offline` recovery.
- [ ] Document why presence liveness is restart-recoverable query state but not durable proof of a still-live session.
- [ ] Record the next residual private-deployment restart boundary after this wave.
