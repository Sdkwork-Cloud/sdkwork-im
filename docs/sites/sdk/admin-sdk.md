# Admin SDK

The admin SDK family is the checked contract boundary for governance and control-plane work.

## Choose This Family When

- you need protocol registry reads, protocol governance snapshots, or runtime control-plane audit access
- you are building provider governance tooling for bindings, previews, diffs, rollbacks, or storage/provider policy review
- you need node lifecycle operations such as activation, drain, or route migration
- you are automating social control-plane and shared-channel runtime repair flows

## Do Not Use The Admin SDK For

- app-runtime chat flows such as conversations, message send, media upload, live receive, or RTC participant UX
- browser or mobile session bootstrap for the public product runtime
- message-builder ergonomics like `sdk.createTextMessage(...)` or `sdk.send(...)`

Use the App SDK instead: [App SDK](/sdk/app-sdk).

## Workspace Layout

- Root workspace: `sdks/sdkwork-craw-chat-sdk-admin`
- Authority contract: `sdks/sdkwork-craw-chat-sdk-admin/openapi/admin-control-plane.openapi.yaml`
- Derived generator input: `sdks/sdkwork-craw-chat-sdk-admin/openapi/admin-control-plane.sdkgen.yaml`
- Language workspaces:
  - `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-typescript`
  - `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-flutter`

## Scope

The admin SDK family is for the administrative control plane:

- protocol governance and protocol registry reads
- provider registry, binding policy, diff, preview, commit, and rollback flows
- social graph and shared-channel control-plane operations
- node activation, drain, and route migration workflows
- runtime health and queue-repair operations

It is not the product chat SDK. Conversation timeline, message send, session resume, and user-facing
realtime flows remain in `sdkwork-craw-chat-sdk`.

It is also not a fully generated replacement for every browser-facing `/api/admin/*` route used by
the standalone operator shell. The current formal OpenAPI authority is the control-plane document
served from `services/control-plane-api`, so only the `/api/v1/control/*` portion is generated from
OpenAPI today.

## Source Of Truth

The admin SDK now has a checked-in OpenAPI 3.0.3 authority file inside the workspace.

Runtime contract source:

- live endpoint: `/openapi.json`
- alias endpoint: `/api/v1/control/openapi.json`
- runtime implementation: `services/control-plane-api/src/lib.rs`

Checked-in workspace authority:

- `sdks/sdkwork-craw-chat-sdk-admin/openapi/admin-control-plane.openapi.yaml`
- `sdks/sdkwork-craw-chat-sdk-admin/openapi/admin-control-plane.sdkgen.yaml`

Refresh flow:

```powershell
node .\sdks\sdkwork-craw-chat-sdk-admin\bin\fetch-openapi-source.mjs
node .\sdks\sdkwork-craw-chat-sdk-admin\bin\prepare-openapi-source.mjs --base .\sdks\sdkwork-craw-chat-sdk-admin\openapi\admin-control-plane.openapi.yaml --derived .\sdks\sdkwork-craw-chat-sdk-admin\openapi\admin-control-plane.sdkgen.yaml
```

## Package Split

Each language follows the same two-layer ownership model:

| Language | Generated transport package | Composed ergonomic package |
| --- | --- | --- |
| TypeScript | `@sdkwork/craw-chat-admin-backend-sdk` | `@sdkwork/craw-chat-admin-sdk` |
| Flutter | `craw_chat_admin_backend_sdk` | `craw_chat_admin_sdk` |

Generated code lives only under:

- `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-typescript/generated/server-openapi`
- `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-flutter/generated/server-openapi`

Manual code lives only under:

- `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-typescript/composed`
- `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-flutter/composed`

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

- [Admin TypeScript SDK](/sdk/admin-typescript-sdk)
- [Admin Flutter SDK](/sdk/admin-flutter-sdk)
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
import { CrawChatAdminSdkClient } from '@sdkwork/craw-chat-admin-sdk';

const sdk = await CrawChatAdminSdkClient.create({
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

The composed client name is `CrawChatAdminSdkClient`.

## Flutter Usage

```dart
import 'package:craw_chat_admin_sdk/craw_chat_admin_sdk.dart';

final sdk = CrawChatAdminSdkClient.create(
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

## Current Boundary Gap

The verified TypeScript package is now the single package boundary used by the admin app, but it
contains two ownership modes:

That means:

- `@sdkwork/craw-chat-admin-sdk` is the correct package for control-plane governance and runtime
  operations
- `/api/v1/control/*` routes are generated from the checked-in admin control-plane OpenAPI authority
- browser-facing `/api/admin/*` routes now live as manual-owned composed exports in the same package,
  which lets `apps/craw-chat-admin` depend on one formal SDK package instead of a separate
  handwritten workspace package
- documentation must still describe the gap explicitly instead of pretending auth, tenant CRUD,
  project CRUD, storage sandbox routes, and other browser-only admin surfaces are already generated
  from OpenAPI

That TypeScript-specific browser helper surface is documented in
[Admin TypeScript SDK](/sdk/admin-typescript-sdk). The Flutter package currently exposes only the
generated control-plane modules documented in [Admin Flutter SDK](/sdk/admin-flutter-sdk).

## Verification

Root workspace verification:

```powershell
node .\sdks\sdkwork-craw-chat-sdk-admin\bin\verify-sdk.mjs --language typescript --language flutter
```

TypeScript-only verification:

```powershell
node .\sdks\sdkwork-craw-chat-sdk-admin\bin\verify-typescript-workspace.mjs
```

Flutter-only verification:

```powershell
node .\sdks\sdkwork-craw-chat-sdk-admin\bin\verify-flutter-workspace.mjs
```

## Current Language Status

| Language | Current state |
| --- | --- |
| TypeScript | Generated and composed workspace implemented, root verification passes, assembly metadata is produced under `.sdkwork-assembly.json` |
| Flutter | Generated and composed workspace implemented with `generated/server-openapi` plus `composed` layers, root verification is wired through `verify-flutter-workspace.mjs` |
