# 2026-04-06 Stale Session Route Fencing Review Cycle

## 1. Finding

### 1.1 High: stale device sessions could reclaim route ownership after a newer cross-node resume

- Affected services:
  - `services/session-gateway`
  - `services/local-minimal-node`
- Root cause:
  - non-resume device-bound requests implicitly rebound device ownership on every call:
    - `session.disconnect`
    - `presence.heartbeat`
    - realtime subscription sync / pull / ack / websocket attach
    - `local-minimal-node` command paths that call `ensure_registered_device(...)`
  - the shared route directory only tracked:
    - `tenantId`
    - `principalId`
    - `deviceId`
    - `ownerNodeId`
  - the currently active `session_id` only existed in node-local presence memory
  - after device `d_resume` resumed on `node_b` with `s_new`, a stale request from `node_a` with `s_old` still passed and rebound the shared route back to `node_a`

## 2. Impact

- a stale session could steal device ownership from the current session after cross-node takeover
- the failure was not limited to presence UI state; it could redirect actual realtime delivery back to the wrong node
- explicit `session.disconnect` was especially dangerous:
  - old session remained accepted
  - route ownership could move away from the active node
  - subsequent online delivery could miss the real owner transport

This is a correctness defect in distributed session ownership, not only a snapshot inconsistency.

## 3. Reproduction

Red regression coverage was added first in:

- `services/local-minimal-node/tests/cluster_realtime_routing_e2e_test.rs`
  - `test_local_minimal_profile_rejects_stale_disconnect_after_cross_node_resume_takeover`

Red evidence:

- `cargo test -p local-minimal-node --offline test_local_minimal_profile_rejects_stale_disconnect_after_cross_node_resume_takeover -- --exact`
  - failed with:
    - expected `409`
    - actual `200`

Observed pre-fix behavior:

1. device resumed on `node_a` with `s_old`
2. device resumed again on `node_b` with `s_new`
3. stale `session.disconnect` from `node_a` still succeeded
4. the stale request was allowed to re-enter the shared bind path

## 4. Fix Design

The correct truth boundary is the shared route directory, not node-local presence memory.

Chosen rule:

1. `session.resume` remains the only request that may intentionally take over a device route with a new `session_id`
2. the shared `RealtimeDeviceRoute` must carry the currently bound `session_id`
3. every non-resume device-binding request must fence against the route directory before it can register or rebind the device
4. if the incoming `session_id` does not match the route's current `session_id`, reject with:
   - `409 stale_session`
5. do the rejection before:
   - device registration mutation
   - route ownership mutation
   - presence mutation

This keeps the latest-resume-wins rule while preventing stale follow-up requests from reclaiming ownership.

## 5. Implementation

- `services/session-gateway/src/cluster.rs`
  - `RealtimeDeviceRoute` now stores `session_id`
  - `bind_device_route(...)` now persists the bound session id together with owner node
  - added `ensure_route_session_current(...)` to enforce cluster-wide current-session fencing
- `services/session-gateway/src/lib.rs`
  - resume path now binds routes with `auth.session_id`
  - non-resume device-bound handlers now call `ensure_route_session_current(...)` before rebinding
- `services/local-minimal-node/src/lib.rs`
  - `bind_registered_device(...)` now accepts:
    - `session_id`
    - `allow_session_takeover`
  - resume path passes takeover allowed
  - other device-bound paths, including `ensure_registered_device(...)`, are fenced before registration / rebind
- `services/session-gateway/src/cluster.rs`
  - added `test_route_session_fence_rejects_stale_session_after_takeover`
- `services/local-minimal-node/tests/cluster_realtime_routing_e2e_test.rs`
  - added end-to-end stale-disconnect takeover regression coverage

## 6. Verification

### Red

- `cargo test -p local-minimal-node --offline test_local_minimal_profile_rejects_stale_disconnect_after_cross_node_resume_takeover -- --exact`
  - failed with status mismatch:
    - expected `409`
    - actual `200`

### Green

- `cargo test -p local-minimal-node --offline test_local_minimal_profile_rejects_stale_disconnect_after_cross_node_resume_takeover -- --exact`
- `cargo test -p session-gateway --offline`
- `cargo test -p control-plane-api --offline test_control_plane_can_drain_and_migrate_routes -- --exact`
- `cargo fmt --all --check`
- `cargo test --workspace --offline`

Observed green result:

- stale cross-node `disconnect` is rejected with `409 stale_session`
- route ownership stays on the active node
- cluster session fence is covered directly in `session-gateway`
- existing route migration / drain flows remain compatible with the new route metadata

## 7. Remaining Risks

- requests without `session_id` still cannot be fenced by session identity
- presence snapshot truth remains node-local and non-durable in the minimal profile
- the current solution depends on shared route directory correctness; later durable route storage and lease epochs are still needed for stronger commercial guarantees

## 8. Next Wave

1. decide whether `stale_session` should surface richer diagnostics in ops / audit views without leaking unnecessary metadata
2. review whether current-session fencing should also be reflected in explicit route release semantics on current-session disconnect
3. continue toward durable route lease / epoch / fencing so session ownership survives process loss and split-brain scenarios more formally
