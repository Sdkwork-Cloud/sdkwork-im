# SDKWork Craw Chat SDK TypeScript Workspace

This workspace owns the TypeScript package surface for the Craw Chat app SDK family.

## Layout

- `generated/server-openapi`
  Generator-owned HTTP SDK output from `sdkwork-sdk-generator`.
- `composed`
  Manual-owned consumer package `@sdkwork/craw-chat-sdk` built above the generated HTTP layer.
- `bin/`
  Thin forwarding scripts to the root workspace wrappers.
- `README.md`
  Manual-owned workspace documentation.

## Generation Boundary

This workspace follows the layered TypeScript IM-family pattern:

- generated HTTP SDK lives in `generated/server-openapi`
- composed SDK lives in `composed`
- any future handwritten realtime adapter must live outside generated output

Do not hand-edit the generated package. Change the root OpenAPI inputs or generator wrappers and regenerate.
The root generation wrapper also normalizes the generated package's public auth surface back to Craw Chat's bearer-only contract before verification continues.
The same wrapper now strips generator-only dead auth scaffolding plus stray `src/index.js` and `src/index.d.ts` residue, so the generated package stays cleanly root-entrypoint-only after every regenerate.

For manual TypeScript composition, route all generated type imports through `composed/src/generated-backend-types.ts`.
Do not import generated private paths such as `generated/server-openapi/src/types/*` from any other composed file.
The root workspace wrappers also build and verify the generated transport package into `generated/server-openapi/dist` through a stable outer script, rather than relying on Vite/esbuild child-process behavior inside the generated package itself.
The stable generated-package build now also uses a workspace lock plus per-run temporary directories so overlapping root verification or generation flows do not collide on shared TypeScript build scratch space.

## Consumer Package

The primary app-facing TypeScript package is `composed/package.json`:

- package name: `@sdkwork/craw-chat-sdk`
- entrypoint: `composed/dist/index.js`
- main capabilities:
  - `CrawChatSdkClient`
  - business modules for sessions, presence, realtime HTTP, devices, inbox, conversations, messages, media, streams, and RTC
  - convenience builders for text messages, text stream frames, and JSON RTC signals

The generator-owned transport package remains `@sdkwork/craw-chat-backend-sdk` under
`generated/server-openapi`.

## Generate

From this workspace:

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

These scripts forward to the root `sdkwork-craw-chat-sdk/bin/generate-sdk.*` wrapper and constrain generation to the TypeScript target.
The forwarded flow ends by running the shared `bin/verify-typescript-workspace.mjs` suite, so regeneration also rechecks the generated package, the packed tarball boundary through `npm pack --dry-run`, bearer-auth surface alignment, temporary verification-directory cleanup, runtime root exports, dead-auth/dead-residue cleanup, composed public API boundary, typecheck, build, dist cleanup, and smoke tests.
The same generation flow then runs `bin/verify-typescript-generated-build-determinism.mjs`, so repeated stable generated-package builds keep `dist/index.cjs.map` free of run-specific temporary paths before regeneration is treated as complete.

## Assemble

From this workspace:

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

These scripts forward to the root `sdkwork-craw-chat-sdk/bin/assemble-sdk.*` wrapper and constrain assembly to the TypeScript target.
Use them when you need to refresh `.sdkwork-assembly.json` from already-verified generated and composed package manifests without rerunning a full regeneration flow.

## Verify

From this workspace:

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

These scripts forward to the root `sdkwork-craw-chat-sdk/bin/verify-sdk.*` wrapper and constrain verification to the TypeScript target.
The forwarded verification path delegates to the shared `bin/verify-typescript-workspace.mjs` suite, including generated-package artifact checks, the `npm pack --dry-run` tarball boundary check, usage-surface checks, public API boundary checks, the generated-package temporary verification-directory cleanup check, runtime root-export checks, and dead-auth/dead-residue cleanup checks.
The root verification chain also runs `bin/verify-typescript-generated-build-determinism.mjs` so repeated stable generated-package builds keep `dist/index.cjs.map` free of run-specific temporary paths and byte-stable across identical inputs.
On Windows, the root verification chain also runs `bin/verify-typescript-generated-build-concurrency.mjs` to prove that two overlapping generated-package builds can complete without shared-temp collisions.

## Assembly Metadata

Root verification refreshes `.sdkwork-assembly.json` for the app SDK workspace.

That metadata file records:

- the TypeScript package `manifestPath`
- the explicit `generated` and `composed` package layers
- a `generatedAt` timestamp that stays stable when assembly content is unchanged

Use that file for release-facing inspection instead of scanning directories by hand.

## Current Round Scope

This round generates the app-facing HTTP SDK for:

- sessions
- presence
- realtime HTTP coordination
- conversations
- members
- messages
- media
- streams
- RTC

The websocket transport is documented at the workspace root but is not implemented as a handwritten TypeScript adapter in this round.

## Release Placeholder Boundary

This workspace inherits the current SDK release placeholder contract from `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`.

- `template_only_pending_generation`
- `not_published`
- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`
