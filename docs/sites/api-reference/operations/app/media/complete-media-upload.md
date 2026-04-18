# `POST /api/v1/media/uploads/{media_asset_id}/complete`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Media</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/media">Back to Media</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/media/uploads/{media_asset_id}/complete</code>
  <span class="api-op-id">operationId: completeMediaUpload</span>
</div>

Marks the upload as complete, finalizes object metadata, and returns the same mutation envelope
without another upload session.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token or trusted headers</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / media</span></div>
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
