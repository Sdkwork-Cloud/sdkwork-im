# SDK Workspace Overview

`sdks/` is the repository home for the Craw Chat SDK workspaces. The directory is organized by
consumer-facing SDK family, not by one-off generated package dumps.

The repository currently maintains four SDK families:

- `sdkwork-im-sdk`
  App-facing product SDKs for the public chat surface.
- `sdkwork-control-plane-sdk`
  Control-plane SDKs for `/api/v1/control/*`.
- `sdkwork-im-admin-sdk`
  IM admin SDKs for `/api/admin/*`.
- `sdkwork-rtc-sdk`
  Provider-standard RTC SDKs for unified multi-provider audio/video integration.

## Current Repository Truth

For day-to-day engineering, the checked-in SDK workspaces and their
`.sdkwork-assembly.json` snapshots are the current source of truth.

At the repository level, all three API-contract SDK families now have materialized language
workspaces for:

- TypeScript
- Flutter

Across those language workspaces, the standard package layering is:

- `generated/server-openapi`
  Generator-owned HTTP transport package output.
- `composed`
  Manual-owned, consumer-facing SDK package built above the generated transport layer.

This means the repository no longer treats the SDK families as placeholder-only scaffolding.
Generated packages, composed packages, regeneration wrappers, verification entrypoints, and package
documentation all exist in-repo for the currently supported language lines.

## Compatibility And Validation Inputs

The SDK workspace index also tracks the shared contract vocabulary that downstream verification
consumes.

- The public app-facing family remains pinned to the current `compatibility matrix` vocabulary and
  recovery registry.
- Control-plane governance remains the upstream source for protocol registry, protocol governance,
  compatibility visibility, and recovery-state terminology shared across SDK families.
- The single verification index for SDK, CLI, and operator consumers is
  `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`.

## Release Snapshot

The machine-readable release catalog under
`artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json` is now aligned with the checked-in
workspace generation state for all three API-contract SDK families.

Current release-catalog values include:

- `state = generated_pending_publication`
- `generationStatus = generated`
- `releaseStatus = not_published`
- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`

Use that catalog for release-state auditing. For package boundaries, consumer surfaces, and
regeneration behavior, the checked-in workspaces and `.sdkwork-assembly.json` snapshots remain the
richer engineering truth.

## Workspace Matrix

| Workspace | Audience | Languages | Primary composed package(s) | Primary generated package(s) |
| --- | --- | --- | --- | --- |
| `sdkwork-im-sdk` | App and product integrations | TypeScript, Flutter | `@sdkwork/im-sdk`, `im_sdk` | `@sdkwork-internal/im-sdk-generated`, `im_sdk_generated` |
| `sdkwork-control-plane-sdk` | Control-plane integrations | TypeScript, Flutter | `@sdkwork/control-plane-sdk`, `control_plane_sdk` | `@sdkwork/control-plane-backend-sdk`, `control_plane_backend_sdk` |
| `sdkwork-im-admin-sdk` | IM admin and operator-console integrations | TypeScript, Flutter | `@sdkwork/im-admin-sdk`, `im_admin_sdk` | `@sdkwork/im-admin-backend-sdk`, `im_admin_backend_sdk` |
| `sdkwork-rtc-sdk` | RTC provider-standard integrations | TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python | `@sdkwork/rtc-sdk` | Provider-adapter workspaces, not OpenAPI-generated transport packages |

All current package lines are materialized locally and remain `not_published` until a release
freeze assigns publishable versions.

For `sdkwork-im-sdk`, the TypeScript generated package name
`@sdkwork-internal/im-sdk-generated` is a workspace-internal identity only. App consumers
should install `@sdkwork/im-sdk`, not the generated package directly.

## Standard Package Boundary

Every SDK family follows the same boundary rules:

- The OpenAPI 3.x authority contract is checked into the workspace under `openapi/`.
- Generator-compatible derived contracts stay in the same workspace and remain traceable to the
  authority contract.
- Generated code is owned only under `generated/server-openapi`.
- Consumer-facing orchestration, ergonomic client facades, and manual integration helpers live only
  under `composed`.
- Manual code must consume generated output through package root entrypoints only.
- Downstream code must not import generated private source paths such as
  `generated/server-openapi/src/*` or language-specific private internals.

`sdkwork-rtc-sdk` intentionally differs from the OpenAPI-first families:

- it is a provider-standard workspace, not a route-generated SDK workspace
- it standardizes `Driver`, `DriverManager`, `DataSource`, `capabilities`, and `unwrap()`
- it materializes provider adapters instead of generated transport packages

## Endpoint Targeting Model

The SDK families are intentionally split by runtime surface:

- App SDK clients target the app-facing public surface. In packaged installs, that is the unified
  `craw-chat-server` / `web-gateway` public origin.
- Control-plane SDK clients may target `control-plane-api` directly during standalone governance
  development, but packaged installs should also move to the unified public origin.
- IM admin SDK clients target the deployed `/api/admin/*` surface, which is likewise served from
  the unified public origin in packaged installs.

## Regeneration And Verification

Each workspace owns its own wrappers under `bin/` and supports the same high-level loop:

1. Refresh or materialize the checked-in authority contract.
2. Produce derived generator inputs.
3. Generate the language-specific transport package.
4. Reapply workspace-owned normalization rules.
5. Refresh the composed package and assembly metadata.
6. Run workspace verification before treating the output as valid.

Current verification coverage includes:

- generated package artifact checks
- composed package boundary validation
- discovery-surface alignment checks
- README and package metadata quality gates
- Flutter workspace structure validation
- TypeScript workspace build and smoke verification

## Recommended Reading

- [`sdks/sdkwork-im-sdk/README.md`](./sdkwork-im-sdk/README.md)
- [`sdks/sdkwork-control-plane-sdk/README.md`](./sdkwork-control-plane-sdk/README.md)
- [`sdks/sdkwork-im-admin-sdk/README.md`](./sdkwork-im-admin-sdk/README.md)
- [`sdks/sdkwork-rtc-sdk/README.md`](./sdkwork-rtc-sdk/README.md)
- [`docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`](../docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md)
- [`docs/sites/sdk/index.md`](../docs/sites/sdk/index.md)
