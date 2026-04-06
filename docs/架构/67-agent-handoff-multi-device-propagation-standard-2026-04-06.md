# 67. Agent Handoff Multi-Device Propagation Standard (2026-04-06)

## 1. Objective

`agent_handoff` lifecycle changes are not complete when they only update runtime state or summary views.

For a commercial IM platform, a lifecycle transition is complete only when it reaches all of these layers:

1. commit journal
2. read model (`summary`, `inbox`)
3. device sync feed
4. realtime HTTP event window
5. websocket push

## 2. Canonical Event

The canonical lifecycle event is:

- `conversation.agent_handoff_status_changed`

Allowed transition examples:

- `open -> accepted`
- `accepted -> resolved`
- `open|accepted|resolved -> closed`

## 3. Propagation Rules

### 3.1 Journal Rule

The runtime must append `conversation.agent_handoff_status_changed` to the commit journal whenever the lifecycle state mutates.

Idempotent requests must not append a new event.

### 3.2 Read Model Rule

The projection layer must update:

- `conversation summary`
- `inbox entry`

from the event payload `state`, not from guessed incremental logic.

### 3.3 Device Sync Rule

The projection layer must append a sync entry for every active conversation member device.

Required sync entry fields:

- `originEventType = conversation.agent_handoff_status_changed`
- `conversationId`
- `actorId = changedBy.id`
- `occurredAt = changedAt`

Recommended generic hint field:

- `summary = current handoff status`

If the conversation has no projected active members yet, fallback delivery may target the changing actor so the event is not silently dropped.

### 3.4 Realtime Rule

The gateway/node layer must publish a realtime business event for every non-idempotent lifecycle change.

Event routing:

- `scopeType = conversation`
- `scopeId = conversationId`
- recipients = all current conversation members' registered devices

### 3.5 Websocket Rule

Websocket push is not a special event path. It must receive the same event through the same realtime window mechanism used by HTTP polling.

If HTTP realtime can see the event but websocket push cannot, the implementation is still incomplete.

## 4. Payload Contract

Realtime payload must match the runtime lifecycle payload shape:

```json
{
  "tenantId": "t_demo",
  "conversationId": "c_demo",
  "previousStatus": "open",
  "currentStatus": "accepted",
  "changedBy": {
    "id": "u_demo",
    "kind": "user"
  },
  "changedAt": "2026-04-06T11:01:00Z",
  "state": {
    "tenantId": "t_demo",
    "conversationId": "c_demo",
    "status": "accepted",
    "source": {
      "id": "ag_source",
      "kind": "agent"
    },
    "target": {
      "id": "u_demo",
      "kind": "user"
    },
    "handoffSessionId": "hs_demo",
    "handoffReason": "manual_escalation",
    "acceptedAt": "2026-04-06T11:01:00Z",
    "acceptedBy": {
      "id": "u_demo",
      "kind": "user"
    },
    "resolvedAt": null,
    "resolvedBy": null,
    "closedAt": null,
    "closedBy": null
  }
}
```

## 5. Implementation Standard

### 5.1 Projection Service

`projection-service` must:

- parse `changedBy`
- parse `changedAt`
- update summary/inbox
- append sync feed entries for active members

### 5.2 Access Node / Gateway

`local-minimal-node` or any future gateway must:

- read the pre-change handoff state
- execute the lifecycle command
- compare previous and current state
- publish realtime only when the state actually changed

This prevents false duplicate push on idempotent requests.

## 6. Test Standard

Every implementation profile must carry at least these tests:

1. projection test proving sync-feed fanout for lifecycle change
2. HTTP e2e test proving subscribed device window receives the lifecycle event
3. websocket e2e test proving push delivery receives the same lifecycle event

## 7. Non-Goals

This standard does not yet require:

- a dedicated typed sync envelope for handoff lifecycle entries
- admin-only lifecycle read models
- analytics dashboards for handoff transitions

Those can be layered later without weakening the propagation guarantee defined here.
