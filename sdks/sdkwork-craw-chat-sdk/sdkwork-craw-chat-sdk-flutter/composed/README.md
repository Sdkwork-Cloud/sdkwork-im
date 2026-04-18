# craw_chat_sdk

Official app-facing Flutter SDK package for Craw Chat.

This package is the manual-owned consumer layer inside the Flutter workspace.
It sits above and re-exports the generated `backend_sdk` package, so most Flutter app consumers
should import `package:craw_chat_sdk/craw_chat_sdk.dart` first.
Use `backend_sdk` directly only when you explicitly want the standalone generated transport package.

This package provides:

- a consumer-facing `CrawChatClient`
- business-oriented module names
- convenience builders for common message, stream, and RTC flows

Current checked-in scope:

- `craw_chat_sdk` re-exports `backend_sdk`, whose package root now exports generated `AuthApi` and
  `PortalApi` symbols.
- `CrawChatClient` exposes `sdk.auth` and `sdk.portal`, while `SdkworkBackendClient` mounts the
  same generated groups as `client.auth` and `client.portal`.
- `sdk.auth.login(...)` automatically applies the returned `accessToken` when present, and
  `sdk.auth.useToken(...)` plus `sdk.auth.clearToken()` handle explicit bearer-token control.
- The consumer package has no delivered WebSocket adapter and does not ship `sdk.connect(...)`.
- The package does not yet ship a TypeScript-style `sdk.createXxxMessage()` / `send()` /
  `decodeMessage()` message-first surface.
- Route-aligned outbound shortcuts currently live on `sdk.conversations.postText(...)`,
  `sdk.conversations.publishSystemText(...)`, `sdk.media.attachText(...)`, and `CrawChatBuilders.*`.

The generated `backend_sdk` package remains generator-owned under `../generated/server-openapi`.
This `composed` package is manual-owned and remains the official Flutter app-consumer package in
this workspace.

## Usage

```dart
import 'package:craw_chat_sdk/craw_chat_sdk.dart';

final sdk = CrawChatClient.create(
  baseUrl: 'https://api.example.com',
  authToken: '<token>',
);

await sdk.auth.me();
final workspace = await sdk.portal.getWorkspace();

await sdk.conversations.postText(
  'conversation-1',
  text: 'hello world',
);
```

Because `craw_chat_sdk` re-exports the generated `backend_sdk` package, most consumers do not need
to add both packages separately during local workspace development.

## Local Dependency Override

This workspace keeps `pubspec.yaml` publish-friendly and resolves the local generated package through `pubspec_overrides.yaml`:

```yaml
dependency_overrides:
  backend_sdk:
    path: ../generated/server-openapi
```
