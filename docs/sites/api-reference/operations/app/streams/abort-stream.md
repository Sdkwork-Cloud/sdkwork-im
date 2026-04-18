# `POST /api/v1/streams/{stream_id}/abort`

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
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/streams/{stream_id}/abort</code>
  <span class="api-op-id">operationId: abortStream</span>
</div>

Aborts the stream lifecycle.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/craw-chat-sdk` / `sdk.generated.stream.abort(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `stream.abort` capability or device stream permission.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 StreamSession`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `stream_id` | `string` | Yes | Stream identifier. |

### Request Body

<ApiSchemaTable schema="AbortStreamRequest" />

### Response `200`

<ApiSchemaTable schema="StreamSession" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `404` | `stream_not_found` | The target stream does not exist. |
| `409` | `invalid_stream_state` | The stream lifecycle does not permit the requested transition. |

</section>
