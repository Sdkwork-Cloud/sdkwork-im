# Flutter Quick Start

## Audience

Use this page when you are integrating Craw Chat into a Flutter application and want the preferred
composed SDK layer.

## Package

- preferred public package: `craw_chat_sdk`
- generated transport package: `backend_sdk`
- workspace path: `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-flutter/composed`

## Install

If you are consuming from a local repository checkout before public publication, wire the package as
a local path dependency:

```yaml
dependencies:
  craw_chat_sdk:
    path: ../../sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-flutter/composed
```

## Create a client

```dart
import 'package:craw_chat_sdk/craw_chat_sdk.dart';

final client = CrawChatClient.create(
  baseUrl: 'http://127.0.0.1:18090',
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

## Common module entrypoints

```dart
await client.session.resume(
  ResumeSessionRequest(
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

## Builder helpers

The Flutter package exposes a `CrawChatBuilders` utility surface:

```dart
final draft = CrawChatBuilders.textMessage(text: 'hello');
final frame = CrawChatBuilders.textFrame(
  CrawChatAppendTextFrameOptions(frameSeq: 1, text: 'partial'),
);
```

## Next Steps

- [Auth and Client Init](/sdk/auth-and-client-init)
- [Module Map](/sdk/module-map)
- [Messages Module](/sdk/modules/messages)
- [Message and Media](/sdk/examples/message-and-media)
