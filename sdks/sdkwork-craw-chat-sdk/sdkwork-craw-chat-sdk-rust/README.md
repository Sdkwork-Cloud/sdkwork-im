# SDKWork Craw Chat SDK Rust Workspace

This workspace owns the Rust package surface for the Craw Chat app SDK family.

## Layout

- `generated/server-openapi`
  Generator-owned Rust HTTP SDK output from `sdkwork-sdk-generator`.
- `composed`
  Manual-owned consumer crate `craw-chat-sdk` built above the generated HTTP layer.
- `bin/`
  Thin forwarding scripts to the root workspace wrappers.
- `README.md`
  Manual-owned workspace documentation.

## Generation Boundary

This workspace follows the layered Craw Chat SDK pattern:

- generated HTTP SDK lives in `generated/server-openapi`
- composed Rust SDK lives in `composed`
- any future handwritten realtime adapter must live outside generated output

Do not hand-edit the generated package. Change the root OpenAPI inputs or generator wrappers and regenerate.
The root workspace wrappers remain the canonical generation, verification, and assembly entrypoints for this workspace.
The root generation flow also normalizes the generated crate back to Craw Chat's bearer-only public auth contract and keeps the composed layer consuming generated APIs only through the generated crate root exports.

## Package Layers

This workspace exposes two Rust crates:

- generated crate: `sdkwork-craw-chat-backend-sdk`
  Generator-owned transport crate in `generated/server-openapi`.
- composed crate: `craw-chat-sdk`
  Manual-owned consumer crate in `composed`.

## Consumer Crate

The primary app-facing Rust crate is `composed/Cargo.toml`:

- crate name: `craw-chat-sdk`
- library name: `craw_chat_sdk`
- generated dependency: `sdkwork-craw-chat-backend-sdk`
- primary entrypoint: `CrawChatClient`
- main capabilities:
  - business modules for sessions, presence, realtime HTTP, devices, inbox, conversations, messages, media, streams, and RTC
  - convenience builders for text messages, text edits, text stream frames, and JSON RTC signals

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

These scripts forward to the root `sdkwork-craw-chat-sdk/bin/generate-sdk.*` wrapper and constrain generation to the Rust target.
The forwarded flow also rechecks generated crate metadata, root-only public API imports, publish-core `check/build`, and composed crate tests before Rust regeneration is treated as complete.

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

These scripts forward to the root `sdkwork-craw-chat-sdk/bin/verify-sdk.*` wrapper and constrain verification to the Rust target.
The forwarded verification path delegates to `bin/verify-rust-workspace.mjs`, which rechecks bearer-only auth normalization, generated crate metadata, public API boundaries, generated crate publish-core `check/build`, and composed crate `cargo test`.
On Windows, the root Rust verification path sets a short `CARGO_TARGET_DIR` under `.sdkwork/rust-target` so Cargo build-script paths stay below linker limits.
If the machine cannot reach `crates.io` but already has the required crates cached locally, the root verifier retries the Rust Cargo steps with `CARGO_NET_OFFLINE=true`.

## Assemble

From this workspace:

```powershell
.\bin\sdk-assemble.ps1
```

```bash
./bin/sdk-assemble.sh
```

These scripts forward to the root `sdkwork-craw-chat-sdk/bin/assemble-sdk.mjs` wrapper and constrain assembly to the Rust target.

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

The websocket transport is documented at the workspace root but is not implemented as a handwritten Rust adapter in this round.

## Release Placeholder Boundary

This workspace inherits the current SDK release placeholder contract from `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`.

- `template_only_pending_generation`
- `not_published`
- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`
