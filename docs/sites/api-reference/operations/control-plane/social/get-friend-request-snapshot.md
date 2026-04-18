# `GET /api/v1/control/social/friend-requests/{request_id}`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Social Graph Control</strong> in the <strong>Control Plane API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane/social">Back to Social Graph Control</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/social/friend-requests/{request_id}</code>
  <span class="api-op-id">operationId: getFriendRequestSnapshot</span>
</div>

Read a friend request snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-graph</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialFriendRequestSnapshotResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `request_id` | `string` | Yes | Friend request aggregate identifier. |

### Response `200`

`SocialFriendRequestSnapshotResponse` is currently modeled as an open-ended social snapshot payload
in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The friend request identifier is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested friend request aggregate does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the read. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>
