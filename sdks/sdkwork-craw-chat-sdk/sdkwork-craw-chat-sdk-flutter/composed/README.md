# craw_chat_sdk

Composed Flutter SDK for Craw Chat.

This package sits above the generated `backend_sdk` package and provides:

- a consumer-facing `CrawChatSdkClient`
- business-oriented module names
- convenience builders for common message, stream, and RTC flows
- a high-level media upload helper that completes the presigned upload flow

The generated `backend_sdk` package remains generator-owned under `../generated/server-openapi`.
This `composed` package is manual-owned.

## Usage

```dart
import 'package:craw_chat_sdk/craw_chat_sdk.dart';

final sdk = CrawChatSdkClient.create(
  baseUrl: 'https://api.example.com',
  authToken: '<token>',
);

await sdk.conversations.postText(
  'conversation-1',
  text: 'hello world',
);
```

## Media Upload

`sdk.media.createUpload()` and `sdk.media.completeUpload()` now mirror the real backend contract and return `MediaUploadMutationResponse?`.
The create step includes the presigned upload session in `response.upload`, while the complete step returns the same mutation envelope without a new presigned upload session.

```dart
import 'dart:typed_data';

import 'package:craw_chat_sdk/craw_chat_sdk.dart';

final sdk = CrawChatSdkClient.create(
  baseUrl: 'https://api.example.com',
  authToken: '<token>',
);

final bytes = Uint8List.fromList(<int>[1, 2, 3]);

final uploaded = await sdk.media.upload(
  CreateUploadRequest(
    mediaAssetId: 'media-1',
    resource: MediaResource(
      type: 'image',
      mimeType: 'image/png',
      size: bytes.length,
      name: 'avatar.png',
    ),
  ),
  bytes,
  checksum: 'sha256:...',
);

final attached = await sdk.media.attachText(
  uploaded?.mediaAssetId ?? 'media-1',
  const CrawChatAttachTextMediaOptions(
    conversationId: 'conversation-1',
    text: 'uploaded from Flutter',
  ),
);
```

If you need lower-level control, call the three steps separately:

1. `createUpload()` to allocate the media asset and receive `MediaUploadSession`
2. `uploadContent()` to send bytes to the presigned upload session
3. `completeUpload()` to finalize the asset and persist checksum / storage metadata

## Local Dependency Override

This workspace keeps `pubspec.yaml` publish-friendly and resolves the local generated package through `pubspec_overrides.yaml`:

```yaml
dependency_overrides:
  backend_sdk:
    path: ../generated/server-openapi
  sdkwork_common_flutter:
    path: ../../../../../../../../sdk/sdkwork-sdk-commons/sdkwork-sdk-common-flutter
```
