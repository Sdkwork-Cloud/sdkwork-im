# `GET /api/v1/realtime/events`

<p class="api-page-intro">
  Exact request and response contract for <strong>Session and Realtime</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/session-and-realtime"><code>Session and Realtime</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Shared bearer, trusted-header, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/realtime/events</code>
  <span class="api-op-id">operationId: listRealtimeEvents</span>
</div>

Fetches realtime events from the device event window.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token or trusted headers</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/craw-chat-sdk` / `sdk.sync.catchUp(...)`, `sdk.generated.realtime.listRealtimeEvents(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; device ownership and session binding are enforced where required.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RealtimeEventWindow`</span></div>
</div>

### Query Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `afterSeq` | `uint64 \| null` | No | Continue reading after this realtime sequence. |
| `limit` | `uint64 \| null` | No | Maximum number of events to return. The current default is `100`. |

### Response `200`

<ApiSchemaTable schema="RealtimeEventWindow" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to access the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the read or handshake flow. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
