# `GET /api/v1/control/protocol-governance`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Protocol Governance</strong> in the <strong>Control Plane API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane/protocol">Back to Protocol Governance</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/protocol-governance</code>
  <span class="api-op-id">operationId: getProtocolGovernance</span>
</div>

Returns the effective control-plane governance snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / protocol-governance</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProtocolGovernanceResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ProtocolGovernanceResponse" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | Query or path parameters are invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `503` | `*_unavailable` | The governance snapshot or provider runtime is unavailable. |

</section>
