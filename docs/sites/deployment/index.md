# Deployment

This section covers the deployment and operational entry points that are actually implemented in the
repository today.

## Deployment Modes

| Mode | Entry points | Best use |
| --- | --- | --- |
| Local binary | `bin/install-local.*`, `bin/start-local.*`, `bin/status-local.*` | Development, debugging, runtime inspection, restore rehearsal |
| Docker Compose | `bin/deploy-local.*`, `deployments/scripts/bootstrap-local.ps1` | Container validation, demos, smoke automation |
| Standalone control plane | `cargo run -p control-plane-api --offline` or the compiled binary | Governance and admin API development |
| Unified server lifecycle | `bin/install-server.*`, `bin/init-config-server.*`, `bin/init-storage-server.*`, `bin/verify-server.*`, `bin/install-service-server.*`, `bin/start-server.*` | Packaged single-port installs, PostgreSQL-backed config roots, and service-manager delivery |

## How To Choose A Path

### For development and debugging

Use [Local Binary](/deployment/local-binary):

- easiest access to runtime logs and PID files
- cleanest path for runtime inspection, repair, preview, and restore
- no container indirection during debugging

### For container validation and demos

Use [Docker](/deployment/docker):

- `local-minimal` Compose profile is fully defined
- `local-default` Compose profile already exists as a compatibility layer
- smoke verification is built into the bootstrap workflow

### For governance-only work

Run the control-plane service directly:

```bash
cargo run -p control-plane-api --offline
```

The current control-plane binary binds `127.0.0.1:18081`.

## Current Profile Boundary

::: warning Current profile boundary
`local-minimal` is the only complete closed-loop local profile. `local-default` already has
scripts, templates, and Compose entry points, but it still reuses the current
`local-minimal` runtime contract and topology.
::: 

The current packaged server boundary is the unified gateway and the canonical binary is
`sdkwork-im-server`.

In the current server boundary section, the canonical binary is `sdkwork-im-server`.

## What To Read Next

- [Local Binary](/deployment/local-binary)
- [Docker](/deployment/docker)
- [Server Lifecycle](/deployment/server-lifecycle)
- [Profiles and Environment](/deployment/profiles-and-env)
- [Runtime Operations](/deployment/runtime-operations)
