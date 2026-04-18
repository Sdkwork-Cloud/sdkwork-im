# Multilanguage Audit Report

This report records the real generation and verification baseline captured on 2026-04-16 for the
`sdkwork-craw-chat-sdk` workspace.

## Audit Scope

Commands run in this audit window:

- `node sdks/sdkwork-craw-chat-sdk/bin/verify-sdk.mjs --language typescript --language flutter --language rust --language java --language csharp --language swift --language kotlin --language go --language python`
- `powershell -NoProfile -ExecutionPolicy Bypass -File sdks/sdkwork-craw-chat-sdk/bin/generate-sdk.ps1 -Languages typescript,flutter,rust,java,csharp,swift,kotlin,go,python`
- `node docs/sites/scripts/docs-verify.mjs`

Both commands completed successfully. The live-schema generation wrapper now stages the
generator-owned TypeScript entries that trigger local overwrite locks before invoking the external
generator, so the full nine-language regeneration flow now completes in this environment.

The docs-site verification also completed successfully after regeneration, including the generated
operation pages and SDK contract pages.

## Verification Ownership Baseline

The checked-in verification boundary is now intentionally split this way:

- `bin/verify-sdk-automation.mjs` owns wrapper wiring, assembly metadata, workspace presence, and
  the remaining non-duplicated generated-output normalization checks
- language workspace verifiers own language-specific README contracts and package-structure rules
- docs contract tests and `docs/sites/scripts/docs-verify.mjs` own public docs wording and site
  synchronization

That keeps the root automation verifier from duplicating TypeScript and Flutter README rules that
already live in the language-specific verification layer.

## TypeScript

- generation status: passed from the live schema on 2026-04-16 after the workspace wrapper staged
  the generator-owned TypeScript files that are locally locked for in-place overwrite
- verification status: passed through the full root verification flow against the current checked-in
  TypeScript package and generated boundary
- package naming correctness: `@sdkwork/craw-chat-sdk` remains the official consumer package and
  `@sdkwork/craw-chat-backend-sdk` remains the generated transport package
- client naming correctness: `CrawChatSdkClient` is correct and verified
- auth surface correctness: workspace auth-surface alignment verification passed
- README correctness: public and workspace TypeScript docs passed current contract checks
- semantic reserve correctness: not applicable because TypeScript assembles the generated layer into
  `src/generated/**` instead of using a separate semantic reserve package
- native toolchain verification status: TypeScript package build, pack, public API, and live
  contract verification passed
- action required in workspace: keep `bin/prepare-generated-output.mjs` wired into both generation
  wrappers so the live-schema flow continues to stage only the conflicting generator-owned
  TypeScript entries while preserving `custom/`
- action required in external generator: none confirmed from this specific failure

## Flutter

- generation status: passed from the live schema on 2026-04-16
- verification status: Flutter generated model verification, auth surface alignment, composed
  parity, public API boundary, and package metadata verification all passed
- package naming correctness: `craw_chat_sdk` remains the official app-facing package and
  `backend_sdk` remains the generated transport package
- client naming correctness: `CrawChatClient` remains correct
- auth surface correctness: workspace normalization and alignment verification passed
- README correctness: generated, composed, and workspace README guidance remained aligned
- semantic reserve correctness: Flutter continues to ship a consumer package above the generated
  boundary instead of a pure reserve-only state
- native toolchain verification status: workspace verification passed in the current environment
- action required in workspace: continue documenting that Flutter now ships mounted auth and portal
  helpers while websocket live runtime and message-first builders still lag the TypeScript baseline
- action required in external generator: none newly identified in this audit

## Rust

- generation status: passed from the live schema on 2026-04-16 with no output drift
- verification status: rust workspace verification passed
- package naming correctness: generated crate name `sdkwork-craw-chat-backend-sdk` was detected
  correctly in assembly metadata
- client naming correctness: target semantic client remains `CrawChatSdkClient`
- auth surface correctness: no workspace-specific overrides are currently required
- README correctness: workspace README correctly documents `generated/server-openapi` and `composed`
- semantic reserve correctness: the semantic layer remains a reserved manual boundary under
  `composed`
- native toolchain verification status: structural workspace verification passed
- action required in workspace: continue the Tier A semantic crate work in `composed`
- action required in external generator: none identified in this audit

## Java

- generation status: passed from the live schema on 2026-04-16 with no output drift
- verification status: java workspace verification passed
- package naming correctness: generated artifact `com.sdkwork:craw-chat-backend-sdk` was detected
  correctly in assembly metadata
- client naming correctness: target semantic client remains `CrawChatSdkClient`
- auth surface correctness: no workspace-specific overrides are currently required
- README correctness: workspace README correctly documents `generated/server-openapi` and `composed`
- semantic reserve correctness: the semantic layer remains a reserved manual boundary under
  `composed`
- native toolchain verification status: structural workspace verification passed
- action required in workspace: keep the docs honest about transport-standardized delivery
- action required in external generator: none identified in this audit

## C#

- generation status: passed from the live schema on 2026-04-16 with no output drift
- verification status: csharp workspace verification passed
- package naming correctness: generated package `Sdkwork.CrawChat.BackendSdk` was detected
  correctly in assembly metadata
- client naming correctness: target semantic client remains `CrawChatSdkClient`
- auth surface correctness: no workspace-specific overrides are currently required
- README correctness: workspace README correctly documents `generated/server-openapi` and `composed`
- semantic reserve correctness: the semantic layer remains a reserved manual boundary under
  `composed`
- native toolchain verification status: structural workspace verification passed
- action required in workspace: keep transport-first documentation aligned with actual .NET output
- action required in external generator: none identified in this audit

## Swift

- generation status: passed from the live schema on 2026-04-16
- verification status: swift workspace verification passed after workspace normalization
- package naming correctness: the workspace-normalized generated package exposes
  `CrawChatBackendSdk`
- client naming correctness: target semantic client remains `CrawChatSdkClient`
- auth surface correctness: workspace normalization completed successfully
- README correctness: generated README is normalized to import `CrawChatBackendSdk`
- semantic reserve correctness: the semantic layer remains a reserved manual boundary under
  `composed`
- native toolchain verification status: structural workspace verification passed
- action required in workspace: keep the Swift normalization step active until upstream naming is
  corrected
- action required in external generator: Swift package naming still ignores the requested
  `packageName` and falls back to a generic backend target name

## Kotlin

- generation status: passed from the live schema on 2026-04-16 with no output drift
- verification status: kotlin workspace verification passed
- package naming correctness: generated artifact `com.sdkwork:craw-chat-backend-sdk` was detected
  correctly in assembly metadata
- client naming correctness: target semantic client remains `CrawChatSdkClient`
- auth surface correctness: no workspace-specific overrides are currently required
- README correctness: workspace README correctly documents `generated/server-openapi` and `composed`
- semantic reserve correctness: the semantic layer remains a reserved manual boundary under
  `composed`
- native toolchain verification status: structural workspace verification passed
- action required in workspace: keep transport-standardized docs and verification aligned
- action required in external generator: none identified in this audit

## Go

- generation status: passed from the live schema on 2026-04-16 with no output drift
- verification status: go workspace verification passed
- package naming correctness: generated module `github.com/sdkwork/craw-chat-backend-sdk` was
  detected correctly in assembly metadata
- client naming correctness: target semantic client remains `CrawChatSdkClient`
- auth surface correctness: no workspace-specific overrides are currently required
- README correctness: workspace README correctly documents `generated/server-openapi` and `composed`
- semantic reserve correctness: the semantic layer remains a reserved manual boundary under
  `composed`
- native toolchain verification status: structural workspace verification passed
- action required in workspace: continue documenting Go as transport-standardized rather than
  semantically complete
- action required in external generator: none identified in this audit

## Python

- generation status: passed from the live schema on 2026-04-16 with no output drift
- verification status: python workspace verification passed
- package naming correctness: generated package `sdkwork-craw-chat-backend-sdk` was detected
  correctly in assembly metadata
- client naming correctness: target semantic client remains `CrawChatSdkClient`
- auth surface correctness: no workspace-specific overrides are currently required
- README correctness: workspace README correctly documents `generated/server-openapi` and `composed`
- semantic reserve correctness: the semantic layer remains a reserved manual boundary under
  `composed`
- native toolchain verification status: structural workspace verification passed
- action required in workspace: keep the Python transport package and semantic reserve naming stable
- action required in external generator: none identified in this audit
