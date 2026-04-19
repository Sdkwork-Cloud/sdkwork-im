# im_admin_backend_sdk

Generated Flutter transport package for the IM admin backend.

## Package Role

This package is the generator-owned Flutter transport layer for the checked-in
IM admin backend contract. Use it when you need direct access to grouped HTTP operations and public
transport types.

For business-facing admin integrations, prefer the composed Flutter package
`im_admin_sdk`, which wraps this transport package with the higher-level
`ImAdminSdkClient` facade.

## Installation

Add to `pubspec.yaml`:

```yaml
dependencies:
  im_admin_backend_sdk: ^0.1.0
```

## Quick Start

```dart
import 'package:im_admin_backend_sdk/im_admin_backend_sdk.dart';

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
- In packaged installs, that surface is the unified public origin that fronts the admin gateway.
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
- `client.storage` - storage API

## Package Boundary

- Use only the package root entrypoint:
  `package:im_admin_backend_sdk/im_admin_backend_sdk.dart`.
- Do not import generated `lib/src/` paths from downstream code.
- Keep business orchestration in the composed Flutter package
  `package:im_admin_sdk/im_admin_sdk.dart`.

## Regeneration Contract

- Generated files live under `generated/server-openapi`.
- Hand-written orchestration belongs under `composed`.
- Refresh the authority contract through the root workspace wrappers, then rerun the local
  materializer rather than editing generated transport files by hand.
