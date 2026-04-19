# `GET /api/v1/control/social/external-connections/{connection_id}`

<p class="api-page-intro">
  Exact request and response contract for <strong>Social Graph Control</strong> in the <strong>Control Plane API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane/social"><code>Social Graph Control</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/control-plane-api"><code>Control Plane API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Shared bearer, trusted-header, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/social/external-connections/{connection_id}</code>
  <span class="api-op-id">operationId: getExternalConnectionSnapshot</span>
</div>

Read an external connection snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-control-plane-sdk` / `sdk.social`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `connection_id` | `string` | Yes | External connection aggregate identifier. |

### Response `200`

`SocialExternalConnectionSnapshotResponse` is currently modeled as an open-ended social snapshot
payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The external connection identifier is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested external connection aggregate does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the read. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>
