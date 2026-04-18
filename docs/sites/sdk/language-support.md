# Language Support

This page explains which SDK workspaces exist today, which package names are the official consumer
entrypoints, and how to interpret repository support without overstating registry publication.

## Current Verified Baseline

The current docs baseline covers two SDK families and four checked-in consumer packages:

- app TypeScript: `@sdkwork/craw-chat-sdk`
- app Flutter: `craw_chat_sdk`
- admin TypeScript: `@sdkwork/craw-chat-admin-sdk`
- admin Flutter: `craw_chat_admin_sdk`

Across both families, the checked-in workspace rule is the same:

- generated output lives under `generated/server-openapi`
- manual ergonomic code lives under `composed`
- root verification refreshes `.sdkwork-assembly.json`

Those package names and directory rules are a repo contract. They are not a claim that the current
wave has already published every package.

## How To Use This Page

Use this page to answer three questions in order:

1. Which SDK family owns the consumer boundary: app or admin?
2. Which language package is the official consumer package versus the raw generated transport?
3. Is the package already published, or is it still a checked-in repo contract with
   `not_published` release status?

That order keeps the docs precise. A workspace directory and a package name do not automatically
prove registry publication.

## Workspace Boundary Matrix

| Family | Language | Official consumer package | Generated transport package | Primary client | Workspace boundaries | Current release reading |
| --- | --- | --- | --- | --- | --- | --- |
| App | TypeScript | `@sdkwork/craw-chat-sdk` | `@sdkwork/craw-chat-backend-sdk` | `CrawChatSdkClient` | `generated/server-openapi` plus `composed` | Repo contract present, current catalog still `not_published` |
| App | Flutter | `craw_chat_sdk` | `backend_sdk` | `CrawChatSdkClient` | `generated/server-openapi` plus `composed` | Repo contract present, current catalog still `not_published` |
| Admin | TypeScript | `@sdkwork/craw-chat-admin-sdk` | `@sdkwork/craw-chat-admin-backend-sdk` | `CrawChatAdminSdkClient` | `generated/server-openapi` plus `composed` | Verified local workspace contract, current catalog still `not_published` |
| Admin | Flutter | `craw_chat_admin_sdk` | `craw_chat_admin_backend_sdk` | `CrawChatAdminSdkClient` | `generated/server-openapi` plus `composed` | Verified local workspace contract, current catalog still `not_published` |

## Assembly And Verification Semantics

Root verification is the canonical way to refresh release-facing SDK metadata.

For the checked-in SDK families, `.sdkwork-assembly.json` records:

- authority and derived spec paths
- per-language package `manifestPath` values
- the explicit `generated` and `composed` package layers
- a `generatedAt` timestamp that stays stable when assembly content is unchanged

Representative root verification entrypoints are:

- `node .\sdks\sdkwork-craw-chat-sdk\bin\verify-sdk.mjs`
- `node .\sdks\sdkwork-craw-chat-sdk-admin\bin\verify-sdk.mjs --language typescript --language flutter`

Use those commands as the maintainer contract. Do not infer support by scanning directories alone.

## Release Catalog Facts

From `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`:

| Artifact ID | Family | Language | Generation | Release |
| --- | --- | --- | --- | --- |
| `app-typescript` | app | typescript | `template_only_pending_generation` | `not_published` |
| `app-flutter` | app | flutter | `template_only_pending_generation` | `not_published` |
| `admin-typescript` | admin | typescript | `template_only_pending_generation` | `not_published` |
| `admin-flutter` | admin | flutter | `template_only_pending_generation` | `not_published` |

That means the current language pages should describe package names, workspace boundaries, and
verification rules as a repo contract rather than promising a published package release.

## Reference Entry Points

- [App SDK Overview](/sdk/app-sdk)
  Family-level source-of-truth and assembly semantics for the app runtime SDKs.
- [TypeScript SDK](/sdk/typescript-sdk)
  Current app TypeScript consumer package, upload flow, and maintainer workflow.
- [Flutter SDK](/sdk/flutter-sdk)
  Current app Flutter consumer package, parity gap, and local workflow.
- [Admin SDK](/sdk/admin-sdk)
  Family-level source-of-truth and assembly semantics for the control-plane SDKs.
- [Admin TypeScript SDK](/sdk/admin-typescript-sdk)
  Admin control-plane TypeScript consumer package and browser-only helper boundary.
- [Admin Flutter SDK](/sdk/admin-flutter-sdk)
  Admin control-plane Flutter consumer package and Dart verification flow.

## What "Supported Language" Means Here

In this documentation, language support means:

- the workspace boundary exists
- the official package name is documented
- generated versus manual ownership is explicit
- root verification and assembly behavior are documented

It does not mean:

- a package is already published to npm or pub.dev
- every language has identical runtime ergonomics
- the release wave has assigned a frozen version number

That distinction is necessary to keep the docs precise, trustworthy, and aligned with the current
repo contract.
