# App SDK

## Workspace Layout

- Root workspace: `sdks/sdkwork-craw-chat-sdk`
- Authority contract: `sdks/sdkwork-craw-chat-sdk/openapi/craw-chat-app.openapi.yaml`
- Derived sdkgen input: `sdks/sdkwork-craw-chat-sdk/openapi/craw-chat-app.sdkgen.yaml`
- Language workspaces:
  - `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-typescript`
  - `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-flutter`

## Scope

The app SDK workspace is intended to generate app-facing HTTP SDK support for:

- sessions and presence
- realtime HTTP coordination
- device registration and sync feed
- inbox, conversations, membership, and read cursor
- messages and mutation flows
- media upload, lookup, and attachment
- stream lifecycle and frame transport
- RTC lifecycle, signals, and participant credentials

It intentionally excludes:

- control-plane governance APIs
- ops, audit, and diagnostics routes
- IoT routes
- provider-health-only routes

## Contract Source

The canonical route surface still comes from
`services/local-minimal-node/src/node/build.rs`. The app SDK workspace then stores two checked-in
contract files:

- `craw-chat-app.openapi.yaml`
  The OpenAPI 3.0.3 authority contract.
- `craw-chat-app.sdkgen.yaml`
  The generator-compatible derived contract.

For this family, the checked-in OpenAPI authority is real and should be documented as such.

## Auth Model

The app SDK models public bearer auth only:

- `Authorization: Bearer <token>`
- signed with `CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET`

Trusted headers are still valid for tests and embedded internal flows, but they are not the public
consumer contract for generated SDK packages.

## Realtime Boundary

The authority contract includes `GET /api/v1/realtime/ws`, and the unified `web-gateway` now
proxies that websocket upgrade on the same external port as the HTTP app API. The current SDK
generation round still covers HTTP only.

- generated support covers resume, subscription sync, event pull, and ack flow
- websocket transport notes remain manual-owned even though the runtime route is live through the
  gateway
- the current workspace docs explicitly call out close code `4001` and `session.disconnect` as
  transport considerations rather than generated adapter behavior

## Regeneration

From the workspace root:

```powershell
.\bin\generate-sdk.ps1 -Languages typescript,flutter
```

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\generate-sdk.ps1 -Languages typescript,flutter
```

```bash
./bin/generate-sdk.sh --language typescript --language flutter
```

Per-language forwarding wrappers are also present inside each language workspace.

## Package Layers

The app SDK workspace now exposes two layers per language:

- TypeScript generated transport package: `@sdkwork/craw-chat-backend-sdk`
- TypeScript composed facade package: `@sdkwork/craw-chat-sdk`
- Flutter generated transport package: `backend_sdk`
- Flutter composed facade package: `craw_chat_sdk`

Use the composed package by default. Reach for the generated transport package only when a
consumer explicitly needs the low-level HTTP surface.

## TypeScript Client Surface

The preferred TypeScript consumer entrypoint is `CrawChatClient`, exported from
`@sdkwork/craw-chat-sdk`.

`CrawChatClient` exposes:

- `session`
- `presence`
- `realtime`
- `devices`
- `inbox`
- `conversations`
- `messages`
- `media`
- `streams`
- `rtc`

`CrawChatClient.create({ backendConfig })`, `CrawChatClient.create({ backendClient })`, and
`createCrawChatClient(...)` all resolve to the same composed facade.

For the current generated transport layer, `backendConfig` is the standard direct-create path and
supports:

- `baseUrl`
- `authToken`
- `tokenManager`
- `timeout`
- `headers`

## Endpoint Targeting

- For `local-minimal-node` development, set `baseUrl` to the node origin such as
  `http://127.0.0.1:18090`.
- For packaged installs, set `baseUrl` to the unified `craw-chat-server` / `web-gateway` public
  origin documented in [Gateway OpenAPI](/api-reference/gateway-openapi) and
  [Server Lifecycle](/deployment/server-lifecycle).
- The live websocket handshake at `GET /api/v1/realtime/ws` uses that same public origin, but the
  websocket transport remains outside the generated HTTP SDK round.

Manual-owned bridge code and downstream consumers must import generated symbols through the package
root entrypoint only. They must not import `generated/server-openapi/src/*` private source paths.

## Verification

Validate the checked-in authority contract, derived sdkgen input, generated package boundary, and
composed facade:

```bash
node ./sdks/sdkwork-craw-chat-sdk/bin/verify-sdk.mjs
```

The TypeScript verification path covers:

- generated package build into `generated/server-openapi/dist`
- generated package `npm pack --dry-run` validation
- composed package boundary validation so generated private source paths do not leak into the
  public TypeScript surface
- composed package typecheck and build
- composed smoke test for `CrawChatClient`

## Current Release Status

In practical terms:

- the app TypeScript and Flutter SDK workspaces are checked in and locally verifiable
- both language lines are usable from the repository workspace today
- neither package line has been published yet

The current machine-readable release snapshot records:

| Artifact | Language | Generation state | Release state |
| --- | --- | --- | --- |
| `sdkwork-craw-chat-sdk-typescript` | TypeScript | `generated` | `not_published` |
| `sdkwork-craw-chat-sdk-flutter` | Flutter | `generated` | `not_published` |

That means the current documentation standard distinguishes between:

- a real checked-in OpenAPI and workspace contract
- a real generation wrapper layout
- materialized generated and composed language workspaces that are locally verifiable
- a release wave that still does not publish packages
