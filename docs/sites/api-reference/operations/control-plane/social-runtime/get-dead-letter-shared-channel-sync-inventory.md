# `GET /api/v1/control/social/runtime/dead-letter-shared-channel-sync`

<p class="api-page-intro">
  Exact request and response contract for <strong>Social Runtime</strong> in the <strong>Control Plane API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane/social-runtime"><code>Social Runtime</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/control-plane-api"><code>Control Plane API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Shared bearer, trusted-header, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/social/runtime/dead-letter-shared-channel-sync</code>
  <span class="api-op-id">operationId: getDeadLetterSharedChannelSyncInventory</span>
</div>

Read the dead-letter shared-channel sync queue.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Response `200`

`SocialSharedChannelSyncDeadLetterInventoryResponse` is currently modeled as an open-ended runtime
inventory payload in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The queue read request is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested inventory source does not exist. |
| `409` | `*_conflict` | Current runtime state blocks the read. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
