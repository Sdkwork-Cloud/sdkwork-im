# SDKWork IM App SDK OpenAPI Sources

This directory stores the OpenAPI source documents for the `/app/v3/api` SDK family. The app family
owns app-business and non-management HTTP APIs that are not part of the IM standardized development
API.

## Files

- `craw-chat-app-api.openapi.yaml`
  Authority OpenAPI 3.x contract for app developers building instant messaging apps.
- `craw-chat-app-api.sdkgen.yaml`
  Generator-compatible derived input.
- `craw-chat-app-api.flutter.sdkgen.yaml`
  Flutter-compatible derived input.

## Rules

- `/app/v3/openapi.json` is the live service schema export.
- `CRAW_CHAT_APP_API_OPENAPI_SCHEMA_PATH` can override the runtime schema export source.
- `/app/v3/api/device/sessions/*` is the device route lifecycle namespace, not the upstream
  platform identity session namespace.
- Non-management provider, IoT, RTC, and platform callback or health HTTP routes belong here when
  they are exposed to app/business integrations.
- Management-system, control-plane, and admin-console APIs stay in `sdkwork-im-backend-sdk`.
- Generated SDK packages must never edit these files in place.
