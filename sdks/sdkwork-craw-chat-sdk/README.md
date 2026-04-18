# SDKWork Craw Chat SDK Workspace

`sdkwork-craw-chat-sdk` is the app-facing SDK workspace for Craw Chat.

It is a workspace, not a single package. The workspace owns:

- the authority OpenAPI 3.x contract for app-facing Craw Chat APIs
- the sdkgen-compatible derived contract used by `sdkwork-sdk-generator`
- root regeneration wrappers
- a TypeScript single-package SDK that assembles generated transport into `src/generated/**`
- a Flutter generated transport package plus composed consumer package
- the official multi-language workspace family for `typescript`, `flutter`, `rust`, `java`,
  `csharp`, `swift`, `kotlin`, `go`, and `python`
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

The canonical app-facing route surface is exported by the running `local-minimal-node` service at `/openapi/craw-chat-app.openapi.yaml`.

The workspace stores three contract files under `openapi/`:

- `craw-chat-app.openapi.yaml`
  The authority OpenAPI 3.x contract.
- `craw-chat-app.sdkgen.yaml`
  The default generator-compatible derived input.
- `craw-chat-app.flutter.sdkgen.yaml`
  The Flutter-compatible derived input that expands primitive component refs before Dart generation.

The authority file is the last successful live snapshot checked into the repo for review and diffability. Regeneration scripts may normalize the derived files for generator compatibility, but they must not treat generated output as the source contract.
Before generation, the root wrapper refreshes `openapi/craw-chat-app.openapi.yaml` from the live service schema endpoint `/openapi/craw-chat-app.openapi.yaml`.
For loopback base URLs such as `http://127.0.0.1:18090`, the same wrapper can start `local-minimal-node` through `bin/start-local.ps1` or `bin/start-local.sh`, fetch the latest service schema, and stop the service again unless the caller explicitly keeps it running.
The service export resolves schema content in this order:

- `CRAW_CHAT_APP_OPENAPI_SCHEMA_PATH` when explicitly configured
- `sdks/sdkwork-craw-chat-sdk/openapi/craw-chat-app.openapi.yaml` from the repository workspace
- the embedded build-time fallback bundled into `local-minimal-node`

## Auth Model

The public app contract is bearer-token based.

- Public app routes use `Authorization: Bearer <token>`.
- Trusted internal headers such as `x-tenant-id` and `x-user-id` are not the canonical app SDK auth model.
- The workspace therefore models bearer auth only for the generated app SDKs.

## Realtime Boundary

The authority contract documents `GET /api/v1/realtime/ws`, but generation still owns only the HTTP
SDK surface.

- HTTP-generated SDK support includes session resume, subscription sync, pull windows, and ack flow.
- the TypeScript single-package SDK additionally ships a handwritten `connect()` live runtime plus
  `sdk.sync.catchUp()` in its non-generated layer for payload-first live domain-stream receive,
  durable replay, and compile-time live contract verification
- The current compatibility matrix freezes `payload.json` as the default negotiated payload capability for the public realtime handshake.
- WebSocket protocol details such as `ccp/ws/1`, close code `4001`, and `session.disconnect`
  remain transport notes and non-generated adapter behavior, not generator-owned output.

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
  sdkwork-craw-chat-sdk-rust/
  sdkwork-craw-chat-sdk-java/
  sdkwork-craw-chat-sdk-csharp/
  sdkwork-craw-chat-sdk-swift/
  sdkwork-craw-chat-sdk-kotlin/
  sdkwork-craw-chat-sdk-go/
  sdkwork-craw-chat-sdk-python/
```

Per language workspace:

- TypeScript:
  - `generated/server-openapi`
    Generator-owned transport authoring output only.
  - `composed`
    Manual-owned authoring source for stable business modules and smoke tests before assembly.
  - root `src/generated/**`
    Assembled generator-owned transport inside the publishable `@sdkwork/craw-chat-sdk` package.
  - root `src/**` outside `src/generated/**`
    Assembled manual-owned business modules for the publishable `@sdkwork/craw-chat-sdk` package.
- Flutter:
  - `generated/server-openapi`
    Generator-owned transport package.
  - `composed`
    Manual-owned consumer-facing SDK package built above the generated HTTP layer.
- Rust / Java / C# / Swift / Kotlin / Go / Python:
  - `generated/server-openapi`
    Official generator-owned transport boundary for each language workspace.
  - `composed`
    Manual-owned semantic reserve or future consumer-facing SDK layer for each language workspace.
- `bin/`
  Thin forwarding scripts back to the root workspace wrappers.
- `README.md`
  Manual-owned docs for the language workspace.

The current checked-in workspace now verifies the full official language family through the shared
root wrappers and per-language workspace verifiers.

- `typescript` remains the semantic baseline and single-package reference implementation.
- `flutter` remains the checked-in consumer-package reference for Dart.
- `rust`, `java`, `csharp`, `swift`, `kotlin`, `go`, and `python` now participate in the same
  live-schema generation and workspace verification flow as transport-standardized workspaces.

## Package Layers

Primary consumer packages in this workspace are:

- TypeScript root package: `sdkwork-craw-chat-sdk-typescript`
  Publishes `@sdkwork/craw-chat-sdk` as the official browser and Node.js consumer package.
  The primary client class is `CrawChatSdkClient`.
  The same root package also re-exports generated transport access through `SdkworkBackendClient`, `createGeneratedBackendClient`, and the `generated` namespace.
- Flutter composed package: `sdkwork-craw-chat-sdk-flutter/composed`
  Publishes `craw_chat_sdk` as the official Flutter consumer package and re-exports `backend_sdk` from its root library.

Generator-owned transport authoring packages remain available inside the workspace:

- TypeScript generated authoring package: `sdkwork-craw-chat-sdk-typescript/generated/server-openapi`
  Uses package name `@sdkwork/craw-chat-backend-sdk` before assembly into the root TypeScript package.
- Flutter generated package: `backend_sdk`

## Regeneration

Run from the workspace root:

```powershell
.\bin\generate-sdk.ps1 -Languages typescript,flutter
```

The full official language family is:

```powershell
.\bin\generate-sdk.ps1 -Languages typescript,flutter,rust,java,csharp,swift,kotlin,go,python
```

If local PowerShell execution policy blocks script execution, use:

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\generate-sdk.ps1 -Languages typescript,flutter
```

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\generate-sdk.ps1 -Languages typescript,flutter,rust,java,csharp,swift,kotlin,go,python
```

```bash
./bin/generate-sdk.sh --language typescript --language flutter
```

The wrapper flow is:

1. Ensure the target service is healthy, auto-starting `local-minimal-node` for supported local base URLs when necessary.
2. Refresh the authority contract from the live service schema endpoint `/openapi/craw-chat-app.openapi.yaml`.
   This always uses the running service as the SDK generation source, rather than reading the checked-in YAML directly.
3. Normalize the refreshed authority OpenAPI contract into the default and Flutter-compatible derived `sdkgen` inputs.
4. Resolve one unified SDK version through `sdkwork-sdk-generator`.
5. Before TypeScript regeneration, stage the generator-owned entries in
   `sdkwork-craw-chat-sdk-typescript/generated/server-openapi` that are locally locked for
   in-place overwrite. This keeps `custom/` intact while preventing local `EPERM` overwrite
   failures from breaking the live-schema flow.
6. Generate TypeScript from `craw-chat-app.sdkgen.yaml`, then run the workspace-owned stable generated-package build and pack verification so the generator-owned transport boundary is valid before assembly.
   Immediately after generator output lands, the root wrapper normalizes the generated public auth surface back to the canonical bearer-only Craw Chat contract before any verification or assembly step runs.
   The same post-generation TypeScript flow then normalizes the generated package manifest and
   restores the local `bin/package-task.mjs` entrypoint before assembling the publishable root
   package so generated transport lands under `src/generated/**`, handwritten business modules stay
   in root `src/**`, the consumer package name stays `@sdkwork/craw-chat-sdk`, and the primary
   client class stays `CrawChatSdkClient`.
   The post-generation TypeScript verification path rechecks single-package assembly correctness, single-package layout, root package typecheck/build, dist cleanup, and smoke tests, then runs the determinism regression so repeated stable generated-package builds keep `dist/index.cjs.map` free of run-specific temporary-directory leakage before regeneration is considered complete.
   The TypeScript normalization layer also strips generator-only dead auth scaffolding and any `src/index.js` or `src/index.d.ts` build residue, so the published package stays root-entrypoint-only and bearer-only.
7. Generate Flutter from `craw-chat-app.flutter.sdkgen.yaml`, normalize the generated public auth surface back to bearer-only semantics, then run the Flutter workspace verification suite so Dart models do not regress into empty primitive wrapper classes, the Flutter composed layer keeps its convenience metadata helpers, it only consumes the generated package through the public `package:backend_sdk/backend_sdk.dart` entrypoint, and the generated package metadata stays aligned on `backend_sdk`.
8. Generate `rust`, `java`, `csharp`, `swift`, `kotlin`, `go`, and `python` from the same live authority contract, then run each language workspace verifier against the transport-standardized output and reserved `composed` boundary.
9. Refresh workspace metadata.

The refreshed `.sdkwork-assembly.json` must keep:

- the generated transport package metadata for every official language
- `consumerPackage` metadata whenever the workspace already ships a real consumer package, such as
  the TypeScript root `@sdkwork/craw-chat-sdk` package and the Flutter `craw_chat_sdk` package
- the TypeScript assembled root package as a distinct `root` layer in addition to `generated` and
  `composed`

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

1. Workspace automation guardrails, including wrapper and documentation drift checks.
2. PowerShell wrapper argument compatibility checks for the documented comma-separated `-Languages` examples.
3. TypeScript workspace verification through `bin/verify-typescript-workspace.mjs`, which runs generated-package stable build, generated-package artifact and `npm pack --dry-run` checks, bearer-auth surface alignment checks, temporary verification-directory cleanup checks, public API boundary validation, explicit live-contract verification for the payload-first realtime surface through `bin/verify-typescript-live-contract.mjs`, single-package assembly correctness, single-package layout validation, runtime root-export validation, dead-auth/dead-residue cleanup validation, root package typecheck/build, dist cleanup, smoke tests, and single-package publishability verification for `@sdkwork/craw-chat-sdk` using an isolated local npm cache.
4. TypeScript generated-package determinism regression verification through `bin/verify-typescript-generated-build-determinism.mjs`, so repeated stable builds keep `dist/index.cjs.map` identical and free of run-specific temporary-directory leakage.
5. TypeScript generated-package concurrency regression verification through `bin/verify-typescript-generated-build-concurrency.mjs` on Windows hosts, so overlapping stable generated-package builds do not regress back into shared-temp or shared-log collisions.
6. TypeScript workspace concurrency regression verification through `bin/verify-typescript-workspace-concurrency.mjs` on Windows hosts, so overlapping full workspace verifications do not race on the shared generated-package `dist/**` boundary during `npm pack --dry-run`.
7. Flutter workspace verification through `bin/verify-flutter-workspace.mjs`, which runs generated-model regression checks, bearer-auth surface alignment checks, composed parity checks, public API boundary checks, and package metadata consistency checks.
8. Transport-standardized workspace verification through:
   `bin/verify-rust-workspace.mjs`, `bin/verify-java-workspace.mjs`,
   `bin/verify-csharp-workspace.mjs`, `bin/verify-swift-workspace.mjs`,
   `bin/verify-kotlin-workspace.mjs`, `bin/verify-go-workspace.mjs`, and
   `bin/verify-python-workspace.mjs`.
9. Optional native Dart verification on top of the default Flutter workspace checks when `--with-dart` or `-WithDart` is requested. On Windows, this path resolves Flutter's bundled `dart.exe`, isolates the local pub cache under `/.sdkwork/dart/pub-cache`, and falls back to `bin/verify-flutter-dart-analysis.dart` when `dart analyze` cannot safely launch its own helper process in the current environment.
10. Workspace assembly refresh.

If a machine has a healthy Dart toolchain, opt into native Dart verification explicitly:

```powershell
node .\bin\verify-sdk.mjs --with-dart
```

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\verify-sdk.ps1 -Languages typescript,flutter -WithDart
```

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\verify-sdk.ps1 -Languages typescript,flutter,rust,java,csharp,swift,kotlin,go,python -WithDart
```

```bash
./bin/verify-sdk.sh --language typescript --language flutter --with-dart
```

## Documentation Standards

Use the internal docs map when you are changing package contracts, generation boundaries, or
maintainer guidance inside this workspace:

- `docs/README.md`
  Internal SDK standards map for package contracts, generation flow, verification, and wording
  rules.
- `docs/sites/README.md`
  Public VitePress sync standard for the docs pages that must stay aligned with SDK package names,
  client names, and API-to-SDK wording.

## Ownership Rules

- Do not hand-edit files inside `generated/server-openapi`.
- Do not hand-edit TypeScript assembled generated files inside `sdkwork-craw-chat-sdk-typescript/src/generated`.
- Fix the authority contract or wrapper inputs, then regenerate.
- Keep manual docs and forwarding scripts outside generated output.
- Treat transient local caches and scratch output such as `.tmp/`, `node_modules/`, `.npm-cache/`, `.dart_tool/`, `.sdkwork/dart/`, `.sdkwork/tmp/`, `.sdkwork-assembly.json`, and `.sdkwork/sdkwork-generator-*.json` as non-source artifacts.

## Language Workspaces

- TypeScript: [sdkwork-craw-chat-sdk-typescript](./sdkwork-craw-chat-sdk-typescript/README.md)
- Flutter: [sdkwork-craw-chat-sdk-flutter](./sdkwork-craw-chat-sdk-flutter/README.md)
- Rust: `sdkwork-craw-chat-sdk-rust`
- Java: `sdkwork-craw-chat-sdk-java`
- C#: `sdkwork-craw-chat-sdk-csharp`
- Swift: `sdkwork-craw-chat-sdk-swift`
- Kotlin: `sdkwork-craw-chat-sdk-kotlin`
- Go: `sdkwork-craw-chat-sdk-go`
- Python: `sdkwork-craw-chat-sdk-python`
