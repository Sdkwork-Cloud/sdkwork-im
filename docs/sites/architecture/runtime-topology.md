# Runtime Topology

This page describes the topology that is implemented now, not a future target topology.

## App-facing Topology

```text
Client / SDK / CLI
        |
        v
  local-minimal-node (:18090)
        |
        +-- session / presence / realtime
        +-- device registration / sync feed
        +-- inbox / conversations / membership / messages
        +-- media / streams / RTC
        +-- notifications / automation / audit / ops
        +-- provider health and IoT protocol ingress
        |
        v
  .runtime/local-minimal/
    config/
    logs/
    pids/
    state/*
```

## Public Route Domains

### App-facing domains

- `/api/v1/sessions/*`
- `/api/v1/presence/*`
- `/api/v1/realtime/*`
- `/api/v1/devices/*`
- `/api/v1/inbox`
- `/api/v1/conversations/*`
- `/api/v1/messages/*`
- `/api/v1/media/*`
- `/api/v1/streams/*`
- `/api/v1/rtc/*`

### Platform and operator domains exposed by the same app node

- `/api/v1/notifications/*`
- `/api/v1/automation/*`
- `/api/v1/audit/*`
- `/api/v1/ops/*`
- `/api/v1/user-module/provider-health`
- `/api/v1/media/provider-health`
- `/api/v1/iot/*`

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

## Default Provider Wiring

| Domain | Selected plugin | Source of truth |
| --- | --- | --- |
| `rtc` | `rtc-volcengine` | Platform default provider registry |
| `object-storage` | `object-storage-volcengine` | Platform default provider registry plus media and RTC tests |
| `user-module` | `user-module-local` | Local node provider wiring and provider-health tests |
| `iot-access` | `iot-access-local` | Local node provider wiring and IoT provider-health tests |
| `iot-protocol` | `iot-mqtt` | Local node adapter wiring and IoT provider-health tests |

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
- the rest of the public app surface requires bearer auth
- the signing secret comes from `CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET`

### Embedded and test path

Internal tests and embedded compositions often use `build_default_app()` and trusted headers for
auth-context injection. That is an implementation and testing convenience layer, not the public
internet-facing consumer contract.
