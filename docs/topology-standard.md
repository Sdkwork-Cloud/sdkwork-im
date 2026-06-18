# Sdkwork IM Runtime Topology

Human-facing summary for IM. Machine contract: `../specs/topology.spec.json`.
Platform naming authority: `../../sdkwork-specs/APP_RUNTIME_TOPOLOGY_NAMING.md`.

## Archetype

**realtime-application-platform** — application HTTP/WebSocket ingress + platform API gateway.

## Default Profile

**self-hosted.split-services.development**

File: `configs/topology/self-hosted.split-services.development.env`

## Surfaces

| Surface id | Process | Client talks to |
| --- | --- | --- |
| `application.public-ingress` | `sdkwork-im-server` | IM OpenAPI + WebSocket |
| `platform.api-gateway` | `sdkwork-api-gateway` | IAM, Drive, Agent, AIoT REST |
| `operations.control-ingress` | (optional) | Operator APIs |

## Env Keys (development)

```bash
# Application plane — IM product APIs
SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND=127.0.0.1:18079
SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=http://127.0.0.1:18079
SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=ws://127.0.0.1:18079

# Platform plane — shared SDKWork APIs
SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=http://127.0.0.1:3900
SDKWORK_IM_PLATFORM_API_GATEWAY_AUTOSTART=true

# Client mirror (Vite)
VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=http://127.0.0.1:18079
VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=ws://127.0.0.1:18079
VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=http://127.0.0.1:3900
```

## Commands (target)

```bash
pnpm im:dev           # self-hosted.split-services.development
pnpm im:dev:unified   # self-hosted.unified-process.development
pnpm im:build         # cloud-hosted.split-services.production
```

## Cloud URLs (Pattern A)

Public application host for IM: **`im.sdkwork.com`**.

| Surface | Production URL |
| --- | --- |
| Application HTTP | `https://im.sdkwork.com` |
| Application WebSocket | `wss://im.sdkwork.com` (path `/im/v3/api/realtime/ws`) |
| Platform gateway | `https://api.sdkwork.com` |

`chat.sdkwork.com` is **not** IM — it is reserved for LLM conversational applications.

Example cloud profile env:

```bash
SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=https://im.sdkwork.com
SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=wss://im.sdkwork.com
SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=https://api.sdkwork.com
VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=https://im.sdkwork.com
VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=wss://im.sdkwork.com
VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=https://api.sdkwork.com
```

## Phrases for reviews

- "WebSocket terminates on **application.public-ingress**, not platform.api-gateway."
- "Foundation SDKs use **SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL** only."
- "This PR does not change the active **profile id**."

See `topology-greenfield.md` for delete list and migration plan.
