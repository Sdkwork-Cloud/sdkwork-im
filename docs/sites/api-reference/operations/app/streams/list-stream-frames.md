# `GET /api/v1/streams/{stream_id}/frames`

<p class="api-page-intro">
  Exact request and response contract for <strong>Streams</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/streams"><code>Streams</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Shared bearer, trusted-header, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/streams/{stream_id}/frames</code>
  <span class="api-op-id">operationId: listStreamFrames</span>
</div>

Reads a paged window of frames for a stream.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/craw-chat-sdk` / `sdk.generated.stream.listStreamFrames(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation member or registered device read scope.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 StreamFrameWindow`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `stream_id` | `string` | Yes | Stream identifier. |

### Query Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `afterFrameSeq` | `uint64 \| null` | No | Return frames strictly after this sequence number. |
| `limit` | `uint64 \| null` | No | Window size. |

### Response `200`

<ApiSchemaTable schema="StreamFrameWindow" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to access the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the read or handshake flow. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
