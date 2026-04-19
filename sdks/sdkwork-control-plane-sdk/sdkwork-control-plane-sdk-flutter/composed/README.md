# control_plane_sdk

Composed Flutter SDK for the control plane.

This package sits above the generated `control_plane_backend_sdk` transport package and exposes:

- a consumer-facing `ControlPlaneSdkClient`
- domain-oriented admin modules
- flat client configuration for `baseUrl`, `authToken`, `headers`, and `timeout`

## Usage

```dart
import 'package:control_plane_sdk/control_plane_sdk.dart';

final sdk = ControlPlaneSdkClient.create(
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
  control_plane_backend_sdk:
    path: ../generated/server-openapi
  sdkwork_common_flutter:
    path: ../../../../../../../../sdk/sdkwork-sdk-commons/sdkwork-sdk-common-flutter
```
