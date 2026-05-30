# 97. Local-Minimal Stream Runtime Persistence Standard (2026-04-06)

## 1. Goal

The managed `local-minimal` deployment profile must preserve stream runtime state across process restart without requiring Redis, Kafka, or any external persistence service.

This standard freezes the runtime-dir-backed private-deployment contract for stream session and frame recovery.

## 2. Scope

This standard applies to:

- `local-minimal-node` managed runtime-dir builders
- `streaming-service` runtime state persistence
- stream session recovery after rebuild
- stream frame recovery after rebuild
- stream store failure handling

This standard does not redefine SaaS production storage choices.

## 3. Required Runtime Layout

Managed local-minimal runtime directories must include:

- `.runtime/local-minimal/config`
- `.runtime/local-minimal/logs`
- `.runtime/local-minimal/pids`
- `.runtime/local-minimal/state`

The durable stream state file path is standardized as:

```text
<runtime-dir>/state/stream-state.json
```

## 4. Store Contract

The platform must persist stream runtime state behind a pluggable `StreamStateStore` seam keyed by:

- `tenant_id`
- `stream_id`

At minimum, the durable record must preserve:

- the full `StreamSession`
- ordered `StreamFrame[]`
- `updated_at`

## 5. Builder Contract

When a runtime dir is configured, managed builders must bind `FileStreamStateStore` at:

```text
<runtime-dir>/state/stream-state.json
```

Default unmanaged/in-process builders may continue to use a memory-backed store.

## 6. Recovery Rule

`StreamingRuntime` must restore stream state lazily on access.

Typical sequence:

1. process restarts with the same runtime dir
2. the client calls `GET /im/v3/api/streams/{id}/frames`
   - or `POST /checkpoint`
   - or `POST /complete`
   - or `POST /abort`
3. runtime loads persisted state for `tenant + stream_id`
4. the operation continues against the restored in-memory state

Global startup replay is not required for stream state.

## 7. Mutation Contract

The durable stream state must be updated on:

- `open_stream(...)`
- `append_frame(...)`
- `checkpoint_stream(...)`
- `complete_stream(...)`
- `abort_stream(...)`

Persistence must include both session metadata and frame history so list/query and continuation semantics remain consistent after restart.

## 8. Failure Semantics

Store failures must never panic the process.

Runtime errors must be surfaced as controlled API errors:

- `stream_store_unavailable -> 503`
- `stream_store_conflict -> 409`
- `stream_store_unsupported -> 501`

The platform must fail closed instead of silently degrading to empty stream state.

## 9. Verification Standard

Regression coverage must prove:

1. the local file adapter persists stream state across reopen
2. a fresh `StreamingRuntime` with the same store restores stream list/continuation behavior
3. a managed `local-minimal` rebuild with the same runtime dir restores stream frames and completion behavior
4. the managed profile writes `stream-state.json` under the runtime state dir

## 10. Composition Rule

This standard composes with:

- [94-local-minimal-runtime-checkpoint-persistence-standard-2026-04-06.md](./94-local-minimal-runtime-checkpoint-persistence-standard-2026-04-06.md)
- [95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md](./95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md)
- [96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md](./96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md)
- [98-local-minimal-rtc-runtime-persistence-standard-2026-04-06.md](./98-local-minimal-rtc-runtime-persistence-standard-2026-04-06.md)

The composition outcome is:

- Standard 94 restores realtime checkpoint truth
- Standard 95 restores conversation-domain state
- Standard 96 restores live subscription intent
- Standard 97 restores stream session/frame state
- Standard 98 restores RTC session/signal state

## 11. Design Consequence

This standard closes another major private-deployment restart gap while preserving replaceable storage boundaries.

Future storage replacement must remain behind `StreamStateStore` instead of coupling stream lifecycle semantics to one runtime-dir file format or one storage vendor.
