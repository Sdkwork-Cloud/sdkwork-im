# 93. Local-Minimal Runtime Disconnect Fence Persistence Standard (2026-04-06)

## 1. Goal

The managed `local-minimal` deployment profile must preserve reconnect-required fences across process restart without requiring external infrastructure.

This standard builds on Standard 92 by freezing the runtime-dir-backed private-deployment default.

## 2. Scope

This standard applies to:

- `local-minimal-node` when launched through managed lifecycle scripts
- explicit runtime-dir builders used by local/private deployment packaging
- restart and rebuild scenarios after `POST /im/v3/api/device/sessions/disconnect`

This standard does not redefine SaaS production storage choices.

## 3. Required Runtime Layout

Managed local-minimal runtime directories must include:

- `.runtime/local-minimal/config`
- `.runtime/local-minimal/logs`
- `.runtime/local-minimal/pids`
- `.runtime/local-minimal/state`

The disconnect fence file path is standardized as:

```text
<runtime-dir>/state/realtime-disconnect-fences.json
```

## 4. Runtime Configuration Contract

Managed startup must provide:

- `CRAW_CHAT_BIND_ADDR`
- `CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET`
- `CRAW_CHAT_RUNTIME_DIR`

`init-config-local.*` must write `CRAW_CHAT_RUNTIME_DIR` into `local-minimal.env`.

`start-local.*` must export `CRAW_CHAT_RUNTIME_DIR` to the process environment before launching `local-minimal-node`.

## 5. Builder Contract

`local-minimal-node` must expose explicit runtime-dir-aware builders:

- `build_default_app_with_runtime_dir(...)`
- `build_public_app_with_runtime_dir(...)`

When a runtime dir is configured, those builders must bind a file-backed `RealtimeDisconnectFenceStore`.

Managed deployment paths must use that durable binding.

## 6. Fallback Rule

Default in-process builders may fall back to in-memory fence state when no runtime dir is configured.

This fallback is allowed only for:

- isolated tests
- unmanaged developer entrypoints
- non-persistent in-process embedding

It is not the required behavior for managed private deployment.

## 7. Failure Semantics

Disconnect fence store failures must not panic the process.

The bridge must convert fence store load/save/clear failures into a controlled cluster error:

```text
disconnect_fence_store_unavailable
```

HTTP adapters must surface that condition as:

- `503 Service Unavailable`

The platform must fail closed instead of silently dropping reconnect-required protection.

## 8. Verification Standard

Regression coverage must prove:

1. a managed local-minimal runtime writes a disconnect fence file under the runtime state dir
2. a rebuilt app using the same runtime dir still rejects stale device-bound traffic with `reconnect_required`
3. a fresh `session.resume` clears the persisted fence
4. fence store failures surface as controlled errors instead of panics
5. lifecycle scripts materialize and export the runtime-dir contract

## 9. Design Consequence

This standard gives private deployment a restart-safe reconnect fence baseline while keeping the adapter boundary vendor-neutral.

That enables future waves to add:

- local checkpoint persistence
- route/session epoch recovery
- richer crash recovery for minimal private deployment

without rewriting the access-plane contract again.
