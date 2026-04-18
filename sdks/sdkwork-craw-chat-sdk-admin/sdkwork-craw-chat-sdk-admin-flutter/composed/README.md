# craw_chat_admin_sdk

Composed Flutter SDK for the Craw Chat admin control plane.

This package sits above the generated `craw_chat_admin_backend_sdk` transport package and exposes:

- a consumer-facing `CrawChatAdminSdkClient`
- domain-oriented admin modules
- flat client configuration for `baseUrl`, `authToken`, `headers`, and `timeout`

## Usage

```dart
import 'package:craw_chat_admin_sdk/craw_chat_admin_sdk.dart';

final sdk = CrawChatAdminSdkClient.create(
  baseUrl: 'https://admin.example.com',
  authToken: '<token>',
);

final health = await sdk.meta.health();
final registry = await sdk.protocol.getRegistry();
final bindings = await sdk.providers.getBindings(<String, dynamic>{
  'tenantId': 'tenant-northstar',
});
```

## Modules

- `meta`
- `protocol`
- `providers`
- `social`
- `socialRuntime`
- `nodes`

## Local Dependency Override

```yaml
dependency_overrides:
  craw_chat_admin_backend_sdk:
    path: ../generated/server-openapi
  sdkwork_common_flutter:
    path: ../../../../../../../../sdk/sdkwork-sdk-commons/sdkwork-sdk-common-flutter
```
