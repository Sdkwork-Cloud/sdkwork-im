# Flutter SDK

The official Flutter app-consumer package is `craw_chat_sdk`.
It sits above the generator-owned transport package `backend_sdk` and documents the checked-in Dart
export surface exactly as it exists today.

::: warning Current delivery status
The package names below describe repo package contracts, not current pub.dev availability. The
release catalog still marks `app-flutter` as `generated` and `not_published`.
:::

## How To Use This Page

- Start with [Package Contract](#package-contract) to understand why `craw_chat_sdk` is the
  preferred consumer package and where `backend_sdk` remains the low-level transport boundary.
- Use [Current Surface Reality](#current-surface-reality) and [Current Parity Gap](#current-parity-gap)
  before promising websocket live push or TypeScript-style message builders.
- Keep [Consumption Reality](#consumption-reality) and [Local Workspace Workflow](#local-workspace-workflow)
  open when you need local path overrides or handoff artifacts before publication.

## Package Contract

| Layer | Package | Entrypoint | Primary exports | Use when |
| --- | --- | --- | --- | --- |
| Generated transport | `backend_sdk` | `package:backend_sdk/backend_sdk.dart` | `SdkworkBackendClient`, generated models, generated API groups | You want direct transport access and generated request or response models |
| Composed client | `craw_chat_sdk` | `package:craw_chat_sdk/craw_chat_sdk.dart` | `CrawChatSdkClient`, semantic app modules, helper builders | You want a higher-level client for the app runtime surface |

For most Flutter app integrations, start from `package:craw_chat_sdk/craw_chat_sdk.dart`.
`craw_chat_sdk` is the official app-facing package and re-exports `backend_sdk`, so generated
models and low-level route groups remain available without making `backend_sdk` the first package
most teams import.

## Consumption Reality

Treat the package names on this page as repo package contracts first. Until the release catalog
changes, the reliable Flutter consumption paths are:

- local workspace development against the checked-in generated and composed packages
- assembled handoff artifacts produced from the Flutter workspace with `sdk-assemble.ps1` or
  `sdk-assemble.sh`

Do not treat `backend_sdk` or `craw_chat_sdk` as current pub.dev coordinates while `app-flutter`
remains `not_published` in the release catalog.

## Generated Transport Quick Start

```dart
import 'package:backend_sdk/backend_sdk.dart';

final client = SdkworkBackendClient.withBaseUrl(
  baseUrl: 'http://127.0.0.1:18090',
  authToken: 'your-bearer-token',
);

await client.auth.me();
final workspace = await client.portal.getWorkspace();
final inbox = await client.inbox.getInbox();
```

The checked-in generated Flutter client currently exports these route groups through
`SdkworkBackendClient`:

- `auth`
- `portal`
- `session`
- `presence`
- `realtime`
- `device`
- `inbox`
- `conversation`
- `message`
- `media`
- `stream`
- `rtc`

## Composed Client Quick Start

```dart
import 'package:craw_chat_sdk/craw_chat_sdk.dart';

final sdk = CrawChatSdkClient.create(
  baseUrl: 'http://127.0.0.1:18090',
  authToken: 'your-bearer-token',
);

await sdk.auth.me();
final workspace = await sdk.portal.getWorkspace();

await sdk.conversations.postText(
  'conversation-1',
  text: 'hello world',
  options: const CrawChatTextMessageOptions(
    clientMsgId: 'client-1',
    summary: 'Greeting',
  ),
);

await sdk.streams.appendTextFrame(
  'stream-1',
  const CrawChatAppendTextFrameOptions(
    frameSeq: 7,
    text: 'partial chunk',
    schemaRef: 'urn:craw-chat:stream:text',
  ),
);

await sdk.rtc.postJsonSignal(
  'rtc-1',
  signalType: 'offer',
  options: const CrawChatPostJsonSignalOptions(
    schemaRef: 'urn:craw-chat:rtc:signal',
    signalingStreamId: 'signal-stream-1',
    payload: <String, Object>{
      'sdp': 'v=0',
      'type': 'offer',
    },
  ),
);
```

The composed Flutter client currently exposes:

- `sdk.auth`
- `sdk.portal`
- `sdk.session`
- `sdk.presence`
- `sdk.realtime`
- `sdk.devices`
- `sdk.inbox`
- `sdk.conversations`
- `sdk.messages`
- `sdk.media`
- `sdk.streams`
- `sdk.rtc`
- `sdk.backendClient` for direct fallback access

## Current Surface Reality

The checked-in Dart surface is intentionally narrower than the TypeScript SDK:

- `craw_chat_sdk` re-exports `backend_sdk`, the generated package root exports `AuthApi` and
  `PortalApi`, `SdkworkBackendClient` mounts `client.auth` and `client.portal`, and
  `CrawChatSdkClient` exposes `sdk.auth` and `sdk.portal`.
- `sdk.auth.login(...)` automatically applies the returned `accessToken` when present, while
  `sdk.auth.useToken(...)`, `sdk.auth.clearToken()`, and `sdk.auth.me()` give the composed layer a
  standard auth workflow.
- The Flutter runtime does not ship `sdk.connect(...)`; realtime is HTTP coordination only through
  `sdk.realtime.replaceSubscriptions(...)`, `sdk.realtime.pullEvents(...)`, and
  `sdk.realtime.ackEvents(...)`.
- The Flutter package does not yet ship `sdk.createXxxMessage()`, `sdk.send()`, or
  `sdk.decodeMessage()`.
- Text posting shortcuts currently live on `sdk.conversations.postText(...)`,
  `sdk.conversations.publishSystemText(...)`, `sdk.media.attachText(...)`, and
  `CrawChatBuilders.*`.
- `sdk.messages` currently covers message mutation only: `edit(...)`, `editText(...)`, and
  `recall(...)`.

## Module Coverage Map

| Concern | Generated transport | Composed client | Primary HTTP reference |
| --- | --- | --- | --- |
| Portal auth | `client.auth` | `sdk.auth` | [Portal and Auth](/api-reference/app/portal-and-auth) |
| Portal snapshots | `client.portal` | `sdk.portal` | [Portal and Auth](/api-reference/app/portal-and-auth) |
| Session, presence, realtime | `client.session`, `client.presence`, `client.realtime` | `sdk.session`, `sdk.presence`, `sdk.realtime` | [Session and Realtime](/api-reference/app/session-and-realtime) |
| Device sync | `client.device` | `sdk.devices` | [Device Sync](/api-reference/app/device-sync) |
| Inbox and conversations | `client.inbox`, `client.conversation` | `sdk.inbox`, `sdk.conversations` | [Conversations and Handoff](/api-reference/app/conversations) |
| Membership and read state | `client.conversation` | `sdk.conversations` | [Membership and Read State](/api-reference/app/membership-and-read-state) |
| Messages | `client.message` | `sdk.messages`, `sdk.conversations` helpers | [Messages](/api-reference/app/messages) |
| Media | `client.media` | `sdk.media` | [Media](/api-reference/app/media) |
| Streams | `client.stream` | `sdk.streams` | [Streams](/api-reference/app/streams) |
| RTC | `client.rtc` | `sdk.rtc` | [RTC](/api-reference/app/rtc) |

## Current Parity Gap

The Flutter SDK currently trails the TypeScript SDK on live transport and message-first authoring:

- the checked-in Flutter runtime does not ship `sdk.connect(...)` or a delivered websocket live
  adapter
- the checked-in Flutter runtime does not yet ship `sdk.createXxxMessage()`,
  `sdk.send()`, or `sdk.decodeMessage()`
- if you need websocket live push or the richer message-first send surface today, use the
  TypeScript SDK until the Flutter semantic runtime catches up

This page intentionally documents the checked-in exported surface, not only the OpenAPI authority.

The corresponding HTTP surface is still documented in:

- [Portal and Auth](/api-reference/app/portal-and-auth)
- [App API Overview](/api-reference/app-api)

If your product needs websocket live push or the richer message-first outbound surface, the
current documented fallback is the TypeScript SDK.

## Helper Builders

The composed Flutter package ships helper builders for common flows:

- `CrawChatBuilders.textMessage()`
- `CrawChatBuilders.textEdit()`
- `CrawChatBuilders.textFrame()`
- `CrawChatBuilders.jsonRtcSignal()`

These helpers are used internally by the composed modules and remain available when you want to mix
semantic helpers with explicit generated request types.

## Auth And Transport Rules

- Public auth is bearer-token only.
- Prefer `sdk.auth.useToken(...)` and `sdk.auth.clearToken()` at the composed layer.
- `setAuthToken()` remains available on `SdkworkBackendClient` and `CrawChatSdkClient` for low-level
  fallback control.
- The WebSocket endpoint is documented at the API layer, but the checked-in Flutter SDK does not
  ship `sdk.connect(...)` and no delivered WebSocket adapter is treated as shipped in this round.

## Local Workspace Workflow

The checked-in Flutter workspace is organized in two layers:

- `generated/server-openapi`
  Generator-owned transport package.
- `composed`
  Manual-owned consumer package `craw_chat_sdk`.

When you consume the checked-in Flutter SDK locally, keep the generated, composed, and override
layers aligned:

- `generated/server-openapi/pubspec.yaml` owns `backend_sdk`
- `composed/pubspec.yaml` owns `craw_chat_sdk` and depends on `backend_sdk: ^0.1.0`
- `composed/pubspec_overrides.yaml` resolves both `backend_sdk` and `sdkwork_common_flutter`
  locally inside the repo

Recommended local loop:

1. Run generation and verification from the Flutter workspace root.
2. Start app-facing integration work from `composed` and `package:craw_chat_sdk/craw_chat_sdk.dart`.
3. Drop to `generated/server-openapi` only when you intentionally want the standalone generated
   transport boundary or raw generated request and response models.
4. Mirror the same `dependency_overrides` structure in any local consumer app, adjusting the paths
   relative to that app.
5. Use `sdk-assemble.ps1` or `sdk-assemble.sh` when you need a packaged handoff before pub.dev
   publication exists.

From the Flutter workspace root:

```powershell
.\bin\sdk-gen.ps1
.\bin\sdk-verify.ps1
```

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\sdk-gen.ps1
powershell -ExecutionPolicy Bypass -File .\bin\sdk-verify.ps1
```

```bash
./bin/sdk-gen.sh
./bin/sdk-verify.sh
```

The composed package keeps `pubspec.yaml` publish-friendly and resolves the generated package
locally through `pubspec_overrides.yaml`:

```yaml
dependency_overrides:
  backend_sdk:
    path: ../generated/server-openapi
```

In the checked-in workspace, the composed override file also resolves the shared common package:

```yaml
dependency_overrides:
  backend_sdk:
    path: ../generated/server-openapi
  sdkwork_common_flutter:
    path: ../../../../../../sdk/sdkwork-sdk-commons/sdkwork-sdk-common-flutter
```

If you wire `craw_chat_sdk` into another local Flutter app before publication, mirror that override
shape in the consuming app and adjust the paths relative to the app location.

## Source-of-Truth Notes

- Authority contract: `sdks/sdkwork-craw-chat-sdk/openapi/craw-chat-app.openapi.yaml`
- Generated transport manifest: `sdkwork-craw-chat-sdk-flutter/generated/server-openapi/pubspec.yaml`
- Composed package manifest: `sdkwork-craw-chat-sdk-flutter/composed/pubspec.yaml`

## What To Read Next

- Read [App SDK](/sdk/app-sdk) for family-wide audience, release, and contract-source rules.
- Read [Language Support](/sdk/language-support) for the current TypeScript versus Flutter parity
  snapshot.
- Read [Portal and Auth](/api-reference/app/portal-and-auth) when you need the underlying HTTP
  contract behind `client.auth`, `client.portal`, `sdk.auth`, and `sdk.portal`.
