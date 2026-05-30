# 101. Local-Minimal Presence Runtime Persistence Standard (2026-04-06)

## 1. Goal

The managed `local-minimal` deployment profile must preserve presence device inventory and last-observed timestamps across process restart without requiring Redis, Kafka, or any external persistence service.

This standard freezes the runtime-dir-backed private-deployment contract for presence recovery.

## 2. Scope

This standard applies to:

- `local-minimal-node` managed runtime-dir builders
- `session-gateway` presence runtime persistence seams
- device inventory recovery after rebuild
- `presence.me` recovery after rebuild
- restart-time fresh-resume enforcement for stale device traffic
- presence-store failure handling

This standard does not redefine SaaS production storage choices.

## 3. Required Runtime Layout

Managed local-minimal runtime directories must include:

- `.runtime/local-minimal/config`
- `.runtime/local-minimal/logs`
- `.runtime/local-minimal/pids`
- `.runtime/local-minimal/state`

The durable presence state path is standardized as:

```text
<runtime-dir>/state/presence-state.json
```

## 4. Store Contract

The platform must persist presence behind a pluggable `PresenceStateStore` seam keyed by:

- `tenant_id`
- `principal_id`
- `device_id`

The store must also support principal-scope listing by:

- `tenant_id`
- `principal_id`

At minimum, the durable record must preserve:

- the full `DevicePresenceView`
- `resume_required`
- `updated_at`

## 5. Builder Contract

When a runtime dir is configured, managed builders must bind `FilePresenceStateStore` at:

```text
<runtime-dir>/state/presence-state.json
```

Default unmanaged/in-process builders may continue to use a memory-backed store.

## 6. Recovery Rule

`DevicePresenceRuntime` must restore presence lazily on access.

Typical sequences:

1. process restarts with the same runtime dir
2. a client calls:
   - `GET /im/v3/api/presence/me`
   - `POST /im/v3/api/presence/heartbeat`
   - `POST /im/v3/api/device/sessions/resume`
3. runtime loads persisted state for `tenant + principal`
4. restored records become in-memory runtime entries
5. the operation continues against restored state

Global startup replay is not required for presence state.

## 7. Restart Normalization Rule

Presence persistence is allowed to recover query history, but it must not claim that a pre-restart live session is still active.

If a restored record was previously `online`, runtime must normalize it to:

- `status = offline`
- `session_id = null`
- preserved `lastResumeAt`
- preserved `lastSeenAt`
- preserved `lastSyncSeq`
- `resume_required = true`

This normalization may happen lazily during restore.

## 8. Fresh Resume Safety Rule

While a device is marked `resume_required`, non-resume device-bound traffic must fail closed with:

- `reconnect_required`

This includes flows such as:

- `presence.heartbeat`
- `register_device`
- equivalent device bootstrap paths that are not `session.resume`

Only a successful `POST /im/v3/api/device/sessions/resume` may clear the restart-time resume requirement.

## 9. Mutation Contract

The durable presence store must be updated on:

- explicit device registration placeholder creation
- `session.resume`
- `presence.heartbeat`
- `session.disconnect`

Persistence must record enough state so restart does not break:

- device inventory continuity
- `presence.me` query continuity
- restart-time stale-session safety

## 10. Failure Semantics

Store failures must never panic the process.

Runtime errors must be surfaced as controlled API errors:

- `presence_store_unavailable -> 503`
- `presence_store_conflict -> 409`
- `presence_store_unsupported -> 501`
- `reconnect_required -> 409`

The platform must fail closed instead of silently degrading to empty or misleading presence state.

## 11. Verification Standard

Regression coverage must prove:

1. the local file adapter persists presence state across reopen
2. a fresh `DevicePresenceRuntime` with the same store restores previous device state as `offline`
3. restored `online` entries require a fresh resume before heartbeat succeeds
4. a managed `local-minimal` rebuild with the same runtime dir restores device inventory and timestamps
5. the managed profile writes `presence-state.json` under the runtime state dir

## 12. Composition Rule

This standard composes with:

- [95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md](./95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md)
- [96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md](./96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md)
- [97-local-minimal-stream-runtime-persistence-standard-2026-04-06.md](./97-local-minimal-stream-runtime-persistence-standard-2026-04-06.md)
- [98-local-minimal-rtc-runtime-persistence-standard-2026-04-06.md](./98-local-minimal-rtc-runtime-persistence-standard-2026-04-06.md)
- [99-local-minimal-notification-runtime-persistence-standard-2026-04-06.md](./99-local-minimal-notification-runtime-persistence-standard-2026-04-06.md)
- [100-local-minimal-automation-runtime-persistence-standard-2026-04-06.md](./100-local-minimal-automation-runtime-persistence-standard-2026-04-06.md)

The composition outcome is:

- conversation-domain truth survives restart
- live subscription intent survives restart
- stream runtime state survives restart
- RTC runtime state survives restart
- notification and automation projections survive restart
- presence inventory and timestamps survive restart
- stale pre-restart presence does not silently become live traffic without a fresh resume

## 13. Design Consequence

Presence durability remains a replaceable runtime seam instead of being hidden inside domain replay, route caches, or projection caches.

Future storage replacement must remain behind `PresenceStateStore`.

Operator inspection and repair workflows are standardized separately by:

- [102-local-minimal-runtime-dir-inspection-repair-standard-2026-04-06.md](./102-local-minimal-runtime-dir-inspection-repair-standard-2026-04-06.md)
- [103-local-minimal-runtime-dir-semantic-validation-standard-2026-04-06.md](./103-local-minimal-runtime-dir-semantic-validation-standard-2026-04-06.md)
