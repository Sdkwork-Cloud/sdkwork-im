# Flutter SDK

The official Flutter app-consumer package is `im_sdk`.
It sits above the generator-owned transport package `im_sdk_generated` and documents the checked-in Dart
export surface exactly as it exists today.

::: warning Current delivery status
The package names below describe repo package contracts, not current pub.dev availability. The
release catalog still marks `app-flutter` as `generated` and `not_published`.
:::

## Current Delivery Reality

The checked-in Flutter standard is:

- public consumer package: `im_sdk`
- primary public client: `ImSdkClient`
- generated transport boundary: `im_sdk_generated`
- route-aligned HTTP coverage for auth, portal, conversations, media, streams, and RTC
- a delivered WebSocket adapter plus `sdk.connect(...)` in the manual-owned `im_sdk` layer

## How To Use This Page

- Start with [Package Contract](#package-contract) to understand why `im_sdk` is the
  preferred consumer package and where `im_sdk_generated` remains the low-level transport boundary.
- Use [Current Surface Reality](#current-surface-reality) and [Current Parity Gap](#current-parity-gap)
  before promising websocket live push or TypeScript-style message builders.
- Keep [Consumption Reality](#consumption-reality) and [Local Workspace Workflow](#local-workspace-workflow)
  open when you need local path overrides or handoff artifacts before publication.

## Package Contract

| Layer | Package | Entrypoint | Primary exports | Use when |
| --- | --- | --- | --- | --- |
| Generated transport | `im_sdk_generated` | `package:im_sdk_generated/im_sdk_generated.dart` | `ImTransportClient`, generated models, generated API groups | You want direct transport access and generated request or response models |
| Composed client | `im_sdk` | `package:im_sdk/im_sdk.dart` | `ImSdkClient`, semantic app modules, helper builders | You want a higher-level client for the app runtime surface |

For most Flutter app integrations, start from `package:im_sdk/im_sdk.dart`.
`im_sdk` is the official IM consumer package and re-exports `im_sdk_generated`, so generated
models and low-level route groups remain available without making `im_sdk_generated` the first package
most teams import.

## Consumption Reality

Treat the package names on this page as repo package contracts first. Until the release catalog
changes, the reliable Flutter consumption paths are:

- local workspace development against the checked-in generated and composed packages
- assembled handoff artifacts produced from the Flutter workspace with `sdk-assemble.ps1` or
  `sdk-assemble.sh`

Do not treat `im_sdk_generated` or `im_sdk` as current pub.dev coordinates while `app-flutter`
remains `not_published` in the release catalog.

## Generated Transport Quick Start

```dart
import 'package:im_sdk_generated/im_sdk_generated.dart';

final client = ImTransportClient.withBaseUrl(
  baseUrl: 'http://127.0.0.1:18090',
  authToken: 'sdkwork-appbase-credential',
);

final presence = await client.presence.getPresenceMe();
final workspace = await client.portal.getWorkspace();
final inbox = await client.inbox.getInbox();
```

The checked-in generated Flutter client currently exports these route groups through
`ImTransportClient`:

- `portal`
- `device.sessions`
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
import 'package:im_sdk/im_sdk.dart';

final sdk = ImSdkClient.create(
  baseUrl: 'http://127.0.0.1:18090',
  authToken: 'sdkwork-appbase-credential',
);

// Tokens are issued by sdkwork-appbase and passed into Craw Chat.
final workspace = await sdk.portal.getWorkspace();

await sdk.conversations.postText(
  'conversation-1',
  text: 'hello world',
  options: const ImTextMessageOptions(
    clientMsgId: 'client-1',
    summary: 'Greeting',
  ),
);

await sdk.streams.appendTextFrame(
  'stream-1',
  const ImAppendTextFrameOptions(
    frameSeq: 7,
    text: 'partial chunk',
    schemaRef: 'urn:craw-chat:stream:text',
  ),
);

await sdk.rtc.postJsonSignal(
  'rtc-1',
  signalType: 'offer',
  options: const ImPostJsonSignalOptions(
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

- `authToken`
- `sdk.portal`
- `sdk.device.sessions`
- `sdk.presence`
- `sdk.realtime`
- `sdk.devices`
- `sdk.inbox`
- `sdk.conversations`
- `sdk.messages`
- `sdk.streams`
- `sdk.rtc`
- `sdk.transportClient` for direct generated fallback access

## Drive-Backed Media Messages

Use `sdkwork-drive` for file lifecycle work. After Drive returns a node reference, send the IM
message with `ContentPart.drive` as the `DriveReference` and `ContentPart.resource` as the
standardized `MediaResource` usage snapshot. The canonical Drive URI shape is
`drive://spaces/{spaceId}/nodes/{nodeId}`.

```dart
final drive = DriveReference(
  driveUri: 'drive://spaces/space_app_upload_demo/nodes/node_storefront_png',
  spaceId: 'space_app_upload_demo',
  nodeId: 'node_storefront_png',
  nodeVersion: '1',
);

final body = PostMessageRequest(
  text: 'Latest storefront concept',
  summary: 'Storefront concept',
  parts: <ContentPart>[
    ContentPart(
      kind: 'media',
      mediaRole: 'attachment',
      drive: drive,
      resource: MediaResource(
        id: drive.nodeId,
        kind: MediaKind.image,
        source: MediaSource.providerAsset,
        uri: drive.driveUri,
        fileName: 'storefront.png',
        mimeType: 'image/png',
        sizeBytes: fileBytes.length.toString(),
      ),
    ),
  ],
);

await sdk.conversations.postMessage('conversation-1', body);
```

The TypeScript composed SDK names the same high-level flow `createImageMessage(...)`; Flutter keeps
the same `DriveReference` and `MediaResource` contract while its message-first helpers continue to
catch up.

## Current Surface Reality

The checked-in Dart surface is intentionally narrower than the TypeScript SDK:

- `im_sdk` re-exports `im_sdk_generated`, the generated package root exports `PortalApi`,
  `ImTransportClient` mounts `client.portal` and `client.device.sessions`, and `ImSdkClient`
  exposes `sdk.portal`, `sdk.device.sessions`, and `sdk.setAuthToken(...)`.
- Tokens are issued by `sdkwork-appbase`; Flutter consumers pass them through `authToken` at
  construction time or update them with `sdk.setAuthToken(...)`.
- The Flutter runtime is WebSocket-first for interactive realtime delivery through
  `sdk.connect(...)` and the delivered adapter in `im_sdk`.
- The generated transport and `sdk.realtime.*` modules still expose explicit HTTP coordination
  through `sdk.realtime.replaceSubscriptions(...)`, `sdk.realtime.catchUpEvents(...)`, and
  `sdk.realtime.ackEvents(...)`, but those are operational coordination surfaces rather than the
  default live receive path.
- The Flutter package does not yet ship `sdk.createXxxMessage()`, `sdk.send()`, or
  `sdk.decodeMessage()`.
- Text posting shortcuts currently live on `sdk.conversations.postText(...)`,
  `sdk.conversations.publishSystemText(...)`, and `ImBuilders.*`.
- `sdk.messages` currently covers message mutation only: `edit(...)`, `editText(...)`, and
  `recall(...)`.

## Live WebSocket Receive

```dart
final sdk = ImSdkClient.create(
  baseUrl: 'https://api.example.com',
  websocketBaseUrl: 'wss://realtime.example.com',
  authToken: 'sdkwork-appbase-credential',
);

final live = await sdk.connect(
  const ImConnectOptions(
    deviceId: 'device-mobile-01',
    subscriptions: ImRealtimeSubscriptionGroups(
      conversations: <String>['conversation-1'],
      rtcSessions: <String>['rtc-1'],
    ),
  ),
);

live.messages.onConversation('conversation-1', (message, context) {
  print(message.summary);
  void context.ack();
});

live.signals.onRtcSession('rtc-1', (signal, context) {
  print(signal.signalType);
  void context.ack();
});
```

The delivered WebSocket adapter is part of the manual-owned `im_sdk` package, not the generated
transport package.

`ImWebSocketAuthOptions.automatic()` is the standard default:

- Flutter mobile and desktop pass SDKWork credentials in the WebSocket upgrade header
- Flutter Web falls back to a query credential because the browser runtime cannot attach custom
  upgrade headers through the default connector
- Flutter Web should prefer `credentialProvider` with a short-lived realtime ticket or exchanged
  query credential when the gateway supports it
- custom gateways can override the transport by providing `webSocketFactory`

## Module Coverage Map

| Concern | Generated transport | Composed client | Primary HTTP reference |
| --- | --- | --- | --- |
| SDKWork credential pass-through | `client.setAuthToken(...)` | `sdk.setAuthToken(...)` | [Portal Access](/api-reference/app/portal-access) |
| Portal access | `client.portal` | `sdk.portal` | [Portal Access](/api-reference/app/portal-access) |
| Portal snapshots | `client.portal` | `sdk.portal` | [Portal Access](/api-reference/app/portal-access) |
| Device Sessions, presence, realtime | `client.device.sessions`, `client.presence`, `client.realtime` | `sdk.device.sessions`, `sdk.presence`, `sdk.realtime` | [Device Sessions and Realtime](/api-reference/im/session-and-realtime) |
| Device sync | `client.device` | `sdk.devices` | [Device Sync](/api-reference/im/device-sync) |
| Inbox and conversations | `client.inbox`, `client.conversation` | `sdk.inbox`, `sdk.conversations` | [Conversations and Handoff](/api-reference/im/conversations) |
| Membership and read state | `client.conversation` | `sdk.conversations` | [Membership and Read State](/api-reference/im/membership-and-read-state) |
| Messages | `client.message` | `sdk.messages`, `sdk.conversations` helpers | [Messages](/api-reference/im/messages) |
| Media usage references | Drive transport plus generated message models | `sdk.conversations` with `ContentPart.drive` | [Media](/api-reference/im/media) |
| Streams | `client.stream` | `sdk.streams` | [Streams](/api-reference/im/streams) |
| RTC | `client.rtc` | `sdk.rtc` | [RTC](/api-reference/im/rtc) |

## Current Parity Gap

The Flutter SDK still trails the TypeScript SDK on message-first authoring:

- the checked-in Flutter runtime does not yet ship `sdk.createXxxMessage()`,
  `sdk.send()`, or `sdk.decodeMessage()`
- the generated transport package remains HTTP-only; the delivered WebSocket adapter lives in the
  manual-owned `im_sdk` layer
- if you need the richer message-first send surface today, use the TypeScript SDK until the
  Flutter semantic runtime catches up

This page intentionally documents the checked-in exported surface, not only the OpenAPI authority.

The corresponding HTTP surface is still documented in:

- [Portal Access](/api-reference/app/portal-access)
- [App API Overview](/api-reference/app-api)

If your product needs the richer message-first outbound surface, the current documented fallback is
the TypeScript SDK.

## Helper Builders

The composed Flutter package ships helper builders for common flows:

- `ImBuilders.textMessage()`
- `ImBuilders.textEdit()`
- `ImBuilders.textFrame()`
- `ImBuilders.jsonRtcSignal()`

These helpers are used internally by the composed modules and remain available when you want to mix
semantic helpers with explicit generated request types.

## Auth And Transport Rules

- Public auth is SDKWork appbase based.
- Prefer constructor `authToken` or `sdk.setAuthToken(...)` at the composed layer.
- `setAuthToken()` remains available on `ImTransportClient` and `ImSdkClient` for low-level
  fallback control.
- The WebSocket endpoint is documented at the API layer and the delivered WebSocket adapter now
  ships from the manual-owned `im_sdk` package.
- `ImWebSocketAuthOptions.automatic()` is the standard default.
- `ImWebSocketAuthOptions.headerBearer()` is preferred on native runtimes.
- Flutter Web should keep `automatic()` or use `queryBearer()` unless a custom `webSocketFactory`
  can attach authenticated upgrade headers.

## Assembly Metadata

The workspace root emits `.sdkwork-assembly.json` as the verified package-layer record for Flutter.

Use it when you need to trace package ownership instead of inferring from folder names:

- `manifestPath` maps the generated and composed package manifests into the assembly output
- `generatedAt` stays stable when assembly content is unchanged
- Flutter records the `generated` `im_sdk_generated` layer and the `composed` `im_sdk` layer

## Local Workspace Workflow

The checked-in Flutter workspace is organized in two layers:

- `generated/server-openapi`
  Generator-owned transport package.
- `composed`
  Manual-owned consumer package `im_sdk`.

When you consume the checked-in Flutter SDK locally, keep the generated, composed, and override
layers aligned:

- `generated/server-openapi/pubspec.yaml` owns `im_sdk_generated`
- `composed/pubspec.yaml` owns `im_sdk` and depends on `im_sdk_generated: ^0.1.0`
- `composed/pubspec_overrides.yaml` resolves both `im_sdk_generated` and `sdkwork_common_flutter`
  locally inside the repo

Recommended local loop:

1. Run generation and verification from the Flutter workspace root.
2. Start IM consumer integration work from `composed` and `package:im_sdk/im_sdk.dart`.
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
  im_sdk_generated:
    path: ../generated/server-openapi
```

In the checked-in workspace, the composed override file also resolves the shared common package:

```yaml
dependency_overrides:
  im_sdk_generated:
    path: ../generated/server-openapi
  sdkwork_common_flutter:
    path: ../../../../../../sdk/sdkwork-sdk-commons/sdkwork-sdk-common-flutter
```

If you wire `im_sdk` into another local Flutter app before publication, mirror that override
shape in the consuming app and adjust the paths relative to the app location.

## Verification

From the repository root, the family-level verification entrypoint is:

```bash
node ./sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language flutter --with-dart
```

Use `verify-sdk.mjs --with-dart` when you want native Dart verification in addition to the default
source-level guards. Add `--language flutter` when you want to run only the Flutter lane.

On Windows, the native Dart verification path also falls back to
`bin/verify-flutter-dart-analysis.dart` when `dart analyze` cannot safely launch its helper
process in the current environment.

## Source-of-Truth Notes

- Authority contract: `sdks/sdkwork-im-sdk/openapi/craw-chat-im.openapi.yaml`
- Generated transport manifest: `sdkwork-im-sdk-flutter/generated/server-openapi/pubspec.yaml`
- Composed package manifest: `sdkwork-im-sdk-flutter/composed/pubspec.yaml`
- Assembly metadata: `sdks/sdkwork-im-sdk/.sdkwork-assembly.json`

## What To Read Next

- Read [App SDK](/sdk/app-sdk) for family-wide audience, release, and contract-source rules.
- Read [Language Support](/sdk/language-support) for the current TypeScript versus Flutter parity
  snapshot.
- Read [Portal Access](/api-reference/app/portal-access) when you need the underlying HTTP
  contract behind `client.setAuthToken(...)`, `client.portal`, `authToken`, and `sdk.portal`.
