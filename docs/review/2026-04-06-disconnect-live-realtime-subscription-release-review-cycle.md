# 2026-04-06 Disconnect Live Realtime Subscription Release Review Cycle

## 1. Finding

### 1.1 High: `session.disconnect` left live realtime subscriptions active

- Affected services:
  - `services/session-gateway`
  - `services/local-minimal-node`
- Root cause:
  - `session.disconnect` only switched presence to `offline`
  - the device remained in the live realtime subscription table
  - later business publishes still matched that device through the existing subscription set

This meant an explicitly disconnected device could continue to receive new online realtime events even though the durable recovery path was already defined as `sync-feed`.

## 2. Impact

- the online/offline boundary was blurred:
  - UI presence showed `offline`
  - but new realtime window entries still accumulated for the same device
- this weakened the architecture standard that:
  - realtime window is for online low-latency delivery
  - `sync-feed` is the durable recovery truth after disconnect or resume gaps
- commercial consequence:
  - explicit disconnect did not actually stop live downlink delivery

## 3. Reproduction

Red coverage was added first in:

- `services/session-gateway/tests/realtime_runtime_test.rs`
  - `test_clearing_device_subscriptions_stops_future_realtime_delivery`
- `services/local-minimal-node/tests/http_e2e_test.rs`
  - `test_local_minimal_profile_disconnect_stops_new_realtime_delivery_and_preserves_sync_feed`

Red evidence:

- `cargo test -p session-gateway --offline test_clearing_device_subscriptions_stops_future_realtime_delivery -- --exact`
  - failed to compile before the runtime exposed any subscription-release operation
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_disconnect_stops_new_realtime_delivery_and_preserves_sync_feed -- --exact`
  - failed with:
    - expected realtime items: `0`
    - actual realtime items: `1`

Observed pre-fix behavior:

1. device synced realtime subscriptions
2. device called `session.disconnect`
3. another device published a new message
4. disconnected device still read the new event from `/im/v3/api/realtime/events`

## 4. Fix Design

The minimal safe rule is:

1. `session.disconnect` must keep durable state intact:
   - `sync-feed`
   - checkpoint / ack metadata
   - latest durable sync sequence
2. `session.disconnect` must revoke the device's live realtime subscriptions immediately
3. after disconnect:
   - new online realtime publishes must stop matching that device
   - durable recovery remains available through `sync-feed`

This intentionally does not yet freeze:

- explicit route release
- durable route ownership
- reconnect orchestration

## 5. Implementation

- `services/session-gateway/src/realtime.rs`
  - added `clear_device_subscriptions(...)`
- `services/session-gateway/src/lib.rs`
  - `disconnect_session(...)` now clears live device subscriptions before returning the offline snapshot
- `services/local-minimal-node/src/lib.rs`
  - `disconnect_session(...)` now clears live device subscriptions before returning the offline snapshot
- `services/session-gateway/tests/realtime_runtime_test.rs`
  - added unit regression proving no new delivery after subscription release
- `services/local-minimal-node/tests/http_e2e_test.rs`
  - added end-to-end regression proving disconnect stops realtime delivery while preserving sync-feed recovery

## 6. Verification

### Red

- `cargo test -p session-gateway --offline test_clearing_device_subscriptions_stops_future_realtime_delivery -- --exact`
  - failed because the runtime had no subscription-release API
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_disconnect_stops_new_realtime_delivery_and_preserves_sync_feed -- --exact`
  - failed with status mismatch in expectations:
    - expected realtime item count `0`
    - actual realtime item count `1`

### Green

- `cargo test -p session-gateway --offline test_clearing_device_subscriptions_stops_future_realtime_delivery -- --exact`
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_disconnect_stops_new_realtime_delivery_and_preserves_sync_feed -- --exact`

Observed green result:

- disconnected devices stop receiving new realtime window entries
- durable `sync-feed` still contains the committed event for later recovery

## 7. Remaining Risks

- disconnect still does not explicitly release route ownership
- a later reconnect policy is not yet frozen:
  - whether fresh resume is mandatory before re-subscribing
  - whether route ownership should be downgraded or removed
- websocket transport cutover on explicit disconnect remains future work

## 8. Next Wave

1. review whether explicit disconnect must also release or downgrade route ownership
2. decide whether post-disconnect device-bound traffic must require a fresh `session.resume`
3. keep commercial hardening focused on explicit lifecycle truth instead of implicit local heuristics
