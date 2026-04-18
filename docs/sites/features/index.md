# Feature Overview

This section answers two questions:

1. What can the current application do today?
2. Which capabilities are part of the main delivery path versus governance, operations, or SDK
   planning boundaries?

## Capability Domains

### App collaboration

- conversation creation
- agent dialog, handoff, and system-channel flows
- membership changes, ownership transfer, role changes, and leave
- message send, edit, recall, and timeline reads
- inbox, conversation summary, and read-cursor views

### Realtime and session delivery

- session resume and disconnect
- presence heartbeat and current-presence reads
- realtime subscription sync, poll windows, ack flow, and websocket upgrade
- device registration and sync-feed reads

### Media, streaming, and RTC

- media upload initiation, completion, lookup, signed download URL, and attachment
- stream open, frame append, list, checkpoint, complete, and abort
- RTC create, invite, accept, reject, end, signal, participant credential, recording artifact, and provider callback

### Platform and operations

- notifications and automation execution
- audit record creation, listing, and export
- ops health, cluster, lag, replay status, runtime directory, provider bindings, drift, and diagnostics

### Governance and extension

- provider health for media, RTC, user-module, IoT access, and IoT protocol
- IoT uplink and downlink ingestion
- protocol registry and governance
- provider registry and policy lifecycle
- node drain, activate, and route migration

## Maturity Model Used In These Docs

::: info Documentation maturity rule
Capability maturity here means "implemented and verifiable in the current repository". It does not
mean roadmap intent or naming-only presence.
:::

- Implemented runtime capability: exposed as a route or script and backed by tests or executable
  runtime wiring.
- Implemented governance capability: exposed by the control plane and validated by control-plane
  tests.
- Materialized SDK boundary: checked-in OpenAPI authority, generated packages, or assembled
  workspace metadata already exist in-repo even when publication is still pending.
- Scaffolded SDK boundary: the workspace contract is reserved and named, but one or more language
  packages are still placeholder-only or pending generator output.

Continue with [Capability Matrix](/features/capabilities) for the evidence-oriented breakdown.
