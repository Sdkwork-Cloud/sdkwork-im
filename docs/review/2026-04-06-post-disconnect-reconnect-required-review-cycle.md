# 2026-04-06 Post-Disconnect Reconnect Required Review Cycle

## 1. Finding

### 1.1 High: `session.disconnect` still allowed the old session to silently rebind the device route

- Affected services:
  - `services/session-gateway`
  - `services/local-minimal-node`
- Root cause:
  - the previous wave correctly released live subscriptions and route ownership on disconnect
  - but non-`resume` device-bound requests only fenced on an already-existing route
  - once disconnect removed the route entry, later requests fell through the "no route, bind a new one" path

This left an incorrect lifecycle gap:

1. device explicitly called `session.disconnect`
2. presence became `offline`
3. route ownership disappeared
4. the same old `session_id` could immediately call `presence.heartbeat` or similar device-bound APIs
5. the request rebound route ownership without a fresh `session.resume`

An explicit disconnect therefore did not actually establish a reconnect boundary.

## 2. Impact

- explicit disconnect was not authoritative
- stale or already-ended sessions could make the device look active again
- route ownership could be recreated without an explicit resume handshake
- future live-delivery or websocket attach flows would inherit the same bypass if they reused the shared bind path

For a commercial IM platform, this is a correctness problem, not a cosmetic issue. Disconnect semantics must be deterministic across presence, routing, realtime delivery, and future transport extensions.

## 3. Reproduction

Red coverage was added first in:

- `services/session-gateway/tests/http_smoke_test.rs`
  - `test_session_gateway_requires_fresh_resume_after_disconnect`
- `services/local-minimal-node/tests/http_e2e_test.rs`
  - `test_local_minimal_profile_requires_fresh_resume_after_disconnect`

Red evidence:

- `cargo test -p session-gateway --offline test_session_gateway_requires_fresh_resume_after_disconnect -- --exact --nocapture`
  - failed with:
    - expected status: `409`
    - actual status: `200`
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_requires_fresh_resume_after_disconnect -- --exact --nocapture`
  - failed with:
    - expected status: `409`
    - actual status: `200`

Observed pre-fix behavior:

1. device resumed with `session_id = s_old`
2. device called `session.disconnect`
3. same old `session_id` called `presence.heartbeat`
4. request succeeded and rebound the device route instead of forcing a fresh resume

## 4. Fix Design

The minimum correct rule is:

1. disconnect must mark a shared device-level reconnect fence after route release
2. every non-`resume` device-bound bind path must reject while that fence exists
3. only a successful `session.resume` may clear the fence

The fence must live in the shared cluster bridge, not in a single handler, because the same reconnect boundary must govern:

- heartbeat
- device register
- realtime subscription sync
- realtime event poll/ack
- websocket attach
- any future device-bound transport or streaming entrypoint

## 5. Implementation

- `services/session-gateway/src/cluster.rs`
  - added shared disconnect fence storage keyed by `tenant_id + principal_id + device_id`
  - added:
    - `mark_device_disconnected(...)`
    - `clear_device_disconnect_fence(...)`
    - `ensure_device_resume_not_required(...)`
  - added cluster regression:
    - `test_disconnect_fence_requires_resume_until_cleared`
- `services/session-gateway/src/lib.rs`
  - `disconnect_session(...)` now marks the reconnect fence after releasing route ownership
  - `AppState::register_device(...)` now distinguishes `resume` from non-`resume` binds
  - non-`resume` binds reject with `reconnect_required` while the fence exists
  - successful `session.resume` clears the fence only after route bind succeeds
- `services/local-minimal-node/src/lib.rs`
  - `disconnect_session(...)` now marks the reconnect fence after route release
  - shared `bind_registered_device(...)` now rejects non-`resume` binds while the fence exists
  - successful `resume` clears the fence after route bind
- `services/session-gateway/tests/http_smoke_test.rs`
  - added HTTP regression proving stale post-disconnect heartbeat is rejected and fresh resume clears the condition
- `services/local-minimal-node/tests/http_e2e_test.rs`
  - added integrated regression proving the same behavior in the local minimal profile

## 6. Verification

### Red

- `cargo test -p session-gateway --offline test_session_gateway_requires_fresh_resume_after_disconnect -- --exact --nocapture`
  - failed with:
    - left: `200`
    - right: `409`
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_requires_fresh_resume_after_disconnect -- --exact --nocapture`
  - failed with:
    - left: `200`
    - right: `409`

### Green

- `cargo test -p session-gateway --offline disconnect_fence_requires_resume_until_cleared -- --nocapture`
- `cargo test -p session-gateway --offline test_session_gateway_requires_fresh_resume_after_disconnect -- --exact --nocapture`
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_requires_fresh_resume_after_disconnect -- --exact --nocapture`

Observed green result:

- old post-disconnect session traffic is rejected with `409 reconnect_required`
- old post-disconnect live realtime polling no longer resurrects route ownership
- durable device sync feed remains queryable without restoring live route ownership
- successful fresh `session.resume` clears the reconnect fence
- later normal device-bound traffic succeeds again under the new session

## 7. Remaining Risks

- the reconnect fence is currently in-memory, so process crash recovery still relies on higher-level session reconstruction rather than durable disconnect intent
- websocket graceful close semantics are still separate from the reconnect-required contract
- route epoch persistence is still future work

## 8. Next Wave

1. review whether disconnect fence intent must become durable across process restart or active-passive failover
2. freeze websocket reconnect behavior so transport attach and HTTP polling share the exact same resume boundary
3. continue hardening route/session lifecycle toward durable epochs only after explicit reconnect semantics are now frozen
