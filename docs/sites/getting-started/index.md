# Getting Started

This section is for engineers, integrators, and operators who need to run the current Craw Chat
repository with minimal surprises.

## What You Get

- A runnable `local-minimal-node` app surface for local development and verification.
- Scripted install, config, start, stop, restart, status, and Docker bootstrap entry points.
- OpenAPI-style API documentation that tracks the implemented HTTP surface.
- Clear boundaries between IM standard APIs, app-business APIs, backend control/admin APIs, and SDK workspaces.

## Supported Runtime Modes

| Mode | Entry points | Best use | Current status |
| --- | --- | --- | --- |
| Local binary | `bin/install-local.*`, `bin/start-local.*`, `bin/status-local.*` | Development, debugging, runtime inspection, restore rehearsal | Recommended |
| Docker Compose | `bin/deploy-local.*`, `deployments/scripts/bootstrap-local.ps1` | Local stack demos, container validation, smoke automation | Recommended |
| Standalone control plane | `cargo run -p control-plane-api --offline` or the compiled binary | Governance API development and admin integration | Implemented, but not part of the default one-command local stack |
| Unified server lifecycle | `bin/install-server.*`, `bin/start-server.*`, `bin/verify-server.*` | Packaged single-port installs, service-manager delivery, gateway-backed public origins | Implemented |

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

### IM Open-Platform Public Routes

Public clients authenticate through SDKWork appbase. `sdkwork-appbase` owns login, IAM sessions,
tenant/user/org context, and dual-token validation. `craw-chat` receives only the verified
AppContext projection:

- `x-sdkwork-tenant-id`
- `x-sdkwork-user-id`
- `x-sdkwork-session-id`
- `x-sdkwork-device-id`
- `x-sdkwork-permission-scope`

### Control-plane routes

The control-plane surface uses the same SDKWork AppContext boundary and adds permission checks:

- `control.read` for read operations
- `control.write` for mutating operations

The implementation also allows `control.write` to satisfy read access.

## What To Read Next

- Want the shortest verified path: [Quick Start](/getting-started/quick-start)
- Want the packaged server install contract: [Server Lifecycle](/deployment/server-lifecycle)
- Want the runtime model first: [Architecture Overview](/architecture/overview)
- Want the full endpoint inventory: [API Reference](/api-reference/index)
- Want to understand SDK reality before promising packages: [SDK Overview](/sdk/index)

The IM standard SDK family lives in `sdkwork-im-sdk`, with the public TypeScript package
`@sdkwork/im-sdk`, and maps to `/im/v3/api/*`.

`sdkwork-im-app-sdk` maps to `/app/v3/api/*` for app-business and non-management HTTP APIs outside
the IM standardized surface.

`sdkwork-im-backend-sdk` maps to `/backend/v3/api/*`. Control-plane governance under
`/backend/v3/api/control/*` and admin under `/backend/v3/api/admin/*` are backend SDK modules, not
separate SDK families.

`sdkwork-rtc-sdk` is independent from OpenAPI-generated HTTP SDKs and owns RTC provider runtime,
provider package, and native driver boundaries.
