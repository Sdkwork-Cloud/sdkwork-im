# 87. Disconnect Releases Route Ownership Standard (2026-04-06)

## 1. Goal

When a device explicitly calls `session.disconnect`, the platform must remove that device's live route ownership from the shared route directory so operational views and drain lifecycle reflect the real online state.

## 2. Problem Boundary

This standard applies when:

- a device currently owns a route entry in the shared route directory
- the same device explicitly calls:
  - `POST /im/v3/api/device/sessions/disconnect`

This standard refines:

- realtime subscription release from Standard 86
- route observability from Standard 22
- drain lifecycle correctness from Standard 23

## 3. Required Rule

`session.disconnect` must release the device's current route ownership after validating current-session access and after revoking live subscriptions.

Required outcome:

1. presence becomes `offline`
2. live realtime subscriptions are cleared
3. the device route entry is removed from the shared route directory
4. ops and drain lifecycle immediately observe the new route count

## 4. Lifecycle Consequence

When route release removes the last remaining route on a draining node:

- `drainStatus` must become `drained`
- `rebalanceState` must become `stable`
- `ownedRouteCount` / `deviceRouteCount` must become `0`

If other routes still remain:

- node stays `draining + moving_routes`

This reuses the same lifecycle truth already required for route migration and direct rebind departure.

## 5. Operational Visibility Contract

After a successful `session.disconnect`:

- `ops/cluster` must no longer count the released route in `deviceRouteCount`
- `ops/diagnostics` must no longer list the released device in `deviceRoutes`

The route directory is not allowed to expose a ghost route for a device that is no longer considered online by access-plane semantics.

## 6. Minimal Implementation Rule

The minimal implementation may satisfy this standard by:

1. releasing the route only when it is owned by the expected local node after the current request's bind/fence path
2. reconciling source node lifecycle immediately after removal

This standard still does not require:

- durable route store
- route epoch fencing
- transport graceful close orchestration

## 7. Relationship To Resume

`session.resume` remains the request that re-establishes explicit device activity and may create or replace route ownership.

This standard does not yet decide whether all post-disconnect device-bound traffic must require a fresh resume. It only freezes that a successful explicit disconnect must not leave stale route ownership behind.

## 8. Verification Standard

Regression coverage must prove:

1. a device owns a route on `node_a`
2. the device calls `session.disconnect`
3. `ops/diagnostics` on `node_a` shows no route for that device
4. draining `node_a` with no other routes converges directly to:
   - `drained`
   - `stable`
   - route count `0`

Coverage must exist at both levels:

- cluster bridge unit test
- end-to-end node or control-plane integration test

## 9. Design Consequence

This standard prevents a high-availability mismatch where:

- presence says the device is offline
- realtime subscriptions are already gone
- but route ownership still blocks drain and pollutes ops counts

That mismatch is not acceptable in a commercial distributed IM platform because route ownership is part of the platform's operational truth, not just an optimization hint.

## 10. Non-Goals

This standard still does not freeze:

- reconnect-required error semantics after disconnect
- websocket graceful close sequencing
- durable route ownership across process crashes
