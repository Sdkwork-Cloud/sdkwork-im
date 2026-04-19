# `POST /api/v1/control/provider-policies/preview`

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
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/provider-policies/preview</code>
  <span class="api-op-id">operationId: previewProviderPolicy</span>
</div>

Previews a provider policy mutation without persisting it.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-control-plane-sdk` / provider-governance</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderPolicyPreview`</span></div>
</div>

### Request Body

Uses the same request schema as `POST /api/v1/control/provider-bindings`.

<ApiSchemaTable schema="UpsertProviderBindingPolicyRequest" />

### Response `200`

<ApiSchemaTable schema="ProviderPolicyPreview" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `invalid_provider_policy` | The mutation payload is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found`, `provider_plugin_not_found` | The requested node, plugin, or target resource does not exist. |
| `409` | `*_conflict`, `provider_policy_conflict` | Current control-plane state blocks the mutation. |
| `503` | `*_unavailable` | The governance snapshot or provider runtime is unavailable. |

</section>
