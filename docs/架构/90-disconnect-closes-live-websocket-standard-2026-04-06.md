# 90. Disconnect Closes Live WebSocket Standard (2026-04-06)

## 1. Goal

When a device explicitly calls `session.disconnect`, any already-open realtime websocket transport for that device must be closed promptly by the server.

## 2. Problem Boundary

This standard applies when:

- a device already has an active websocket connection through `/api/v1/realtime/ws`
- the same device successfully calls `POST /api/v1/sessions/disconnect`

This standard complements:

- Standard 86: clear live subscriptions
- Standard 87: release route ownership
- Standard 88: require fresh resume for later device-bound requests
- Standard 89: keep duplicate disconnect idempotent

## 3. Required Rule

After a successful `session.disconnect`:

1. live subscriptions must be cleared
2. route ownership must be released
3. reconnect-required fence must be active
4. any currently-open websocket transport for that device must receive a server-initiated close and terminate promptly

The platform is not allowed to leave an already-open websocket attached after disconnect and wait for the client to notice on its own.

## 4. Scope Contract

The websocket close applies to currently-open transports scoped by:

- `tenant_id`
- `principal_id`
- `device_id`

The close signal is transport-scoped but must derive from the same device lifecycle semantics as the reconnect fence.

## 5. Minimal Implementation Rule

The minimal implementation may satisfy this standard by:

1. tracking a per-device websocket disconnect generation or equivalent close signal
2. subscribing live websocket handlers to that signal
3. sending a websocket close frame when disconnect is signaled
4. refusing to continue processing late frames from a socket opened before the disconnect generation changed

This standard does not yet require a vendor-stable websocket close reason contract.

## 6. Verification Standard

Regression coverage must prove:

1. websocket connects successfully for a device
2. the same device calls `session.disconnect`
3. the websocket receives a server close frame within a bounded timeout

Coverage must exist for:

- session-gateway websocket smoke path
- local-minimal integrated websocket path

## 7. Design Consequence

This standard aligns transport lifecycle with access-plane lifecycle.

Without it, the system can claim:

- presence offline
- route released
- reconnect required

while still keeping a live websocket transport open. That mismatch is not acceptable in a production IM platform.

## 8. Non-Goals

This standard still does not freeze:

- durable reconnect fence persistence
- a stable public websocket close code/reason contract
- crash-recovery route epochs or transport resurrection semantics
