# Getting Started

This section is for engineers, integrators, and operators who need to run Sdkwork IM with topology v2
defaults and minimal surprises.

## What You Get

- Topology v2 development orchestration via `pnpm im:dev` and `pnpm server:dev`.
- Application ingress at `sdkwork-im-server` plus platform plane via sibling `sdkwork-api-gateway`.
- OpenAPI-style API documentation aligned to the implemented HTTP surface.
- Clear boundaries between IM standard APIs, app-business APIs, backend control/admin APIs, and SDK workspaces.

## Supported Runtime Modes

| Mode | Entry points | Best use |
| --- | --- | --- |
| Development stack | `pnpm im:dev`, `pnpm im:dev:unified`, `pnpm server:dev` | Local development, PC integration, smoke |
| Packaged server | `bin/install-server.*`, `bin/start-server.*`, `bin/verify-server.*` | Production-style single-port installs |
| Standalone control plane | `cargo run -p governance-service --offline` | Governance API development |

## Prerequisites

- Rust toolchain with `cargo`
- Node.js 22 + pnpm 10
- Sibling checkout: `sdkwork-api-gateway`

## Runtime Profiles

Authority: `specs/topology.spec.json` and `configs/topology/*.env`.

| Profile id | Command | Application ingress |
| --- | --- | --- |
| `self-hosted.split-services.development` | `pnpm im:dev` | `http://127.0.0.1:18079` |
| `self-hosted.unified-process.development` | `pnpm im:dev:unified` | `http://127.0.0.1:18079` |

## Auth Boundary

Public clients authenticate through SDKWork dual-token headers:
`Authorization: Bearer <auth-token>` and `Access-Token: <access-token>`.

Control-plane routes require `control.read` or `control.write` permissions from AppContext projection.

## What To Read Next

- [Quick Start](/getting-started/quick-start)
- [Server Lifecycle](/deployment/server-lifecycle)
- [Architecture Overview](/architecture/overview)
- [API Reference](/api-reference/index)
- [SDK Overview](/sdk/index)
