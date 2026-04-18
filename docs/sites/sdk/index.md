# SDK Overview

The Craw Chat documentation site currently covers two SDK families with different audiences and
truth sources:

- `sdkwork-craw-chat-sdk`
  The primary app-facing SDK family for product integrations.
- `sdkwork-craw-chat-sdk-admin`
  The secondary admin and control-plane SDK family.

For most readers, the correct place to start is the app SDK family, not the admin family.

## SDK Family Matrix

| Family | Audience | Languages | Preferred entrypoint | Contract source |
| --- | --- | --- | --- | --- |
| `sdkwork-craw-chat-sdk` | App and product integrations | TypeScript, Flutter, Rust | Manual-owned `composed` packages and crate exposing `CrawChatClient` | Checked-in app OpenAPI authority at `sdks/sdkwork-craw-chat-sdk/openapi/craw-chat-app.openapi.yaml` |
| `sdkwork-craw-chat-sdk-admin` | Admin and control-plane integrations | TypeScript, Flutter | Admin workspace pages and control-plane API reference | `services/control-plane-api/src/lib.rs` plus control-plane tests |

## Quick Start By Language

| Language | Preferred public package or crate | Transport package or crate | Start here |
| --- | --- | --- | --- |
| TypeScript | `@sdkwork/craw-chat-sdk` | `@sdkwork/craw-chat-backend-sdk` | [/sdk/typescript-quick-start](/sdk/typescript-quick-start) |
| Flutter | `craw_chat_sdk` | `backend_sdk` | [/sdk/flutter-quick-start](/sdk/flutter-quick-start) |
| Rust | `craw-chat-sdk` | `sdkwork-craw-chat-backend-sdk` | [/sdk/rust-quick-start](/sdk/rust-quick-start) |

## Delivery-State Model

This site distinguishes implementation status from publication status:

- `Implemented and verified`
  The SDK surface exists in the current SDK worktree and is part of the maintained workspace.
- `Generated and verified`
  The generated transport layer is part of the supported workspace flow.
- `Documented contract only`
  The HTTP or transport contract is documented, but no manual SDK surface is being claimed.
- `Workspace present but not published`
  A local workspace exists, but registry publication is not implied.

For the app SDK family, the key distinction is:

- local workspaces for TypeScript, Flutter, and Rust exist now
- public registry publication remains a separate release concern

## Capability Map

| Integration area | Primary SDK page | App API page |
| --- | --- | --- |
| Session and presence | [/sdk/modules/session-and-presence](/sdk/modules/session-and-presence) | [/api-reference/app/session-and-realtime](/api-reference/app/session-and-realtime) |
| Realtime coordination | [/sdk/modules/realtime](/sdk/modules/realtime) | [/api-reference/app/session-and-realtime](/api-reference/app/session-and-realtime) |
| Devices and inbox | [/sdk/modules/devices-and-inbox](/sdk/modules/devices-and-inbox) | [/api-reference/app/device-sync](/api-reference/app/device-sync) |
| Conversations and membership | [/sdk/modules/conversations](/sdk/modules/conversations) | [/api-reference/app/conversations](/api-reference/app/conversations) |
| Messages | [/sdk/modules/messages](/sdk/modules/messages) | [/api-reference/app/messages](/api-reference/app/messages) |
| Media | [/sdk/modules/media](/sdk/modules/media) | [/api-reference/app/media](/api-reference/app/media) |
| Streams | [/sdk/modules/streams](/sdk/modules/streams) | [/api-reference/app/streams](/api-reference/app/streams) |
| RTC | [/sdk/modules/rtc](/sdk/modules/rtc) | [/api-reference/app/rtc](/api-reference/app/rtc) |

## Source-of-Truth Rules

- The app SDK family uses the checked-in app OpenAPI authority contract plus derived generator inputs.
- The admin SDK family remains a separate control-plane audience with a different source-of-truth model.
- Generated output under `generated/server-openapi` must not be edited in place.
- The preferred integration surface is the manual-owned `composed` layer.

## Recommended Reading

- [App SDK](/sdk/app-sdk)
- [Language Support](/sdk/language-support)
- [Auth and Client Init](/sdk/auth-and-client-init)
- [Module Map](/sdk/module-map)
- [Admin SDK](/sdk/admin-sdk)
