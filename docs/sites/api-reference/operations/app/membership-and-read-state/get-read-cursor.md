# `GET /api/v1/conversations/{conversation_id}/read-cursor`

<p class="api-page-intro">
  Exact request and response contract for <strong>Membership and Read State</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/membership-and-read-state"><code>Membership and Read State</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Shared bearer, trusted-header, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/conversations/{conversation_id}/read-cursor</code>
  <span class="api-op-id">operationId: getReadCursor</span>
</div>

Returns the read cursor for the current principal in the target conversation.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token or trusted headers</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/craw-chat-sdk` / `sdk.conversations`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Active conversation member.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ConversationReadCursorView`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `conversation_id` | `string` | Yes | Conversation identifier. |

### Response `200`

<ApiSchemaTable schema="ConversationReadCursorView" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to access the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the read or handshake flow. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
