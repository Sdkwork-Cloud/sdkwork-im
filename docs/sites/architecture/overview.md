# Architecture Overview

Craw Chat is a multi-service Rust workspace, not a single binary with optional extras. The current
documentation is easiest to understand through four architectural lenses:

1. The workspace layout and contract crates
2. The app-facing `local-minimal-node`
3. The separate `control-plane-api`
4. The runtime-directory persistence contract

## Core Architecture Facts

| Fact | Current implementation |
| --- | --- |
| Default app runtime | `services/local-minimal-node` |
| Default public app prefix | `/api/v1/*` |
| Default local app bind address | `127.0.0.1:18090` |
| Standalone control-plane bind address | `127.0.0.1:18081` |
| Public auth model | HS256 bearer tokens |
| Default local runtime directory | `.runtime/local-minimal` |
| Control-plane permissions | `control.read` and `control.write` |

## App Runtime

`services/local-minimal-node` is the current default business runtime. It assembles the following
domains into one HTTP process:

- session recovery, presence, and realtime delivery
- conversation lifecycle, inbox projection, membership, and read state
- messages, media, streams, and RTC
- notifications, automation, audit, and operator diagnostics
- user-module, object-storage, RTC, and IoT-related provider health surfaces

The main routing surface is declared in `services/local-minimal-node/src/node/build.rs`.

## Control Plane

`services/control-plane-api` is a distinct governance service. It is responsible for:

- protocol registry snapshots
- protocol governance snapshots
- provider registry and effective binding views
- provider policy preview, commit, diff, history, and rollback
- realtime node drain, activate, and route migration

This surface is implemented in `services/control-plane-api/src/lib.rs` and started by a separate
binary that binds `127.0.0.1:18081` in `services/control-plane-api/src/main.rs`.

## Runtime Directory Is Architectural, Not Auxiliary

When `CRAW_CHAT_RUNTIME_DIR` is set, the app node switches from in-memory defaults to file-backed
stores for replay, realtime checkpoints, subscriptions, presence, streams, RTC, notifications,
automation, and projection snapshots.

That means the runtime directory is part of the runtime contract, not just a convenience folder for
logs.

## Provider Defaults

The platform-default provider registry currently selects these defaults:

| Domain | Selected plugin |
| --- | --- |
| `rtc` | `rtc-volcengine` |
| `object-storage` | `object-storage-volcengine` |
| `user-module` | `user-module-local` |
| `iot-access` | `iot-access-local` |
| `iot-protocol` | `iot-mqtt` |

These defaults come from the platform provider registry contract and are surfaced through runtime
tests for app, ops, and control-plane endpoints.

## Why `local-default` Is Not Yet Its Own Topology

The repository already includes `local-default` script, config, and Docker entry points, but the
profile still reuses the current `local-minimal` service contract:

- `deployments/docker-compose/local-default.yml` extends `local-minimal.yml`
- the profile helper falls back to `.runtime/local-minimal`
- `deployments/templates/local-default.env.example` still points at `.runtime/local-minimal`

The docs therefore treat `local-default` as a compatibility and naming layer, not as a separate
completed topology.
