# Admin Flutter SDK

## Package Boundary

The Craw Chat admin Flutter SDK is split into two layers:

- generated transport package: `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-flutter/generated/server-openapi`
  Published package name: `craw_chat_admin_backend_sdk`
- composed product package: `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-flutter/composed`
  Published package name: `craw_chat_admin_sdk`

Use the composed package for Flutter integrations. It exposes `CrawChatAdminSdkClient`, keeps the
generated transport package available, and preserves the same semantic module grouping used by the
TypeScript admin SDK.

## Create The Client

```dart
import 'package:craw_chat_admin_sdk/craw_chat_admin_sdk.dart';

final sdk = CrawChatAdminSdkClient.create(
  baseUrl: 'https://admin.example.com',
  authToken: '<token>',
);
```

`CrawChatAdminSdkClient.create()` accepts the flat admin-facing form directly:

- `baseUrl`
- `authToken`
- `headers`
- `timeout`

You can also inject an already constructed `CrawChatAdminBackendClient` when the host app owns the
transport layer.

## Control-plane Modules

The Flutter package mirrors the same generated control-plane modules:

- `sdk.meta`
- `sdk.protocol`
- `sdk.providers`
- `sdk.social`
- `sdk.socialRuntime`
- `sdk.nodes`

## Module To API Reference

| Flutter surface | API reference |
| --- | --- |
| `sdk.meta` | [/api-reference/control-plane/protocol#get-control-healthz](/api-reference/control-plane/protocol#get-control-healthz) |
| `sdk.protocol` | [/api-reference/control-plane/protocol](/api-reference/control-plane/protocol) |
| `sdk.providers` | [/api-reference/control-plane/providers](/api-reference/control-plane/providers) |
| `sdk.social` | [/api-reference/control-plane/social](/api-reference/control-plane/social) |
| `sdk.socialRuntime` | [/api-reference/control-plane/social-runtime](/api-reference/control-plane/social-runtime) |
| `sdk.nodes` | [/api-reference/control-plane/nodes](/api-reference/control-plane/nodes) |

Example:

```dart
final health = await sdk.meta.health();
final registry = await sdk.protocol.getRegistry();
final bindings = await sdk.providers.getBindings(<String, dynamic>{
  'tenantId': 'tenant-northstar',
});

await sdk.nodes.activate('node-east-1');
```

The Flutter admin package currently models the control-plane contract only. It does not expose the
browser-only `/api/admin/*` helpers that remain manual-owned inside the TypeScript package for the
operator shell.

## Verification

Source-level workspace verification:

```powershell
node .\sdks\sdkwork-craw-chat-sdk-admin\bin\verify-flutter-workspace.mjs
```

Native Dart verification:

```powershell
node .\sdks\sdkwork-craw-chat-sdk-admin\bin\verify-flutter-workspace.mjs --with-dart
node .\sdks\sdkwork-craw-chat-sdk-admin\bin\verify-sdk.mjs --language flutter --with-dart
```

On Windows, the verifier falls back to
`sdks/sdkwork-craw-chat-sdk-admin/bin/verify-flutter-dart-analysis.dart` instead of raw
`dart analyze` so analysis still works when the bundled Flutter toolchain cannot safely spawn its
own helper process in the current environment.

## Regeneration Rule

- Do not hand-edit `generated/server-openapi`
- Keep ergonomic Flutter code inside `composed`
- Refresh `openapi/admin-control-plane.openapi.yaml` first, then regenerate

If more admin routes are promoted into the formal OpenAPI authority, regenerate the workspace
instead of extending the generated transport manually.
