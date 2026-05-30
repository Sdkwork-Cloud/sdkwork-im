# 2026-04-06 Local-Minimal Presence Runtime Recovery Review Cycle

## 1. Findings

### 1.1 High: managed `local-minimal` rebuilds lost presence device inventory and timestamps after restart

- `DevicePresenceRuntime` kept all device presence state in a node-local `HashMap`.
- managed runtime-dir builders already persisted:
  - disconnect fences
  - realtime checkpoints
  - live subscription intent
  - domain replay state
  - stream runtime state
  - RTC runtime state
  - notification and automation projections
- presence still reset to empty memory on rebuild.
- the operational effect was direct:
  - `GET /im/v3/api/presence/me` lost previously known devices unless the process re-registered them
  - `lastResumeAt` / `lastSeenAt` continuity disappeared after restart

### 1.2 High: stale pre-restart device traffic could silently become online again without a fresh resume

- before this wave, route ownership was memory-first and `ensure_route_session_current(...)` allowed requests when no route existed.
- after an abrupt restart, a device with a previously live session could therefore send `presence.heartbeat` and recreate online state without a new `session.resume`.
- for a commercial IM platform, this is a correctness issue:
  - restart must not be treated as proof that a previous session is still alive
  - only a fresh bootstrap may reactivate device liveness after rebuild

### 1.3 Medium: presence recovery depended on unrelated in-memory registration caches

- `projection_service.registered_devices(...)` is a helpful query cache, but not a durable truth boundary.
- presence needed its own replaceable persistence seam for:
  - device inventory recovery
  - timestamp continuity
  - restart-time resume gating

## 2. Root Cause

The root cause was an unfinished runtime boundary:

1. there was no pluggable `PresenceStateStore`
2. `DevicePresenceRuntime` stored state only in process memory
3. managed runtime-dir builders never replaced the default runtime with a file-backed presence store
4. registration inventory, presence timestamps, and pre-restart online truth were therefore lost or incorrectly re-inferred after rebuild

So the platform already had durable restart standards for domain and realtime surfaces, but presence remained a transient cache with no restart contract.

## 3. Implementation

This review cycle completed the missing presence recovery path:

- added a pluggable presence store contract in `im-platform-contracts`
  - `PresenceStateRecord`
  - `PresenceStateStore`
- added adapters:
  - `MemoryPresenceStateStore`
  - `FilePresenceStateStore`
- extended `DevicePresenceRuntime`
  - added `with_store(...)`
  - persisted offline placeholder records for explicit device registration
  - restored principal-scoped presence lazily on access
  - normalized restored `online` entries to `offline`
  - cleared restored `session_id` on normalization
  - preserved `lastResumeAt`, `lastSeenAt`, and `lastSyncSeq`
  - marked normalized devices as `resume required`
  - rejected stale non-resume traffic with `reconnect_required`
  - cleared the resume-required marker on successful `session.resume`
  - surfaced controlled `presence_store_*` runtime errors
- wired managed `local-minimal` builders to:
  - `<runtime-dir>/state/presence-state.json`
- kept unmanaged/default builders memory-backed

## 4. Recovery Rule

This wave intentionally restores **presence query continuity**, not proof of a still-live session.

The standardized restart behavior is:

1. persist device presence state and inventory behind the presence store
2. lazily restore principal-scoped device records after rebuild
3. if a restored record was previously `online`, normalize it to:
   - `status = offline`
   - `session_id = null`
   - preserved timestamps and sync sequence
   - `resume required = true`
4. reject stale device-bound non-resume traffic until a fresh `POST /im/v3/api/device/sessions/resume`
5. allow fresh resume to reactivate the device and clear the restart fence

This preserves commercial correctness: restart-safe visibility without pretending that process survival and client liveness are the same thing.

## 5. Regression Coverage

- `services/session-gateway/tests/presence_runtime_persistence_test.rs`
  - `test_runtime_restores_presence_as_offline_and_requires_fresh_resume_after_rebuild`
- `adapters/local-disk/src/lib.rs`
  - `test_file_presence_state_store_persists_across_reopen`
- `services/local-minimal-node/tests/presence_runtime_persistence_test.rs`
  - `test_default_local_minimal_profile_restores_presence_runtime_and_requires_fresh_resume_after_restart`

## 6. Verification

Verified in this cycle with fresh command output:

- `cargo test -p session-gateway --offline --test presence_runtime_persistence_test -- --nocapture`
- `cargo test -p im-adapters-local-disk --offline test_file_presence_state_store_persists_across_reopen -- --nocapture`
- `cargo test -p local-minimal-node --offline --test presence_runtime_persistence_test -- --nocapture`

Additional broad verification ran after implementation stabilization:

- `cargo fmt --all --check`
- `cargo test -p session-gateway --offline`
- `cargo test -p im-adapters-local-disk --offline`
- `cargo test -p local-minimal-node --offline`

## 7. Standardized Outcome

Managed `local-minimal` private deployment now restores presence runtime state across rebuild when the same runtime dir is reused.

The recovered surface covers:

- device inventory continuity
- `lastResumeAt` continuity
- `lastSeenAt` continuity
- restart normalization of stale `online` state back to `offline`
- rejection of stale pre-restart heartbeat traffic
- fresh resume reactivation after rebuild
- runtime-dir file persistence for presence state

Clients and operators no longer lose presence history just because the node restarts, and the platform no longer treats a stale pre-restart session as if it were still alive.

## 8. Residual Risk

This wave still leaves several restart / failover boundaries outside the durable baseline:

- websocket/socket continuity
- cross-node durable route ownership recovery beyond the current memory-first route directory
- operator runtime-dir inspection and repair tooling
- explicit reconciliation tooling for runtime-dir drift after partial write failures

## 9. Next Wave

The next durability review wave should target one of these:

1. restart-safe operational tooling for runtime-dir inspection, validation, and repair
2. stronger multi-node durable route ownership / failover reconstruction
3. reconciliation tooling for runtime-store drift detection and repair
