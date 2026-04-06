# 86. Disconnect Clears Live Realtime Subscriptions Standard (2026-04-06)

## 1. Goal

When a device explicitly calls `session.disconnect`, the platform must stop future live realtime delivery to that device while preserving durable recovery through `sync-feed`.

## 2. Problem Boundary

This standard applies when all conditions hold:

- the device already owns live realtime subscriptions
- the device explicitly calls:
  - `POST /api/v1/sessions/disconnect`
- later business events are committed after that disconnect

The standard governs only the live realtime delivery boundary. It does not yet define route release.

## 3. Required Rule

`session.disconnect` must clear the device's live realtime subscriptions before returning the offline presence snapshot.

That means:

1. the device presence snapshot transitions to `offline`
2. future realtime publishes must no longer match the disconnected device through the live subscription table
3. durable recovery state must remain intact

Required preserved state:

- device `sync-feed`
- latest durable sync sequence
- realtime checkpoint / ack metadata already persisted for observability or recovery

## 4. Delivery Semantics After Disconnect

After a successful `session.disconnect`:

- new committed business events may still enter durable `sync-feed`
- new committed business events must not be appended to the device's live realtime window through existing subscriptions

This freezes the distinction:

- realtime window:
  - online low-latency downlink only
- `sync-feed`:
  - durable recovery path

## 5. Covered Paths

At minimum this rule applies to:

- `POST /api/v1/sessions/disconnect`
- any internal handler that implements the same explicit device disconnect semantic

## 6. Minimal Implementation Rule

The minimal implementation may satisfy this standard by revoking the device's live subscription set during disconnect.

This standard does not yet require:

- explicit route deletion
- route downgrade markers
- transport cutover orchestration
- durable route store

Those remain future hardening layers.

## 7. Relationship To Resume And Realtime Sync

- `session.resume` still establishes or refreshes online device activity
- `POST /api/v1/realtime/subscriptions/sync` still defines which scopes receive live realtime delivery
- `session.disconnect` revokes the existing live subscription effect for that device

So the platform must not treat an explicitly disconnected device as still live-subscribed only because old subscription rows remain in memory.

## 8. Verification Standard

Regression coverage must prove:

1. a device syncs realtime subscriptions
2. the same device calls `session.disconnect`
3. another committed event happens afterward
4. `/api/v1/realtime/events` for that device shows no new live event from the post-disconnect publish
5. `/api/v1/devices/{deviceId}/sync-feed` still exposes the durable event

Coverage must exist at both levels:

- runtime unit test
- end-to-end HTTP or node integration test

## 9. Design Consequence

This standard prevents a false-offline state where:

- presence says `offline`
- but the device still receives new live realtime delivery

That mismatch is not acceptable for a commercial IM platform because it collapses the boundary between:

- online ephemeral delivery
- durable post-disconnect recovery

## 10. Non-Goals

This standard still does not freeze:

- explicit route release on disconnect
- post-disconnect mandatory resume before any device-bound follow-up request
- websocket graceful close protocol
- durable route epoch fencing
