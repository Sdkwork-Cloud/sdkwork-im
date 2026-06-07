# 94. Local-Minimal Runtime Checkpoint Persistence Standard (2026-04-06)

## 1. Goal

The managed `local-minimal` deployment profile must preserve realtime checkpoint truth across process restart without requiring Redis, Kafka, or any external persistence service.

This standard freezes the runtime-dir-backed private-deployment default for checkpoint recovery.

## 2. Scope

This standard applies to:

- `local-minimal-node` when launched through managed lifecycle scripts
- explicit runtime-dir builders used by installable private deployment
- restart and rebuild scenarios for realtime event polling and acknowledgement
- checkpoint-store failure handling in `session-gateway` and `local-minimal-node`

This standard does not redefine SaaS production storage choices.

## 3. Required Runtime Layout

Managed local-minimal runtime directories must include:

- `.runtime/local-minimal/config`
- `.runtime/local-minimal/logs`
- `.runtime/local-minimal/pids`
- `.runtime/local-minimal/state`

The realtime checkpoint file path is standardized as:

```text
<runtime-dir>/state/realtime-checkpoints.json
```

## 4. Runtime Configuration Contract

Managed startup must provide:

- `CRAW_CHAT_BIND_ADDR`
- `CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET`
- `CRAW_CHAT_RUNTIME_DIR`

`init-config-local.*` must write `CRAW_CHAT_RUNTIME_DIR` into `local-minimal.env`.

`start-local.*` must export `CRAW_CHAT_RUNTIME_DIR` before launching `local-minimal-node`.

## 5. Builder Contract

`local-minimal-node` must expose runtime-dir-aware builders:

- `build_default_app_with_runtime_dir(...)`
- `build_public_app_with_runtime_dir(...)`

When a runtime dir is configured, those builders must bind:

- `FileRealtimeCheckpointStore` at `<runtime-dir>/state/realtime-checkpoints.json`
- `FileRealtimeDisconnectFenceStore` at `<runtime-dir>/state/realtime-disconnect-fences.json`
- `FileRealtimeSubscriptionStore` at `<runtime-dir>/state/realtime-subscriptions.json` when Standard 96 is composed into the managed profile

Managed private deployment must use these durable bindings.

## 6. Fallback Rule

Default in-process builders may still use `MemoryRealtimeCheckpointStore` when no runtime dir is configured.

This fallback is allowed only for:

- isolated tests
- unmanaged developer entrypoints
- non-persistent in-process embedding

It is not the required behavior for managed private deployment.

## 7. Persisted Recovery Boundary

This checkpoint standard persists only realtime delivery truth for a device scope:

- `latest_realtime_seq`
- `acked_through_seq`
- `trimmed_through_seq`
- `updated_at`

This standard does **not** yet require automatic persistence of:

- conversation aggregates
- conversation memberships
- live realtime subscription sets
- stream/RTC/session domain state outside the checkpoint record itself

After rebuild, this checkpoint standard by itself restores checkpoint truth only.

Managed private deployment may additionally compose:

- [95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md](./95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md)
- [96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md](./96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md)
- [97-local-minimal-stream-runtime-persistence-standard-2026-04-06.md](./97-local-minimal-stream-runtime-persistence-standard-2026-04-06.md)

When those standards are enabled together with this one:

- Standard 95 rebuilds conversation-domain context automatically
- Standard 96 restores live subscription intent on fresh client route bootstrap
- Standard 97 restores stream runtime state on demand after rebuild

That boundary is intentional and must be documented clearly in tests and deployment expectations.

## 8. Failure Semantics

Checkpoint store failures must never panic the process.

Checkpoint load/save errors must be converted into a controlled runtime or cluster error:

```text
checkpoint_store_unavailable
```

HTTP adapters must surface that condition as:

- `503 Service Unavailable`

The platform must fail closed instead of silently dropping checkpoint protection or crashing the process.

## 9. Verification Standard

Regression coverage must prove:

1. a managed local-minimal runtime writes `realtime-checkpoints.json` under the runtime state dir
2. a rebuilt app using the same runtime dir restores `ackedThroughSeq` and `trimmedThroughSeq`
3. the next delivered event continues from the restored sequence boundary instead of resetting to `1`
4. checkpoint store load/save failures surface as controlled `503` errors
5. checkpoint tests do not implicitly assume conversation or subscription persistence unless that durability is explicitly added by another standard such as Standard 95 or Standard 96

## 10. Design Consequence

This standard gives private deployment a restart-safe realtime checkpoint baseline while keeping the adapter seam pluggable and vendor-neutral.

It also prevents architectural ambiguity:

- checkpoint durability is part of the access-plane recovery contract now
- broader conversation-domain cold-start recovery is standardized separately by Standard 95
- live subscription bootstrap recovery is standardized separately by Standard 96
- stream runtime persistence is standardized separately by Standard 97

That separation preserves modularity and allows future replacement of the local file adapter without rewriting the higher-level runtime contract.
