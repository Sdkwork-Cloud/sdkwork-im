# SDKWork Craw Chat SDK Workspace

`sdkwork-craw-chat-sdk` is the app-facing SDK workspace for Craw Chat.

It is a workspace, not a single package. The workspace owns:

- the authority OpenAPI 3.x contract for app-facing Craw Chat APIs
- the sdkgen-compatible derived contract used by `sdkwork-sdk-generator`
- root regeneration wrappers
- generated TypeScript and Flutter HTTP SDK packages
- composed TypeScript and Flutter product SDK packages
- package-level documentation for regeneration and release preparation

## Scope

This workspace generates app-facing SDK capability for:

- sessions and presence
- realtime HTTP coordination
- device registration and sync feed
- inbox, conversations, members, and read cursor
- messages and message mutation
- media upload, lookup, and attachment
- stream lifecycle and frame transport
- RTC session lifecycle, signals, and participant credentials

This workspace does not generate:

- admin control-plane APIs
- ops, audit, or diagnostics routes
- IoT routes
- provider-health-only routes

## Contract Source

The canonical app-facing route surface comes from `services/local-minimal-node/src/node/build.rs`.

The workspace stores three contract files under `openapi/`:

- `craw-chat-app.openapi.yaml`
  The authority OpenAPI 3.x contract.
- `craw-chat-app.sdkgen.yaml`
  The default generator-compatible derived input.
- `craw-chat-app.flutter.sdkgen.yaml`
  The Flutter-compatible derived input that expands primitive component refs before Dart generation.

The authority file is the source of truth. Regeneration scripts may normalize the derived files for generator compatibility, but they must not treat generated output as the source contract.

## Auth Model

The public app contract is bearer-token based.

- Public app routes use `Authorization: Bearer <token>`.
- Trusted internal headers such as `x-tenant-id` and `x-user-id` are not the canonical app SDK auth model.
- The workspace therefore models bearer auth only for the generated app SDKs.

## Realtime Boundary

The authority contract documents `GET /api/v1/realtime/ws`, but this round only generates the HTTP SDK surface.

- HTTP-generated SDK support includes session resume, subscription sync, pull windows, and ack flow.
- The current compatibility matrix freezes `payload.json` as the default negotiated payload capability for the public realtime handshake.
- WebSocket protocol details such as `ccp/ws/1`, close code `4001`, and `session.disconnect` are documented as transport notes, not as a generated realtime adapter.

## Recovery Baseline

App-facing SDK docs must stay aligned with the compatibility matrix and close/error registry.

- `session.disconnect` is a formal recovery event; consumers must treat `goaway` / close as an authoritative signal and perform fresh `resume fallback`.
- The public recovery baseline includes websocket close code `4001`, close reason `session.disconnect`, and stale follow-up requests returning `reconnect_required`.
- `realtime.overload` must distinguish `pull-only` degradation from a hard disconnect path.
- During `pull-only`, consumers continue catch-up through `events.pull` / `event.window`; loss of live push is not equivalent to data loss.
- `goaway` code / message remains a first-class client recovery input alongside the `compatibility matrix`.

## Governance Baseline

The app-facing SDK workspace is pinned to the control-plane `sdkCompatibilityBaseline`.

- `appSdkFacade = sdkwork-craw-chat-sdk`
- `adminSdkFacade = sdkwork-craw-chat-sdk-admin`
- `protocolRegistryPath = /api/v1/control/protocol-registry`
- `protocolGovernancePath = /api/v1/control/protocol-governance`
- `matrixClientTypes = backend / desktop / mobile / web`

The same governance source also freezes `businessPolicyVocabulary` for app-facing policy surfaces:

- `policyVersionField = policy_version`
- `capabilityFlagsField = capability_flags`
- `historyVisibilityField = history_visibility`
- `retentionPolicyRefField = retention_policy_ref`
- `historyVisibilityModes = invited / joined / shared / world_readable`
- `retentionPolicyScopes = tenant / space / group / channel / thread`

This workspace consumes compatibility and governance results from those control-plane snapshots. It must not invent protocol capability decisions locally.
It must not rename these policy fields or invent local aliases when exposing space/group/channel/thread policy surfaces.

## Release Placeholder Boundary

This workspace inherits the current SDK release placeholder contract from `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`.

- `generationStatus = template_only_pending_generation`
- `releaseStatus = not_published`
- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`

## Workspace Layout

```text
sdkwork-craw-chat-sdk/
  openapi/
  bin/
  sdkwork-craw-chat-sdk-typescript/
  sdkwork-craw-chat-sdk-flutter/
```

Per language workspace:

- `generated/server-openapi`
  Generator-owned output only.
- `composed`
  Manual-owned consumer-facing SDK package built above the generated HTTP layer.
- `bin/`
  Thin forwarding scripts back to the root workspace wrappers.
- `README.md`
  Manual-owned docs for the language workspace.

## Package Layers

Primary consumer packages in this workspace are:

- TypeScript composed package: `sdkwork-craw-chat-sdk-typescript/composed`
  Publishes `@sdkwork/craw-chat-sdk`
- Flutter composed package: `sdkwork-craw-chat-sdk-flutter/composed`
  Publishes `craw_chat_sdk`

Generated transport packages remain available for lower-level HTTP use:

- TypeScript generated package: `@sdkwork/craw-chat-backend-sdk`
- Flutter generated package: `backend_sdk`

## Regeneration

Run from the workspace root:

```powershell
.\bin\generate-sdk.ps1 -Languages typescript,flutter
```

If local PowerShell execution policy blocks script execution, use:

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\generate-sdk.ps1 -Languages typescript,flutter
```

```bash
./bin/generate-sdk.sh --language typescript --language flutter
```

The wrapper flow is:

1. Normalize the authority OpenAPI contract into the default and Flutter-compatible derived `sdkgen` inputs.
2. Resolve one unified SDK version through `sdkwork-sdk-generator`.
3. Generate TypeScript from `craw-chat-app.sdkgen.yaml`, then run the workspace-owned stable generated-package build and pack verification so `dist/index.js`, `dist/index.cjs`, and `dist/index.d.ts` are present even when Vite or npm child-process behavior is unstable on Windows.
   Immediately after generator output lands, the root wrapper normalizes the generated public auth surface back to the canonical bearer-only Craw Chat contract before any verification or assembly step runs.
   The same post-generation TypeScript verification path also rechecks the composed package boundary, typecheck, build, dist cleanup, and smoke tests, then runs the determinism regression so repeated stable generated-package builds keep `dist/index.cjs.map` free of run-specific temporary-directory leakage before regeneration is considered complete.
   The TypeScript normalization layer also strips generator-only dead auth scaffolding and any `src/index.js` or `src/index.d.ts` build residue, so the published package stays root-entrypoint-only and bearer-only.
4. Generate Flutter from `craw-chat-app.flutter.sdkgen.yaml`, normalize the generated public auth surface back to bearer-only semantics, then run the Flutter workspace verification suite so Dart models do not regress into empty primitive wrapper classes, the Flutter composed layer keeps its convenience metadata helpers, it only consumes the generated package through the public `package:backend_sdk/backend_sdk.dart` entrypoint, and the generated package metadata stays aligned on `backend_sdk`.
5. Refresh workspace metadata.

## Assemble

When generated and composed manifests are already current, refresh workspace assembly metadata directly:

```powershell
.\bin\assemble-sdk.ps1 -Languages typescript,flutter
```

If local PowerShell execution policy blocks script execution, use:

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\assemble-sdk.ps1 -Languages typescript,flutter
```

```bash
./bin/assemble-sdk.sh --language typescript --language flutter
```

These root wrappers rebuild `.sdkwork-assembly.json` from the checked-in contract metadata plus the language package manifests.
Language-local `sdk-assemble` wrappers forward to this root entrypoint and pin a single language.

## Assembly Metadata

Root verification refreshes `.sdkwork-assembly.json`.

That release-facing metadata file records:

- authority and derived spec paths
- one language package entry per workspace
- each package `manifestPath`
- the explicit `generated` and `composed` package layers
- a `generatedAt` timestamp that stays stable when assembly content is unchanged

## Verification

Run the stable cross-language verification entrypoint from the workspace root:

```powershell
node .\bin\verify-sdk.mjs
```

If local PowerShell execution policy blocks script execution, use:

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\verify-sdk.ps1
```

```bash
./bin/verify-sdk.sh
```

This default verification flow runs:

1. Workspace automation guardrails, including wrapper and documentation drift checks, plus the workspace automation meta-test and assembly regression test that keep the root verify chain honest.
2. PowerShell wrapper argument compatibility checks for the documented comma-separated `-Languages` examples.
3. TypeScript workspace verification through `bin/verify-typescript-workspace.mjs`, which runs generated-package stable build, generated-package artifact and `npm pack --dry-run` checks, bearer-auth surface alignment checks, usage-surface checks, temporary verification-directory cleanup checks, public API boundary validation, runtime root-export validation, dead-auth/dead-residue cleanup validation, composed typecheck/build, dist cleanup, and smoke tests.
4. TypeScript generated-package determinism regression verification through `bin/verify-typescript-generated-build-determinism.mjs`, so repeated stable builds keep `dist/index.cjs.map` identical and free of run-specific temporary-directory leakage.
5. TypeScript generated-package concurrency regression verification through `bin/verify-typescript-generated-build-concurrency.mjs` on Windows hosts, so overlapping root verification invocations do not regress back into shared-temp or shared-log collisions.
6. Flutter workspace verification through `bin/verify-flutter-workspace.mjs`, which runs generated-model regression checks, bearer-auth surface alignment checks, composed parity checks, app-facing usage-surface consistency checks, media-upload surface checks, public API boundary checks, and package metadata consistency checks.
7. Optional native Dart verification on top of the default Flutter workspace checks when `--with-dart` or `-WithDart` is requested. On Windows, this path resolves Flutter's bundled `dart.exe`, isolates the local pub cache under `/.sdkwork/dart/pub-cache`, and falls back to `bin/verify-flutter-dart-analysis.dart` when `dart analyze` cannot safely launch its own helper process in the current environment.
8. Workspace assembly refresh.

If a machine has a healthy Dart toolchain, opt into native Dart verification explicitly:

```powershell
node .\bin\verify-sdk.mjs --with-dart
```

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\verify-sdk.ps1 -Languages typescript,flutter -WithDart
```

```bash
./bin/verify-sdk.sh --language typescript --language flutter --with-dart
```

## Ownership Rules

- Do not hand-edit files inside `generated/server-openapi`.
- Fix the authority contract or wrapper inputs, then regenerate.
- Keep manual docs and forwarding scripts outside generated output.
- Treat transient local caches and scratch output such as `.tmp/`, `node_modules/`, `.npm-cache/`, `.dart_tool/`, `.sdkwork/dart/`, `.sdkwork/tmp/`, `.sdkwork-assembly.json`, and `.sdkwork/sdkwork-generator-*.json` as non-source artifacts.

## Language Workspaces

- TypeScript: [sdkwork-craw-chat-sdk-typescript](./sdkwork-craw-chat-sdk-typescript/README.md)
- Flutter: [sdkwork-craw-chat-sdk-flutter](./sdkwork-craw-chat-sdk-flutter/README.md)
