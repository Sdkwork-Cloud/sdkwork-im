# Getting Started

This section is for engineers, integrators, and operators who need to run the current Craw Chat
repository with the fewest surprises.

## What You Get

- A runnable `local-minimal-node` app surface for local development and verification.
- A formal `craw-chat-server` lifecycle that uses the unified `web-gateway` as the default
  external entrypoint.
- Scripted install, config, start, stop, restart, status, and Docker bootstrap entry points.
- Frozen `server.yaml` and PostgreSQL config templates for packaged server installs.
- OpenAPI-style API documentation that tracks the implemented HTTP surface.
- Clear boundaries between app APIs, control-plane governance APIs, and SDK workspaces.

## Supported Runtime Modes

| Mode | Entry points | Best use | Current status |
| --- | --- | --- | --- |
| Local binary | `bin/install-local.*`, `bin/start-local.*`, `bin/status-local.*` | Development, debugging, runtime inspection, restore rehearsal | Recommended |
| Docker Compose | `bin/deploy-local.*`, `deployments/scripts/bootstrap-local.ps1` | Local stack demos, container validation, smoke automation | Recommended |
| Unified server lifecycle | `bin/install-server.*`, `bin/start-server.*`, `bin/verify-server.*` | Production-style single-port installs, service-manager integration, and release-contract validation | Implemented |
| Standalone control plane | `cargo run -p control-plane-api --offline` or the compiled binary | Governance API development and admin integration | Implemented, but not part of the default one-command local stack |

## Prerequisites

### Required

- Rust toolchain with `cargo`
- One supported shell workflow:
  - PowerShell 7+
  - Bash
  - Windows CMD

### Optional

- Docker with the Docker Compose plugin
- Node.js and `npm`
  Needed only when you want to build or preview the VitePress docs site under `docs/sites`.
- PostgreSQL
  Required when you are validating the packaged `craw-chat-server` storage contract instead of the
  local file-backed development profile.

## Runtime Profiles

| Profile | Config file | Runtime contract | Notes |
| --- | --- | --- | --- |
| `local-minimal` | `.runtime/local-minimal/config/local-minimal.env` | Owns the current runtime directory contract | The only complete closed-loop local profile |
| `local-default` | `.runtime/local-default/config/local-default.env` | Still falls back to `.runtime/local-minimal` unless a dedicated runtime directory is configured | Compatibility and naming layer for the next local topology phase |

The fallback behavior is not documentation guesswork. It is implemented in the shared
`bin/_runtime-profile-common.ps1` profile helper and mirrored across the shell wrappers.

## Auth Boundary You Need To Know

### App-facing public routes

The public app surface uses bearer-token auth:

- Header: `Authorization: Bearer <token>`
- Signing secret: `CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET`

Trusted internal headers such as `x-tenant-id`, `x-user-id`, and `x-session-id` are still used in
tests and embedded compositions, but they are not the canonical public SDK auth model.

### Control-plane routes

The control-plane surface also uses public bearer auth and adds permission checks:

- `control.read` for read operations
- `control.write` for mutating operations

The implementation allows `control.write` to satisfy read access as well.

## SDK Entry Targets

- `sdkwork-craw-chat-sdk` maps to the app-facing routes. Local development points at
  `local-minimal-node`; packaged installs point at the unified `craw-chat-server` /
  `web-gateway` public origin.
- `sdkwork-craw-chat-sdk-admin` maps to governance and control-plane routes. Standalone governance
  development can point directly at `control-plane-api`; packaged installs should switch to the
  unified gateway public origin.
- `sdkwork-craw-chat-sdk-management` maps to the deployed `/api/admin/*` surface. In packaged
  installs that surface is also reached through the unified gateway public origin.

## Where To Start

- Want the shortest runnable path: [Quick Start](/getting-started/quick-start)
- Want the runtime model first: [Architecture Overview](/architecture/overview)
- Want the packaged server install contract: [Server Lifecycle](/deployment/server-lifecycle)
- Want the full endpoint inventory: [API Reference](/api-reference/index)
- Want to understand SDK reality before promising packages: [SDK Overview](/sdk/index)
