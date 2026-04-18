# Multilanguage Generator Standard

This document defines the root-workspace standard for `sdkwork-craw-chat-sdk` as a nine-language
SDK generation and verification hub.

## Official Language Set

The official app SDK language set is:

- `typescript`
- `flutter`
- `rust`
- `java`
- `csharp`
- `swift`
- `kotlin`
- `go`
- `python`

Every official language must be represented in:

- `generate-sdk.ps1`
- `generate-sdk.sh`
- `verify-sdk.mjs`
- `verify-sdk.ps1`
- `verify-sdk.sh`
- `.sdkwork-assembly.json`
- public SDK docs
- workspace verification entrypoints

`.sdkwork-assembly.json` is not just a language list. It must record:

- the generated transport package for every official language
- `consumerPackage` when a real consumer package is already shipped
- the extra TypeScript `root` layer for the assembled single-package consumer output

## Live Schema Refresh

The source of truth is the live service export at `/openapi/craw-chat-app.openapi.yaml`.

Required generation sequence:

1. verify or start the local service
2. refresh `openapi/craw-chat-app.openapi.yaml`
3. derive normalized generator inputs
4. stage generator-owned TypeScript entries that cannot be overwritten in place in the current
   local environment while preserving `custom/`
5. run `generate-sdk.ps1` or `generate-sdk.sh`
6. run `normalize-generated-auth-surface.mjs`
7. normalize the TypeScript generated package manifest and restore its local `bin/package-task.mjs`
   entrypoint
8. refresh `.sdkwork-assembly.json` through `assemble-sdk.mjs`
9. run `verify-sdk.mjs`

The checked-in authority snapshot is a reviewable copy of the last successful live schema refresh.
It is not the place to invent schema changes by hand.

## Generated Versus Semantic Boundary

The boundary is fixed across languages:

- `generated/server-openapi`
  Generator-owned HTTP transport layer produced from the OpenAPI 3.x contract.
- `composed`
  Manual-owned semantic layer or semantic reserve.

Special rule for TypeScript:

- the generator authoring boundary still lives under `generated/server-openapi`
- the public consumer package assembles that transport into `src/generated/**`
- the root wrappers may stage only the conflicting generator-owned entries into
  `.sdkwork/tmp/prepare-generated-output/typescript/**` before regeneration when local overwrite
  locks would otherwise break live-schema generation

Manual runtime concerns such as WebSocket receive, live runtime orchestration, message builders,
sync replay helpers, RTC helpers, and future business-facing semantic APIs must not be added to the
raw generated transport layer.

## Verification Contract

Every official language must participate in the same root verification flow:

- `verify-sdk-automation.mjs`
  root workspace guardrails
- `verify-sdk.mjs`
  root verification dispatcher
- per-language workspace verifier
- docs verification
- assembly refresh

Verification is allowed to report three kinds of outcomes:

- passed
- failed
- skipped because the native toolchain is not required for the current structural contract

The generated transport package name, the target semantic client name, the maturity tier, and the
boundary ownership must all stay aligned with `.sdkwork-assembly.json`.

For languages with a shipped consumer package, verification must also keep the assembly
`consumerPackage` descriptor aligned with the checked-in manifest:

- TypeScript: root `@sdkwork/craw-chat-sdk`
- Flutter: composed `craw_chat_sdk`

## Workspace-Owned Fixes Versus External Generator Gaps

The workspace owns:

- root wrapper behavior
- workspace layout
- normalization steps
- assembly metadata
- verification rules
- docs sync

The external generator owns:

- raw generated file structure
- language-specific template correctness
- package metadata defaults that belong in core templates

If a gap can be normalized safely in the workspace, the workspace may enforce that rule locally and
record the generator-core gap explicitly.

## Current Known External Generator Gap

The current confirmed generator-core gap is Swift package naming:

- the external generator still derives the Swift package target from `sdkType`
- the workspace requires the generated Swift package and import surface to expose
  `CrawChatBackendSdk`
- `normalize-generated-auth-surface.mjs` therefore rewrites generated Swift `Package.swift` and
  `README.md` so the workspace contract stays stable

That normalization must remain documented until the upstream generator applies the same naming rule
directly.
