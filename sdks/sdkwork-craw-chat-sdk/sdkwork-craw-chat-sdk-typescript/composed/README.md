# @sdkwork/craw-chat-sdk

Composed TypeScript SDK for Craw Chat.

This package sits above the generated `@sdkwork/craw-chat-backend-sdk` package and provides:

- a consumer-facing `CrawChatSdkClient`
- business-oriented module names
- convenience builders for common message, stream, and RTC flows
- a high-level media upload helper that wraps the presigned upload flow

`generated/server-openapi` remains generator-owned. This `composed` package is manual-owned.

Within this workspace, manual code may reference generated type files only through `src/generated-backend-types.ts`.
Do not import from `../generated/server-openapi/src/types/*` anywhere else in the composed package.

## Usage

```ts
import { CrawChatSdkClient } from '@sdkwork/craw-chat-sdk';

const sdk = await CrawChatSdkClient.create({
  baseUrl: 'https://api.example.com',
  authToken: '<token>',
});

await sdk.conversations.postText('conversation-1', 'hello world', {
  clientMsgId: 'msg-1',
});
```

`CrawChatSdkClient.create()` accepts the flat app-facing form directly:

- `baseUrl`
- `authToken`
- `headers`
- `timeout`
- `tokenManager`

## Media Upload

`sdk.media.createUpload()` and `sdk.media.completeUpload()` return the backend mutation envelope `MediaUploadMutationResponse`.
The create step includes a presigned upload session in `response.upload`, while the complete step returns the finalized media state plus idempotency metadata.

```ts
import { CrawChatSdkClient } from '@sdkwork/craw-chat-sdk';

const sdk = await CrawChatSdkClient.create({
  baseUrl: 'https://api.example.com',
  authToken: '<token>',
});

const bytes = new Uint8Array([1, 2, 3]);

const uploaded = await sdk.media.upload(
  {
    mediaAssetId: 'asset-1',
    resource: {
      type: 'image',
      name: 'photo.png',
      mimeType: 'image/png',
      size: bytes.length,
    },
  },
  bytes,
  {
    checksum: 'sha256:demo',
  },
);

await sdk.media.attachText(uploaded.mediaAssetId, {
  conversationId: 'conversation-1',
  text: 'uploaded from TypeScript',
});
```

Use the lower-level flow when your app needs custom upload progress or storage orchestration:

1. `createUpload()` to allocate the asset and receive `MediaUploadSession`
2. `uploadContent()` to send bytes to the presigned upload target
3. `completeUpload()` to persist checksum and storage metadata

## Verification

Local verification used in this workspace:

- `node ../../bin/verify-typescript-public-api-boundary.mjs`
- `node ../../bin/verify-typescript-usage-surface.mjs`
- `node ../../bin/build-typescript-generated-package.mjs`
- `node ../../bin/verify-typescript-generated-package.mjs`
- `node ./bin/run-tsc.mjs -p tsconfig.build.json --noEmit`
- `node ./bin/run-tsc.mjs -p tsconfig.build.json`
- `node ./test/craw-chat-client.test.mjs`
