# Sdkwork IM

> Realtime communication infrastructure for the AI era — messaging, streaming, call signaling, and agent-native collaboration in a single Rust workspace.

[![License: AGPL-3.0-or-later](https://img.shields.io/badge/license-AGPL--3.0--or--later-blue.svg)](./LICENSE)
[![Rust: 2024 edition](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)
[![Code style: SDKWork](https://img.shields.io/badge/code%20style-SDKWork-purple.svg)](../sdkwork-specs/CODE_STYLE_SPEC.md)

Official docs site: [docs/sites](./docs/sites). SDK index: [sdks/README.md](./sdks/README.md).

## Why Sdkwork IM

Most IM backends were designed for person-to-person chat and later patched for bots, AI, and devices. Sdkwork IM is built the other way around: **streams are a first-class transport**, **agents and devices are first-class actors**, and **the messaging core stays isolated from RTC media, control-plane governance, and storage provider choice**.

| Differentiator | What it means |
| --- | --- |
| **Stream-native, not AI-patched** | `/im/v3/api/streams/*` is a generic frame transport (open → delta/patch → checkpoint → finalize/abort). It carries LLM tokens, task progress, audio transcription, device telemetry, and structured patches through the same lifecycle as messages. |
| **Actor model, not `senderId` string** | Realtime subjects are typed `user`, `agent`, `device`, `bot`, `system` with an explicit `actor/sender` model. No more guessing whether a `senderId` is a human or a webhook. |
| **IM owns signaling, RTC owns media** | Call lifecycle and signaling live here (`/im/v3/api/calls/*`); media/provider runtime comes from the sibling [`../sdkwork-rtc`](../sdkwork-rtc) workspace. The boundary is enforced by contract tests. |
| **Durable truth vs query truth** | Writes hit durable storage before acknowledgement; timelines, inboxes, and summaries are rebuildable projections. Cache is never the source of truth. |
| **Topology v2 as a contract** | Hosting, service layout, environment, and connectivity planes are versioned in [`specs/topology.spec.json`](./specs/topology.spec.json). Retired vocabulary is rejected by governance tests. |
| **9-language SDK matrix** | Four SDK families (IM / App / Backend / RTC) each ship across TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python — generated from OpenAPI authorities, not hand-written. |

## Capabilities

| Domain | Service | API prefix | Highlights |
| --- | --- | --- | --- |
| **Conversations** | `conversation-runtime` | `/im/v3/api/chat/conversations/*` | Standard + agent conversations, handoff, system channels, membership governance (list/add/remove/transfer/role/leave) |
| **Messages** | `conversation-runtime` | `/im/v3/api/chat/messages/*` | Send, edit, recall, timeline read, system-channel publish |
| **Realtime** | `session-gateway` | `/im/v3/api/realtime/*`, `/im/v3/api/realtime/ws` | Presence, subscribe/sync, WebSocket delivery, ACK, compensation, disconnect recovery |
| **Streams** | `streaming-service` | `/im/v3/api/streams/*` | Full lifecycle: open, frame append, list, checkpoint, complete, abort |
| **Call signaling** | `sdkwork-im-gateway` | `/im/v3/api/calls/*` | IM-owned signaling lifecycle, credential issuance, RTC media handoff (media via `../sdkwork-rtc`) |
| **Media** | `media-service` | `/im/v3/api/media/*` | Upload lifecycle, lookup, signed download, attachment binding (file truth in `sdkwork-drive`) |
| **Social** | `social-service` | `/im/v3/api/social/*` | Friend requests, friendships, user blocks, external collaboration |
| **Spaces** | `space-service` | `/im/v3/api/spaces/*` | Space/member/role containers, groups, channels, channel access rules |
| **Projection** | `projection-service` | `/im/v3/api/chat/inbox`, `contacts`, `conversations` (GET) | Inbox, timeline, summary, read-model projections |
| **Notifications** | `notification-service` | `/app/v3/api/notifications/*` | Notification task submission and retrieval |
| **Automation** | `automation-service` | `/app/v3/api/automation/*` | Automation execution submission and retrieval |
| **Audit** | `audit-service` | `/backend/v3/api/audit/*` | Audit record storage and export |
| **Ops** | `ops-service` | `/backend/v3/api/ops/*` | Health, cluster, lag, diagnostics, runtime-dir, provider-binding views |
| **Governance** | `governance-service` | `/backend/v3/api/control/*` | Protocol registry, provider registration/binding, policy preview/commit/diff/history/rollback, node drain/activate/route migration |
| **Desktop persistence** | `apps/sdkwork-im-pc` | — | SQLite persistence, repair, backup, restore |

## Architecture

### Plane model

Six primary planes carry traffic; two cross-cutting planes govern and observe them.

```
┌─────────────────────────────────────────────────────────────────┐
│  Control Plane   tenant / identity / policy / quota / routing   │
│  Ops Plane       observability / diagnostics / drain / restore  │
├─────────────────────────────────────────────────────────────────┤
│  Link Plane      WebSocket / SSE / MQTT connection lifecycle    │
│  Route Plane     session ownership, epoch + fencing, handoff    │
│  Messaging Plane send / edit / recall, durable truth, outbox    │
│  Stream Plane    open / delta / checkpoint / finalize / abort   │
│  Projection Plane inbox / timeline / summary / read-model      │
│  Storage Plane   PostgreSQL / SQLite / Redis / S3 / adapters    │
└─────────────────────────────────────────────────────────────────┘
```

### Code layering

| Layer | Directories | Responsibility |
| --- | --- | --- |
| **Contracts** | `crates/sdkwork-im-contract-*`, `crates/sdkwork-im-ccp-*`, `crates/im-platform-contracts`, `crates/im-storage-contracts` | Stable DTOs, event envelopes, protocol schemas, provider traits |
| **Domain** | `crates/im-domain-core`, `crates/im-domain-events` | Aggregate models, invariants, state machines, domain events |
| **Runtime** | `crates/sdkwork-im-runtime-{link,route,id}`, `crates/im-storage-runtime`, `crates/im-app-context` | Use-case orchestration, connection runtime, storage runtime, AppContext projection |
| **Adapters** | `adapters/local-disk`, `local-memory`, `redis-cache`, `postgres-journal`, `postgres-realtime`, `social-postgres`, `object-storage-s3`, `push-providers/*` | Storage and provider integrations |
| **Services** | `services/sdkwork-im-gateway`, `session-gateway`, `conversation-runtime`, `streaming-service`, `media-service`, `notification-service`, `automation-service`, `audit-service`, `ops-service`, `governance-service`, `projection-service`, `social-service`, `space-service`, `contact-service`, `interaction-service` | Runnable HTTP service processes |

### CCP — Client Connect Protocol

A four-layer protocol family that decouples transport from codec from control:

| Layer | Crates | Role |
| --- | --- | --- |
| **Core** | `sdkwork-im-ccp-core` | Frame model, envelope, auth handshake |
| **Codec** | `sdkwork-im-ccp-codec`, `-codec-json`, `-codec-cbor` | JSON + CBOR dual serialization |
| **Control** | `sdkwork-im-ccp-control` | Flow control, backpressure, compensation |
| **Registry** | `sdkwork-im-ccp-registry` | Compatibility matrix, governance snapshots |
| **Bindings** | `sdkwork-im-ccp-binding-{ws,sse,mqtt,http}` | Transport bindings for WebSocket, SSE, MQTT, HTTP |

Deep dive: [docs/架构/02-架构标准与总体设计.md](./docs/架构/02-架构标准与总体设计.md), [docs/sites/architecture/overview.md](./docs/sites/architecture/overview.md).

## Technology stack

**Backend (Rust 2024):**

| Concern | Choice |
| --- | --- |
| Web framework | `axum 0.8` (WebSocket) |
| Async runtime | `tokio 1.48` |
| Middleware | `tower 0.5`, `tower-http` |
| Serialization | `serde`, `serde_json`, CBOR |
| Database | `sqlx` via [`../sdkwork-database`](../sdkwork-database) (PostgreSQL, SQLite) |
| Cache / bus | Redis (`redis-cache` adapter) |
| Object storage | S3-compatible (`object-storage-s3` adapter) |
| Tracing | `tracing`, `tracing-subscriber` |
| Release profile | `lto=true`, `codegen-units=1`, `panic=abort`, `opt-level=3` |

**Frontend / Desktop (`apps/sdkwork-im-pc`):**

| Concern | Choice |
| --- | --- |
| Framework | React + Vite + TypeScript |
| Desktop shell | Tauri |
| Styling | TailwindCSS |
| Rich text | TipTap |
| i18n | i18next |
| SDK | `@sdkwork/im-sdk`, `@sdkwork/rtc-sdk`, `@sdkwork/drive-app-sdk` |

**Sibling workspace dependencies:** `sdkwork-web-framework`, `sdkwork-database`, `sdkwork-appbase`, `sdkwork-rtc`, `sdkwork-app-topology`, `sdkwork-drive`, `sdkwork-notary`, `sdkwork-core`, `sdkwork-ui`, `sdkwork-kernel`, `sdkwork-aiot`, `sdkwork-sdk-commons`, `sdkwork-sdk-generator`.

## Repository layout

```text
sdkwork-im/
├─ apis/           OpenAPI + RPC contract authorities (app/open/backend + apis/rpc)
├─ apps/           sdkwork-im-pc application root (React + Tauri)
├─ crates/         domain contracts, CCP protocol, runtime, shared libraries
├─ sdks/           IM SDK families (RTC SDK lives in ../sdkwork-rtc)
├─ adapters/       storage/provider adapters (local-disk, redis, postgres, s3, push)
├─ services/       runnable HTTP service processes
├─ configs/        topology profiles (configs/topology/*.env)
├─ deployments/    production templates
├─ docs/           architecture, deployment, docs/sites
├─ scripts/        standard command dispatch, governance, release
├─ tools/          chat-cli and helpers
├─ specs/          component.spec.json, topology.spec.json, database registries
└─ .sdkwork/       repository skills/plugins workspace metadata
```

Platform framework integration:

- HTTP ingress uses [`../sdkwork-web-framework`](../sdkwork-web-framework) through `services/sdkwork-im-gateway` (`WebFramework`, `WebRequestContext`, IAM adapter, 18-stage interceptor chain).
- PostgreSQL/SQLite pools use [`../sdkwork-database`](../sdkwork-database) through `crates/sdkwork-im-database-pool` and postgres adapters.
- `sdkwork-discovery` is **not** integrated yet: RPC contracts exist under `apis/rpc/`, but no hosted gRPC server is deployed. Register with discovery when RPC service hosts ship.

## Topology v2

Topology is a versioned contract, not a convention. Four official profiles are supported:

| Profile | Hosting | Layout | Env | Command |
| --- | --- | --- | --- | --- |
| `self-hosted.split-services.development` | self-hosted | split-services | development | `pnpm dev` (default) |
| `self-hosted.unified-process.development` | self-hosted | unified-process | development | `pnpm dev:browser` / `pnpm dev:browser:postgres:unified-process:standalone` |
| `self-hosted.split-services.production` | self-hosted | split-services | production | — |
| `cloud-hosted.split-services.production` | cloud-hosted | split-services | production | `pnpm build` |

- `split-services`: ingress gateway proxies to internal upstream services (default for dev and all production).
- `unified-process`: single-process in-memory assembly (CI smoke only).
- Profile files: `configs/topology/{hosting}.{serviceLayout}.{environment}.env`.
- Retired vocabulary is rejected by `pnpm test:topology-baggage`; see [`specs/topology.spec.json`](./specs/topology.spec.json) for the full retired list.

Authority: [docs/topology-greenfield.md](./docs/topology-greenfield.md), [specs/topology.spec.json](./specs/topology.spec.json), [configs/topology/](./configs/topology/).

## API surface

Three API prefixes, each owned by a distinct SDK family:

| Prefix | Purpose | SDK family |
| --- | --- | --- |
| `/im/v3/api/*` | IM standard HTTP surface (messages, conversations, realtime, media, streams, call signaling) | `sdkwork-im-sdk` |
| `/app/v3/api/*` | Application business (auth/iam/oauth, notifications, automation, drive, provider health, IoT, RTC callbacks) | `sdkwork-im-app-sdk` |
| `/backend/v3/api/*` | Backend admin/control/ops/audit | `sdkwork-im-backend-sdk` |

**Transports:** HTTP REST, WebSocket (`/im/v3/api/realtime/ws`, `auth.init` frame handshake), SSE, MQTT (AIoT).

**OpenAPI discovery:** `GET /openapi.json` (aggregated), `GET /openapi/index.json`, `GET /openapi/runtime-summary.json`, `GET /openapi/services/{id}.openapi.json`, `GET /docs`.

**Auth boundary:** public clients use dual-token headers (`Authorization: Bearer <auth-token>` + `Access-Token: <access-token>`); control-plane routes require `control.read` / `control.write` from the AppContext projection.

Reference: [docs/sites/api-reference/im-api.md](./docs/sites/api-reference/im-api.md), [docs/sites/api-reference/app-api.md](./docs/sites/api-reference/app-api.md), [docs/sites/api-reference/backend-api.md](./docs/sites/api-reference/backend-api.md).

## SDK families

Four SDK families, each generated from its OpenAPI authority across nine languages:

| Family | OpenAPI authority | Languages |
| --- | --- | --- |
| `sdkwork-im-sdk` | `openapi/sdkwork-im-im.openapi.yaml` | TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python |
| `sdkwork-im-app-sdk` | `openapi/sdkwork-im-app-api.openapi.yaml` | same 9 languages |
| `sdkwork-im-backend-sdk` | `openapi/sdkwork-im-backend-api.openapi.yaml` | same 9 languages |
| `sdkwork-rtc-sdk` | `.sdkwork-assembly.json` (in `../sdkwork-rtc`) | provider-neutral RTC runtime + provider packages |

Boundary rules: IM SDK must not absorb backend/admin APIs; App SDK must not expose `/backend/*` or `/im/*`; Backend SDK owns all admin/control APIs; RTC SDK is independent of OpenAPI-generated families and lives in `../sdkwork-rtc`.

Index: [sdks/README.md](./sdks/README.md), [docs/sites/sdk/index.md](./docs/sites/sdk/index.md).

## Quick start

**Prerequisites:** Rust (stable), Node.js 22, pnpm 10. Sibling checkouts: `sdkwork-api-gateway` (platform plane), `sdkwork-rtc` (for PC RTC).

```bash
pnpm install
pnpm dev
```

Default development surfaces:

| Surface | URL |
| --- | --- |
| Application ingress | `http://127.0.0.1:18079` |
| Platform API gateway | `http://127.0.0.1:3900` |
| PC renderer | `http://127.0.0.1:4176` |

Health check: `curl http://127.0.0.1:18079/healthz`

Other dev commands:

```bash
pnpm dev:browser   # PostgreSQL + standalone unified browser dev
pnpm dev:server       # Rust server only, no PC renderer
pnpm dev:desktop   # PostgreSQL + standalone Tauri desktop dev
pnpm dev:browser:postgres  # browser + PostgreSQL
pnpm dev:browser:sqlite    # browser + SQLite
```

### CLI chat validation

```bash
./bin/chat-cli.sh --help
./bin/chat-window.sh --help
```

See [docs/部署/CLI聊天验证与兼容矩阵.md](./docs/部署/CLI聊天验证与兼容矩阵.md) and [docs/sites/reference/cli-and-scripts.md](./docs/sites/reference/cli-and-scripts.md).

## Build and verification

```bash
# Rust
cargo fmt --all --check
cargo clippy --workspace --tests -- -D warnings
cargo test --workspace

# Governance (Node)
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

# Topology validation
node ../sdkwork-app-topology/scripts/sdkwork-topology.mjs validate --root . --spec specs/topology.spec.json
```

Maintenance: `pnpm migrate:topology-v2-baggage` re-applies archive vocabulary migration when needed.

## Deployment

| Mode | Entry | Use case |
| --- | --- | --- |
| Dev stack | `pnpm dev` / `pnpm dev:browser` / `pnpm dev:desktop` / `pnpm dev:server` | Local development, PC integration, smoke |
| Packaged server | `bin/install-server.*`, `bin/start-server.*`, `bin/verify-server.*` | Production single-port install + service hosting |
| Standalone control plane | `cargo run -p governance-service --offline` | Governance API development |

**Packaging targets** (12 total in [`sdkwork.workflow.json`](./sdkwork.workflow.json)):

- Server: `linux-x64/arm64`, `macos-x64/arm64`, `windows-x64/arm64` (tar.gz / zip)
- Desktop: same 6 OS+arch combos (zip)

**Desktop release packages** (in [`sdkwork.app.config.json`](./sdkwork.app.config.json)): `web-production` (ZIP), `desktop-windows` (ZIP), `desktop-macos` (DMG), `desktop-linux` (APPIMAGE). Install: `sdkwork install chat`; launch: `sdkwork open chat`.

**Docker:** optional container validation path. Production containers should use topology v2 env keys. See [docs/sites/deployment/docker.md](./docs/sites/deployment/docker.md).

**Service hosting:** `sdkwork-im-server` single-port contract, PostgreSQL-backed config root, service-manager wrapper (`bin/install-service-server.*`, `bin/uninstall-service-server.*`). Templates: `deployments/templates/server.env.example`, `deployments/templates/chat.toml.example`.

Production source deploy: [docs/部署/源码部署.md](./docs/部署/源码部署.md).

## Performance and disaster recovery

Three execution tiers, eight scenario families:

| Tier | Purpose | Baseline |
| --- | --- | --- |
| CI Smoke | Fast regression of connection/message/stream/drain/recovery semantics | `self-hosted.split-services.development` + `cargo test` |
| Pre-Release | Medium-scale concurrency, throughput, fault drill | Pre-release topology |
| Capacity | Dedicated environment for connection density, throughput ceiling, tail latency | Dedicated capacity cell/region |

Scenario families: `connection`, `message`, `stream`, `im-realtime-core` (commercial gate), `im-websocket-e2e` (commercial gate), `drain-rebalance`, `restore-recovery`, `failover`, `upgrade-rollback`.

Key reliability mechanisms:

- Route and session ownership use `epoch + fencing` to reject stale writes.
- Node shutdown uses `graceful drain`; no forceful removal.
- Single-writer per session; multi-node handoff via explicit ownership transfer.
- Backup recovery order: metadata → message log/stream checkpoint → projection rebuild → route/presence hot state → object storage reference consistency.

Full scenarios: [docs/部署/性能与灾备演练场景.md](./docs/部署/性能与灾备演练场景.md).

## Security and multi-tenancy

**Authoritative context principle:** `tenantId`, `actorId`, `actorKind`, `sessionId`, `clientRouteId` come from authenticated context or trusted bindings — never from business request bodies. `sender`, `routeEpoch`, `serverTs` are server-derived. The gateway converts authoritative context into internal command context and audit fields.

**Dual-track security model:**

- **Privacy track** — direct messages, sensitive sessions, private device control: optional E2E encryption, device binding, key rotation, stricter content visibility.
- **Collaboration track** — enterprise groups, AI workflows, knowledge collaboration, bots, audit: server-side search, compliance audit, governance, AI context visibility.

**Five-dimension tenant isolation:** identity, quota (connection / send TPS / stream throughput / media upload / automation / AI token), scheduling (fair queue, shuffle sharding, noisy-neighbor control), data (shared logical / dedicated cell / dedicated storage lane / tenant-scoped backup), fault (projection failure isolation, automation worker isolation, agent/IoT sidecar isolation).

**Deployment forms:** SaaS Shared Cell, SaaS Dedicated Cell, Private Standard, Private Restricted, Private Air-Gapped. Cell-based architecture: each cell is a fault, deployment, scaling, and ops domain; cross-region writes are controlled; no multi-master for the same session.

> Note: Shared Cell / Dedicated Cell / Restricted / Air-Gapped are design targets and evolution goals, not all completed product forms yet. The current engineering form is `sdkwork-im-server` + service split composition, supporting local minimal closed loop, provider injection, and basic control/ops surfaces.

Deep dive: [docs/架构/08-安全-多租户-SaaS-私有化-部署设计.md](./docs/架构/08-安全-多租户-SaaS-私有化-部署设计.md).

## Documentation

| Topic | Entry |
| --- | --- |
| Architecture overview | [docs/sites/architecture/overview.md](./docs/sites/architecture/overview.md) |
| Module map | [docs/sites/architecture/module-map.md](./docs/sites/architecture/module-map.md) |
| Runtime topology | [docs/sites/architecture/runtime-topology.md](./docs/sites/architecture/runtime-topology.md) |
| Storage management | [docs/sites/architecture/storage-management.md](./docs/sites/architecture/storage-management.md) |
| Capabilities | [docs/sites/features/capabilities.md](./docs/sites/features/capabilities.md) |
| Getting started | [docs/sites/getting-started/index.md](./docs/sites/getting-started/index.md) |
| IM API reference | [docs/sites/api-reference/im-api.md](./docs/sites/api-reference/im-api.md) |
| App API reference | [docs/sites/api-reference/app-api.md](./docs/sites/api-reference/app-api.md) |
| Backend API reference | [docs/sites/api-reference/backend-api.md](./docs/sites/api-reference/backend-api.md) |
| SDK index | [docs/sites/sdk/index.md](./docs/sites/sdk/index.md) |
| Deployment | [docs/sites/deployment/index.md](./docs/sites/deployment/index.md) |
| Architecture standard | [docs/架构/02-架构标准与总体设计.md](./docs/架构/02-架构标准与总体设计.md) |
| Security & multi-tenancy | [docs/架构/08-安全-多租户-SaaS-私有化-部署设计.md](./docs/架构/08-安全-多租户-SaaS-私有化-部署设计.md) |
| Topology design | [docs/topology-greenfield.md](./docs/topology-greenfield.md) |
| Performance & DR | [docs/部署/性能与灾备演练场景.md](./docs/部署/性能与灾备演练场景.md) |
| Step execution index | [docs/step/README.md](./docs/step/README.md) |
| Review archive | [docs/review/README.md](./docs/review/README.md) |
| Release bundles | [artifacts/releases/README.md](./artifacts/releases/README.md) |
| Source deploy | [docs/部署/源码部署.md](./docs/部署/源码部署.md) |
| CLI & compatibility | [docs/部署/CLI聊天验证与兼容矩阵.md](./docs/部署/CLI聊天验证与兼容矩阵.md) |

## Constraints

- Tenant and caller identity come from auth context, not business request bodies.
- Topology v2 profiles only; default dev is `pnpm dev`.
- RTC SDK authority is sibling [`../sdkwork-rtc`](../sdkwork-rtc) (`sdkwork-rtc-sdk` must not live under this repo's `sdks/`).
- Do not hand-edit generated SDK output; do not replace generated SDK calls with raw HTTP.
- Runtime directory (`SDKWORK_IM_RUNTIME_DIR`) is an architectural contract, not an auxiliary log directory.
- Storage management converges on shared module baselines (`im-storage-contracts` + `im-storage-runtime`); do not rebuild provider logic in consuming surfaces.
- Retired topology vocabulary must not be reintroduced; see the `bannedPatterns` list in `scripts/dev/sdkwork-im-topology-baggage.test.mjs` and the `retired` section of [`specs/topology.spec.json`](./specs/topology.spec.json).

## License

`AGPL-3.0-or-later` with commercial licensing — see [COMMERCIAL-LICENSE.md](./COMMERCIAL-LICENSE.md) and [LICENSE](./LICENSE).
