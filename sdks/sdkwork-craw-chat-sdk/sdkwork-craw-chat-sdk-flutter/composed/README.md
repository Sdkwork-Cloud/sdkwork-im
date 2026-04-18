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

This workspace keeps `pubspec.yaml` publish-friendly and lets the root Craw Chat SDK wrappers materialize a local `pubspec_overrides.yaml` for the current checkout layout.
That generated local override keeps both the generated `backend_sdk` package and `sdkwork_common_flutter` pointed at the correct workspace paths in either the main checkout or a nested git worktree:

```yaml
dependency_overrides:
  backend_sdk:
    path: ../generated/server-openapi
  sdkwork_common_flutter:
    path: <computed by root wrapper>
```
