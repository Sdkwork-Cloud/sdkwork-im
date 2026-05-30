# 89. Duplicate Disconnect Idempotency Standard (2026-04-06)

## 1. Goal

`session.disconnect` must remain safe to retry for the same session even after the platform has established a reconnect-required fence.

## 2. Problem Boundary

This standard applies when:

- a device has already completed a successful `POST /im/v3/api/device/sessions/disconnect`
- the same session retries `POST /im/v3/api/device/sessions/disconnect` again

This standard refines Standard 88. It is a narrow exception for the disconnect endpoint itself and does not relax reconnect-required behavior for ordinary non-`resume` device-bound requests.

## 3. Required Rule

After a successful disconnect has already established the reconnect-required fence:

1. the same session may retry `session.disconnect`
2. the retry must return success
3. the returned presence snapshot must remain `offline`
4. the retry must not recreate route ownership
5. the retry must not clear the reconnect-required fence

All other non-`resume` device-bound requests must still fail with `409 reconnect_required`.

## 4. Matching Contract

The duplicate-disconnect idempotent retry path is allowed only when:

- `tenant_id` matches
- `principal_id` matches
- `device_id` matches
- `session_id` matches the disconnect fence session

If the session does not match, the request remains governed by the reconnect-required rule.

## 5. Minimal Implementation Rule

The minimal implementation may satisfy this standard by:

1. checking whether a disconnect fence already exists for the device scope
2. checking whether that fence belongs to the same session
3. if so, bypassing the normal non-`resume` bind path
4. returning the existing offline presence snapshot directly

This bypass must not be reused by heartbeat, register, realtime poll, websocket attach, or any other device-bound request.

## 6. Verification Standard

Regression coverage must prove:

1. device resumes
2. device disconnects successfully
3. the same session retries disconnect
4. retry returns:
   - HTTP `200`
   - offline snapshot
5. other non-`resume` requests from that same pre-resume state still fail with `reconnect_required`

Coverage must exist in:

- access-plane service test
- integrated local profile test

## 7. Design Consequence

This standard preserves both sides of the contract:

- disconnect establishes a real reconnect boundary
- disconnect itself remains safe to retry under normal client/network uncertainty

Commercial IM clients and SDKs need both guarantees at the same time.

## 8. Non-Goals

This standard still does not freeze:

- durable reconnect fence persistence
- websocket graceful close acknowledgement semantics
- cross-process exactly-once disconnect write guarantees
