# Flutter SDK

The Flutter guide documents the checked-in Dart workspace exactly as it exists today. Use this page
when you need the package contract, current delivery limits, or the local maintainer workflow for
the Flutter app-consumer surface.

## Current Delivery Reality

The official Flutter consumer package is `craw_chat_sdk`.

The lower-level generated transport package remains `backend_sdk`, but normal Flutter application
code should start from the composed package instead of importing the generated transport boundary
first.

The release catalog still marks the current wave as `template_only_pending_generation` and
`not_published`. Treat the package names on this page as a repo contract, not as a statement that
the package is already available on pub.dev.

## Package Contract

| Concern | Value |
| --- | --- |
| Workspace | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-flutter` |
| Consumer package | `craw_chat_sdk` |
| Generated transport package | `backend_sdk` |
| Primary client | `CrawChatSdkClient` |
| Generated authoring boundary | `generated/server-openapi` |
| Manual authoring boundary | `composed` |
| Local override file | `pubspec_overrides.yaml` |

Use `package:craw_chat_sdk/craw_chat_sdk.dart` for normal product integration. Drop to
`package:backend_sdk/backend_sdk.dart` only when you intentionally need the raw generated HTTP
surface.

## Create The Client

```dart
import 'package:craw_chat_sdk/craw_chat_sdk.dart';

final sdk = CrawChatSdkClient.create(
  baseUrl: 'https://api.example.com',
  authToken: '<token>',
);
```

`CrawChatSdkClient.create()` accepts the flat app-facing form directly:

- `baseUrl`
- `authToken`
- `headers`
- `timeout`

You can also inject an already constructed `SdkworkBackendClient` when the host app owns transport
creation.

## Current Surface Reality

The current Flutter package keeps the checked-in app surface route-aligned and explicit:

- sessions
- presence
- realtime HTTP coordination
- devices
- inbox
- conversations
- messages
- media
- streams
- RTC

Helper builders remain available through `CrawChatBuilders`, which is the manual convenience layer
for common message, stream, and RTC payload construction on top of the generated models.

The boundary remains deliberate:

- `generated/server-openapi`
  Generator-owned Flutter HTTP transport package
- `composed`
  Manual-owned Flutter ergonomics and consumer-facing package docs

## Media Upload

The Flutter SDK follows the real backend upload contract:

- `createUpload()` returns `MediaUploadMutationResponse?`
- the create response carries a `MediaUploadSession` in `response.upload`
- `uploadContent()` sends raw bytes to the presigned upload session
- `completeUpload()` returns `MediaUploadMutationResponse?`
- `upload()` wraps the full create-upload-complete flow

### High-level Upload

```dart
import 'dart:typed_data';

import 'package:craw_chat_sdk/craw_chat_sdk.dart';

final bytes = Uint8List.fromList(<int>[1, 2, 3, 4]);

final uploaded = await sdk.media.upload(
  CreateUploadRequest(
    mediaAssetId: 'media-asset-1',
    resource: MediaResource(
      type: 'image',
      mimeType: 'image/png',
      size: bytes.length,
      name: 'hero.png',
    ),
  ),
  bytes,
  checksum: 'sha256:...',
);

await sdk.media.attachText(
  uploaded?.mediaAssetId ?? 'media-asset-1',
  const CrawChatAttachTextMediaOptions(
    conversationId: 'conversation-1',
    text: 'image uploaded from Flutter',
  ),
);
```

### Low-level Upload

Use the low-level flow when your app needs progress tracking, resumable upload orchestration, or
external checksum generation:

```dart
final created = await sdk.media.createUpload(
  CreateUploadRequest(
    mediaAssetId: 'media-asset-2',
    resource: MediaResource(
      type: 'file',
      mimeType: 'application/pdf',
      size: bytes.length,
      name: 'report.pdf',
    ),
  ),
);

final uploadSession = created?.upload;
if (uploadSession == null) {
  throw StateError('Backend did not return a presigned upload session');
}

await sdk.media.uploadContent(uploadSession, bytes);

final completed = await sdk.media.completeUpload(
  created?.mediaAssetId ?? 'media-asset-2',
  CompleteUploadRequest(
    bucket: uploadSession.bucket,
    objectKey: uploadSession.objectKey,
    storageProvider: uploadSession.storageProvider,
    url: uploadSession.url,
    checksum: 'sha256:...',
  ),
);
```

`MediaUploadMutationResponse` is the canonical media mutation envelope. It includes the persisted
media asset fields plus:

- `upload`
  Present on create when the backend issues a presigned upload session
- `requestKey`
  The backend mutation idempotency key
- `deliveryStatus`
  `applied` or `replayed`
- `proofVersion`
  Mutation proof version emitted by the backend

The exact HTTP contract remains documented in [Media](/api-reference/app/media).

## Current Parity Gap

The current Flutter SDK still trails a future richer client runtime in two important ways:

- it does not ship a manual realtime websocket adapter
- it is documented as a checked-in repo package contract, not a published registry package

Treat realtime in Flutter as generated HTTP coordination plus manual app orchestration for now. The
websocket transport boundary remains manual-owned and documented separately from generated output.

## Assembly Metadata

Root verification refreshes `.sdkwork-assembly.json` for the workspace.

That metadata file records:

- the authority and derived spec paths
- each language package `manifestPath`
- the `generated` and `composed` package layers
- a `generatedAt` timestamp that stays stable when assembly content is unchanged

That lets release tooling and docs verification discover Flutter package manifests without scanning
the full repository tree.

## Local Workspace Workflow

Use the Flutter workspace this way:

1. Keep generated code inside `generated/server-openapi`.
2. Keep Flutter ergonomics and public package docs inside `composed`.
3. Resolve the generated package locally through `pubspec_overrides.yaml`.
4. Refresh the authority OpenAPI contract before regeneration instead of patching generated output.

Workspace-local wrappers:

```powershell
.\bin\sdk-gen.ps1
.\bin\sdk-verify.ps1
```

Root verification commands:

```powershell
node .\sdks\sdkwork-craw-chat-sdk\bin\verify-flutter-workspace.mjs
node .\sdks\sdkwork-craw-chat-sdk\bin\verify-sdk.mjs --with-dart
```

## What To Read Next

- Read [App SDK](/sdk/app-sdk) for the family-level contract and source-of-truth rules.
- Read [Language Support](/sdk/language-support) for the cross-family package and release matrix.
- Read [TypeScript SDK](/sdk/typescript-sdk) when you need the current TypeScript consumer
  baseline.
- Read [Media](/api-reference/app/media) for the exact upload and attachment route contract.
- Read [App API Overview](/api-reference/app-api) for the broader app-facing HTTP map.
