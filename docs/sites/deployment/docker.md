# Docker

Docker deployment is the recommended path for local container validation and demo-style bring-up.
It is not the formal packaged `sdkwork-im-server` install contract.

## Compose Profiles

| Profile | Compose file | Current status |
| --- | --- | --- |
| `local-minimal` | `deployments/docker-compose/local-minimal.yml` | Fully defined |
| `local-default` | `deployments/docker-compose/local-default.yml` | Compatibility layer that extends `local-minimal.yml` |

## `local-minimal` Compose Facts

The repo Compose profile currently sets:

- container name: `sdkwork-im-local-minimal`
- `SDKWORK_IM_BIND_ADDR=0.0.0.0:18090`
- `SDKWORK_IM_FRIEND_REQUEST_CURSOR_HS256_SECRET=local-minimal-friend-request-cursor-dev-secret`
- `SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE=true`
- `SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET=local-minimal-app-context-signature-dev-secret`
- port mapping: `18090:18090`
- healthcheck: `curl -fsS http://127.0.0.1:18090/healthz`

## Recommended Commands

### PowerShell wrapper

```powershell
./bin/deploy-local.ps1 -ProfileName local-minimal
./bin/deploy-local.ps1 -ProfileName local-default -SmokeBaseUrl http://127.0.0.1:28090
./bin/deploy-local.ps1 -Help
```

### Direct bootstrap

```powershell
powershell -ExecutionPolicy Bypass -File deployments\scripts\bootstrap-local.ps1 -ProfileName local-minimal
```

### Direct Compose

```bash
docker compose -f deployments/docker-compose/local-minimal.yml up -d --build
```

## What The Bootstrap Script Does

`deployments/scripts/bootstrap-local.ps1`:

1. verifies the Docker CLI
2. verifies the Docker daemon
3. verifies the Docker Compose plugin
4. runs `docker compose -f <profile>.yml up -d --build`
5. runs smoke verification unless `-SkipSmoke` is passed
6. collects `docker compose ps` and `docker compose logs --tail 200` on failure

## Smoke Behavior

By default the Docker bootstrap calls `tools/smoke/local_stack_smoke.ps1`, which:

- waits for `/healthz`
- sends SDKWork dual-token headers: `Authorization: Bearer <auth-token>` and `Access-Token: <access-token>`
- creates a conversation
- posts a message
- verifies the conversation summary path

## Current Boundary

`local-default.yml` currently contains only an `extends` relationship:

```yaml
services:
  local-minimal-node:
    extends:
      file: local-minimal.yml
      service: local-minimal-node
```

So it is a profile compatibility layer, not a separate image, port layout, or service graph.

For the single-port packaged server, config root layout, PostgreSQL baseline, and service-management
wrappers, use [Server Lifecycle](/deployment/server-lifecycle).

For the single-port packaged server, config root layout, PostgreSQL baseline, and service-management wrappers, use [Server Lifecycle](/deployment/server-lifecycle).

If you are validating a production-style install shape with the unified `web-gateway` entrypoint,
runtime OpenAPI discovery, or PostgreSQL-backed storage configuration, switch to
[Server Lifecycle](/deployment/server-lifecycle).

If you are validating a production-style install shape with the unified `web-gateway` entrypoint, runtime OpenAPI discovery, or PostgreSQL-backed storage configuration, switch to [Server Lifecycle](/deployment/server-lifecycle).

## What To Read Next

- [Deployment](/deployment/index)
- [Profiles and Environment](/deployment/profiles-and-env)
- [Quick Start](/getting-started/quick-start)
