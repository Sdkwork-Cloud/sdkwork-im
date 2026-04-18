# Language Support

This page separates two questions:

1. Which language workspaces are usable inside the repository today?
2. Which package lines have been published as part of a historical release wave?

## Workspace Matrix

| Audience | Language | Workspace | Current state |
| --- | --- | --- | --- |
| App | TypeScript | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-typescript` | Workspace materialized, locally verifiable, publication still pending |
| App | Flutter | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-flutter` | Workspace materialized, locally verifiable, publication still pending |
| Admin | TypeScript | `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-typescript` | Workspace materialized, locally verifiable, publication still pending |
| Admin | Flutter | `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-flutter` | Workspace materialized, locally verifiable, publication still pending |
| Management | TypeScript | `sdks/sdkwork-craw-chat-sdk-management/sdkwork-craw-chat-sdk-management-typescript` | Workspace materialized, locally verifiable, publication still pending |
| Management | Flutter | `sdks/sdkwork-craw-chat-sdk-management/sdkwork-craw-chat-sdk-management-flutter` | Workspace materialized, locally verifiable, publication still pending |

## Practical Interpretation

If you are integrating against the current repository state:

- app TypeScript and Flutter are available as checked-in workspaces
- admin TypeScript and Flutter are available as checked-in workspaces
- management TypeScript and Flutter are available as checked-in workspaces

If you are auditing release readiness instead of repository readiness, use the machine-readable
release snapshot below.

## Current Package Roots

For the currently materialized package lines:

- app TypeScript uses generated `@sdkwork/craw-chat-backend-sdk` and composed `@sdkwork/craw-chat-sdk`
- app Flutter uses generated `backend_sdk` and composed `craw_chat_sdk`
- admin TypeScript uses generated `@sdkwork/craw-chat-admin-backend-sdk` and composed `@sdkwork/craw-chat-sdk-admin`
- admin Flutter uses generated `craw_chat_admin_backend_sdk` and composed `craw_chat_sdk_admin`
- management TypeScript uses generated `@sdkwork/craw-chat-management-backend-sdk` and composed `@sdkwork/craw-chat-sdk-management`
- management Flutter uses generated `craw_chat_management_backend_sdk` and composed `craw_chat_sdk_management`

## Runtime Entry Targets

- app SDK consumers target `local-minimal-node` during direct local development and the unified
  gateway public origin in packaged installs
- admin SDK consumers can target `control-plane-api` during standalone governance development, but
  packaged installs should switch to the unified gateway public origin
- management SDK consumers target the deployed `/api/admin/*` surface; in packaged installs that
  surface is also reached through the unified gateway public origin

## Release Catalog Facts

From the current `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`:

| Artifact ID | Audience | Language | Generation | Release |
| --- | --- | --- | --- | --- |
| `app-typescript` | app | typescript | `generated` | `not_published` |
| `app-flutter` | app | flutter | `generated` | `not_published` |
| `admin-typescript` | admin | typescript | `generated` | `not_published` |
| `admin-flutter` | admin | flutter | `generated` | `not_published` |
| `management-typescript` | management | typescript | `generated` | `not_published` |
| `management-flutter` | management | flutter | `generated` | `not_published` |

The current release catalog and the checked-in workspaces agree that all six language lines are
generated locally and remain unpublished.

Version freeze is still pending across the catalog:

- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`

## What "Supported Language" Means Here

In this documentation, language support means:

- the workspace boundary exists
- the target language directory exists
- generation wrappers and documentation are present
- the repository workspace and verification chain can prove the current state

It does not mean:

- a package has been published to npm or pub.dev
- a stable version number has been assigned
- every historical release-catalog entry has been refreshed to match current repository truth

That distinction is necessary to keep the docs precise and trustworthy.
