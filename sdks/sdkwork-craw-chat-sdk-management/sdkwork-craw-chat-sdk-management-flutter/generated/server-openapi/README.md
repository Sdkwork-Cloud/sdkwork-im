# craw_chat_management_backend_sdk

Generated Flutter transport package for the Craw Chat operator-console management backend.

## Package Role

This package is the generator-owned Flutter transport layer for the checked-in
Craw Chat operator-console management backend contract. Use it when you need direct access to grouped HTTP operations and public
transport types.

For business-facing management integrations, prefer the composed Flutter package
`craw_chat_sdk_management`, which wraps this transport package with the higher-level
`CrawChatManagementClient` facade.

## Installation

Add to `pubspec.yaml`:

```yaml
dependencies:
  craw_chat_management_backend_sdk: ^0.1.0
```

## Quick Start

```dart
import 'package:craw_chat_management_backend_sdk/craw_chat_management_backend_sdk.dart';

final client = SdkworkBackendClient.withBaseUrl(
  baseUrl: 'http://127.0.0.1:18080',
  authToken: 'your-auth-token',
);

final tenants = await client.tenants.listTenants();
print(tenants);
```

## Authentication Modes

Choose one authentication mode per client instance.

### Bearer Token

```dart
final client = SdkworkBackendClient.withBaseUrl(
  baseUrl: 'http://127.0.0.1:18080',
);

client.setAuthToken('your-auth-token');
```

### API Key

```dart
final client = SdkworkBackendClient.withBaseUrl(
  baseUrl: 'http://127.0.0.1:18080',
);

client.setApiKey('your-api-key');
```

### Dual Token

```dart
final client = SdkworkBackendClient.withBaseUrl(
  baseUrl: 'http://127.0.0.1:18080',
);

client.setAuthToken('your-auth-token');
client.setAccessToken('your-access-token');
```

## Endpoint Targeting

- Point baseUrl at the deployed surface that serves the checked-in /api/admin/* contract.
- In packaged installs, that surface is the unified craw-chat-server / web-gateway public origin.
- In direct backend development, use the environment-specific origin that already owns /api/admin/*.

## Surface Groups

- `client.auth` - auth API
- `client.users` - users API
- `client.marketing` - marketing API
- `client.tenants` - tenants API
- `client.access` - access API
- `client.routing` - routing API
- `client.catalog` - catalog API
- `client.usage` - usage API
- `client.billing` - billing API
- `client.operations` - operations API

## Package Boundary

- Use only the package root entrypoint:
  `package:craw_chat_management_backend_sdk/craw_chat_management_backend_sdk.dart`.
- Do not import generated `lib/src/` paths from downstream code.
- Keep business orchestration in the composed Flutter package
  `package:craw_chat_sdk_management/craw_chat_sdk_management.dart`.

## Regeneration Contract

- Generated files live under `generated/server-openapi`.
- Hand-written orchestration belongs under `composed`.
- Refresh the authority contract through the root workspace wrappers, then rerun the local
  materializer rather than editing generated transport files by hand.
