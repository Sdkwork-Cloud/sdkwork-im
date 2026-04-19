# `GET /api/v1/control/provider-bindings`

<p class="api-page-intro">
  Exact request and response contract for <strong>Provider Governance</strong> in the <strong>Control Plane API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane/providers"><code>Provider Governance</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/control-plane-api"><code>Control Plane API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Shared bearer, trusted-header, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/provider-bindings</code>
  <span class="api-op-id">operationId: getProviderBindings</span>
</div>

Reads effective provider bindings for the deployment scope or a tenant override scope.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-control-plane-sdk` / provider-governance</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderBindingsResponse`</span></div>
</div>

### Query Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `tenantId` | `string \| null` | No | Tenant identifier for override scope. Omit for deployment-level bindings. |

### Response `200`

<ApiSchemaTable schema="ProviderBindingsResponse" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | Query or path parameters are invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `503` | `*_unavailable` | The governance snapshot or provider runtime is unavailable. |

</section>
