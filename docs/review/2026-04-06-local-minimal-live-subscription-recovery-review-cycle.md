# 2026-04-06 Local-Minimal Live Subscription Recovery Review Cycle

## 1. Findings

### 1.1 High: managed `local-minimal` rebuilds lost live realtime delivery until the client called `subscriptions.sync` again

- Previous hardening waves already persisted:
  - realtime disconnect fences
  - realtime checkpoint truth
  - conversation-domain replay state
- But live realtime subscriptions still lived only in runtime memory.
- After a restart, a device could successfully perform a fresh `session.resume`, yet matching realtime delivery still returned `0` because no subscriptions were restored into the runtime before publish matching.

### 1.2 High: the runtime restored checkpoint truth but not subscription intent

- `RealtimeDeliveryRuntime::ensure_device_state(...)` only restored checkpoint data.
- Publish matching happens in `publish_scope_event(...)`, which reads the in-memory subscription map directly.
- Restoring subscriptions only in poll/list APIs would have been too late. The repair had to happen before future publish matching, at device bootstrap time.

### 1.3 Medium: managed runtime-dir composition had no durable subscription adapter

- `local-minimal-node` already bound:
  - `FileRealtimeDisconnectFenceStore`
  - `FileRealtimeCheckpointStore`
  - `FileCommitJournal`
- There was no equivalent durable adapter for device-scope realtime subscription intent, so private deployment still had a restart gap even after the prior durability waves.

## 2. Root Cause

The root cause was an unfinished recovery boundary:

1. the platform already had a stable lazy bootstrap seam in `ensure_device_state(...)`
2. the managed profile already had a runtime-dir durability model
3. subscription state was still memory-only
4. bootstrap paths such as `session.resume` and `register_device(...)` did not restore durable subscriptions before bind and future publish matching

So restart recovery restored device checkpoint numbers, but not the live delivery intent needed for subsequent realtime fan-out.

## 3. Implementation

This review cycle completed the missing live subscription recovery path:

- added a pluggable durable `RealtimeSubscriptionStore` contract
  - device-scope load/save/clear
  - vendor-neutral seam
- added adapters:
  - `MemoryRealtimeSubscriptionStore`
  - `FileRealtimeSubscriptionStore`
- extended `RealtimeDeliveryRuntime`
  - added `with_stores(...)`
  - kept `with_checkpoint_store(...)` as memory-backed fallback for subscription storage
  - restored persisted subscriptions inside `ensure_device_state(...)`
  - persisted subscription mutations on:
    - `sync_subscriptions(...)`
    - `clear_device_subscriptions(...)`
    - `restore_device_state(...)`
- rewired bootstrap paths
  - `session-gateway::AppState::register_device(...)`
  - `local-minimal-node::bind_registered_device(...)`
  - both now call `ensure_device_state(...)` before route bind completes
- rewired managed runtime-dir composition
  - runtime-dir builders now bind `FileRealtimeSubscriptionStore` at:
    - `<runtime-dir>/state/realtime-subscriptions.json`
- hardened API error mapping
  - `subscription_store_unavailable -> 503`
  - `subscription_store_conflict -> 409`
  - `subscription_store_unsupported -> 501`

## 4. Recovery Rule

This wave intentionally does **not** revive dead sockets or stale route bindings after process restart.

The standardized behavior is:

1. persist device-scope subscription intent durably
2. require the device to re-enter through a legitimate bootstrap path such as `session.resume`
3. lazily restore subscription intent during that bootstrap
4. allow future `publish_scope_event(...)` matching to continue without another explicit `subscriptions.sync`

This preserves session safety while closing the largest private-deployment live-delivery restart gap.

## 5. Regression Coverage

- `services/session-gateway/tests/realtime_runtime_test.rs`
  - `test_runtime_restores_persisted_subscriptions_on_rebuild_without_resync`
- `adapters/local-disk/src/lib.rs`
  - `test_file_subscription_store_persists_across_reopen`
- `services/local-minimal-node/tests/live_subscription_recovery_persistence_test.rs`
  - `test_default_local_minimal_profile_restores_live_subscriptions_after_rebuild_with_fresh_resume`

## 6. Verification

Verified in this cycle with fresh command output:

- `cargo test -p im-adapters-local-disk --offline test_file_subscription_store_persists_across_reopen -- --nocapture`
- `cargo test -p session-gateway --offline test_runtime_restores_persisted_subscriptions_on_rebuild_without_resync -- --nocapture`
- `cargo test -p local-minimal-node --offline --test live_subscription_recovery_persistence_test -- --nocapture`
- `cargo fmt --all --check`
- `cargo test -p session-gateway --offline`
- `cargo test -p im-adapters-local-disk --offline`
- `cargo test -p local-minimal-node --offline`

## 7. Standardized Outcome

Managed `local-minimal` private deployment now restores live realtime subscription intent across rebuild when the same runtime dir is reused **and** the same device performs a fresh bootstrap such as `session.resume`.

The device no longer needs a second `POST /im/v3/api/realtime/subscriptions/sync` after restart just to receive new matching realtime events.

## 8. Residual Risk

This wave still does not make restart behavior universal for every subsystem. The following remain separate boundaries:

- presence heartbeat truth
- websocket/socket continuity
- stream runtime state
- RTC runtime state
- notification and automation runtime projections
- any future cross-node durable shared subscription topology beyond the current device-scope store contract

## 9. Next Wave

The next durability review wave should target one of these:

1. durable recovery standards for stream / RTC runtime state
2. controlled persistence boundaries for notification / automation runtime projections
3. stronger restart-safe operational tooling around runtime-dir inspection and repair

The architectural rule remains unchanged: every recovery surface must be standardized, implemented, and regression-tested independently.
