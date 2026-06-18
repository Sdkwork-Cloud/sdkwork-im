# Sdkwork IM

Official docs site: [docs/sites](./docs/sites). SDK index: [sdks/README.md](./sdks/README.md).

Topology v2 authority: [docs/topology-greenfield.md](./docs/topology-greenfield.md), [specs/topology.spec.json](./specs/topology.spec.json), [configs/topology/](./configs/topology/).

`sdkwork-im` is a Rust workspace for instant messaging: conversations, realtime delivery, streams, IM-owned call signaling, media, and operations.

## Capabilities

- Conversation and membership governance
- Message send, edit, recall, and timeline
- Inbox and read cursors
- Realtime events, ACK, compensation, WebSocket
- Streaming transport
- IM call signaling (`/im/v3/api/calls`; media via sibling `../sdkwork-rtc`)
- Media upload and resource binding
- Notifications, automation, audit, ops diagnostics
- Desktop SQLite persistence, repair, backup, and restore

## Repository layout

SDKWork standard project-root dictionary (`SDKWORK_WORKSPACE_SPEC.md`):

```text
sdkwork-im/
├─ apis/           # OpenAPI + RPC contract authorities (app/open/backend + apis/rpc)
├─ apps/           # sdkwork-im-pc application root
├─ crates/         # domain contracts, repositories, route crates, shared libraries
├─ sdks/           # IM SDK families (RTC SDK lives in ../sdkwork-rtc)
├─ configs/        # topology profiles (configs/topology/*.env)
├─ deployments/    # production templates
├─ docs/           # architecture, deployment, docs/sites
├─ scripts/        # im:dev, governance, release
├─ tools/          # chat-cli and helpers
├─ jobs/           # reserved; background workers land in crates/*-worker when added
├─ plugins/        # reserved; runtime plugins (RTC plugins live in ../sdkwork-rtc)
├─ examples/       # reserved integration examples
├─ tests/          # reserved cross-package tests (crate tests stay colocated)
├─ specs/          # component.spec.json and local narrowing specs
└─ .sdkwork/       # repository skills/plugins workspace metadata
```

Legacy layout directories retained during crate-naming migration
(`docs/architecture/decisions/ADR-20260615-crate-naming-alignment.md`):

```text
├─ services/       # runnable HTTP service processes (target: sdkwork-im-*-service-host crates)
├─ adapters/     # storage/provider adapters (target: sdkwork-*-repository-sqlx crates)
├─ bin/            # chat-cli / chat-window wrappers (not lifecycle scripts)
├─ config/         # release source-link metadata (prefer configs/ for runtime profiles)
├─ artifacts/      # release and perf evidence outputs
└─ external/       # external inputs
```

Platform framework integration:

- HTTP ingress uses sibling `../sdkwork-web-framework` through `services/sdkwork-im-gateway`
  (`WebFramework`, `WebRequestContext`, IAM adapter, 18-stage interceptor chain).
- PostgreSQL/SQLite pools use sibling `../sdkwork-database` through
  `crates/sdkwork-im-database-pool` and postgres adapters.
- `sdkwork-discovery` is **not** integrated yet: RPC contracts exist under `apis/rpc/`, but no
  hosted gRPC server is deployed. Register with discovery when RPC service hosts ship.

## Services

- `sdkwork-im-gateway` / `sdkwork-im-server` — `application.public-ingress`
- `sdkwork-api-gateway` — platform plane (sibling repo)
- `conversation-runtime`, `session-gateway`, `streaming-service`
- `im-call-runtime` — IM signaling; `../sdkwork-rtc` for provider bridges only
- `media-service`, `notification-service`, `automation-service`, `audit-service`, `ops-service`

## Quick start

**Prerequisites:** Rust, Node.js 22, pnpm 10, sibling checkouts `sdkwork-api-gateway` and `sdkwork-rtc` (for PC RTC).

```bash
pnpm install
pnpm im:dev
```

Default development surfaces:

| Surface | URL |
| --- | --- |
| Application ingress | `http://127.0.0.1:18079` |
| Platform API gateway | `http://127.0.0.1:3900` |
| PC renderer | `http://127.0.0.1:4176` |

Other dev commands:

```bash
pnpm im:dev:unified   # self-hosted.unified-process.development (smoke)
pnpm server:dev       # Rust server only
pnpm im:dev:desktop   # Tauri desktop shell
```

Health check: `curl http://127.0.0.1:18079/healthz`

Production source deploy: [docs/部署/源码部署.md](./docs/部署/源码部署.md).

## CLI chat validation

Use `chat-cli` and `chat-window` wrappers under `bin/`:

```bash
./bin/chat-cli.sh --help
./bin/chat-window.sh --help
```

See [docs/部署/CLI聊天验证与兼容矩阵.md](./docs/部署/CLI聊天验证与兼容矩阵.md) and [docs/sites/reference/cli-and-scripts.md](./docs/sites/reference/cli-and-scripts.md).

## Documentation

| Topic | Entry |
| --- | --- |
| Deployment | [docs/部署/README.md](./docs/部署/README.md) |
| Architecture (current) | [docs/sites/architecture/](./docs/sites/architecture/) |
| Architecture (archive) | [docs/架构/README.md](./docs/架构/README.md) |
| Step execution index | [docs/step/README.md](./docs/step/README.md) |
| Review archive | [docs/review/README.md](./docs/review/README.md) |
| SDK families | [sdks/README.md](./sdks/README.md) |
| Release bundles | [artifacts/releases/README.md](./artifacts/releases/README.md) |
| Step 11 perf / DR | [docs/部署/性能与灾备演练场景.md](./docs/部署/性能与灾备演练场景.md) |

## Build and verification

```bash
cargo test --workspace
pnpm test:sdkwork-workspace-structure-standard
pnpm test:web-framework-standard
pnpm test:database-framework-standard
pnpm test:topology-baggage
pnpm test:runtime-standard
pnpm test:rtc-signaling-boundary
pnpm test:rpc-contract
pnpm test:workflow-commercial-gates
pnpm check:dependency-management
pnpm test:sdkwork-im-pc-dev-command
node ../sdkwork-app-topology/scripts/sdkwork-topology.mjs validate --root . --spec specs/topology.spec.json
```

Maintenance: `pnpm migrate:topology-v2-baggage` re-applies archive vocabulary migration when needed.

## Constraints

- Tenant and caller identity come from auth context, not business request bodies
- Topology v2 profiles only; default dev is `pnpm im:dev`
- RTC SDK authority is sibling `../sdkwork-rtc` (`sdkwork-rtc-sdk` must not live under this repo's `sdks/`)

## License

`AGPL-3.0-or-later` with commercial licensing — see [COMMERCIAL-LICENSE.md](./COMMERCIAL-LICENSE.md) and [LICENSE](./LICENSE).
