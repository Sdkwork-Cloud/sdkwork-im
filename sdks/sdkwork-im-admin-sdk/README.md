# sdkwork-im-admin-sdk

`sdkwork-im-admin-sdk` formalizes the IM admin backend served behind
`/api/admin/*`.

## Scope

This workspace is the source-of-truth boundary for:

- operator login and session introspection
- operator and portal identity management
- tenant, project, API key, and routing governance
- catalog, usage, billing, and runtime operation reads

It is intentionally separate from `sdkwork-im-sdk`, which remains the app-facing chat product SDK.

## Workspace Layout

- Authority contract: `openapi/im-admin.openapi.json`
- Derived sdkgen contract: `openapi/im-admin.sdkgen.json`
- Assembly snapshot: `.sdkwork-assembly.json`
- Authority materializer: `bin/materialize-im-admin-authority.mjs`
- TypeScript workspace materializer: `bin/materialize-im-admin-typescript-workspace.mjs`
- Flutter workspace materializer: `bin/materialize-im-admin-flutter-workspace.mjs`
- Assembly reconciler: `bin/assemble-sdk.mjs`
- TypeScript workspace verifier: `bin/verify-typescript-workspace.mjs`
- Flutter workspace verifier: `bin/verify-flutter-workspace.mjs`

## Current State

This workspace now materializes a standard two-layer TypeScript SDK workspace and a standard
two-layer Flutter SDK workspace for both supported consumer languages:

It continues to include a materialized TypeScript workspace and a materialized Flutter workspace.

- TypeScript
  - generated package: `sdkwork-im-admin-sdk-typescript/generated/server-openapi`
  - composed package: `sdkwork-im-admin-sdk-typescript/composed`
- Flutter
  - generated package: `sdkwork-im-admin-sdk-flutter/generated/server-openapi`
  - composed package: `sdkwork-im-admin-sdk-flutter/composed`

The TypeScript generated package is published as `@sdkwork/im-admin-backend-sdk`.
The TypeScript composed package is published as `@sdkwork/im-admin-sdk`.
The primary TypeScript consumer entrypoint is `ImAdminSdkClient`.

The Flutter generated package is published as `im_admin_backend_sdk`.
The Flutter composed package is published as `im_admin_sdk`.
The primary Flutter consumer entrypoint is `ImAdminSdkClient`.

## Endpoint Targeting

- Target the deployed surface that serves the checked-in `/api/admin/*` contract.
- In packaged installs, that means the unified `craw-chat-server` / `web-gateway` public origin.
- In direct admin-backend development, use the environment-specific backend origin that already
  owns `/api/admin/*`.

## Package Boundary Rules

Manual and composed layers must consume the generated transport package only through the package
root `@sdkwork/im-admin-backend-sdk`.
Do not import `generated/server-openapi/src/*` from manual sources, declaration shims, or published
public types.

## Current Release Status

| Artifact | Language | Generation state | Release state |
| --- | --- | --- | --- |
| `sdkwork-im-admin-sdk-typescript` | TypeScript | `materialized` | `not_published` |
| `sdkwork-im-admin-sdk-flutter` | Flutter | `materialized` | `not_published` |

The TypeScript and Flutter workspaces are both materialized and locally verifiable for this family.
The current machine-readable release catalog under `artifacts/releases/wave-d-2026-04-08/` now
records both IM admin language lines as generated but not published, while version freeze
metadata remains pending.

Generate or refresh the full IM admin SDK workspace:

```bash
./sdks/sdkwork-im-admin-sdk/bin/generate-sdk.sh
```

Cross-platform wrappers are also available:

- `./sdks/sdkwork-im-admin-sdk/bin/generate-sdk.sh`
- `.\sdks\sdkwork-im-admin-sdk\bin\generate-sdk.ps1`
- `.\sdks\sdkwork-im-admin-sdk\bin\generate-sdk.cmd`

You can also run the individual stages directly:

```bash
node ./sdks/sdkwork-im-admin-sdk/bin/materialize-im-admin-authority.mjs
node ./sdks/sdkwork-im-admin-sdk/bin/materialize-im-admin-typescript-workspace.mjs
node ./sdks/sdkwork-im-admin-sdk/bin/materialize-im-admin-flutter-workspace.mjs
node ./sdks/sdkwork-im-admin-sdk/bin/assemble-sdk.mjs
```

Verify the authority boundary plus both language workspaces:

```bash
node ./sdks/sdkwork-im-admin-sdk/bin/verify-sdk.mjs
node ./sdks/sdkwork-im-admin-sdk/bin/verify-typescript-workspace.mjs
node ./sdks/sdkwork-im-admin-sdk/bin/verify-flutter-workspace.mjs
```

Root verification also folds Flutter checks into `verify-sdk.mjs`. That path now validates the
generated `im_admin_backend_sdk` package, the composed
`im_admin_sdk` package, and the package-root-only public boundary for Flutter
consumers. Flutter workspace verification also remains available as a direct workspace command.
