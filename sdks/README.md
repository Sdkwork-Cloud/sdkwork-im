# SDK Workspace Overview

`sdks/` is the repository home for the Craw Chat SDK workspaces. The directory is organized by
consumer-facing SDK family, not by one-off generated package dumps.

The repository currently maintains three SDK families:

- `sdkwork-craw-chat-sdk`
  App-facing product SDKs for the public chat surface.
- `sdkwork-craw-chat-sdk-admin`
  Admin and control-plane SDKs for `/api/v1/control/*`.
- `sdkwork-craw-chat-sdk-management`
  Operator-console management SDKs for `/api/admin/*`.

## Current Repository Truth

For day-to-day engineering, the checked-in SDK workspaces and their
`.sdkwork-assembly.json` snapshots are the current source of truth.

At the repository level, all three SDK families now have materialized language workspaces for:

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

## Release Snapshot

The machine-readable release catalog under `artifacts/releases/wave-d-2026-04-08/` is now aligned
with the checked-in workspace generation state for all three SDK families.

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
| `sdkwork-craw-chat-sdk` | App and product integrations | TypeScript, Flutter | `@sdkwork/craw-chat-sdk`, `craw_chat_sdk` | `@sdkwork/craw-chat-backend-sdk`, `backend_sdk` |
| `sdkwork-craw-chat-sdk-admin` | Admin and control-plane integrations | TypeScript, Flutter | `@sdkwork/craw-chat-sdk-admin`, `craw_chat_sdk_admin` | `@sdkwork/craw-chat-admin-backend-sdk`, `craw_chat_admin_backend_sdk` |
| `sdkwork-craw-chat-sdk-management` | Operator-console management integrations | TypeScript, Flutter | `@sdkwork/craw-chat-sdk-management`, `craw_chat_sdk_management` | `@sdkwork/craw-chat-management-backend-sdk`, `craw_chat_management_backend_sdk` |

All current package lines are materialized locally and remain `not_published` until a release
freeze assigns publishable versions.

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

## Endpoint Targeting Model

The SDK families are intentionally split by runtime surface:

- App SDK clients target the app-facing public surface. In packaged installs, that is the unified
  `craw-chat-server` / `web-gateway` public origin.
- Admin SDK clients may target `control-plane-api` directly during standalone governance
  development, but packaged installs should also move to the unified public origin.
- Management SDK clients target the deployed `/api/admin/*` surface, which is likewise served from
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

- [`sdks/sdkwork-craw-chat-sdk/README.md`](./sdkwork-craw-chat-sdk/README.md)
- [`sdks/sdkwork-craw-chat-sdk-admin/README.md`](./sdkwork-craw-chat-sdk-admin/README.md)
- [`sdks/sdkwork-craw-chat-sdk-management/README.md`](./sdkwork-craw-chat-sdk-management/README.md)
- [`docs/sites/sdk/index.md`](../docs/sites/sdk/index.md)
