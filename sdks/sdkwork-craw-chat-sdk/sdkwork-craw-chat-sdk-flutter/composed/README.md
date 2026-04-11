# craw_chat_sdk

Composed Flutter SDK for Craw Chat.

This package sits above the generated `backend_sdk` package and provides:

- a consumer-facing `CrawChatClient`
- business-oriented module names
- convenience builders for common message, stream, and RTC flows

The generated `backend_sdk` package remains generator-owned under `../generated/server-openapi`.
This `composed` package is manual-owned.

## Usage

```dart
import 'package:craw_chat_sdk/craw_chat_sdk.dart';

final sdk = CrawChatClient.create(
  baseUrl: 'https://api.example.com',
  authToken: '<token>',
);

await sdk.conversations.postText(
  'conversation-1',
  text: 'hello world',
);
```

## Local Dependency Override

This workspace keeps `pubspec.yaml` publish-friendly and resolves the local generated package through `pubspec_overrides.yaml`:

```yaml
dependency_overrides:
  backend_sdk:
    path: ../generated/server-openapi
```
