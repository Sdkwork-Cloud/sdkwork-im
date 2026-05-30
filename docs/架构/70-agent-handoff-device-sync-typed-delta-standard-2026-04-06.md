# 70. Agent Handoff Device Sync Typed Delta Standard (2026-04-06)

## 1. Objective

After the baseline propagation standard was completed, `agent_handoff` already reached sync-feed, realtime HTTP, and websocket push.

That is still not sufficient for a commercial multi-device IM system if the sync-feed entry only exposes a weak `summary` hint.

The sync-feed contract must allow an offline client to reconstruct the lifecycle transition without guessing from UI-oriented fields.

## 2. Canonical Event

This standard applies to:

- `conversation.agent_handoff_status_changed`

Canonical runtime payload schema:

- `conversation.agent_handoff_status_changed.v1`

## 3. Device Sync Rule

For every delivered `conversation.agent_handoff_status_changed` sync entry, `DeviceSyncFeedEntry` must contain:

- `originEventType = conversation.agent_handoff_status_changed`
- `conversationId`
- `actorId = changedBy.id`
- `actorKind = changedBy.kind`
- `occurredAt = changedAt`
- `payloadSchema = conversation.agent_handoff_status_changed.v1`
- `payload = canonical runtime payload json`

Optional compatibility hint:

- `summary = current status`

The hint field may remain for lightweight clients, but it must not be the only durable business carrier.

## 4. Payload Rule

The sync-feed payload must be copied verbatim from the runtime commit envelope:

- do not reconstruct from projected summary state
- do not flatten into partial fields
- do not emit a second handoff-specific payload schema

This keeps:

- journal truth
- realtime payload
- websocket push payload
- sync-feed payload

on one canonical event contract.

## 5. Why This Standard Exists

If sync-feed exposes only:

- `summary = accepted`

then the client cannot know from sync-feed alone:

- previous status
- changedBy
- state source / target topology
- accepted / resolved / closed attribution fields
- handoff session metadata

That forces clients to guess, re-fetch other views, or pin behavior to non-contractual projection details.

For a commercial platform, offline resume must be self-sufficient.

## 6. Implementation Standard

`projection-service` must:

- preserve current member-device fanout logic
- preserve active-member fallback semantics
- keep `summary` as an additive compatibility field
- copy:
  - `event.payload_schema`
  - `event.payload`
  into the sync entry

The projection layer must not produce a divergent handoff-specific JSON shape.

## 7. Test Standard

At minimum, implementations must carry:

1. a projection test proving handoff sync entries expose `payloadSchema + payload`
2. an end-to-end node test proving `/im/v3/api/devices/{deviceId}/sync-feed` exposes the same typed payload after a real handoff transition

Recommended assertions:

- `conversationId`
- `currentStatus`
- `changedBy.id`
- `state.status`

## 8. Compatibility Rule

This standard is additive.

Existing clients that only use:

- `originEventType`
- `summary`

continue to work.

New clients may upgrade to typed delta consumption without requiring another server contract change.

## 9. Follow-Up Boundary

This standard does not yet require every sync-feed event to carry typed payload.

That broader rule should be decided explicitly after auditing:

- `agent_dialog`
- `system_channel`
- other business events that already cross the journal -> projection -> realtime chain
