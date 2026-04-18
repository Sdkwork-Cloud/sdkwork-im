# SDK Overview

The repository currently defines three SDK families with different consumers, contracts, and release
truth sources:

- `sdkwork-craw-chat-sdk`
  The app-facing SDK workspace.
- `sdkwork-craw-chat-sdk-admin`
  The admin and control-plane SDK workspace.
- `sdkwork-craw-chat-sdk-management`
  The operator-console management SDK workspace.

## Current Delivery Reality

For day-to-day engineering, treat the checked-in SDK workspaces and their `.sdkwork-assembly.json`
files as the current repository truth. The machine-readable release catalog at
`artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json` now agrees with the checked-in
workspace generation state for all six tracked SDK language artifacts.

The checked-in SDK assemblies show that:

- `sdkwork-craw-chat-sdk` has materialized TypeScript and Flutter package workspaces
- `sdkwork-craw-chat-sdk-admin` has materialized TypeScript and Flutter package workspaces
- `sdkwork-craw-chat-sdk-management` has materialized TypeScript and Flutter package workspaces
- none of the packages are published yet

### Release Snapshot

The current wave-d bundle declares:

- `state = generated_pending_publication`
- all tracked artifacts show `generationStatus = generated`
- all tracked artifacts show `releaseStatus = not_published`
- `plannedVersion` remains `null` and `versionStatus` remains `version_unassigned_pending_freeze`

That snapshot is useful for release-state auditing. The checked-in workspaces and
`.sdkwork-assembly.json` files remain the richer engineering truth for consumer surfaces,
regeneration behavior, and package boundaries.

Generated symbols must be consumed through package root entrypoints only. Manual-owned bridge code
and composed packages must not import `generated/server-openapi/src/*` private source paths.

## SDK Family Matrix

| Family | Audience | Languages | Contract source | Current release state |
| --- | --- | --- | --- | --- |
| `sdkwork-craw-chat-sdk` | App and product integrations | TypeScript, Flutter | Checked-in OpenAPI 3.0.3 authority at `sdks/sdkwork-craw-chat-sdk/openapi/craw-chat-app.openapi.yaml` plus the derived `craw-chat-app.sdkgen.yaml` | App workspace materialized locally, still `not_published` |
| `sdkwork-craw-chat-sdk-admin` | Admin and control-plane integrations | TypeScript, Flutter | Checked-in OpenAPI 3.1 authority at `sdks/sdkwork-craw-chat-sdk-admin/openapi/craw-chat-control-plane.openapi.json` plus the derived `craw-chat-control-plane.sdkgen.json`, both sourced from `control-plane-api` | Admin TypeScript and Flutter workspaces are materialized locally, all still `not_published` |
| `sdkwork-craw-chat-sdk-management` | Operator-console management integrations | TypeScript, Flutter | Checked-in OpenAPI 3.1 authority at `sdks/sdkwork-craw-chat-sdk-management/openapi/craw-chat-management.openapi.json` plus the derived `craw-chat-management.sdkgen.json`, both sourced from the current `/api/admin/*` boundary inventory | TypeScript and Flutter workspaces are materialized locally; the admin console already consumes the TypeScript line through its compatibility layer, all still `not_published` |

Read the matrix as follows:

- "materialized locally" means the package workspace, generated output, and verification path exist
  in-repo today
- "`not_published`" means the package is not yet available from a public package registry

## API Group To SDK Mapping

| API group | SDK family | Notes |
| --- | --- | --- |
| App API | `sdkwork-craw-chat-sdk` | Public app-facing surface; the unified gateway proxies the realtime websocket upgrade, but the SDK generation round still treats the websocket adapter as manual-owned |
| Platform API | No separate published family | Routes exist and are documented, but not split into a standalone SDK family |
| IoT API | No separate published family | Currently documented as HTTP and provider-integration surfaces |
| Control Plane API | `sdkwork-craw-chat-sdk-admin` | Administrative and governance surface |
| Operator Console Admin API (`/api/admin/*`) | `sdkwork-craw-chat-sdk-management` | Checked-in authority plus materialized TypeScript and Flutter generated/composed packages exist; the admin console already consumes this family through its TypeScript compatibility layer |

## Endpoint Selection

- App SDK consumers target `local-minimal-node` during direct local development and the unified
  `craw-chat-server` / `web-gateway` public origin in packaged installs.
- Admin SDK consumers can target `control-plane-api` directly during standalone governance
  development, but packaged installs should switch to the unified gateway public origin.
- Management SDK consumers target the deployed surface that serves `/api/admin/*`; in packaged
  installs that is also the unified gateway public origin.

## Source-of-truth Rules

- The app SDK workspace has a checked-in OpenAPI authority contract and a derived sdkgen input.
- The admin SDK workspace now has a checked-in OpenAPI authority contract, a derived sdkgen input,
  and an assembly snapshot. Those artifacts are refreshed from the control-plane service export.
- The management SDK workspace now has a checked-in OpenAPI authority contract, a derived sdkgen
  input, and an assembly snapshot for the operator-console `/api/admin/*` backend.
- Generated output must not be hand-edited in place. Change the authority contract or workspace
  wrapper inputs and regenerate.

## Recommended Reading

- [App SDK](/sdk/app-sdk)
- [Admin SDK](/sdk/admin-sdk)
- [Management SDK](/sdk/management-sdk)
- [Language Support](/sdk/language-support)
- [App API Overview](/api-reference/app-api)
- [Control Plane API Overview](/api-reference/control-plane-api)
