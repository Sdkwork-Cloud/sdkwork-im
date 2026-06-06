# im_app_sdk

Official consumer-facing Flutter package for the app SDK family.

This package is the manual-owned `composed` layer in `sdkwork-im-app-sdk-flutter`. It sits above
and re-exports the generated `im_app_api_generated` transport package.

Use this package first when building app-business capabilities on `/app/v3/api`:

- tenant portal snapshots
- device twin
- notifications
- automation runtime
- provider health
- IoT protocol provider routes
- RTC provider callbacks and health

Current boundary:

- `im_app_sdk` is consumer-facing and manual-owned.
- `im_app_api_generated` remains generator-owned under `../generated/server-openapi`.
- IM standard capability stays in `im_sdk`; RTC provider-standard capability stays in `rtc_sdk`.

## Usage

```dart
import 'package:im_app_sdk/im_app_sdk.dart';

final sdk = ImAppSdkClient.create(
  baseUrl: 'https://api.example.com',
  authToken: '<auth-token>',
  accessToken: '<access-token>',
);

final workspace = await sdk.portal.workspace();
final twin = await sdk.device.getTwin('device-mobile-01');
```

`ImAppSdkClient` also exposes raw generated route groups (`automationApi`, `deviceApi`,
`notificationApi`, `portalApi`, `providerApi`, `iotApi`, `rtcApi`) when direct transport access is
needed.
