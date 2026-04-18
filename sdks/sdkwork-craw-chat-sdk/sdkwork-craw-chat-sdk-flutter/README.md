# SDKWork Craw Chat SDK Flutter Workspace

This workspace owns the Flutter package surface for the Craw Chat app SDK family.

## Layout

- `generated/server-openapi`
  Generator-owned Flutter HTTP SDK output from `sdkwork-sdk-generator`.
- `composed`
  Manual-owned consumer package `craw_chat_sdk` built above the generated HTTP layer.
- `bin/`
  Thin forwarding scripts to the root workspace wrappers.
- `README.md`
  Manual-owned workspace documentation.

## Generation Boundary

This workspace follows the layered Flutter IM-family pattern:

- generated HTTP SDK lives in `generated/server-openapi`
- composed Flutter SDK lives in `composed`
- any future handwritten realtime adapter must live outside generated output

Do not hand-edit the generated package. Change the root OpenAPI inputs or generator wrappers and regenerate.
The root generation wrapper also normalizes the generated package's public auth surface back to Craw Chat's bearer-only contract before verification continues.

The root wrapper feeds Flutter from `openapi/craw-chat-app.flutter.sdkgen.yaml`, which expands primitive component refs before generation so Dart models stay correctly typed.
The manual `composed` layer consumes the generated package only through `package:backend_sdk/backend_sdk.dart`; it does not import generated `src/` paths directly.

## Consumer Package

The primary app-facing Flutter package is `composed/pubspec.yaml`:

- package name: `craw_chat_sdk`
- library entrypoint: `composed/lib/craw_chat_sdk.dart`
- main capabilities:
  - `CrawChatSdkClient`
  - business modules for sessions, presence, realtime HTTP, devices, inbox, conversations, messages, media, streams, and RTC
  - convenience builders for text messages, text stream frames, and JSON RTC signals

The generated transport package remains `backend_sdk` under `generated/server-openapi`.

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

These scripts forward to the root `sdkwork-craw-chat-sdk/bin/generate-sdk.*` wrapper and constrain generation to the Flutter target.
The forwarded flow ends by running the shared `bin/verify-flutter-workspace.mjs` suite, so regeneration immediately rechecks generated-model regressions, bearer-auth surface alignment, composed parity, app-facing usage-surface consistency, public API boundaries, and package metadata alignment.

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

These scripts forward to the root `sdkwork-craw-chat-sdk/bin/assemble-sdk.*` wrapper and constrain assembly to the Flutter target.
Use them when you need to refresh `.sdkwork-assembly.json` from the current generated and composed Flutter package manifests without rerunning a full generation flow.

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

These scripts forward to the root `sdkwork-craw-chat-sdk/bin/verify-sdk.*` wrapper and constrain verification to the Flutter target.
The forwarded verification path delegates to the shared `bin/verify-flutter-workspace.mjs` suite, including generated-model regressions, bearer-auth alignment, composed parity, app-facing usage-surface checks, media-upload surface checks, public API boundary checks, and package metadata verification.
Add `-WithDart` on PowerShell or `--with-dart` on shell when the machine has a responsive Dart toolchain and you want native `dart pub get` plus `dart analyze` checks in addition to the default source-level regression guards.
On Windows, that native Dart path resolves Flutter's bundled `dart.exe`, isolates the pub cache under `.sdkwork/dart/pub-cache`, and falls back to the workspace `bin/verify-flutter-dart-analysis.dart` analyzer entrypoint when `dart analyze` cannot safely launch its own helper process in the current environment.

## Local Dependency Override

The Flutter manual layer stays publish-friendly by resolving local development dependencies through
`pubspec_overrides.yaml`.

That override file is the correct place to point `backend_sdk` back to `../generated/server-openapi`
when you are working inside the repository.

## Assembly Metadata

Root verification refreshes `.sdkwork-assembly.json` for the app SDK workspace.

That metadata file records:

- the Flutter package `manifestPath`
- the explicit `generated` and `composed` package layers
- a `generatedAt` timestamp that stays stable when assembly content is unchanged

Use that file for release-facing inspection instead of discovering package boundaries manually.

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

The websocket transport is documented at the workspace root but is not implemented as a handwritten Flutter adapter in this round.

## Release Placeholder Boundary

This workspace inherits the current SDK release placeholder contract from `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`.

- `template_only_pending_generation`
- `not_published`
- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`
