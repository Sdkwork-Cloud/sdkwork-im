# `POST /api/v1/control/social/external-connections`

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
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/social/external-connections</code>
  <span class="api-op-id">operationId: establishExternalConnection</span>
</div>

Establish an external collaboration connection.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="EstablishExternalConnectionRequest" />

### Response `200`

`SocialExternalConnectionCommitResponse` is currently modeled as an open-ended social commit payload
in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The external-connection payload is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | A referenced external tenant or aggregate does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the mutation. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>
