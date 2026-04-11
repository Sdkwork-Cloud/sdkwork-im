# `GET /api/v1/presence/me`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Session and Realtime</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/session-and-realtime">Back to Session and Realtime</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/presence/me</code>
  <span class="api-op-id">operationId: getPresenceMe</span>
</div>

Reads the current principal's presence snapshot.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token or trusted headers</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / session</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; device ownership and session binding are enforced where required.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PresenceSnapshotView`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PresenceSnapshotView" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to access the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the read or handshake flow. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
