# 96. Local-Minimal Live Subscription Bootstrap Recovery Standard (2026-04-06)

## 1. Goal

The managed `local-minimal` deployment profile must restore device-scope live realtime subscription intent across process restart without requiring Redis, Kafka, or any external persistence service.

This standard freezes the runtime-dir-backed bootstrap recovery contract for live realtime subscription matching in private deployment.

## 2. Scope

This standard applies to:

- `local-minimal-node` when launched through managed runtime-dir lifecycle
- device-scope realtime subscription persistence
- fresh bootstrap flows such as:
  - `session.resume`
  - `register_device`
  - equivalent route/device rebind flows
- runtime and HTTP failure handling for subscription-store access

This standard does not redefine SaaS production storage choices.

## 3. Required Runtime Layout

Managed local-minimal runtime directories must include:

- `.runtime/local-minimal/config`
- `.runtime/local-minimal/logs`
- `.runtime/local-minimal/pids`
- `.runtime/local-minimal/state`

The durable subscription file path is standardized as:

```text
<runtime-dir>/state/realtime-subscriptions.json
```

## 4. Builder Contract

`local-minimal-node` must expose runtime-dir-aware builders:

- `build_default_app_with_runtime_dir(...)`
- `build_public_app_with_runtime_dir(...)`

When a runtime dir is configured, those builders must bind:

- `FileRealtimeCheckpointStore` at `<runtime-dir>/state/realtime-checkpoints.json`
- `FileRealtimeDisconnectFenceStore` at `<runtime-dir>/state/realtime-disconnect-fences.json`
- `FileRealtimeSubscriptionStore` at `<runtime-dir>/state/realtime-subscriptions.json`

Managed private deployment must use this durable subscription binding.

Default unmanaged/in-process builders may remain memory-backed.

## 5. Recovery Contract

The platform must persist device-scope realtime subscription intent behind a pluggable `RealtimeSubscriptionStore` seam.

At minimum, the durable record must preserve:

- `tenant_id`
- `principal_id`
- `device_id`
- `items`
- `synced_at`

## 6. Lazy Bootstrap Rule

Subscriptions must be restored lazily during device bootstrap, not globally at process startup.

The required sequence is:

1. the process restarts with the same runtime dir
2. the device performs a legitimate fresh bootstrap such as `session.resume`
3. runtime bootstrap calls `ensure_device_state(...)`
4. `ensure_device_state(...)` restores:
   - checkpoint truth
   - persisted subscriptions
5. future `publish_scope_event(...)` calls can match the restored subscription set without another explicit `subscriptions.sync`

## 7. Safety Rule

This standard must **not** revive dead connections, stale websocket sessions, or obsolete route bindings after restart.

Allowed behavior:

- restore subscription intent for a device that legitimately re-enters

Forbidden behavior:

- auto-resurrecting a socket connection
- silently binding a stale route without a new bootstrap
- bypassing disconnect-fence / route-session safety checks

## 8. Mutation Contract

The durable subscription store must be updated on:

- `sync_subscriptions(...)`
- `clear_device_subscriptions(...)`
- `restore_device_state(...)` when route migration restores a snapshot

An explicit disconnect that clears subscriptions must also clear the durable subscription record so restart does not silently resurrect cleared intent.

## 9. Failure Semantics

Subscription-store failures must never panic the process.

Runtime errors must use controlled codes:

```text
subscription_store_unavailable
subscription_store_conflict
subscription_store_unsupported
```

HTTP adapters must surface those conditions as:

- `503 Service Unavailable`
- `409 Conflict`
- `501 Not Implemented`

The platform must fail closed instead of silently dropping subscription durability.

## 10. Verification Standard

Regression coverage must prove:

1. the local file adapter persists device-scope subscriptions across reopen
2. a fresh runtime with the same stores restores subscriptions without another `sync_subscriptions(...)`
3. a managed local-minimal rebuild with the same runtime dir restores live delivery after a fresh `session.resume`
4. explicit subscription clearing prevents automatic restart resurrection
5. recovery remains lazy and bootstrap-driven rather than process-startup-driven

## 11. Composition Rule

This standard composes with:

- [94-local-minimal-runtime-checkpoint-persistence-standard-2026-04-06.md](./94-local-minimal-runtime-checkpoint-persistence-standard-2026-04-06.md)
- [95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md](./95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md)
- [97-local-minimal-stream-runtime-persistence-standard-2026-04-06.md](./97-local-minimal-stream-runtime-persistence-standard-2026-04-06.md)
- [98-local-minimal-rtc-runtime-persistence-standard-2026-04-06.md](./98-local-minimal-rtc-runtime-persistence-standard-2026-04-06.md)

The composition outcome is:

- Standard 94 restores checkpoint truth
- Standard 95 restores conversation-domain state
- Standard 96 restores live subscription intent on fresh device bootstrap
- Standard 97 restores stream session/frame state on demand after rebuild
- Standard 98 restores RTC session/signal state on demand after rebuild

The boundaries are complementary but intentionally separate.

## 12. Design Consequence

This standard closes the highest-value remaining private-deployment realtime restart gap while preserving vendor-neutral replaceability.

Future storage changes must remain behind the same `RealtimeSubscriptionStore` contract instead of rewriting the session or realtime API surface.
