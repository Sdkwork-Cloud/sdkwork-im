# 99. Local-Minimal Notification Runtime Persistence Standard (2026-04-06)

## 1. Goal

The managed `local-minimal` deployment profile must preserve notification task projections across process restart without requiring Redis, Kafka, or any external persistence service.

This standard freezes the runtime-dir-backed private-deployment contract for notification task recovery.

## 2. Scope

This standard applies to:

- `local-minimal-node` managed runtime-dir builders
- `notification-service` runtime projection persistence
- notification point lookup recovery after rebuild
- notification recipient-list recovery after rebuild
- notification store failure handling

This standard does not redefine SaaS production storage choices.

## 3. Required Runtime Layout

Managed local-minimal runtime directories must include:

- `.runtime/local-minimal/config`
- `.runtime/local-minimal/logs`
- `.runtime/local-minimal/pids`
- `.runtime/local-minimal/state`

The durable notification projection path is standardized as:

```text
<runtime-dir>/state/notification-tasks.json
```

## 4. Store Contract

The platform must persist notification projections behind a pluggable `NotificationTaskStore` seam keyed by:

- `tenant_id`
- `notification_id`

The store must also support recipient-scope listing by:

- `tenant_id`
- `recipient_id`

At minimum, the durable record must preserve:

- the full `NotificationTask`
- `updated_at`

## 5. Builder Contract

When a runtime dir is configured, managed builders must bind `FileNotificationTaskStore` at:

```text
<runtime-dir>/state/notification-tasks.json
```

Default unmanaged/in-process builders may continue to use a memory-backed store.

## 6. Recovery Rule

`NotificationRuntime` must restore notification projections lazily on access.

Typical sequences:

1. process restarts with the same runtime dir
2. a client calls:
   - `GET /api/v1/notifications`
   - `GET /api/v1/notifications/{id}`
   - `POST /api/v1/notifications/requests`
3. runtime loads persisted state for:
   - `tenant + notification_id` on direct lookup and request idempotency
   - `tenant + recipient_id` on recipient list queries
4. the operation continues against restored in-memory state

Global startup replay is not required for notification task projections.

## 7. Mutation Contract

The durable notification projection must be updated after successful final task mutation in:

- `request_notification(...)`

Persistence must record the final dispatched task state so restart does not break:

- idempotent request behavior
- recipient notification list continuity
- direct notification detail lookup

## 8. Failure Semantics

Store failures must never panic the process.

Runtime errors must be surfaced as controlled API errors:

- `notification_store_unavailable -> 503`
- `notification_store_conflict -> 409`
- `notification_store_unsupported -> 501`

The platform must fail closed instead of silently degrading to empty notification state.

## 9. Verification Standard

Regression coverage must prove:

1. the local file adapter persists notification tasks across reopen
2. a fresh `NotificationRuntime` with the same store restores recipient-query continuity
3. a managed `local-minimal` rebuild with the same runtime dir restores notification list and direct lookup behavior
4. the managed profile writes `notification-tasks.json` under the runtime state dir

## 10. Composition Rule

This standard composes with:

- [95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md](./95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md)
- [96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md](./96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md)
- [97-local-minimal-stream-runtime-persistence-standard-2026-04-06.md](./97-local-minimal-stream-runtime-persistence-standard-2026-04-06.md)
- [98-local-minimal-rtc-runtime-persistence-standard-2026-04-06.md](./98-local-minimal-rtc-runtime-persistence-standard-2026-04-06.md)
- [101-local-minimal-presence-runtime-persistence-standard-2026-04-06.md](./101-local-minimal-presence-runtime-persistence-standard-2026-04-06.md)

The composition outcome is:

- conversation-domain truth survives restart
- realtime live intent survives restart
- stream runtime state survives restart
- RTC runtime state survives restart
- presence inventory and timestamps survive restart
- notification task query surfaces survive restart

## 11. Design Consequence

Notification durability remains a replaceable runtime seam instead of being hidden inside domain replay or hard-coded to one file format.

Future storage replacement must remain behind `NotificationTaskStore`.

Operator inspection and repair workflows are standardized separately by:

- [102-local-minimal-runtime-dir-inspection-repair-standard-2026-04-06.md](./102-local-minimal-runtime-dir-inspection-repair-standard-2026-04-06.md)
- [103-local-minimal-runtime-dir-semantic-validation-standard-2026-04-06.md](./103-local-minimal-runtime-dir-semantic-validation-standard-2026-04-06.md)
