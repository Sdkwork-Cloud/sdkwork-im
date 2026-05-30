# SDK Workspace Overview

`sdks/` is the repository home for Craw Chat SDK workspaces. The directory is organized by public
consumer SDK family and by authoritative API boundary, not by historical generated-package dumps.

The current standard model has three OpenAPI-generated HTTP SDK families plus one independent RTC
provider-standard SDK family:

- `sdkwork-im-sdk`
  IM standardized development SDKs for `/im/v3/api`.
- `sdkwork-im-app-sdk`
  App-business and non-management HTTP SDKs for `/app/v3/api`.
- `sdkwork-im-backend-sdk`
  Backend management, operator, control-plane, and admin SDKs for `/backend/v3/api`.
- `sdkwork-rtc-sdk`
  Provider-standard RTC SDKs for multi-provider audio/video runtime integration.

## Current Repository Truth

For day-to-day engineering, the checked-in SDK workspaces and their `.sdkwork-assembly.json`
snapshots are the source of truth.

The three Craw Chat HTTP-contract SDK families are separated by target surface:

- `sdkwork-im-sdk` owns the IM standardized development API under `/im/v3/api/*`.
- `sdkwork-im-app-sdk` owns app-business API under `/app/v3/api/*`.
- `sdkwork-im-backend-sdk` owns backend management API under `/backend/v3/api/*`.

The RTC workspace is intentionally separate from the OpenAPI-generated HTTP SDK families:

- `sdkwork-rtc-sdk` owns provider standards, driver contracts, provider catalogs, runtime surface
  rules, and provider package boundaries.
- It is not a route-generated SDK workspace and must not be collapsed into app or backend generated
  transport packages.

## API Boundary Rules

Every API must map to exactly one SDK family:

- IM standardized development API: `/im/v3/api/*` -> `sdkwork-im-sdk`.
- App-business and non-management API: `/app/v3/api/*` -> `sdkwork-im-app-sdk`.
- Backend management, operator, governance, control-plane, and admin API:
  `/backend/v3/api/*` -> `sdkwork-im-backend-sdk`.
- RTC provider/runtime standard: `sdkwork-rtc-sdk`, not an OpenAPI HTTP family.

Backend management modules currently include:

- `/backend/v3/api/ops/*`
- `/backend/v3/api/audit/*`
- `/backend/v3/api/automation/*`
- `/backend/v3/api/control/*`
- `/backend/v3/api/admin/*`

App API absorbs HTTP surfaces that are not management-system APIs and are not part of the IM
standardized development API. Representative examples include provider health, IoT protocol, and
RTC provider callback or health routes after they are exposed under `/app/v3/api/*`.

## Language Baseline

The official OpenAPI-generated language set for the three HTTP SDK families is:

- TypeScript
- Flutter
- Rust
- Java
- C#
- Swift
- Kotlin
- Go
- Python

For OpenAPI-generated SDK families, generator-owned transport output is always under
`generated/server-openapi`. Manual-owned consumer-facing facades live under `composed` only when a
family has a semantic SDK layer. `sdkwork-im-app-sdk` and `sdkwork-im-backend-sdk` currently publish
generated transport packages directly and verify the generated primary clients `SdkworkAppClient`
and `SdkworkBackendClient`.

The IM TypeScript line publishes the consumer package `@sdkwork/im-sdk`; its generated TypeScript
transport package uses `@sdkwork-internal/im-sdk-generated` as a workspace-internal identity only.
The IM Flutter line publishes `im_sdk` for consumers and keeps `im_sdk_generated` as the generated
transport package.

## Workspace Matrix

| Workspace | Audience | Languages | Primary package boundary |
| --- | --- | --- | --- |
| `sdkwork-im-sdk` | IM standardized development integrations | TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python | Semantic IM SDK package plus generated IM transport |
| `sdkwork-im-app-sdk` | App developers and app-business integrations | TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python | Generated app transport package with `SdkworkAppClient` |
| `sdkwork-im-backend-sdk` | Backend, operator, control-plane, and admin integrations | TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python | Generated backend transport package with `SdkworkBackendClient` |
| `sdkwork-rtc-sdk` | RTC provider-standard integrations | TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python | Provider-standard packages and adapters, not OpenAPI-generated transport |

All current package lines remain `not_published` until a release freeze assigns publishable
versions. The release snapshot is recorded in
`artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`.

- IM, App API, and Backend API HTTP SDK artifacts use `state = generated_pending_publication` and
  `generationStatus = generated`.
- RTC provider-runtime SDK artifacts use `state = template_only_pending_generation` and
  `generationStatus = template_only_pending_generation`.
- `releaseStatus = not_published`
- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`

## Standard Package Boundary

Every OpenAPI-generated SDK family follows the same boundary rules:

- The OpenAPI 3.x authority contract is checked into the workspace under `openapi/`.
- Generator-compatible derived contracts stay in the same workspace and remain traceable to the
  authority contract.
- Generated code is owned only under `generated/server-openapi`.
- Consumer-facing orchestration, ergonomic client facades, and manual integration helpers live only
  under `composed` when that SDK family has a semantic manual layer.
- Manual code must consume generated output through package root entrypoints only.
- Downstream code must not import generated private source paths such as
  `generated/server-openapi/src/*` or language-specific private internals.
- `/app/v3/api` and `/backend/v3/api` SDK families use the shared `sdkwork-v3` generation profile,
  dual-token `AuthToken` plus `AccessToken` security, and `application/problem+json` error
  responses.

`sdkwork-rtc-sdk` intentionally differs from the OpenAPI-first families:

- it is a provider-standard workspace, not a route-generated SDK workspace
- it standardizes `Driver`, `DriverManager`, `DataSource`, `capabilities`, and `unwrap()`
- it materializes provider adapters instead of generated transport packages

## Regeneration And Verification

The standard SDK boundary materialization entrypoint is:

```powershell
node .\sdks\materialize-im-v3-openapi-boundaries.mjs
```

That command consolidates backend control/admin authority into `sdkwork-im-backend-sdk`, keeps
non-management HTTP APIs in `sdkwork-im-app-sdk`, and refreshes derived OpenAPI inputs.

Each OpenAPI workspace then owns its own wrappers under `bin/`:

```powershell
node .\sdks\sdkwork-im-sdk\bin\verify-sdk.mjs
node .\sdks\sdkwork-im-app-sdk\bin\verify-sdk.mjs
node .\sdks\sdkwork-im-backend-sdk\bin\verify-sdk.mjs
```

Use the RTC verifier for the independent provider-standard SDK:

```powershell
node .\sdks\sdkwork-rtc-sdk\bin\verify-sdk.mjs
```

## Recommended Reading

- [`sdks/sdkwork-im-sdk/README.md`](./sdkwork-im-sdk/README.md)
- [`sdks/sdkwork-im-app-sdk/README.md`](./sdkwork-im-app-sdk/README.md)
- [`sdks/sdkwork-im-backend-sdk/README.md`](./sdkwork-im-backend-sdk/README.md)
- [`sdks/sdkwork-rtc-sdk/README.md`](./sdkwork-rtc-sdk/README.md)
- [`docs/sites/sdk/index.md`](../docs/sites/sdk/index.md)
