# 92. Disconnect Fence Store Recovery Standard (2026-04-06)

## 1. Goal

The reconnect-required fence created by `session.disconnect` must remain recoverable when the access-plane bridge is rebuilt.

This standard tightens Standard 88 for restart, failover, and service-rebuild scenarios.

## 2. Problem Boundary

This standard applies when:

- a device successfully calls `POST /api/v1/sessions/disconnect`
- the access plane later rebuilds the `RealtimeClusterBridge`
- the device sends later device-bound traffic before a fresh `session.resume`

Without this standard, Standard 88 only holds while one in-memory bridge instance stays alive.

## 3. Required Rule

The platform must provide a persistence boundary for disconnect fences such that:

1. disconnect intent can be stored outside the current bridge instance
2. a rebuilt bridge can restore that fence on demand
3. stale device-bound traffic remains blocked until a fresh `session.resume`
4. successful fresh `session.resume` clears both cached state and stored fence state

## 4. Store Contract

The disconnect fence store must support:

- load by:
  - `tenant_id`
  - `principal_id`
  - `device_id`
- save of:
  - device scope
  - disconnected session id
  - owner node id
  - disconnect timestamp
- clear by device scope

The contract must be storage-vendor-neutral.

## 5. Bridge Contract

`RealtimeClusterBridge` must:

1. write through to the fence store when `mark_device_disconnected(...)` is called
2. lazily restore a fence from the store when:
   - `ensure_device_resume_not_required(...)` is called
   - `disconnect_fence_matches_session(...)` is called
3. clear both memory and store state when `clear_device_disconnect_fence(...)` is called

The bridge is not allowed to assume that reconnect fences only exist in the current process memory.

## 6. Service Contract

Access-plane services that use the bridge must preserve the same reconnect behavior after rebuild:

- `session-gateway`
- `local-minimal-node`
- future clustered access-plane entrypoints

After rebuild:

1. stale `presence.heartbeat` must still fail with `409 reconnect_required`
2. stale realtime device bind paths must still fail
3. fresh `session.resume` must clear the restored fence

## 7. Default Adapter Rule

The platform may ship a local-memory disconnect fence adapter for:

- tests
- local minimal profile
- development environments

But that adapter is not a production durability guarantee.

Production/private-deployment profiles should bind `RealtimeDisconnectFenceStore` to a durable backend.

## 8. Verification Standard

Regression coverage must prove:

1. a first bridge/app instance records disconnect intent
2. a second bridge/app instance built with the same fence store starts without the original in-memory state
3. stale traffic still fails with `reconnect_required`
4. fresh `session.resume` clears the restored fence
5. later device-bound traffic succeeds again

Coverage must exist for:

- bridge-level recovery test
- `session-gateway` restart-style test
- `local-minimal-node` restart-style test

## 9. Design Consequence

This standard freezes a pluggable persistence seam for disconnect semantics.

That enables:

- SaaS production backends to bind durable stores
- private deployments to swap storage implementations
- future route/session recovery work to compose with the same access-plane lifecycle rules

## 10. Non-Goals

This standard still does not freeze:

- which durable storage vendor must be used
- route ownership epoch recovery
- full crash-safe route migration semantics
- node-id normalization in recovered reconnect-required errors
