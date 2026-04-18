# craw_chat_admin_backend_sdk

Generated Flutter transport package for the Craw Chat control-plane API.

## Package Role

This package is the generator-owned Flutter transport layer for the checked-in
Craw Chat control-plane API contract. Use it when you need direct access to grouped HTTP operations and public
transport types.

For business-facing admin integrations, prefer the composed Flutter package
`craw_chat_sdk_admin`, which wraps this transport package with the higher-level
`CrawChatAdminClient` facade.

## Installation

Add to `pubspec.yaml`:

```yaml
dependencies:
  craw_chat_admin_backend_sdk: ^0.1.0
```

## Quick Start

```dart
import 'package:craw_chat_admin_backend_sdk/craw_chat_admin_backend_sdk.dart';

final client = SdkworkBackendClient.withBaseUrl(
  baseUrl: 'http://127.0.0.1:18081',
  authToken: 'your-auth-token',
);

final registry = await client.protocol.getApiV1ControlProtocolRegistry();
print(registry);
```

## Authentication Modes

Choose one authentication mode per client instance.

### Bearer Token

```dart
final client = SdkworkBackendClient.withBaseUrl(
  baseUrl: 'http://127.0.0.1:18081',
);

client.setAuthToken('your-auth-token');
```

### API Key

```dart
final client = SdkworkBackendClient.withBaseUrl(
  baseUrl: 'http://127.0.0.1:18081',
);

client.setApiKey('your-api-key');
```

### Dual Token

```dart
final client = SdkworkBackendClient.withBaseUrl(
  baseUrl: 'http://127.0.0.1:18081',
);

client.setAuthToken('your-auth-token');
client.setAccessToken('your-access-token');
```

## Endpoint Targeting

- For standalone governance development, point baseUrl directly at control-plane-api, which defaults to http://127.0.0.1:18081.
- For packaged installs, point the same client at the unified craw-chat-server / web-gateway public origin.
- Do not mix direct control-plane origins and packaged single-port gateway assumptions in the same client instance.

## Surface Groups

- `client.cluster` - cluster API
- `client.protocol` - protocol API
- `client.providers` - providers API
- `client.social` - social API
- `client.system` - system API

## Package Boundary

- Use only the package root entrypoint:
  `package:craw_chat_admin_backend_sdk/craw_chat_admin_backend_sdk.dart`.
- Do not import generated `lib/src/` paths from downstream code.
- Keep business orchestration in the composed Flutter package
  `package:craw_chat_sdk_admin/craw_chat_sdk_admin.dart`.

## Regeneration Contract

- Generated files live under `generated/server-openapi`.
- Hand-written orchestration belongs under `composed`.
- Refresh the authority contract through the root workspace wrappers, then rerun the local
  materializer rather than editing generated transport files by hand.
