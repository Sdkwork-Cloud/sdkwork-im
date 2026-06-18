# Quick Start

This is the shortest verified path to a working local Sdkwork IM development stack.

## Prerequisites

- Rust toolchain
- Node.js 22 + pnpm 10
- Sibling checkout: `sdkwork-api-gateway` (platform plane)

## 1. Install Dependencies

```bash
pnpm install
```

## 2. Start the Development Stack

```bash
pnpm im:dev
```

This loads topology profile `self-hosted.split-services.development` from
`configs/topology/self-hosted.split-services.development.env`.

Default listeners:

| Surface | URL |
| --- | --- |
| IM application ingress | `http://127.0.0.1:18079` |
| Platform API gateway | `http://127.0.0.1:3900` |
| PC renderer (when started) | `http://127.0.0.1:4176` |

Single-process smoke:

```bash
pnpm im:dev:unified
```

Server only (no PC renderer):

```bash
pnpm server:dev
```

## 3. Verify Health

```bash
curl http://127.0.0.1:18079/healthz
curl http://127.0.0.1:18079/readyz
```

## 4. Auth Expectations

Public and smoke requests use SDKWork dual-token headers:

- `Authorization: Bearer <auth-token>`
- `Access-Token: <access-token>`

Tenant, user, session, device, actor, and permission context comes from those token claims. Do not
send client-controlled identity projection headers.

## 5. First SDK Integration

For local app-SDK integration against the development profile, use
`baseUrl = http://127.0.0.1:18079`.

The public app surface is documented in [App API Overview](/api-reference/app-api).

## 6. Packaged Server Install

If you need the production-style single-port server contract instead of the development orchestrator,
use [Server Lifecycle](/deployment/server-lifecycle).

## What To Read Next

- [Deployment](/deployment/index)
- [Runtime Topology](/architecture/runtime-topology)
- [API Reference](/api-reference/index)
- [SDK Overview](/sdk/index)
