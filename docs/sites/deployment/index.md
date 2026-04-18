# Deployment

This section covers the deployment and operational entry points that are actually implemented in the
repository today.

## Deployment Modes

| Mode | Entry points | Best use |
| --- | --- | --- |
| Local binary | `bin/install-local.*`, `bin/start-local.*`, `bin/status-local.*` | Development, debugging, runtime inspection, restore rehearsal |
| Unified server lifecycle | `bin/install-server.*`, `bin/init-config-server.*`, `bin/init-storage-server.*`, `bin/verify-server.*`, `bin/install-service-server.*`, `bin/start-server.*` | Production-style install, PostgreSQL-backed config validation, and single-port service management |
| Docker Compose | `bin/deploy-local.*`, `deployments/scripts/bootstrap-local.ps1` | Container validation, demos, smoke automation |
| Standalone control plane | `cargo run -p control-plane-api --offline` or the compiled binary | Governance and admin API development |

## Recommended Paths

### For development and debugging

Use [Local Binary](/deployment/local-binary):

- easiest access to runtime logs and PID files
- cleanest path for runtime inspection, repair, preview, and restore
- no container indirection during debugging

### For packaged server installs and service managers

Use [Server Lifecycle](/deployment/server-lifecycle):

- `craw-chat-server` is the canonical packaged binary published from `services/web-gateway`
- `server.yaml` freezes the unified gateway bind and public endpoint contract
- PostgreSQL is the frozen storage baseline for the server install flow
- generated `systemd`, `launchd`, and Windows Service wrapper contracts come from the same startup command

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

## Current Server Boundary

The formal packaged server contract is separate from the local development profile:

- the canonical binary is `craw-chat-server`
- the startup contract is `craw-chat-server --config <config-root>/server.yaml`
- the default frozen server template binds `0.0.0.0:18080`
- the operator-facing discovery surface includes `/healthz`, `/readyz`, `/openapi.json`,
  `/openapi/index.json`, and `/docs`
- PostgreSQL is the frozen storage baseline through `storage/postgresql.yaml`

## Next Pages

- [Local Binary](/deployment/local-binary)
- [Server Lifecycle](/deployment/server-lifecycle)
- [Docker](/deployment/docker)
- [Profiles and Environment](/deployment/profiles-and-env)
- [Runtime Operations](/deployment/runtime-operations)
