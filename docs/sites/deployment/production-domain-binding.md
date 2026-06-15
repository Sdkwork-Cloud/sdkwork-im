# Production Domain Binding

This page freezes the Sdkwork IM production domain contract for the unified gateway, the `sdkwork-im-pc` Vite build, and browser CORS.

Canonical application identity:

- app code: `chat`
- public mount root: `/sdkwork/chat`
- standard server env prefix: `SDKWORK_IM_*`

## Supported Shapes

### Same-origin Web And API

Use this shape when the browser app, HTTP APIs, OpenAPI docs, and realtime WebSocket gateway are served from the same public origin and mounted under `/sdkwork/chat`.

```dotenv
SDKWORK_IM_SERVER_BASE_URL=https://chat.example.com/sdkwork/chat
SDKWORK_IM_SERVER_API_BASE_URL=https://chat.example.com/sdkwork/chat
SDKWORK_IM_SERVER_WEBSOCKET_BASE_URL=wss://chat.example.com/sdkwork/chat
SDKWORK_IM_BROWSER_ORIGINS=https://chat.example.com
```

When no explicit Vite API base URLs are configured for a web release build, the PC app resolves:

- App SDK HTTP base URL from `window.location.origin`
- IM SDK HTTP base URL from `window.location.origin`
- IM SDK WebSocket base URL from `ws://` or `wss://` plus `window.location.host`

For installed server packages, prefer the explicit `SDKWORK_IM_SERVER_*` values above so generated server config, logs, docs, and release manifests all point at the SDKWork mount root.

### Split Web, API, And Realtime Domains

Use this shape when the web app is served from one origin and the API gateway is exposed on a separate domain.

```dotenv
SDKWORK_IM_SERVER_BASE_URL=https://chat.example.com/sdkwork/chat
SDKWORK_IM_SERVER_API_BASE_URL=https://api.chat.example.com/sdkwork/chat
SDKWORK_IM_SERVER_WEBSOCKET_BASE_URL=wss://realtime.chat.example.com/sdkwork/chat
SDKWORK_IM_BROWSER_ORIGINS=https://chat.example.com
```

`SDKWORK_IM_BROWSER_ORIGINS` must contain every browser origin that is allowed to call the public gateway. Do not put API paths in this value; it is a comma-separated list of origins.

## SDK Base URL Semantics

The generated SDKs own their API prefixes:

| SDK | Env source | SDK-owned prefix |
| --- | --- | --- |
| `sdkwork-im-app-sdk` / `@sdkwork-internal/im-app-api-generated` | `SDKWORK_IM_SERVER_API_BASE_URL` or `SDKWORK_IM_APP_API_BASE_URL` | `/app/v3/api` |
| `@sdkwork/im-sdk` | `SDKWORK_IM_SERVER_API_BASE_URL` or `SDKWORK_IM_IM_API_BASE_URL` | `/im/v3/api` |
| `@sdkwork/im-sdk` realtime | `SDKWORK_IM_SERVER_WEBSOCKET_BASE_URL` or `SDKWORK_IM_IM_WEBSOCKET_BASE_URL` | `/im/v3/api/realtime/ws` |

Set the environment variables to the gateway mount root. Do not set them to the full generated SDK endpoint.

Correct:

```dotenv
SDKWORK_IM_SERVER_API_BASE_URL=https://api.chat.example.com/sdkwork/chat
SDKWORK_IM_SERVER_WEBSOCKET_BASE_URL=wss://realtime.chat.example.com/sdkwork/chat
```

Wrong:

```dotenv
SDKWORK_IM_SERVER_API_BASE_URL=https://api.chat.example.com/sdkwork/chat/im/v3/api
SDKWORK_IM_SERVER_WEBSOCKET_BASE_URL=wss://realtime.chat.example.com/sdkwork/chat/im/v3/api/realtime/ws
```

The release resolver and frontend SDK wrappers strip known SDK-owned suffixes as a defensive compatibility layer, but deployment templates should keep the base URL semantics above.

Legacy `SDKWORK_IM_SERVER_API_BASE_URL`, `SDKWORK_IM_SERVER_BASE_URL`, and `SDKWORK_IM_SERVER_WEBSOCKET_BASE_URL` are compatibility inputs for older scripts. New production config should use `SDKWORK_IM_SERVER_*`.

## Vite Release Build Contract

The canonical release command is:

```bash
pnpm release:build
```

It runs `scripts/release/run-sdkwork-im-pc-release-build.mjs`, which:

- prepares git-backed shared SDK sources via `prepare:shared-sdk`
- forces `SDKWORK_SHARED_SDK_MODE=git`
- resolves IAM mode as `server-private`
- converts `SDKWORK_IM_SERVER_API_BASE_URL` and `SDKWORK_IM_SERVER_WEBSOCKET_BASE_URL` into the matching `VITE_SDKWORK_IM_*` variables for Vite
- fails early if an explicitly configured release URL is not an absolute `http(s)` or `ws(s)` URL

If `SDKWORK_IM_SERVER_WEBSOCKET_BASE_URL` is omitted but `SDKWORK_IM_SERVER_API_BASE_URL` is set, the resolver derives the WebSocket base URL by converting `https://` to `wss://` and `http://` to `ws://`.

## Runtime Gateway Binding

The Rust gateway/runtime recognizes these public base URL envs:

- `SDKWORK_IM_PORTAL_API_BASE_URL`
- `SDKWORK_PORTAL_API_BASE_URL`
- `SDKWORK_IM_SERVER_API_BASE_URL`
- `SDKWORK_IM_SERVER_BASE_URL`
- `SDKWORK_IM_SERVER_API_BASE_URL` as a compatibility fallback
- `SDKWORK_IM_SERVER_BASE_URL` as a compatibility fallback
- `SDKWORK_IM_BIND_ADDR` as a local fallback only

Explicit public URLs must not use bind-only hosts such as `0.0.0.0` or `::`. Use a browser-reachable URL instead.

## PC Auxiliary API

The PC app has auxiliary `/api/...` routes for non-IM product features. In the Rust product runtime:

- `GET /api/config/modules` is served locally
- `/api/agent/*` is proxied only when `SDKWORK_IM_PC_API_UPSTREAM` is configured
- missing auxiliary upstreams return structured `503` responses instead of silently falling back to the SPA shell

Set `SDKWORK_IM_PC_API_UPSTREAM` when a production AI/agent backend is deployed.
