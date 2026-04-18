# Getting Started

This section is for engineers, integrators, and operators who need to run the current Craw Chat
repository with minimal surprises.

## What You Get

- A runnable `local-minimal-node` app surface for local development and verification.
- Scripted install, config, start, stop, restart, status, and Docker bootstrap entry points.
- OpenAPI-style API documentation that tracks the implemented HTTP surface.
- Clear boundaries between app APIs, control-plane governance APIs, and SDK workspaces.

## Supported Runtime Modes

| Mode | Entry points | Best use | Current status |
| --- | --- | --- | --- |
| Local binary | `bin/install-local.*`, `bin/start-local.*`, `bin/status-local.*` | Development, debugging, runtime inspection, restore rehearsal | Recommended |
| Docker Compose | `bin/deploy-local.*`, `deployments/scripts/bootstrap-local.ps1` | Local stack demos, container validation, smoke automation | Recommended |
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

The implementation also allows `control.write` to satisfy read access.

## What To Read Next

- Want the shortest verified path: [Quick Start](/getting-started/quick-start)
- Want the runtime model first: [Architecture Overview](/architecture/overview)
- Want the full endpoint inventory: [API Reference](/api-reference/index)
- Want to understand SDK reality before promising packages: [SDK Overview](/sdk/index)
