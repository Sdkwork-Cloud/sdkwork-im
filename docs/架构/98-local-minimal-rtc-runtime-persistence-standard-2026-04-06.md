# 98. Local-Minimal RTC Runtime Persistence Standard (2026-04-06)

## 1. Goal

The managed `local-minimal` deployment profile must preserve RTC runtime state across process restart without requiring Redis, Kafka, or any external persistence service.

This standard freezes the runtime-dir-backed private-deployment contract for RTC session and signal recovery.

## 2. Scope

This standard applies to:

- `local-minimal-node` managed runtime-dir builders
- `rtc-signaling-service` runtime state persistence
- RTC session recovery after rebuild
- RTC signal history recovery after rebuild
- RTC store failure handling

This standard does not redefine SaaS production storage choices.

## 3. Required Runtime Layout

Managed local-minimal runtime directories must include:

- `.runtime/local-minimal/config`
- `.runtime/local-minimal/logs`
- `.runtime/local-minimal/pids`
- `.runtime/local-minimal/state`

The durable RTC state file path is standardized as:

```text
<runtime-dir>/state/rtc-state.json
```

## 4. Store Contract

The platform must persist RTC runtime state behind a pluggable `RtcStateStore` seam keyed by:

- `tenant_id`
- `rtc_session_id`

At minimum, the durable record must preserve:

- the full `RtcSession`
- ordered `RtcSignalEvent[]`
- `updated_at`

## 5. Builder Contract

When a runtime dir is configured, managed builders must bind `FileRtcStateStore` at:

```text
<runtime-dir>/state/rtc-state.json
```

Default unmanaged/in-process builders may continue to use a memory-backed store.

## 6. Recovery Rule

`RtcRuntime` must restore RTC state lazily on access.

Typical sequence:

1. process restarts with the same runtime dir
2. the client calls:
   - `POST /im/v3/api/rtc/sessions/{id}/invite`
   - `POST /im/v3/api/rtc/sessions/{id}/accept`
   - `POST /im/v3/api/rtc/sessions/{id}/reject`
   - `POST /im/v3/api/rtc/sessions/{id}/end`
   - `POST /im/v3/api/rtc/sessions/{id}/signals`
3. runtime loads persisted state for `tenant + rtc_session_id`
4. the operation continues against the restored in-memory state

Global startup replay is not required for RTC state.

## 7. Mutation Contract

The durable RTC state must be updated on:

- `create_session(...)`
- `invite_session(...)`
- `accept_session(...)`
- `reject_session(...)`
- `end_session(...)`
- `post_signal(...)`

Persistence must include both session metadata and signal history so restart does not break signaling continuity or idempotent session semantics.

## 8. Failure Semantics

Store failures must never panic the process.

Runtime errors must be surfaced as controlled API errors:

- `rtc_store_unavailable -> 503`
- `rtc_store_conflict -> 409`
- `rtc_store_unsupported -> 501`

The platform must fail closed instead of silently degrading to empty RTC state.

## 9. Verification Standard

Regression coverage must prove:

1. the local file adapter persists RTC state across reopen
2. a fresh `RtcRuntime` with the same store restores session and signal continuation behavior
3. a managed `local-minimal` rebuild with the same runtime dir restores conversation-bound RTC continuation behavior
4. the managed profile writes `rtc-state.json` under the runtime state dir

## 10. Composition Rule

This standard composes with:

- [94-local-minimal-runtime-checkpoint-persistence-standard-2026-04-06.md](./94-local-minimal-runtime-checkpoint-persistence-standard-2026-04-06.md)
- [95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md](./95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md)
- [96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md](./96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md)
- [97-local-minimal-stream-runtime-persistence-standard-2026-04-06.md](./97-local-minimal-stream-runtime-persistence-standard-2026-04-06.md)
- [99-local-minimal-notification-runtime-persistence-standard-2026-04-06.md](./99-local-minimal-notification-runtime-persistence-standard-2026-04-06.md)
- [100-local-minimal-automation-runtime-persistence-standard-2026-04-06.md](./100-local-minimal-automation-runtime-persistence-standard-2026-04-06.md)

The composition outcome is:

- Standard 94 restores realtime checkpoint truth
- Standard 95 restores conversation-domain state
- Standard 96 restores live subscription intent
- Standard 97 restores stream session/frame state
- Standard 98 restores RTC session/signal state
- Standard 99 restores notification task query state
- Standard 100 restores automation execution query state

## 11. Design Consequence

This standard closes another major private-deployment restart gap while preserving replaceable storage boundaries.

Future storage replacement must remain behind `RtcStateStore` instead of coupling RTC lifecycle semantics to one runtime-dir file format or one storage vendor.
