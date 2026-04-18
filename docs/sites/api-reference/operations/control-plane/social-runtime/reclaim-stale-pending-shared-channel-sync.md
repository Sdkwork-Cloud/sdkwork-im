# `POST /api/v1/control/social/runtime/reclaim-stale-pending-shared-channel-sync`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Social Runtime</strong> in the <strong>Control Plane API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane/social-runtime">Back to Social Runtime</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/social/runtime/reclaim-stale-pending-shared-channel-sync</code>
  <span class="api-op-id">operationId: reclaimStalePendingSharedChannelSync</span>
</div>

Reclaim stale shared-channel sync pending ownership.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-runtime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialSharedChannelSyncPendingStaleReclaimResponse`</span></div>
</div>

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

`SocialSharedChannelSyncPendingStaleReclaimResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The reclaim request is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | The requested queue or ownership records do not exist. |
| `409` | `*_conflict` | Current runtime ownership blocks the reclaim. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
