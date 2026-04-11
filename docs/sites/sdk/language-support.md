# Language Support

## Workspace Matrix

| Audience | Language | Workspace | Current state |
| --- | --- | --- | --- |
| App | TypeScript | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-typescript` | Workspace present, generation and publication still pending |
| App | Flutter | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-flutter` | Workspace present, generation and publication still pending |
| Admin | TypeScript | `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-typescript` | Workspace present, generation and publication still pending |
| Admin | Flutter | `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-flutter` | Workspace present, generation and publication still pending |

## Release Catalog Facts

From `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`:

| Artifact ID | Audience | Language | Generation | Release |
| --- | --- | --- | --- | --- |
| `app-typescript` | app | typescript | `template_only_pending_generation` | `not_published` |
| `app-flutter` | app | flutter | `template_only_pending_generation` | `not_published` |
| `admin-typescript` | admin | typescript | `template_only_pending_generation` | `not_published` |
| `admin-flutter` | admin | flutter | `template_only_pending_generation` | `not_published` |

## What "Supported Language" Means Here

In this documentation, language support means:

- the workspace boundary exists
- the target language directory exists
- generation wrappers and documentation are present
- the release catalog already tracks the artifact

It does not mean:

- a package has been published to npm or pub.dev
- a stable version number has been assigned
- the current wave has completed generation

That distinction is necessary to keep the docs precise and trustworthy.
