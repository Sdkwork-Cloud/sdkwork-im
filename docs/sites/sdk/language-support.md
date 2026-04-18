# Language Support

The app SDK family currently spans three implementation languages in this worktree and one
secondary admin audience that remains separate from the primary app integration path.

## App SDK Languages

| Language | Workspace | Preferred public package or crate | Generated transport package or crate | Start here |
| --- | --- | --- | --- | --- |
| TypeScript | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-typescript` | `@sdkwork/craw-chat-sdk` | `@sdkwork/craw-chat-backend-sdk` | [/sdk/typescript-quick-start](/sdk/typescript-quick-start) |
| Flutter | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-flutter` | `craw_chat_sdk` | `backend_sdk` | [/sdk/flutter-quick-start](/sdk/flutter-quick-start) |
| Rust | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-rust` | `craw-chat-sdk` | `sdkwork-craw-chat-backend-sdk` | [/sdk/rust-quick-start](/sdk/rust-quick-start) |

## Admin SDK Languages

The admin family is documented separately because its audience and source-of-truth model differ:

| Language | Workspace | Notes |
| --- | --- | --- |
| TypeScript | `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-typescript` | Secondary control-plane audience |
| Flutter | `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-flutter` | Secondary control-plane audience |

See [Admin SDK](/sdk/admin-sdk) when you are integrating governance or control-plane behavior.

## What "Supported" Means In This Site

In this documentation, a supported SDK language means:

- the workspace exists in the repository
- the language has a manual-owned `composed` SDK surface
- the language has a generated transport package or crate
- the language has a dedicated quick-start path in the docs

It does not automatically mean:

- a registry package has been published
- a public version line has been frozen
- every consumer environment can resolve the package without a local workspace checkout

## Delivery-State Vocabulary

Use these labels consistently when reading the SDK docs:

- `Implemented and verified`
- `Generated and verified`
- `Documented contract only`
- `Workspace present but not published`

For the app SDK pages, the most important distinction is between the local implemented workspaces
and public registry publication. These docs treat those as separate states.
