# 69. Member Governance Device Sync Delta Standard (2026-04-06)

## 1. Objective

Member governance is not complete when it only reaches:

1. commit journal
2. member snapshot
3. realtime HTTP poll
4. websocket push

A commercial IM system must also provide durable offline compensation through `device sync feed`.

For these canonical events:

- `conversation.member_joined`
- `conversation.member_role_changed`
- `conversation.member_removed`
- `conversation.member_left`

the sync-feed layer must emit replayable roster deltas.

## 2. Envelope Standard

Member governance does **not** get a second feed type.

It reuses the unified `DeviceSyncFeedEntry` envelope and extends it additively with optional fields:

- `actorKind`
- `payloadSchema`
- `payload`

Required sync entry fields for member governance:

- `originEventType`
- `conversationId`
- `memberId`
- `actorId`
- `actorKind`
- `occurredAt`
- `payloadSchema`
- `payload`

The old flat fields remain valid. This is a forward-only additive contract.

## 3. Payload Rule

### 3.1 Join / Remove / Leave

The sync entry must carry:

- `payloadSchema = conversation.member.v1`
- `payload = durable ConversationMember JSON`

Example payload:

```json
{
  "tenantId": "t_demo",
  "conversationId": "c_demo",
  "memberId": "cm_c_demo_u_member",
  "principalId": "u_member",
  "principalKind": "user",
  "role": "member",
  "state": "removed",
  "invitedBy": "u_owner",
  "joinedAt": "2026-04-06T12:01:00Z",
  "removedAt": "2026-04-06T12:03:00Z",
  "attributes": {}
}
```

### 3.2 Role Change

The sync entry must carry:

- `payloadSchema = conversation.member_role_changed.v1`
- `payload = durable role-change JSON`

Example payload:

```json
{
  "tenantId": "t_demo",
  "conversationId": "c_demo",
  "previousMember": {
    "memberId": "cm_c_demo_u_member",
    "role": "member"
  },
  "updatedMember": {
    "memberId": "cm_c_demo_u_member",
    "role": "admin"
  },
  "changedAt": "2026-04-06T12:02:00Z"
}
```

## 4. Actor Rule

Actor information must come from the commit envelope actor, not from guessed roster state.

Required fields:

- `actorId = event.actor.actor_id`
- `actorKind = event.actor.actor_kind`

This matters because the durable payload for `conversation.member.v1` does not itself contain the actor.

## 5. Recipient Rule

### 5.1 Active member rule

For `conversation.member_joined` and `conversation.member_role_changed`, the sync delta must be appended to every registered device of every current active member.

### 5.2 Affected principal fallback

For `conversation.member_removed` and `conversation.member_left`, the affected principal must still receive the sync delta on their other registered devices even though they are no longer active after mutation.

This is mandatory.

A pure post-mutation active-member query is insufficient.

## 6. Implementation Rule

### 6.1 Projection layer

`projection-service` must:

- update the member snapshot first
- append sync-feed deltas from the same durable event
- copy `payloadSchema` and `payload` from the durable commit envelope verbatim

### 6.2 Endpoint layer

`local-minimal-node` and future gateways must expose the projection result through:

- `GET /api/v1/devices/{deviceId}/sync-feed`

without inventing a second roster-sync path.

## 7. Compatibility Rule

This standard is intentionally additive:

- existing sync consumers that ignore unknown fields continue to work
- new consumers can opt into typed roster-delta handling
- other event types may keep `payloadSchema = null` and `payload = null` until they are explicitly promoted

## 8. Test Standard

Every implementation profile that claims member-governance sync support must include:

1. a projection test proving join/role-change/remove/left create sync entries
2. a regression test proving the removed principal still receives `conversation.member_removed`
3. an e2e HTTP sync-feed test proving the serialized contract exposes:
   - `actorKind`
   - `payloadSchema`
   - `payload`

## 9. Non-Goals

This standard does not yet require:

- typed payload sync entries for every existing event type
- a separate roster-only feed
- a sync-feed backfill migration strategy for historical entries

Those can be layered later without weakening the member-governance guarantee frozen here.
