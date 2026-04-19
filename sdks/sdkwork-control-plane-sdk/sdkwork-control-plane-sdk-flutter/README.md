# sdkwork-control-plane-sdk Flutter

This language workspace packages the Flutter implementation for `sdkwork-control-plane-sdk`.

Layout:

- `generated/server-openapi`
  OpenAPI-derived transport layer for the control-plane contract.
- `composed`
  Manual ergonomic layer that exposes `ControlPlaneSdkClient` and control-plane-oriented modules.
- `bin`
  Language-local wrappers for generate, assemble, and verify entrypoints.

Published package names:

- generated: `control_plane_backend_sdk`
- composed: `control_plane_sdk`

## Consumer Rule

Use the composed package as the supported Flutter boundary. It exposes
`ControlPlaneSdkClient` and the same semantic control-plane module split used by the TypeScript
workspace.

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

```dart
import 'package:control_plane_sdk/control_plane_sdk.dart';

final sdk = ControlPlaneSdkClient.create(
  baseUrl: 'https://admin.example.com',
  authToken: '<token>',
);
```

Preferred options are flat:

- `baseUrl`
- `authToken`
- `headers`
- `timeout`

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

The Flutter package intentionally stays aligned to the generated control-plane authority. It does not
re-export the browser-only `/api/admin/*` helpers that remain TypeScript-specific.

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
entrypoints and pin generation to the Flutter workspace.
The forwarded generation path refreshes the checked-in authority contract, prepares the derived
sdkgen input, regenerates the Flutter transport layer, then runs the Flutter verification chain
before assembly metadata is refreshed.

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
entrypoints and pin assembly to the Flutter workspace.
Use them when generated and composed manifests are already current and you only need to refresh
workspace-level `.sdkwork-assembly.json` metadata.

Verification from this workspace:

```powershell
.\bin\sdk-verify.ps1 -WithDart
```

If local PowerShell execution policy blocks script execution, use:

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\sdk-verify.ps1 -WithDart
```

```bash
./bin/sdk-verify.sh --with-dart
```

Those `sdk-verify` wrappers forward to the root `sdkwork-control-plane-sdk/bin/verify-sdk.*`
entrypoints and pin verification to the Flutter workspace.
The forwarded verification path still delegates to `verify-flutter-workspace.mjs`, so one command
rechecks generated-model regression coverage, control-plane usage-surface checks, public API boundary
checks, and package metadata verification.

Direct workspace verifier:

```powershell
node .\..\bin\verify-flutter-workspace.mjs
```

That workspace verification covers generated-model regression checks, control-plane usage-surface checks, public API boundary checks, and package metadata verification.

Native Dart verification:

```powershell
node .\..\bin\verify-flutter-workspace.mjs --with-dart
```

On Windows, the workspace falls back to `..\bin\verify-flutter-dart-analysis.dart` instead of raw
`dart analyze` so Dart analysis remains reliable when the bundled toolchain cannot spawn its own
analysis helper process.

Cross-language workspace verification:

```powershell
node .\..\bin\verify-sdk.mjs --language flutter --with-dart
```
