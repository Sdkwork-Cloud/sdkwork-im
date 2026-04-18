# TypeScript SDK

The Craw Chat TypeScript guide documents the checked-in app-consumer package exactly as it exists
today. Treat this page as the contract for repository consumers, local app integration, and SDK
maintainers.

## Current Delivery Reality

The official TypeScript consumer package is `@sdkwork/craw-chat-sdk`.

The lower-level generated transport package remains `@sdkwork/craw-chat-backend-sdk`, but normal
product integrations should start from the composed package instead of importing the generated
transport boundary directly.

The release catalog still marks the current wave as `template_only_pending_generation` and
`not_published`. That means the package names on this page are a real repo contract, not a claim
that the package is already published to npm.

## Package Contract

| Concern | Value |
| --- | --- |
| Workspace | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-typescript` |
| Consumer package | `@sdkwork/craw-chat-sdk` |
| Generated transport package | `@sdkwork/craw-chat-backend-sdk` |
| Primary client | `CrawChatSdkClient` |
| Generated authoring boundary | `generated/server-openapi` |
| Manual authoring boundary | `composed` |
| Generated package docs | `generated/server-openapi/README.md` |
| Manual package docs | `composed/README.md` |

Use `@sdkwork/craw-chat-sdk` for application code. Drop to `@sdkwork/craw-chat-backend-sdk` only
when you intentionally need raw generated route groups or direct transport control.

## Create The Client

```ts
import { CrawChatSdkClient } from '@sdkwork/craw-chat-sdk';

const sdk = await CrawChatSdkClient.create({
  baseUrl: import.meta.env.VITE_CRAW_CHAT_BASE_URL,
  authToken: window.localStorage.getItem('craw-chat-token')!,
});
```

`CrawChatSdkClient.create()` accepts the flat app-facing form directly:

- `baseUrl`
- `authToken`
- `headers`
- `timeout`
- `tokenManager`

If you already own transport creation, you can still pass `backendClient`.

## Current Surface Reality

The current composed package groups the checked-in TypeScript surface into product-oriented
modules:

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

This package also keeps the generated HTTP surface available behind the manual layer. That split is
intentional:

- `generated/server-openapi`
  Generator-owned output from `sdkwork-sdk-generator`
- `composed`
  Manual-owned TypeScript ergonomics and consumer-facing docs

When the backend contract changes, refresh the authority OpenAPI contract first and regenerate.
Do not patch generated files in place.

## Media Upload

The TypeScript SDK follows the real backend upload contract:

- `createUpload()` returns `MediaUploadMutationResponse`
- the create response carries a `MediaUploadSession` in `response.upload`
- `uploadContent()` sends bytes to the presigned upload target
- `completeUpload()` returns `MediaUploadMutationResponse`
- `upload()` wraps the full create-upload-complete flow

### High-level Upload

```ts
const bytes = new Uint8Array([1, 2, 3, 4]);

const uploaded = await sdk.media.upload(
  {
    mediaAssetId: 'media-asset-1',
    resource: {
      type: 'image',
      mimeType: 'image/png',
      size: bytes.length,
      name: 'hero.png',
    },
  },
  bytes,
  {
    checksum: 'sha256:...',
  },
);

await sdk.media.attachText(uploaded.mediaAssetId, {
  conversationId: 'conversation-1',
  text: 'image uploaded from TypeScript',
});
```

### Low-level Upload

Use the low-level flow when the host app needs progress reporting or custom upload retries:

```ts
const created = await sdk.media.createUpload({
  mediaAssetId: 'media-asset-2',
  resource: {
    type: 'file',
    mimeType: 'application/pdf',
    size: bytes.length,
    name: 'report.pdf',
  },
});

if (!created.upload) {
  throw new Error('Backend did not return a presigned upload session');
}

await sdk.media.uploadContent(created.upload, bytes);

const completed = await sdk.media.completeUpload(created.mediaAssetId, {
  bucket: created.upload.bucket,
  objectKey: created.upload.objectKey,
  storageProvider: created.upload.storageProvider,
  url: created.upload.url,
  checksum: 'sha256:...',
});
```

`MediaUploadMutationResponse` is the canonical media mutation envelope. It includes the persisted
media asset fields plus:

- `upload`
  Present on create when the backend issues the presigned upload session
- `requestKey`
  The backend mutation idempotency key
- `deliveryStatus`
  `applied` or `replayed`
- `proofVersion`
  Mutation proof version emitted by the backend

The underlying HTTP contract is documented in [Media](/api-reference/app/media).

## Realtime Boundary

The current TypeScript SDK generates the HTTP coordination endpoints for realtime, but it does not
yet generate a websocket transport adapter. That boundary remains manual-owned and documented
separately.

Treat the current realtime split this way:

- generated HTTP support owns resume, subscription sync, event pull, and ACK flow
- manual TypeScript runtime code is the only valid place for any future websocket adapter
- API route semantics still live in [Session and Realtime](/api-reference/app/session-and-realtime)

## Assembly Metadata

Root verification refreshes `.sdkwork-assembly.json` for the workspace.

That release-facing metadata file records:

- the authority and derived spec paths
- per-language package metadata
- each package `manifestPath`
- the explicit `generated` and `composed` package layers
- a `generatedAt` timestamp that stays stable when assembly content is unchanged

That means automation can discover TypeScript package boundaries and entry manifests without
walking the full repository tree.

## Local Workspace Workflow

Use the workspace this way:

1. Treat `generated/server-openapi` as generator-owned and `composed` as manual-owned.
2. Read `generated/server-openapi/README.md` when you need the raw generated transport contract.
3. Read `composed/README.md` when you need package-level consumer examples for
   `@sdkwork/craw-chat-sdk`.
4. Run the workspace verifier when you change package docs, package boundaries, or generated/manual
   integration.

Run from the repository root:

```powershell
node .\sdks\sdkwork-craw-chat-sdk\bin\verify-typescript-workspace.mjs
node .\sdks\sdkwork-craw-chat-sdk\bin\verify-sdk.mjs
```

Run from the TypeScript workspace when you want the forwarding wrappers instead:

```powershell
.\bin\sdk-gen.ps1
.\bin\sdk-verify.ps1
```

## What To Read Next

- Read [App SDK](/sdk/app-sdk) for the family-level contract and root workspace rules.
- Read [Flutter SDK](/sdk/flutter-sdk) when you need the current Dart consumer boundary.
- Read [Language Support](/sdk/language-support) for the cross-family package and release matrix.
- Read [Media](/api-reference/app/media) for the exact upload and attachment route contract.
- Read [App API Overview](/api-reference/app-api) when you need the broader app-facing HTTP map.
