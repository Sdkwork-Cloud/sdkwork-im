# 2026-04-06 Disconnect Ghost Route Drain Review Cycle

## 1. Finding

### 1.1 High: `session.disconnect` left ghost route ownership behind

- Affected services:
  - `services/session-gateway`
  - `services/local-minimal-node`
  - control-plane drain lifecycle through shared route counts
- Root cause:
  - the previous wave cleared live realtime subscriptions on disconnect
  - but the shared cluster route directory still kept the device route entry
  - route-derived operational views therefore continued to count the device as an owned route

This produced a false state:

- presence was already `offline`
- live subscriptions were already revoked
- but `ops/cluster`, `ops/diagnostics`, and drain lifecycle still treated the device as a currently owned route

## 2. Impact

- node operational views exposed a stale `deviceRouteCount`
- diagnostics still showed a disconnected device in `deviceRoutes`
- drain behavior became wrong:
  - after explicit disconnect, a node could still enter `draining + moving_routes`
  - operators would be forced to migrate a route that no longer represented a live device connection

This is not just a cosmetic ops issue. It directly weakens high-availability lifecycle correctness.

## 3. Reproduction

Red coverage was added first in:

- `services/session-gateway/tests/cluster_routing_test.rs`
  - `test_cluster_bridge_release_route_reconciles_draining_node_to_drained`
- `services/local-minimal-node/tests/cluster_drain_rebalance_e2e_test.rs`
  - `test_local_minimal_profile_disconnect_releases_route_before_drain`

Red evidence:

- `cargo test -p session-gateway --offline test_cluster_bridge_release_route_reconciles_draining_node_to_drained -- --exact`
  - failed to compile because no explicit route-release API existed
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_disconnect_releases_route_before_drain -- --exact`
  - failed with:
    - expected diagnostics route count: `0`
    - actual diagnostics route count: `1`

Observed pre-fix behavior:

1. device resumed on `node_a`
2. same device called `session.disconnect`
3. `ops/diagnostics` on `node_a` still showed the device route
4. draining `node_a` still saw an owned route and could not directly converge to `drained + stable`

## 4. Fix Design

The minimum correct rule is:

1. `session.disconnect` must revoke live subscriptions
2. `session.disconnect` must also release the current device route ownership from the shared route directory
3. releasing the route must immediately reconcile source node lifecycle truth:
   - no remaining routes on a draining node -> `drained + stable`
   - remaining routes -> continue `draining + moving_routes`

This keeps route ownership aligned with actual online device connectivity.

## 5. Implementation

- `services/session-gateway/src/cluster.rs`
  - added `release_device_route(...)`
  - route release removes the directory entry only when owned by the expected node
  - route release reuses existing lifecycle reconciliation rules for draining nodes
- `services/session-gateway/src/lib.rs`
  - `disconnect_session(...)` now clears live subscriptions and releases route ownership
- `services/local-minimal-node/src/lib.rs`
  - `disconnect_session(...)` now clears live subscriptions, releases route ownership, and refreshes ops projections
- `services/session-gateway/tests/cluster_routing_test.rs`
  - added cluster regression proving release on a draining node converges lifecycle to `drained`
- `services/local-minimal-node/tests/cluster_drain_rebalance_e2e_test.rs`
  - added end-to-end regression proving disconnect removes route visibility before drain

## 6. Verification

### Red

- `cargo test -p session-gateway --offline test_cluster_bridge_release_route_reconciles_draining_node_to_drained -- --exact`
  - failed because `release_device_route(...)` did not exist
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_disconnect_releases_route_before_drain -- --exact`
  - failed with stale route count:
    - expected `0`
    - actual `1`

### Green

- `cargo test -p session-gateway --offline test_cluster_bridge_release_route_reconciles_draining_node_to_drained -- --exact`
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_disconnect_releases_route_before_drain -- --exact`

Observed green result:

- disconnect removes route ownership
- diagnostics no longer show the disconnected device route
- drain can converge directly to `drained + stable` when no routes remain

## 7. Remaining Risks

- current-session follow-up traffic after disconnect may still rebind a route through later device-bound requests
- the platform still has not frozen whether post-disconnect traffic must require a fresh `session.resume`
- websocket graceful transport cutover remains future work

## 8. Next Wave

1. review whether explicit disconnect must invalidate the current access-plane session for all later device-bound requests except `session.resume`
2. freeze whether post-disconnect `heartbeat`, `register`, `realtime/events`, and websocket attach must reject with a reconnect-required contract
3. continue hardening toward durable route epochs only after explicit lifecycle semantics are fully frozen
