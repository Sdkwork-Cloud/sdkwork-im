# Runtime Topology

This page describes the topology that is implemented today, not a future target topology.

## IM Open-Platform Topology

```text
Client / SDK / CLI
        |
        v
  local-minimal-node (:18090)
        |
        +-- session / presence / realtime
        +-- presence / realtime route coordination
        +-- inbox / conversations / membership / messages
        +-- media / streams / RTC
        +-- notifications / automation / audit / ops
        +-- provider health and sdkwork-aiot bridge routes
        |
        v
  .runtime/local-minimal/
    config/
    logs/
    pids/
    state/*
```

## Unified Server Topology

```text
External Client / Operator / SDK
              |
              v
      web-gateway / craw-chat-server (:18080 by server template)
              |
              +-- /healthz / /readyz
              +-- /openapi.json / /openapi/index.json / /openapi/runtime-summary.json
              +-- /docs / /docs/services/<service-id>
              +-- service-schema proxies
              +-- websocket route ownership on the same external port
              |
              +--> IM open-platform upstream routes
              |
              +--> control-plane and operator-facing upstream routes
```

## Public Route Domains

### Unified gateway discovery domains

- `/openapi.json`
- `/openapi/index.json`
- `/openapi/runtime-summary.json`
- `/openapi/services/*`
- `/docs`
- `/docs/services/*`

### IM open-platform domains

- `/im/v3/api/presence/*`
- `/im/v3/api/realtime/*`
- `/im/v3/api/chat/inbox`
- `/im/v3/api/chat/conversations/*`
- `/im/v3/api/chat/messages/*`
- `/im/v3/api/media/*`
- `/im/v3/api/streams/*`
- `/im/v3/api/rtc/*`

### Platform and operator domains exposed by the same app node

- `/app/v3/api/notifications/*`
- `/app/v3/api/automation/*`
- `/app/v3/api/iot/*`
- `/app/v3/api/principal/profiles/provider_health`
- `/app/v3/api/media/provider_health`
- `/backend/v3/api/iot/*`
- `/backend/v3/api/audit/*`
- `/backend/v3/api/ops/*`

## Control-plane Topology

```text
Admin Client / Admin Integration
            |
            v
      control-plane-api (:18081)
            |
            +-- protocol registry
            +-- protocol governance
            +-- provider registry / bindings / policies
            +-- node drain / activate / route migration
```

The control plane is parallel to the app node. It is not a nested route tree inside
`local-minimal-node`.

## Packaged Server Contract

The formal packaged server install flow uses the unified gateway as the operator-facing entrypoint:

- binary: `craw-chat-server`
- package source: `services/web-gateway`
- startup contract: `craw-chat-server --config <config-root>/server.yaml`
- frozen server template bind: `0.0.0.0:18080`
- storage baseline: PostgreSQL through `storage/postgresql.yaml`

## Default Provider Wiring

| Domain | Selected plugin | Source of truth |
| --- | --- | --- |
| `rtc` | `rtc-volcengine` | Platform default provider registry |
| `object-storage` | `object-storage-volcengine` | Platform default provider registry plus media and RTC tests |
| `principal-profile` | `principal-profile-upstream-context` | Upstream-context default; `principal-profile-external-catalog` is the read-only external catalog mode |

## Profile Matrix

| Item | `local-minimal` | `local-default` |
| --- | --- | --- |
| Config file | `.runtime/local-minimal/config/local-minimal.env` | `.runtime/local-default/config/local-default.env` |
| Effective runtime dir | `.runtime/local-minimal` | Falls back to `.runtime/local-minimal` by default |
| Docker Compose | `deployments/docker-compose/local-minimal.yml` | `deployments/docker-compose/local-default.yml` extends `local-minimal.yml` |
| Topology status | Complete | Compatibility layer over the current `local-minimal` topology |

## Auth Topology

### Public deployment path

Public runtime entry uses `build_public_app()`:

- `/healthz` and `/readyz` stay open
- the rest of the public app surface consumes verified SDKWork AppContext projection
- token validation, login, IAM sessions, tenant, user, and organization context are owned by `sdkwork-appbase`

### Embedded and test path

Internal tests and embedded compositions use `build_default_app()` with `x-sdkwork-*` AppContext
projection headers. That is an implementation and testing convenience layer; public clients still
authenticate through SDKWork appbase.

## What To Read Next

- [Architecture Overview](/architecture/overview)
- [Profiles and Environment](/deployment/profiles-and-env)
- [Runtime Directory](/reference/runtime-directory)
