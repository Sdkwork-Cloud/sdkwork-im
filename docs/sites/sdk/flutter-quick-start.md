# Flutter Quick Start

## Audience

Use this page when you are integrating Craw Chat into a Flutter application and want the preferred
composed SDK layer.

## Package

- preferred public package: `im_sdk`
- generated transport package: `im_sdk_generated`
- workspace path: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/composed`

## Install

If you are consuming from a local repository checkout before public publication, wire the package as
a local path dependency:

```yaml
dependencies:
  im_sdk:
    path: ../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/composed
```

## Create a client

```dart
import 'package:im_sdk/im_sdk.dart';

final client = ImSdkClient.create(
  baseUrl: 'http://127.0.0.1:18090',
  authToken: token,
);
```

If your app uses a split realtime origin, set `websocketBaseUrl` as well:

```dart
final client = ImSdkClient.create(
  baseUrl: 'https://api.example.com',
  websocketBaseUrl: 'wss://realtime.example.com',
  authToken: token,
);
```

## First read call

```dart
final inbox = await client.inbox.list();
```

## First write call

```dart
await client.devices.register(
  RegisterDeviceRequest(deviceId: 'device-mobile-01'),
);
```

## First live receive

```dart
final live = await client.connect(
  const ImConnectOptions(
    deviceId: 'device-mobile-01',
    subscriptions: ImRealtimeSubscriptionGroups(
      conversations: <String>['conv-demo-01'],
    ),
  ),
);

live.messages.onConversation('conv-demo-01', (message, context) {
  print(message.summary);
  void context.ack();
});
```

`ImWebSocketAuthOptions.automatic()` is the standard default. On Flutter mobile and desktop it uses
SDKWork credential upgrade headers. On Flutter Web it falls back to query credential auth so the browser runtime
can still establish the websocket when custom upgrade headers are not available.
When the gateway supports browser-safe token exchange, prefer `credentialProvider` with a
short-lived realtime credential.

## Common module entrypoints

```dart
await client.deviceSessions.resume(
  ResumeDeviceSessionRequest(
    deviceId: 'device-mobile-01',
    lastSeenSyncSeq: 0,
  ),
);

await client.presence.current();
await client.conversations.postText(
  'conv-demo-01',
  text: 'hello from Flutter',
);
```

## Builder Helpers

The Flutter package exposes an `ImBuilders` utility surface:

```dart
final draft = ImBuilders.textMessage(text: 'hello');
final frame = ImBuilders.textFrame(
  ImAppendTextFrameOptions(frameSeq: 1, text: 'partial'),
);
```

## Next Steps

- [Auth and Client Init](/sdk/auth-and-client-init)
- [Module Map](/sdk/module-map)
- [Messages Module](/sdk/modules/messages)
- [Message and Media](/sdk/examples/message-and-media)
