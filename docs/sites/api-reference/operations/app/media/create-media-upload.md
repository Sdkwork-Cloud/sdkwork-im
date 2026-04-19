# `POST /api/v1/media/uploads`

<p class="api-page-intro">
  Exact request and response contract for <strong>Media</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/media"><code>Media</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Shared bearer, trusted-header, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

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
