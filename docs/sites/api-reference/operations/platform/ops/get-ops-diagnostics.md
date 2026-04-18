# `GET /api/v1/ops/diagnostics`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Operations</strong> in the <strong>Platform API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/platform/ops">Back to Operations</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/ops/diagnostics</code>
  <span class="api-op-id">operationId: getOpsDiagnostics</span>
</div>

Returns the aggregated diagnostic bundle for the current node.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>No standalone published SDK family</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`ops.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 DiagnosticBundle`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="DiagnosticBundle" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `ops.read`. |
| `503` | `*_unavailable` | Operational diagnostics are temporarily unavailable. |

</section>
