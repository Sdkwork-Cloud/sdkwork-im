# Media

<p class="api-page-intro">
  Media endpoints cover upload registration, upload completion, metadata reads, signed download
  URLs, and attachment of ready assets to conversation messages.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/messages"><code>Messages</code> Media attachment targets are conversation messages documented on a separate page</a>
  <a href="/sdk/app-sdk"><code>SDK</code> <code>@sdkwork/im-sdk</code> and <code>im_sdk</code> both expose media helpers above these transport routes</a>
</div>

## Recommended SDK Mapping

Use `sdk.media` for route-aligned upload and attachment control, and prefer `sdk.upload(...)` for
the standard client-side S3 presigned upload flow:

- `sdk.media.createUploadSession(...)`
- `sdk.media.createUpload(...)` as the low-level route alias
- `sdk.media.upload(...)`
- `sdk.media.uploadAndComplete(...)`
- `sdk.upload(...)`
- `sdk.media.completeUpload(...)`
- `sdk.media.getDownloadUrl(...)`
- `sdk.media.get(...)`
- `sdk.media.attach(...)`
- `sdk.media.attachText(...)`

### Typical Client Upload Flow

For normal browser and Node.js application code, use the semantic upload helper. It creates the
pending asset, uploads the binary through the returned presigned S3-compatible request, completes
the asset, and gives you `mediaAssetId`, the final asset metadata, and the resolved CDN/download
URL.

```ts
const uploaded = await sdk.upload({
  mediaAssetId: 'asset-image-1',
  bucket: 'tenant-media',
  objectKey: 'conversation-1/storefront.png',
  resource: {
    type: 'image',
    name: 'storefront.png',
    mimeType: 'image/png',
    size: file.size,
  },
  body: file,
});

await sdk.media.attachText(uploaded.mediaAssetId, {
  conversationId: 'conversation-1',
  text: 'Uploaded storefront concept',
  summary: 'Storefront concept',
});

const download = await sdk.media.getDownloadUrl(uploaded.mediaAssetId, {
  expiresInSeconds: 600,
});

console.log(uploaded.url, download.downloadUrl);
```

`sdk.upload(...)` is the preferred root helper for application code. `sdk.media.upload(...)` and
`sdk.media.uploadAndComplete(...)` provide the same presigned upload behavior from the namespaced
media module.

### Namespaced Upload Alias

```ts
const uploaded = await sdk.media.uploadAndComplete({
  mediaAssetId: 'asset-file-1',
  bucket: 'tenant-media',
  objectKey: 'conversation-1/brief.pdf',
  resource: {
    type: 'file',
    name: 'brief.pdf',
    mimeType: 'application/pdf',
    size: file.size,
  },
  body: file,
});

console.log(uploaded.mediaAssetId, uploaded.asset.processingState);
```

### Manual Presigned Upload Flow

When you need full control over the upload transaction, work directly with the upload session:

```ts
const session = await sdk.media.createUploadSession({
  mediaAssetId: 'asset-image-1',
  bucket: 'tenant-media',
  resource: {
    type: 'image',
    name: 'storefront.png',
    mimeType: 'image/png',
    size: file.size,
  },
});

const uploadResponse = await fetch(session.uploadUrl, {
  method: session.uploadMethod,
  headers: session.uploadHeaders,
  body: file,
});

await sdk.media.completeUpload(session.mediaAssetId, {
  bucket: session.bucket,
  objectKey: session.objectKey,
  storageProvider: session.storageProvider,
  url: session.uploadUrl,
  etag: uploadResponse.headers.get('etag') ?? undefined,
});
```

<a id="create-media-upload"></a>
<section class="api-op">

## `POST /api/v1/media/uploads`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/media/uploads</code>
  <span class="api-op-id">operationId: createMediaUpload</span>
</div>

Registers a pending media asset before the binary upload is completed and returns a presigned
upload session that the client can use for direct object-storage transfer.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.media`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal with media asset ownership checks.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 MediaUploadMutationResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="CreateUploadRequest" />

### Response `200`

<ApiSchemaTable schema="MediaUploadMutationResponse" />

### Upload Session

<ApiSchemaTable schema="MediaUploadSession" />

### Response Notes

- `upload` carries the presigned upload session for direct object-storage transfer.
- `requestKey`, `deliveryStatus`, and `proofVersion` let clients reason about idempotent replay.
- The asset fields stay flattened in the same response payload for parity with the current runtime.


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="complete-media-upload"></a>
<section class="api-op">

## `POST /api/v1/media/uploads/{media_asset_id}/complete`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/media/uploads/{media_asset_id}/complete</code>
  <span class="api-op-id">operationId: completeMediaUpload</span>
</div>

Marks the upload as complete, finalizes object metadata, and returns the same mutation envelope
without another upload session.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.media`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal with media asset ownership checks.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 MediaUploadMutationResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `media_asset_id` | `string` | Yes | Media asset identifier. |

### Request Body

<ApiSchemaTable schema="CompleteUploadRequest" />

### Response `200`

<ApiSchemaTable schema="MediaUploadMutationResponse" />

### Response Notes

- `upload` is omitted after completion because the presigned transfer has already been consumed.
- The asset fields reflect the finalized storage binding and ready-state metadata.


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="get-media"></a>
<section class="api-op">

## `GET /api/v1/media/{media_asset_id}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/media/{media_asset_id}</code>
  <span class="api-op-id">operationId: getMedia</span>
</div>

Reads the media asset metadata.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.media`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal with media asset ownership checks.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 MediaAsset`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `media_asset_id` | `string` | Yes | Media asset identifier. |

### Response `200`

<ApiSchemaTable schema="MediaAsset" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to access the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the read or handshake flow. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="get-media-download-url"></a>
<section class="api-op">

## `GET /api/v1/media/{media_asset_id}/download-url`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/media/{media_asset_id}/download-url</code>
  <span class="api-op-id">operationId: getMediaDownloadUrl</span>
</div>

Generates a signed download URL for the media asset.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.media`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal with media asset ownership checks.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 MediaDownloadUrlResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `media_asset_id` | `string` | Yes | Media asset identifier. |

### Query Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `expiresInSeconds` | `uint32 \| null` | No | Requested URL lifetime. Defaults to `3600` in the current implementation. |

### Response `200`

<ApiSchemaTable schema="MediaDownloadUrlResponse" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to access the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the read or handshake flow. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="attach-media"></a>
<section class="api-op">

## `POST /api/v1/media/{media_asset_id}/attach`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/media/{media_asset_id}/attach</code>
  <span class="api-op-id">operationId: attachMedia</span>
</div>

Attaches a ready media asset to a conversation by emitting a message containing a media content part.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.media`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal with media asset ownership and target conversation write access.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PostMessageResult`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `media_asset_id` | `string` | Yes | Media asset identifier. |

### Request Body

<ApiSchemaTable schema="AttachMediaRequest" />

### Response `200`

<ApiSchemaTable schema="PostMessageResult" />

### Response Notes

- `processingState` must already be `ready`.
- The runtime wraps the asset into a `ContentPart` with `kind = media`.


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
