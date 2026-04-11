# `POST /api/v1/streams/{stream_id}/frames`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Streams</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/streams">Back to Streams</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/streams/{stream_id}/frames</code>
  <span class="api-op-id">operationId: appendStreamFrame</span>
</div>

Appends a frame to an open stream.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token or trusted headers</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / streams</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `stream.append` capability or device stream permission.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 StreamFrame`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `stream_id` | `string` | Yes | Stream identifier. |

### Request Body

<ApiSchemaTable schema="AppendStreamFrameRequest" />

### Response `200`

<ApiSchemaTable schema="StreamFrame" />


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
