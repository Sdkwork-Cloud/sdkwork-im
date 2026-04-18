# `GET /api/v1/ops/provider-bindings`

<p class="api-page-intro">
  Exact request and response contract for <strong>Operations</strong> in the <strong>Platform API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/platform/ops"><code>Operations</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/platform-api"><code>Platform API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Shared bearer, trusted-header, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/ops/provider-bindings</code>
  <span class="api-op-id">operationId: getOpsProviderBindings</span>
</div>

Returns the node-local mirror of provider binding snapshots.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>No standalone published SDK family</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`ops.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderBindingsView`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ProviderBindingsView" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `ops.read`. |
| `503` | `*_unavailable` | Operational diagnostics are temporarily unavailable. |

</section>
