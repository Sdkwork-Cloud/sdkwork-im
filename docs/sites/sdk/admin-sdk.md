# Admin SDK

## Workspace Layout

- Root workspace: `sdks/sdkwork-craw-chat-sdk-admin`
- Authority contract: `sdks/sdkwork-craw-chat-sdk-admin/openapi/craw-chat-control-plane.openapi.json`
- Derived sdkgen contract: `sdks/sdkwork-craw-chat-sdk-admin/openapi/craw-chat-control-plane.sdkgen.json`
- Assembly metadata: `sdks/sdkwork-craw-chat-sdk-admin/.sdkwork-assembly.json`
- Language workspaces:
  - `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-typescript`
  - `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-flutter`

## Scope

The admin SDK family is intended for:

- control-plane read surfaces
- protocol registry and governance consumption
- provider registry and provider-policy governance
- node lifecycle management

It is not intended for:

- app-facing chat or conversation facades
- replacing the existing `apps/craw-chat-admin` `/api/admin/*` wrapper package
- `chat-session`, `send-message`, or `timeline` style product flows
- replacing the local verification role of `tools/chat-cli`

The current operator-console backend used by `apps/craw-chat-admin` still lives behind the
`/api/admin/*` proxy contract. That backend is now formalized separately under
`sdks/sdkwork-craw-chat-sdk-management/`, so it is intentionally not generated out of
`sdkwork-craw-chat-sdk-admin`.

## Current Source Of Truth

The admin SDK workspace now contains a checked-in OpenAPI 3.1 authority contract and a derived
sdkgen contract:

- `sdks/sdkwork-craw-chat-sdk-admin/openapi/craw-chat-control-plane.openapi.json`
- `sdks/sdkwork-craw-chat-sdk-admin/openapi/craw-chat-control-plane.sdkgen.json`

The authority snapshot is exported from the live control-plane implementation through:

- `services/control-plane-api::export_openapi_document()`
- `services/control-plane-api/src/bin/export-openapi.rs`

Implementation truth still comes from the control-plane router and its tests:

- `services/control-plane-api/src/lib.rs`
- `services/control-plane-api/tests/protocol_registry_test.rs`
- `services/control-plane-api/tests/protocol_governance_test.rs`
- `services/control-plane-api/tests/provider_registry_test.rs`
- `services/control-plane-api/tests/drain_routes_test.rs`

## Discovery Surface

The derived sdkgen contract embeds `x-sdkwork-sdk-surface` metadata for admin SDK assembly. The
current surface groups are:

- `system`
- `protocol`
- `providers`
- `social`
- `cluster`

The current assembly snapshot reports 40 HTTP operations from `control-plane-api` and no manual
transport gaps.

## TypeScript Packages

The TypeScript workspace is now materially implemented and split into two packages:

- generated transport package: `@sdkwork/craw-chat-admin-backend-sdk`
- composed facade package: `@sdkwork/craw-chat-sdk-admin`

The primary TypeScript client is `CrawChatSdkAdminClient`, which exposes:

- `protocol`
- `providers`
- `cluster`
- `social`
- `system`

Use the generated package only when a low-level transport surface is required. Admin applications
should prefer the composed facade.

`CrawChatSdkAdminClient.create({ backendConfig })`,
`CrawChatSdkAdminClient.create({ backendClient })`, and
`createCrawChatSdkAdminClient(...)` all resolve to the same composed facade.

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

- generated transport package: `craw_chat_admin_backend_sdk`
- composed facade package: `craw_chat_sdk_admin`

The primary Flutter client is `CrawChatAdminClient`, which exposes:

- `protocol`
- `providers`
- `cluster`
- `social`
- `system`

Use the generated package only when a low-level transport surface is required. Flutter
applications should prefer the composed facade.

```dart
import 'package:craw_chat_sdk_admin/craw_chat_sdk_admin.dart';

final sdk = CrawChatAdminClient.create(
  baseUrl: 'http://127.0.0.1:18081',
  authToken: 'your-auth-token',
);

final registry = await sdk.protocol.getApiV1ControlProtocolRegistry();
print(registry);
```

Manual-owned Flutter sources and downstream consumers must import generated symbols through
`package:craw_chat_admin_backend_sdk/craw_chat_admin_backend_sdk.dart`. They must not import
`generated/server-openapi/lib/src` private paths.

## Endpoint Targeting

- For standalone governance development, `baseUrl` can point directly at `control-plane-api`,
  which defaults to `http://127.0.0.1:18081`.
- For packaged installs, point `baseUrl` at the unified `craw-chat-server` / `web-gateway`
  public origin; the gateway proxies control-plane routes on the same external port as the other
  operator-facing HTTP surfaces.
- Choose one deployment contract per environment. Do not mix direct `control-plane-api`
  assumptions into packaged single-port gateway clients.

Manual-owned composed sources and downstream consumers must import generated symbols through the
generated package root entrypoint only. They must not import
`generated/server-openapi/src/*` private source paths.

## Consumer Rules

The admin SDK boundary is already meaningful even before package publication:

- governance consumers should treat control-plane snapshots as the source of truth
- client behavior should not reconstruct protocol compatibility locally when the control plane
  already publishes the decision surface
- admin integrations should not mix app-facing chat features into this SDK family
- operator-console consumers that depend on `/api/admin/*` should target
  `sdkwork-craw-chat-sdk-management`, not this control-plane family

## Verification

Validate the checked-in authority contract, derived sdkgen contract, and assembly metadata:

```bash
node ./sdks/sdkwork-craw-chat-sdk-admin/bin/verify-sdk.mjs
```

The TypeScript verification path now also performs:

- generated package build into `generated/server-openapi/dist`
- generated package `npm pack --dry-run` validation
- composed package boundary validation so generated private source paths do not leak into the
  public TypeScript surface
- composed package typecheck and build
- composed smoke test for `CrawChatSdkAdminClient`
- generated package manifest stabilization so `build` and `prepublishOnly` stay on the workspace-owned wrapper command
- Flutter workspace verification for `craw_chat_admin_backend_sdk`
- composed Flutter surface verification for `CrawChatAdminClient`
- Flutter package-boundary checks so consumers stay on package root entrypoints instead of
  `generated/server-openapi/lib/src`

Refresh the authority snapshot from the live service implementation and rebuild the derived
contract:

```bash
./sdks/sdkwork-craw-chat-sdk-admin/bin/generate-sdk.sh
```

On Windows:

```powershell
.\sdks\sdkwork-craw-chat-sdk-admin\bin\generate-sdk.ps1
```

## Current Release Status

In practical terms:

- the admin TypeScript SDK is checked in, locally verifiable, and ready for local integration work
- the admin Flutter SDK is also checked in and locally verifiable
- neither package line has been published yet

The current machine-readable release snapshot records:

| Artifact | Language | Generation state | Release state |
| --- | --- | --- | --- |
| `sdkwork-craw-chat-sdk-admin-typescript` | TypeScript | `generated` | `not_published` |
| `sdkwork-craw-chat-sdk-admin-flutter` | Flutter | `generated` | `not_published` |

Additional release-metadata fields still report:

- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`

That means both admin language lines are available in the checked-in workspace, pass local
verification, and are reflected in the current release snapshot as generated but still
unpublished. The checked-in workspace plus `.sdkwork-assembly.json` remains the richer engineering
truth for package boundaries and consumer surfaces.
