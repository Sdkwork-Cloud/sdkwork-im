# SDK Verification Matrix

This document summarizes the verification responsibilities inside `sdkwork-craw-chat-sdk`.

Use it when you need to know which script enforces which contract.

## Root Entry Point

The first verification command for maintainers is:

```powershell
node .\bin\verify-sdk.mjs
```

This root entrypoint coordinates both language workspaces and shared automation checks.

## Shared Automation Checks

- `bin/verify-sdk-automation.mjs`
  Verifies wrapper wiring, workspace README requirements, internal docs requirements, and language
  README contract wording.
- `bin/verify-internal-docs.mjs`
  Verifies the internal standards docs keep their required section structure, naming markers, and
  cross-links.
- `bin/verify-docs-contract-tests.mjs`
  Runs the docs-contract regression tests that guard the internal verifier and docs coverage
  expectations themselves.
- `bin/verify-powershell-wrapper-args.mjs`
  Verifies documented PowerShell wrapper argument compatibility.
- `bin/verify-auth-surface-alignment.mjs`
  Verifies both language surfaces stay on the bearer-token contract.

## TypeScript Checks

- `bin/verify-typescript-workspace.mjs`
  Runs the full TypeScript workspace verification flow: generated transport packaging, auth/public API
  boundary checks, explicit live-contract verification for the payload-first realtime surface,
  explicit `composed` package layout plus typecheck/build/test validation, then the assembled
  single-package layout, typecheck/build/smoke, and publishability checks.
- `bin/verify-typescript-live-contract.mjs`
  Runs compile-time contract tests for the TypeScript live receive surface. This verifies the
  domain-stream API stays on `live.messages/data/signals/events/lifecycle`, legacy flat callbacks
  stay removed, and resolved live states stay limited to `connected`, `error`, and `closed`.
- `bin/verify-typescript-generated-package.mjs`
  Verifies generated transport packaging, including `npm pack --dry-run --json`.
- `bin/verify-typescript-single-package-publishability.mjs`
  Verifies the publishable `@sdkwork/craw-chat-sdk` package can pass `npm pack --dry-run --json`
  with an isolated local npm cache and without shipping workspace residue.
- `bin/verify-typescript-generated-package-temp-cleanup.mjs`
  Verifies generation and verification temp directories are cleaned correctly.
- `bin/verify-typescript-generated-build-determinism.mjs`
  Verifies stable TypeScript generated builds remain deterministic.
- `bin/verify-typescript-generated-build-concurrency.mjs`
  Verifies concurrent verification runs do not regress into temp/log collisions.
- `bin/verify-typescript-workspace-concurrency.mjs`
  Verifies overlapping full TypeScript workspace verification runs do not regress into shared
  generated-package artifact races while `npm pack --dry-run` is validating `dist/**`.
- `bin/verify-typescript-single-package-layout.mjs`
  Verifies the assembled root package still follows the single-package layout.
- `bin/verify-typescript-public-api-boundary.mjs`
  Verifies public API boundaries and generated-type import routing.

## Flutter Checks

- `bin/verify-flutter-workspace.mjs`
  Runs the Flutter workspace verification flow.
- `bin/verify-flutter-generated-models.mjs`
  Verifies generated Dart models do not regress into broken primitive wrappers.
- `bin/verify-flutter-public-api-boundary.mjs`
  Verifies manual code only uses the public generated package entrypoint.
- `bin/verify-flutter-composed-parity.mjs`
  Verifies `craw_chat_sdk` keeps the expected app-runtime helper surface.
- `bin/verify-flutter-package-metadata.mjs`
  Verifies Flutter package naming and metadata alignment.
- `bin/verify-flutter-dart-analysis.dart`
  Windows fallback analyzer entrypoint used when direct `dart analyze` spawning is constrained.

## Transport-Standardized Language Checks

- `bin/verify-rust-workspace.mjs`
  Verifies the Rust workspace layout, generated transport boundary, and reserved `composed`
  boundary.
- `bin/verify-java-workspace.mjs`
  Verifies the Java workspace layout, generated artifact identity, and reserved `composed`
  boundary.
- `bin/verify-csharp-workspace.mjs`
  Verifies the C# workspace layout, generated package identity, and reserved `composed`
  boundary.
- `bin/verify-swift-workspace.mjs`
  Verifies the Swift workspace layout plus the workspace-normalized generated package identity.
- `bin/verify-kotlin-workspace.mjs`
  Verifies the Kotlin workspace layout, generated artifact identity, and reserved `composed`
  boundary.
- `bin/verify-go-workspace.mjs`
  Verifies the Go workspace layout, generated module identity, and reserved `composed`
  boundary.
- `bin/verify-python-workspace.mjs`
  Verifies the Python workspace layout, generated package identity, and reserved `composed`
  boundary.

## Docs Checks

When wording or public docs move, also run:

```powershell
node ..\..\docs\sites\scripts\verify-api-docs.mjs
node ..\..\docs\sites\scripts\verify-sdk-docs.mjs
```

These scripts enforce the public docs naming model and API-to-SDK wording standards.
Use `sites/README.md` as the maintainer map for which public pages and package names must stay in
sync.

## Release Gate Expectation

Do not describe generation or wording work as complete until:

- `bin/verify-sdk.mjs` is green for the affected languages
- public docs checks are green when any docs content changed
- changed package standards remain aligned with the live schema and package-entrypoint rules

## Related Docs

- internal docs map: `README.md`
- package standards: `package-standards.md`
- generation pipeline: `generation-pipeline.md`
- realtime extension boundary: `realtime-extension-boundary.md`
- public docs sync standard: `sites/README.md`
