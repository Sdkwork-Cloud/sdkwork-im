# im_admin_sdk

Composed Flutter SDK for IM admin backend.

This package sits above the generated `im_admin_backend_sdk` package and provides:

- the consumer-facing `ImAdminSdkClient`
- business-oriented domain fields
- flattened client creation arguments for straightforward onboarding

The generated `im_admin_backend_sdk` package remains generator-owned under
`../generated/server-openapi`. This `composed` package is manual-owned.

## Usage

```dart
import 'package:im_admin_sdk/im_admin_sdk.dart';

final sdk = ImAdminSdkClient.create(
  baseUrl: 'http://127.0.0.1:18080',
  authToken: 'your-auth-token',
);

final tenants = await sdk.tenants.listTenants();
print(tenants);
```

## Domain Surface

- `auth`
- `users`
- `marketing`
- `tenants`
- `access`
- `routing`
- `catalog`
- `usage`
- `billing`
- `operations`
- `storage`

## Client Creation

The preferred consumer entrypoint is `ImAdminSdkClient.create(...)`.
Use flattened creation arguments for the common path and pass `backendClient` only when you
already own a configured generated transport instance.

## Package Boundary

- Consume generated transport symbols only through
  `package:im_admin_backend_sdk/im_admin_backend_sdk.dart`.
- Do not import `generated/server-openapi/lib/src` private paths from this package or
  downstream applications.

## Local Dependency Override

This workspace keeps `pubspec.yaml` publish-friendly and resolves local dependencies through
`pubspec_overrides.yaml`.
