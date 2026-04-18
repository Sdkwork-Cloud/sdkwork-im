# `GET /api/v1/control/social/direct-chats/{direct_chat_id}`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Social Graph Control</strong> in the <strong>Control Plane API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane/social">Back to Social Graph Control</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/social/direct-chats/{direct_chat_id}</code>
  <span class="api-op-id">operationId: getDirectChatSnapshot</span>
</div>

Read a direct chat snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-graph</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialDirectChatSnapshotResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `direct_chat_id` | `string` | Yes | Direct chat aggregate identifier. |

### Response `200`

`SocialDirectChatSnapshotResponse` is currently modeled as an open-ended social snapshot payload in
the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The direct chat identifier is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested direct chat aggregate does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the read. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>
