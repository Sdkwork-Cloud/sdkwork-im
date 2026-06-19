# Sdkwork IM PC

SDKWork PC application root for the IM browser renderer and Tauri desktop shell.

Topology v2 authority: [../../docs/topology-greenfield.md](../../docs/topology-greenfield.md) · [../../specs/topology.spec.json](../../specs/topology.spec.json)

## Prerequisites

- Node.js 22, pnpm 10
- Rust toolchain (server + desktop host)
- Sibling checkouts: `../sdkwork-api-gateway`, `../sdkwork-rtc` (RTC media), plus shared SDK sources linked from `pnpm-workspace.yaml`

## Development (recommended)

Start the full stack from the repository root so topology profiles, platform gateway, and application ingress stay aligned:

```bash
cd ../..
pnpm install
pnpm dev              # browser renderer + sdkwork-im-server + platform gateway
pnpm dev:desktop      # Tauri desktop shell
pnpm dev:server          # server only
```

Default surfaces when using `pnpm dev`:

| Surface | URL |
| --- | --- |
| PC renderer | `http://127.0.0.1:4176` |
| Application ingress | `http://127.0.0.1:18079` |
| Platform API gateway | `http://127.0.0.1:3900` |

Database profiles:

```bash
pnpm dev:browser:postgres     # PostgreSQL via .env.postgres
pnpm dev:browser:sqlite       # local SQLite user data
```

## App-root commands

Run from `apps/sdkwork-im-pc` when working on renderer packages only (server must already be running or started separately):

```bash
pnpm install
pnpm dev               # Vite renderer only
pnpm build
pnpm lint
pnpm test:notary-app-sdk-integration
pnpm test:qr-scan-standard
```

Desktop host package: `packages/sdkwork-im-pc-desktop`.

## Layout

```text
apps/sdkwork-im-pc/
├─ src/                 # thin bootstrap, providers, route assembly
├─ packages/            # sdkwork-im-pc-* feature and shell packages
├─ specs/               # PC application contract
├─ scripts/             # app-local contract tests
└─ sdkwork.app.config.json
```

## SDK integration

- IM HTTP/WebSocket: generated `@sdkwork/im-sdk` and app/backend SDK families under repository `sdks/`
- Platform IAM/Drive/Agent: `@sdkwork/appbase-app-sdk` via `VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL`
- RTC media: `@sdkwork/rtc-sdk` from sibling `../sdkwork-rtc` (not checked into this repository's `sdks/`)

Do not add raw HTTP wrappers or manual auth headers in feature packages; bootstrap owns SDK construction.

## Verification

From repository root:

```bash
pnpm test:sdkwork-im-pc-dev-command
pnpm test:workflow-commercial-gates
```

From this directory:

```bash
pnpm build
pnpm lint
```

## Related docs

- [../../README.md](../../README.md)
- [../../docs/部署/README.md](../../docs/部署/README.md)
- [AGENTS.md](./AGENTS.md)
