# sdkwork-craw-chat-sdk-admin

`sdkwork-craw-chat-sdk-admin` is the admin and control-plane SDK workspace for Craw Chat.

## Scope

This SDK family is intended for:

- control-plane read surfaces
- protocol registry and governance consumption
- provider registry and provider-policy governance
- node lifecycle and social control workflows

It is not intended for:

- app-facing chat or conversation facades
- `chat-session`, `send-message`, or `timeline` product flows
- replacing the local verification role of `tools/chat-cli`

## Workspace Layout

- Root workspace: `sdks/sdkwork-craw-chat-sdk-admin`
- Authority contract: `sdks/sdkwork-craw-chat-sdk-admin/openapi/craw-chat-control-plane.openapi.json`
- Derived sdkgen contract: `sdks/sdkwork-craw-chat-sdk-admin/openapi/craw-chat-control-plane.sdkgen.json`
- Generated TypeScript package: `@sdkwork/craw-chat-admin-backend-sdk`
- Composed TypeScript package: `@sdkwork/craw-chat-sdk-admin`
- TypeScript workspace: `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-typescript`
- Flutter workspace: `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-flutter`

## Source Of Truth

The checked-in authority snapshot is exported from the live `control-plane-api` implementation through:

- `services/control-plane-api/src/bin/export-openapi.rs`
- `services/control-plane-api::export_openapi_document()`

The derived sdkgen contract embeds admin discovery metadata under `x-sdkwork-sdk-surface` so
assembly and future language packages can consume the same surface definition.

The TypeScript public surface is split into:

- generated transport layer: `@sdkwork/craw-chat-admin-backend-sdk`
- composed admin facade: `@sdkwork/craw-chat-sdk-admin`

The Flutter public surface is split into:

- generated transport layer: `craw_chat_admin_backend_sdk`
- composed admin facade: `craw_chat_sdk_admin`

## Endpoint Targeting

- For standalone governance development, point the admin SDK at the direct `control-plane-api`
  origin, which defaults to `http://127.0.0.1:18081`.
- For packaged installs, point the same SDK at the unified `craw-chat-server` / `web-gateway`
  public origin.
- Choose one runtime contract per environment. Do not mix direct `control-plane-api` assumptions
  into packaged single-port gateway clients.

## Package Boundary Rules

Manual and composed layers must consume the generated transport package only through the package
root `@sdkwork/craw-chat-admin-backend-sdk`.
Do not import `generated/server-openapi/src/*` from manual sources, declaration shims, or published
public types.

## Verification

Verify the admin SDK workspace contract and assembly metadata:

```bash
node ./bin/verify-sdk.mjs
```

That verification chain now does real TypeScript work instead of only checking file presence:

- builds the generated backend transport package into `generated/server-openapi/dist`
- runs `npm pack --dry-run` validation for `@sdkwork/craw-chat-admin-backend-sdk`
- typechecks and builds the composed package `@sdkwork/craw-chat-sdk-admin`
- runs the composed smoke test for `CrawChatSdkAdminClient`
- stabilizes the generated package manifest so `build` and `prepublishOnly` use the workspace-owned wrapper command
- runs Flutter workspace verification for `craw_chat_admin_backend_sdk` and `craw_chat_sdk_admin`
- checks the Flutter package boundary so consumers stay on package root entrypoints instead of `generated/server-openapi/lib/src`
- keeps the primary Flutter consumer entrypoint aligned on `CrawChatAdminClient`

Refresh the authority snapshot and rebuild the derived sdkgen contract:

```bash
./bin/generate-sdk.sh
```

On Windows:

```powershell
.\bin\generate-sdk.ps1
```

## Current Release Status

| Artifact | Language | Generation state | Release state |
| --- | --- | --- | --- |
| `sdkwork-craw-chat-sdk-admin-typescript` | TypeScript | `materialized` | `not_published` |
| `sdkwork-craw-chat-sdk-admin-flutter` | Flutter | `materialized` | `not_published` |

The admin workspace now has a checked-in authority contract, derived sdkgen input, assembly
metadata, a generated/composed TypeScript package line, and a generated/composed Flutter package
line. Flutter workspace verification is now part of the root `verify-sdk` contract, and the admin
consumer cutover from `sdkwork-craw-chat-admin-admin-api` is still pending. The current
machine-readable release catalog under `artifacts/releases/wave-d-2026-04-08/` now records both
admin language lines as generated but not published, while version freeze metadata remains pending.
