# 2026-04-06 Sessionless Route Rebind Bypass Review Cycle

## 1. Finding

### 1.1 High: sessionless device-bound requests could still bypass current-session route fencing

- Affected services:
  - `services/session-gateway`
  - `services/local-minimal-node`
- Root cause:
  - the previous wave introduced shared route fencing for mismatched `session_id`
  - but `ensure_route_session_current(...)` still allowed requests whose auth context had no `session_id`
  - once a device route had already been claimed by a live session, a sessionless request could still enter the non-resume bind path and rebind the device

Affected request classes included:

- `POST /im/v3/api/devices/register`
- `POST /im/v3/api/presence/heartbeat`
- `POST /im/v3/api/device/sessions/disconnect`
- `GET /im/v3/api/realtime/events`
- `POST /im/v3/api/realtime/events/ack`
- websocket attach

## 2. Impact

- a request missing `session_id` could still steal route ownership from the active session
- the bypass was cross-node, not just local:
  - route truth already lived in the shared cluster directory
  - but the missing-session path skipped that truth instead of being rejected
- this weakened the intended commercial invariant:
  - latest resume wins
  - all later device-bound traffic must prove it belongs to that current session

## 3. Reproduction

Red regression coverage was added first in:

- `services/local-minimal-node/tests/cluster_realtime_routing_e2e_test.rs`
  - `test_local_minimal_profile_rejects_sessionless_rebind_after_cross_node_resume_takeover`

Red evidence:

- `cargo test -p local-minimal-node --offline test_local_minimal_profile_rejects_sessionless_rebind_after_cross_node_resume_takeover -- --exact`
  - failed with:
    - expected `409`
    - actual `200`

Observed pre-fix behavior:

1. device resumed on `node_a` with `s_old`
2. device resumed on `node_b` with `s_new`
3. a sessionless `POST /im/v3/api/devices/register` on `node_a` still succeeded
4. current-session fencing was therefore bypassable by omitting `session_id`

## 4. Fix Design

The correct rule is stricter than the previous minimal fallback:

1. if a route has no current `session_id`, minimal behavior may continue
2. if a route already has a current `session_id`:
   - any non-resume device-bound request must present a `session_id`
   - that `session_id` must match the current route session
3. if the request omits `session_id` while the route is session-owned:
   - reject before rebind
   - return `409 session_id_required`

This preserves first registration / pre-session minimal behavior while removing the bypass once current-session truth exists.

## 5. Implementation

- `services/session-gateway/src/cluster.rs`
  - `ensure_route_session_current(...)` now rejects missing `session_id` when the route is already bound to a live session
  - added `test_route_session_fence_requires_session_id_once_route_is_bound_to_session`
- `services/session-gateway/tests/http_smoke_test.rs`
  - added `test_session_gateway_rejects_sessionless_device_rebind_after_session_resume`
- `services/local-minimal-node/tests/cluster_realtime_routing_e2e_test.rs`
  - added `test_local_minimal_profile_rejects_sessionless_rebind_after_cross_node_resume_takeover`

No new production-side rebind path was introduced; the existing shared fence was tightened.

## 6. Verification

### Red

- `cargo test -p local-minimal-node --offline test_local_minimal_profile_rejects_sessionless_rebind_after_cross_node_resume_takeover -- --exact`
  - failed with status mismatch:
    - expected `409`
    - actual `200`

### Green

- `cargo test -p local-minimal-node --offline test_local_minimal_profile_rejects_sessionless_rebind_after_cross_node_resume_takeover -- --exact`
- `cargo test -p session-gateway --offline test_session_gateway_rejects_sessionless_device_rebind_after_session_resume -- --exact`
- `cargo test -p session-gateway --offline cluster::tests::test_route_session_fence_requires_session_id_once_route_is_bound_to_session -- --exact`
- `cargo fmt --all --check`
- `cargo test --workspace --offline`

Observed green result:

- sessionless rebind is rejected once the route has a current session
- route ownership remains with the live session owner node
- standalone `session-gateway` and integrated `local-minimal-node` both enforce the same rule

## 7. Remaining Risks

- the current platform still treats routes with no stored `session_id` as pre-fence minimal behavior
- explicit route release on current-session disconnect is still not frozen
- durable route store / lease epoch / fencing remains future work for stronger cluster guarantees

## 8. Next Wave

1. review whether `session.disconnect` from the current session should explicitly clear or downgrade route ownership
2. decide whether devices/register should become a session-bound-only operation after the first live resume, or whether a separate bootstrap path should remain
3. continue hardening commercial route semantics with explicit release, lease epochs, and durable route metadata
