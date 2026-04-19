# sdkwork-control-plane-sdk TypeScript

This language workspace packages the TypeScript implementation for `sdkwork-control-plane-sdk`.

Layout:

- `generated/server-openapi`
  OpenAPI-derived transport layer for the current control-plane contract.
- `composed`
  Manual ergonomic layer that exposes `ControlPlaneSdkClient` and control-plane-oriented modules.
- `bin`
  Language-local wrappers for generate, assemble, and verify entrypoints.

Published package names:

- generated: `@sdkwork/control-plane-backend-sdk`
- composed: `@sdkwork/control-plane-sdk`

## Consumer Rule

Use the composed package as the only supported consumer boundary. It exposes
`ControlPlaneSdkClient`, semantic control-plane modules, and the manual-owned browser helpers that
the standalone admin app still needs for `/api/admin/*`.

## Release Snapshot Boundary

This workspace inherits the current SDK release snapshot from
`artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`.

- `state = generated_pending_publication`
- `generationStatus = generated`
- `releaseStatus = not_published`
- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`

## Create The Client

```ts
import { ControlPlaneSdkClient } from '@sdkwork/control-plane-sdk';

const sdk = await ControlPlaneSdkClient.create({
  baseUrl: 'https://admin.example.com',
  authToken: '<token>',
});
```

Preferred options are flat:

- `baseUrl`
- `authToken`
- `headers`
- `timeout`
- `fetch`

Advanced callers can still inject `backendClient`.

## Semantic Modules

- `sdk.meta`
  Health and liveness checks.
- `sdk.protocol`
  Protocol registry and governance snapshots.
- `sdk.providers`
  Registry, effective bindings, diff, preview, rollback, and history.
- `sdk.social`
  Direct-chat, external-collaboration, friendship, shared-channel-policy, and block control.
- `sdk.socialRuntime`
  Shared-channel queue inventory plus repair, reclaim, republish, requeue, and takeover.
- `sdk.nodes`
  Drain, activate, and route migration operations.

## Module To API Reference

- `sdk.meta` and `sdk.protocol`
  `docs/sites/api-reference/control-plane/protocol.md`
- `sdk.providers`
  `docs/sites/api-reference/control-plane/providers.md`
- `sdk.social`
  `docs/sites/api-reference/control-plane/social.md`
- `sdk.socialRuntime`
  `docs/sites/api-reference/control-plane/social-runtime.md`
- `sdk.nodes`
  `docs/sites/api-reference/control-plane/nodes.md`

## Browser Admin Surface

The composed TypeScript package also re-exports manual-owned helpers such as:

- `loginAdminUser`
- `getAdminMe`
- `listTenants`
- `saveTenant`
- `listStorageProviders`
- `saveTenantStorageConfig`
- `validateTenantStorageConfig`

Those helpers stay manual because they target browser-facing `/api/admin/*` routes rather than the
generated `/api/v1/control/*` authority.

Generation from this workspace:

```powershell
.\bin\sdk-gen.ps1
```

If local PowerShell execution policy blocks script execution, use:

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\sdk-gen.ps1
```

```bash
./bin/sdk-gen.sh
```

Those `sdk-gen` wrappers forward to the root `sdkwork-control-plane-sdk/bin/generate-sdk.*`
entrypoints and pin generation to the TypeScript workspace.
The forwarded generation path refreshes the checked-in authority contract, prepares the derived
sdkgen input, regenerates the TypeScript transport layer, then runs the TypeScript verification
chain before assembly metadata is refreshed.

Assembly from this workspace:

```powershell
.\bin\sdk-assemble.ps1
```

If local PowerShell execution policy blocks script execution, use:

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\sdk-assemble.ps1
```

```bash
./bin/sdk-assemble.sh
```

Those `sdk-assemble` wrappers forward to the root `sdkwork-control-plane-sdk/bin/assemble-sdk.*`
entrypoints and pin assembly to the TypeScript workspace.
Use them when generated and composed manifests are already current and you only need to refresh
workspace-level `.sdkwork-assembly.json` metadata.

Verification from this workspace:

```powershell
.\bin\sdk-verify.ps1
```

If local PowerShell execution policy blocks script execution, use:

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\sdk-verify.ps1
```

```bash
./bin/sdk-verify.sh
```

Those `sdk-verify` wrappers forward to the root `sdkwork-control-plane-sdk/bin/verify-sdk.*`
entrypoints and pin verification to the TypeScript workspace.
The forwarded verification path still delegates to `verify-typescript-workspace.mjs`, so the
language-local wrapper keeps the control-plane usage-surface checks, public API boundary checks,
generated-package validation, and the manual-owned `/api/admin/*` helper boundary in one command.

Direct workspace verifier:

```powershell
node .\..\bin\verify-typescript-workspace.mjs
```

That workspace verification covers the control-plane usage-surface checks, public API boundary checks, generated-package validation, and the manual-owned `/api/admin/*` helper boundary.

Cross-language workspace verification:

```powershell
node .\..\bin\verify-sdk.mjs --language typescript
```
