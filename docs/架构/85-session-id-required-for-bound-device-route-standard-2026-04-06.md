# 85. Session ID Required For Bound Device Route Standard (2026-04-06)

## 1. Goal

Once a device route is already owned by a current session, any further non-resume device-bound request must prove that same session identity. Omitting `session_id` must not remain a bypass.

## 2. Problem Boundary

This standard applies when all conditions hold:

- route exists for:
  - `tenantId`
  - `principalId`
  - `deviceId`
- route already carries a current `sessionId`
- request is not a fresh `session.resume`
- request can implicitly register, rebind, or mutate access-plane device state

## 3. Required Rule

If a route already has a current `sessionId`, every non-resume device-bound request must present a `session_id`.

Validation order:

1. load current route
2. if route has no `sessionId`, allow current minimal behavior
3. if route has `sessionId` and request has no `session_id`:
   - reject
   - do not mutate route / presence / registration state
4. if route has `sessionId` and request `session_id` differs:
   - reject as stale
5. only matching current-session requests may proceed

## 4. Error Contract

When a current-session-owned route receives a device-bound request without `session_id`, the service must return:

- HTTP status:
  - `409 Conflict`
- code:
  - `session_id_required`

This is intentionally different from:

- `stale_session`

Reason:

- `stale_session` means the caller presented a non-current session
- `session_id_required` means the caller failed to present any session identity even though the route is session-owned

## 5. Covered Paths

At minimum this rule applies to:

- `POST /im/v3/api/devices/register`
- `POST /im/v3/api/presence/heartbeat`
- `POST /im/v3/api/device/sessions/disconnect`
- `GET /im/v3/api/realtime/events`
- `POST /im/v3/api/realtime/events/ack`
- websocket attach
- any gateway helper that implicitly ensures device registration before business writes

## 6. Relationship To Resume

`session.resume` remains the only request allowed to intentionally replace the current route session.

That means:

- resume may legitimately take over ownership with a new `sessionId`
- post-resume traffic may not omit session identity anymore

This freezes the distinction between:

- session establishment
- session-bound follow-up traffic

## 7. Local Minimal Profile Rule

The minimal profile may still allow pre-session bootstrap behavior only before a route has a current `sessionId`.

Examples that may remain valid in the minimal profile:

- first device registration before any live session exists
- diagnostics or bootstrap flows that do not yet own a session-bound route

But once a live session owns the route, session identity becomes mandatory.

## 8. Verification Standard

Regression coverage must prove:

1. device resumes with `s_old`
2. device resumes again with `s_new`
3. a sessionless device-bound request is rejected with `409 session_id_required`
4. route ownership remains with the `s_new` owner node
5. a matching `s_new` request still succeeds

Coverage must exist at both levels:

- shared cluster/session fence unit test
- end-to-end HTTP or node integration test

## 9. Design Consequence

This standard tightens the previous current-session fencing rule into a commercial-safe contract:

- current-session ownership cannot be bypassed by simply omitting session identity
- session establishment is explicit
- session-bound follow-up traffic is identity-bearing by construction

It is the necessary intermediate step before later work on:

- explicit route release
- durable route store
- lease / epoch fencing
- transport cutover orchestration
