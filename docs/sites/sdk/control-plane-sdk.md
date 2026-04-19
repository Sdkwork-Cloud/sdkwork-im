# Control-Plane SDK

The control-plane SDK family is the checked contract boundary for governance and control-plane work.

## Choose This Family When

- you need protocol registry reads, protocol governance snapshots, or runtime control-plane audit access
- you are building provider governance tooling for bindings, previews, diffs, rollbacks, or storage/provider policy review
- you need node lifecycle operations such as activation, drain, or route migration
- you are automating social control-plane and shared-channel runtime repair flows

## Do Not Use The Control-Plane SDK For

- app-runtime chat flows such as conversations, message send, media upload, live receive, or RTC participant UX
- browser or mobile session bootstrap for the public product runtime
- message-builder ergonomics like `sdk.createTextMessage(...)` or `sdk.send(...)`

Use the App SDK instead: [App SDK](/sdk/app-sdk).

## Workspace Layout

- Root workspace: `sdks/sdkwork-control-plane-sdk`
- Authority contract: `sdks/sdkwork-control-plane-sdk/openapi/control-plane.openapi.yaml`
- Derived generator input: `sdks/sdkwork-control-plane-sdk/openapi/control-plane.sdkgen.yaml`
- Language workspaces:
  - `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript`
  - `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter`

## Scope

The control-plane SDK family is for the control plane:

- protocol governance and protocol registry reads
- provider registry, binding policy, diff, preview, commit, and rollback flows
- social graph and shared-channel control-plane operations
- node activation, drain, and route migration workflows
- runtime health and queue-repair operations

It is not the product chat SDK. Conversation timeline, message send, session resume, and user-facing
realtime flows remain in `sdkwork-im-sdk`.

The operator-console `/api/admin/*` IM admin contract is documented separately under
[IM Admin SDK](/sdk/im-admin-sdk). Use this page for `/api/v1/control/*` governance and
control-plane workflows.

It is also not a fully generated replacement for every browser-facing `/api/admin/*` route used by
the standalone operator shell. The current formal OpenAPI authority is the control-plane document
served from `services/control-plane-api`, so only the `/api/v1/control/*` portion is generated from
OpenAPI today.

## Source Of Truth

The control-plane SDK now has a checked-in OpenAPI 3.0.3 authority file inside the workspace.

Runtime contract source:

- live endpoint: `/openapi.json`
- alias endpoint: `/api/v1/control/openapi.json`
- runtime implementation: `services/control-plane-api/src/lib.rs`

Checked-in workspace authority:

- `sdks/sdkwork-control-plane-sdk/openapi/control-plane.openapi.yaml`
- `sdks/sdkwork-control-plane-sdk/openapi/control-plane.sdkgen.yaml`
- `sdks/sdkwork-control-plane-sdk/openapi/control-plane.openapi.json`
- `sdks/sdkwork-control-plane-sdk/openapi/control-plane.sdkgen.json`

Refresh flow:

```powershell
node .\sdks\sdkwork-control-plane-sdk\bin\fetch-openapi-source.mjs
node .\sdks\sdkwork-control-plane-sdk\bin\prepare-openapi-source.mjs --base .\sdks\sdkwork-control-plane-sdk\openapi\control-plane.openapi.yaml --derived .\sdks\sdkwork-control-plane-sdk\openapi\control-plane.sdkgen.yaml
```

## Package Split

Each language follows the same two-layer ownership model:

| Language | Generated transport package | Composed ergonomic package |
| --- | --- | --- |
| TypeScript | `@sdkwork/control-plane-backend-sdk` | `@sdkwork/control-plane-sdk` |
| Flutter | `control_plane_backend_sdk` | `control_plane_sdk` |

Generated code lives only under:

- `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/generated/server-openapi`
- `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/generated/server-openapi`

Manual code lives only under:

- `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed`
- `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/composed`

Generated package entrypoints are public; private generated source paths are not. Do not import
`generated/server-openapi/src/*` from admin TypeScript sources or public declaration surfaces.

## Assembly Metadata

The workspace refreshes `.sdkwork-assembly.json` as part of root verification and final assembly.

That release-facing metadata file records:

- the authority and derived spec paths
- one language entry per workspace
- each generated package `manifestPath`
- the full `packages` layer list so automation can distinguish `generated` and `composed`
- a `generatedAt` timestamp that stays stable when assembly content is unchanged

This means release tooling and workspace audits can discover package manifests, entrypoints, and
layer ownership without walking the whole repository tree.

## Language Guides

- [Control-Plane TypeScript SDK](/sdk/control-plane-typescript-sdk)
- [Control-Plane Flutter SDK](/sdk/control-plane-flutter-sdk)
- [Control Plane API Overview](/api-reference/control-plane-api)
- [Auth And Errors](/api-reference/auth-and-errors)

## Control Plane Reference Map

| Client surface | API reference | Notes |
| --- | --- | --- |
| `sdk.meta` | [/api-reference/control-plane/protocol#get-control-healthz](/api-reference/control-plane/protocol#get-control-healthz) | Health probe surfaced on the protocol governance page. |
| `sdk.protocol` | [/api-reference/control-plane/protocol](/api-reference/control-plane/protocol) | Governance and registry snapshots. |
| `sdk.providers` | [/api-reference/control-plane/providers](/api-reference/control-plane/providers) | Registry, bindings, preview, diff, rollback, and policy history. |
| `sdk.social` | [/api-reference/control-plane/social](/api-reference/control-plane/social) | Direct chats, external collaboration, friendship, shared-channel policy, and user blocks. |
| `sdk.socialRuntime` | [/api-reference/control-plane/social-runtime](/api-reference/control-plane/social-runtime) | Shared-channel sync queue inventory, repair, reclaim, republish, requeue, and takeover flows. |
| `sdk.nodes` | [/api-reference/control-plane/nodes](/api-reference/control-plane/nodes) | Node drain, activate, and route migration. |

This is the entry guide for protocol registry, provider governance, and node lifecycle work. For
app-runtime chat flows, use the App SDK instead.

## TypeScript Usage

```ts
import { ControlPlaneSdkClient } from '@sdkwork/control-plane-sdk';

const sdk = await ControlPlaneSdkClient.create({
  baseUrl: 'https://admin.example.com',
  authToken: '<token>',
});

const health = await sdk.meta.health();
const registry = await sdk.protocol.getRegistry();
const bindings = await sdk.providers.getBindings({ tenantId: 'tenant-northstar' });
```

Preferred create options are flat:

- `baseUrl`
- `authToken`
- `headers`
- `timeout`
- `fetch`

`backendClient` remains valid when the caller owns transport creation.

`createControlPlaneSdkClient(...)` resolves to the same composed facade as
`ControlPlaneSdkClient.create(...)`.

For standalone governance development, `baseUrl` can point directly at `control-plane-api`, which defaults to `http://127.0.0.1:18081`.

For packaged installs, point `baseUrl` at the unified `craw-chat-server` / `web-gateway` public origin; the gateway proxies control-plane routes on the same external port as the other operator-facing HTTP surfaces.

## Current TypeScript Surface

Generated modules:

- `meta`
- `protocol`
- `providers`
- `social`
- `socialRuntime`
- `nodes`

Composed modules:

- `sdk.meta`
- `sdk.protocol`
- `sdk.providers`
- `sdk.social`
- `sdk.socialRuntime`
- `sdk.nodes`
- manual admin-app transport helpers such as `loginAdminUser`, `listTenants`,
  `listStorageProviders`, `saveTenantStorageConfig`, and `adminBaseUrl`

The composed client name is `ControlPlaneSdkClient`.

## Flutter Usage

```dart
import 'package:control_plane_sdk/control_plane_sdk.dart';

final sdk = ControlPlaneSdkClient.create(
  baseUrl: 'https://admin.example.com',
  authToken: '<token>',
);

final health = await sdk.meta.health();
final registry = await sdk.protocol.getRegistry();
final bindings = await sdk.providers.getBindings(<String, dynamic>{
  'tenantId': 'tenant-northstar',
});
```

Flutter mirrors the same semantic modules as TypeScript:

- `sdk.meta`
- `sdk.protocol`
- `sdk.providers`
- `sdk.social`
- `sdk.socialRuntime`
- `sdk.nodes`

The primary Flutter control-plane client is `ControlPlaneSdkClient`, exposed from
`package:control_plane_sdk/control_plane_sdk.dart`.

## Current Boundary Gap

The verified TypeScript package is now the single package boundary used by the admin app, but it
contains two ownership modes:

That means:

- `@sdkwork/control-plane-sdk` is the correct package for control-plane governance and runtime
  operations
- `/api/v1/control/*` routes are generated from the checked-in control-plane OpenAPI authority
- browser-facing `/api/admin/*` routes now live as manual-owned composed exports in the same package,
  which lets `apps/craw-chat-admin` depend on one formal SDK package instead of a separate
  handwritten workspace package
- documentation must still describe the gap explicitly instead of pretending auth, tenant CRUD,
  project CRUD, storage sandbox routes, and other browser-only admin surfaces are already generated
  from OpenAPI

That TypeScript-specific browser helper surface is documented in
[Control-Plane TypeScript SDK](/sdk/control-plane-typescript-sdk). The Flutter package currently exposes only the
generated control-plane modules documented in [Control-Plane Flutter SDK](/sdk/control-plane-flutter-sdk).

## Verification

Root workspace verification:

```powershell
node .\sdks\sdkwork-control-plane-sdk\bin\verify-sdk.mjs --language typescript --language flutter
```

TypeScript-only verification:

```powershell
node .\sdks\sdkwork-control-plane-sdk\bin\verify-typescript-workspace.mjs
```

Flutter-only verification:

```powershell
node .\sdks\sdkwork-control-plane-sdk\bin\verify-flutter-workspace.mjs
```

## Current Language Status

| Language | Current state |
| --- | --- |
| TypeScript | Generated and composed workspace implemented, root verification passes, assembly metadata is produced under `.sdkwork-assembly.json` |
| Flutter | Generated and composed workspace implemented with `generated/server-openapi` plus `composed` layers, root verification is wired through `verify-flutter-workspace.mjs` |
