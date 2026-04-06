# 88. Post-Disconnect Resume Required Standard (2026-04-06)

## 1. Goal

When a device explicitly calls `session.disconnect`, the platform must require a fresh `session.resume` before any later device-bound request may re-establish route ownership or device activity.

## 2. Problem Boundary

This standard applies when:

- a device has already established session-bound activity
- the same device successfully calls:
  - `POST /api/v1/sessions/disconnect`

This standard governs all later device-bound requests that rely on the shared device bind path, including:

- `POST /api/v1/presence/heartbeat`
- `POST /api/v1/devices/register`
- `POST /api/v1/realtime/subscriptions`
- `GET /api/v1/realtime/events`
- `POST /api/v1/realtime/events/ack`
- websocket attach
- future stream, push, transport, or signaling endpoints that bind a device route

This standard does not automatically apply to durable read models that do not bind or recreate live route ownership, such as device sync-feed queries.

## 3. Required Rule

After a successful `session.disconnect`:

1. presence becomes `offline`
2. live realtime subscriptions are cleared
3. route ownership is released
4. a reconnect-required fence is recorded for that device scope
5. every later non-`resume` device-bound request must fail until a fresh `session.resume` succeeds

The platform is not allowed to silently recreate route ownership after disconnect through heartbeat, register, polling, websocket attach, or any other non-`resume` device-bound request.

## 4. Fence Scope Contract

The reconnect-required fence is scoped by:

- `tenant_id`
- `principal_id`
- `device_id`

The fence is therefore device-scoped, not transport-scoped. It must apply consistently across:

- HTTP request/response flows
- long polling
- websocket attach
- future stream-capable endpoints

## 5. Resume Contract

Only a successful `session.resume` may clear the reconnect-required fence.

Required behavior:

1. `session.resume` may bind or rebind the device route
2. the fence must be cleared only after the route bind succeeds
3. once cleared, later normal device-bound requests may proceed again under the resumed session

The fence must not be cleared preemptively before route bind success.

## 6. Error Contract

While the reconnect-required fence exists, every non-`resume` device-bound request must fail with:

- HTTP status: `409 Conflict`
- error code: `reconnect_required`

The contract must be stable across all access-plane services that share the device route bridge.

## 7. Minimal Implementation Rule

The minimal implementation may satisfy this standard by:

1. storing an in-memory reconnect fence in the shared cluster bridge
2. marking that fence during disconnect after route release
3. checking the fence before any non-`resume` device bind
4. clearing the fence after successful `session.resume`

This standard does not yet require durable persistence for the reconnect fence, but the behavior contract is frozen even if the storage implementation changes later.

## 8. Verification Standard

Regression coverage must prove:

1. device resumes with `session_id = s_old`
2. device calls `session.disconnect`
3. same old `session_id` sends a non-`resume` device-bound request
4. request fails with:
   - `409`
   - `reconnect_required`
5. device resumes again with a fresh session
6. later device-bound traffic succeeds again

Coverage must exist at both levels:

- shared cluster bridge unit test
- end-to-end service test

## 9. Design Consequence

This standard makes explicit disconnect a real lifecycle boundary.

That prevents an unacceptable state where:

- presence says the device is offline
- route ownership was already released
- but stale or bypass traffic can immediately recreate live ownership without an explicit resume

In a distributed commercial IM platform, disconnect must be authoritative across presence, routing, and realtime delivery semantics.

## 10. Non-Goals

This standard still does not freeze:

- durable reconnect-fence persistence across process restart
- websocket graceful close orchestration details
- durable route epochs or crash-recovery route ownership
