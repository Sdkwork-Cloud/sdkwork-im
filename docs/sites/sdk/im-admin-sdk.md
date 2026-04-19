# IM Admin SDK

## Workspace Layout

- Root workspace: `sdks/sdkwork-im-admin-sdk`
- Authority contract: `sdks/sdkwork-im-admin-sdk/openapi/im-admin.openapi.json`
- Derived sdkgen contract: `sdks/sdkwork-im-admin-sdk/openapi/im-admin.sdkgen.json`
- Assembly snapshot: `sdks/sdkwork-im-admin-sdk/.sdkwork-assembly.json`
- Authority materializer: `sdks/sdkwork-im-admin-sdk/bin/materialize-im-admin-authority.mjs`

## Scope

The IM admin SDK family formalizes the operator-facing backend served behind `/api/admin/*`.

Its canonical workspace name is `sdkwork-im-admin-sdk`.

It currently covers:

- operator login and current-session reads
- operator and portal identity administration
- tenant, project, API key, and routing governance
- catalog, usage, billing, runtime operations, and storage administration

It is intentionally separate from:

- `sdkwork-im-sdk`
  The app-facing product SDK.
- `sdkwork-control-plane-sdk`
  The governance and control-plane SDK for `/api/v1/control/*`.

## Current Source Of Truth

`sdks/sdkwork-im-admin-sdk` is now the single checked-in workspace for the IM admin boundary.

The admin console no longer depends on handwritten `/api/admin/*` transport wrappers or a
misleading `craw-chat-management` package line. Its app-local runtime delegates to the
materialized `@sdkwork/im-admin-sdk` client surface.

The discovery surface currently exposes these IM admin domains:

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
- `storage`

## Package Split

The IM admin workspace follows the standard two-layer ownership model in both supported languages:

| Language | Generated transport package | Composed consumer package | Primary client |
| --- | --- | --- | --- |
| TypeScript | `@sdkwork/im-admin-backend-sdk` | `@sdkwork/im-admin-sdk` | `ImAdminSdkClient` |
| Flutter | `im_admin_backend_sdk` | `im_admin_sdk` | `ImAdminSdkClient` |

Generated code lives only under `generated/server-openapi`. Manual consumer code lives only under
`composed`.

Do not import `generated/server-openapi/src/*` private paths from TypeScript packages or public
type surfaces. Do not import `generated/server-openapi/lib/src` private paths from Flutter code.

## TypeScript Usage

```ts
import { ImAdminSdkClient } from '@sdkwork/im-admin-sdk';

const sdk = await ImAdminSdkClient.create({
  backendConfig: {
    baseUrl: 'https://admin.example.com',
    authToken: '<operator-session-token>',
  },
});

const tenants = await sdk.tenants.listTenants();
console.log(tenants);
```

The preferred TypeScript entrypoint is `ImAdminSdkClient.create(...)`. Use `backendClient` only
when you already own a configured `ImAdminBackendClient`.

## Flutter Usage

```dart
import 'package:im_admin_sdk/im_admin_sdk.dart';

final sdk = ImAdminSdkClient.create(
  baseUrl: 'https://admin.example.com',
  authToken: '<operator-session-token>',
);

final tenants = await sdk.tenants.listTenants();
print(tenants);
```

The preferred Flutter entrypoint is also `ImAdminSdkClient.create(...)`.

## Endpoint Targeting

- Point `baseUrl` at the deployed surface that serves the checked-in `/api/admin/*` contract.
- In packaged installs, that is the unified `craw-chat-server` / `web-gateway` origin.
- In direct backend development, use the backend origin that already owns the `/api/admin/*`
  routes for that environment.

## Verification

Refresh the checked-in authority and language workspaces:

```bash
./sdks/sdkwork-im-admin-sdk/bin/generate-sdk.sh
```

On Windows:

```powershell
.\sdks\sdkwork-im-admin-sdk\bin\generate-sdk.ps1
```

Verify the authority boundary plus both language workspaces:

```bash
node ./sdks/sdkwork-im-admin-sdk/bin/verify-sdk.mjs
node ./sdks/sdkwork-im-admin-sdk/bin/verify-typescript-workspace.mjs
node ./sdks/sdkwork-im-admin-sdk/bin/verify-flutter-workspace.mjs
```

## Current Release Status

The current machine-readable release snapshot records two IM admin language lines:

| Artifact | Language | Generation state | Release state |
| --- | --- | --- | --- |
| `sdkwork-im-admin-sdk-typescript` | TypeScript | `generated` | `not_published` |
| `sdkwork-im-admin-sdk-flutter` | Flutter | `generated` | `not_published` |

That means the IM admin TypeScript and Flutter SDKs are checked in, locally verifiable, and still
waiting on publication and version freeze.
