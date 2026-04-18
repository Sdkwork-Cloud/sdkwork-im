# `GET /api/v1/audit/verify`

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
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/audit/verify</code>
  <span class="api-op-id">operationId: verifyAuditChain</span>
</div>

Verifies the tenant audit hash-chain and returns a summary integrity status.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token or trusted headers</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / audit</span></div>
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
