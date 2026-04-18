# Management SDK

## Workspace Layout

- Root workspace: `sdks/sdkwork-craw-chat-sdk-management`
- Authority contract: `sdks/sdkwork-craw-chat-sdk-management/openapi/craw-chat-management.openapi.json`
- Derived sdkgen contract: `sdks/sdkwork-craw-chat-sdk-management/openapi/craw-chat-management.sdkgen.json`
- Assembly snapshot: `sdks/sdkwork-craw-chat-sdk-management/.sdkwork-assembly.json`
- Materializer: `sdks/sdkwork-craw-chat-sdk-management/bin/materialize-management-authority.mjs`

## Scope

The management SDK family formalizes the operator-console backend served behind `/api/admin/*`.

It currently captures:

- operator login and current-session reads
- operator and portal identity administration
- tenant, project, API key, and routing governance
- catalog, usage, billing, and runtime operation reads

It is intentionally separate from:

- `sdkwork-craw-chat-sdk`
  The app-facing chat product SDK.
- `sdkwork-craw-chat-sdk-admin`
  The control-plane SDK for `/api/v1/control/*`.

## Current Source Of Truth

The checked-in authority is currently materialized from the boundary inventory already used by
`apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-admin-api/src/index.ts`.

That gives the repository a stable machine-readable contract for the current `/api/admin/*` surface.
The same workspace now also materializes and verifies a TypeScript generated transport package plus
a composed management facade above it. The admin console still exposes
`sdkwork-craw-chat-admin-admin-api` as a thin wrapper package, but that package now delegates
to `@sdkwork/craw-chat-sdk-management` and `@sdkwork/craw-chat-management-backend-sdk` instead of
maintaining handwritten `/api/admin/*` transport logic.

The current discovery surface reports one management service, `admin-console-api`, with the
following groups:

- `auth`
- `users`
- `marketing`
- `tenants`
- `access`
- `routing`
- `catalog`
- `usage`
- `billing`
- `operations`

## Current State

The management SDK workspace now has a materialized standard TypeScript SDK layout:

- the authority contract is checked in
- the derived sdkgen input is checked in
- the assembly snapshot is checked in
- the generated TypeScript transport package is materialized as
  `@sdkwork/craw-chat-management-backend-sdk`
- the composed TypeScript facade package is materialized as `@sdkwork/craw-chat-sdk-management`
- the generated Flutter transport package is materialized as
  `craw_chat_management_backend_sdk`
- the composed Flutter facade package is materialized as `craw_chat_sdk_management`

The preferred TypeScript consumer entrypoint is `CrawChatSdkManagementClient`, which exposes:

- `auth`
- `users`
- `marketing`
- `tenants`
- `access`
- `routing`
- `catalog`
- `usage`
- `billing`
- `operations`

`CrawChatSdkManagementClient.create({ backendConfig })`,
`CrawChatSdkManagementClient.create({ backendClient })`, and
`createCrawChatSdkManagementClient(...)` all resolve to the same composed facade.

The current `backendConfig` surface supports:

- `baseUrl`
- `apiKey`
- `authToken`
- `accessToken`
- `tenantId`
- `organizationId`
- `platform`
- `tokenManager`
- `timeout`
- `authMode`
- `headers`

## Flutter Packages

The Flutter workspace is also materially implemented and split into two packages:

- generated transport package: `craw_chat_management_backend_sdk`
- composed facade package: `craw_chat_sdk_management`

The primary Flutter client is `CrawChatManagementClient`, which exposes:

- `auth`
- `users`
- `marketing`
- `tenants`
- `access`
- `routing`
- `catalog`
- `usage`
- `billing`
- `operations`

```dart
import 'package:craw_chat_sdk_management/craw_chat_sdk_management.dart';

final sdk = CrawChatManagementClient.create(
  baseUrl: 'http://127.0.0.1:18080',
  authToken: 'your-auth-token',
);

final tenants = await sdk.tenants.listTenants();
print(tenants);
```

Manual-owned Flutter sources and downstream consumers must import generated symbols through
`package:craw_chat_management_backend_sdk/craw_chat_management_backend_sdk.dart`. They must not
import `generated/server-openapi/lib/src` private paths.

## Endpoint Targeting

- Point `baseUrl` at the deployed surface that serves the checked-in `/api/admin/*` contract.
- In packaged installs, that is the unified `craw-chat-server` / `web-gateway` origin.
- In direct admin-backend development, use the backend origin that already owns the
  `/api/admin/*` routes for that environment.

Generated symbols must be consumed through the generated package root entrypoint only. Manual-owned
composed code must not import `generated/server-openapi/src/*` private source paths.

Refresh the authority snapshot and assembly metadata:

```bash
node ./sdks/sdkwork-craw-chat-sdk-management/bin/materialize-management-authority.mjs
```

Verify the materialized management SDK workspace:

```bash
node ./sdks/sdkwork-craw-chat-sdk-management/bin/verify-sdk.mjs
```

The verification path now covers:

- authority and derived contract presence
- assembly snapshot integrity
- generated package build into `generated/server-openapi/dist`
- generated package `npm pack --dry-run` validation
- composed package boundary validation so generated private source paths do not leak into the
  public TypeScript surface
- composed package typecheck, build, and smoke test coverage for `CrawChatSdkManagementClient`
- Flutter workspace verification for `craw_chat_management_backend_sdk`
- composed Flutter surface verification for `CrawChatManagementClient`
- Flutter package-boundary checks so consumers stay on package root entrypoints instead of
  `generated/server-openapi/lib/src`

Refresh the full management SDK workspace, including Flutter materialization:

```bash
./sdks/sdkwork-craw-chat-sdk-management/bin/generate-sdk.sh
```

On Windows:

```powershell
.\sdks\sdkwork-craw-chat-sdk-management\bin\generate-sdk.ps1
```

## Current Release Status

In practical terms:

- the management TypeScript SDK is checked in, locally verifiable, and already used through the
  admin console compatibility layer
- the management Flutter SDK is also checked in and locally verifiable
- neither package line has been published yet

The current machine-readable release snapshot records:

| Artifact | Language | Generation state | Release state |
| --- | --- | --- | --- |
| `sdkwork-craw-chat-sdk-management-typescript` | TypeScript | `generated` | `not_published` |
| `sdkwork-craw-chat-sdk-management-flutter` | Flutter | `generated` | `not_published` |

Additional release-metadata fields still report:

- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`

That means both management language lines are materialized in the checked-in workspace, reflected
in the current release snapshot as generated, and still waiting on publication and version freeze.
The checked-in workspace plus `.sdkwork-assembly.json` remains the richer engineering truth for
package boundaries and consumer surfaces.
