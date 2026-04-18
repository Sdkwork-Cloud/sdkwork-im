# Audit

<p class="api-page-intro">
  Audit endpoints record audit anchors and expose read, export, and hash-chain verification flows
  for audit evidence.
</p>

<div class="api-link-list">
  <a href="/api-reference/platform/ops"><code>Ops</code> Runtime diagnostics and cluster views are documented separately</a>
  <a href="/sdk/index"><code>SDK</code> Treat audit flows as HTTP-first surfaces unless a repo consumer layer is documented elsewhere</a>
</div>

<a id="record-audit-anchor"></a>
<section class="api-op">

## `POST /api/v1/audit/records`

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
<a id="list-audit-records"></a>
<section class="api-op">

## `GET /api/v1/audit/records`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/audit/records</code>
  <span class="api-op-id">operationId: listAuditRecords</span>
</div>

Lists audit records visible to the current principal.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>No standalone published SDK family</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`audit.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 AuditRecordListResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="AuditRecordListResponse" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `audit.read`. |

</section>
<a id="export-audit-bundle"></a>
<section class="api-op">

## `GET /api/v1/audit/export`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/audit/export</code>
  <span class="api-op-id">operationId: exportAuditBundle</span>
</div>

Exports an audit bundle containing the visible records at the time of the request.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>No standalone published SDK family</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`audit.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 AuditExportBundle`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="AuditExportBundle" />

The export payload includes `chainHeadHash` and `chainValid` so offline verifiers can detect
tampering before import.


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `audit.read`. |

</section>
<a id="verify-audit-chain"></a>
<section class="api-op">

## `GET /api/v1/audit/verify`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/audit/verify</code>
  <span class="api-op-id">operationId: verifyAuditChain</span>
</div>

Verifies the tenant audit hash-chain and returns a summary integrity status.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>No standalone published SDK family</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`audit.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 AuditChainVerification`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="AuditChainVerification" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `audit.read`. |

</section>
