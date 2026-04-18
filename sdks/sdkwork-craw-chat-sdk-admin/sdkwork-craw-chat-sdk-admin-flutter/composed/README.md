# craw_chat_sdk_admin

Composed Flutter SDK for Craw Chat control-plane API.

This package sits above the generated `craw_chat_admin_backend_sdk` package and provides:

- the consumer-facing `CrawChatAdminClient`
- business-oriented domain fields
- flattened client creation arguments for straightforward onboarding

The generated `craw_chat_admin_backend_sdk` package remains generator-owned under
`../generated/server-openapi`. This `composed` package is manual-owned.

## Usage

```dart
import 'package:craw_chat_sdk_admin/craw_chat_sdk_admin.dart';

final sdk = CrawChatAdminClient.create(
  baseUrl: 'http://127.0.0.1:18081',
  authToken: 'your-auth-token',
);

final registry = await sdk.protocol.getApiV1ControlProtocolRegistry();
print(registry);
```

## Domain Surface

- `cluster`
- `protocol`
- `providers`
- `social`
- `system`

## Client Creation

The preferred consumer entrypoint is `CrawChatAdminClient.create(...)`.
Use flattened creation arguments for the common path and pass `backendClient` only when you
already own a configured generated transport instance.

## Package Boundary

- Consume generated transport symbols only through
  `package:craw_chat_admin_backend_sdk/craw_chat_admin_backend_sdk.dart`.
- Do not import `generated/server-openapi/lib/src` private paths from this package or
  downstream applications.

## Local Dependency Override

This workspace keeps `pubspec.yaml` publish-friendly and resolves local dependencies through
`pubspec_overrides.yaml`.
