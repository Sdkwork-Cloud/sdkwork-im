# `POST /api/v1/audit/records`

<p class="api-page-intro">
  Exact request and response contract for <strong>Audit</strong> in the <strong>Platform API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/platform/audit"><code>Audit</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/platform-api"><code>Platform API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Shared bearer, trusted-header, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/audit/records</code>
  <span class="api-op-id">operationId: recordAuditAnchor</span>
</div>

Writes a new audit record.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>No standalone published SDK family</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`audit.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 AuditRecord`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="RecordAuditAnchor" />

### Response `200`

<ApiSchemaTable schema="AuditRecord" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The audit anchor payload is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `audit.write`. |

</section>
