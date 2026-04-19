# `GET /api/v1/portal/automation`

<p class="api-page-intro">
  Exact request and response contract for <strong>Portal and Auth</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/portal-and-auth"><code>Portal and Auth</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Shared bearer, trusted-header, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/portal/automation</code>
  <span class="api-op-id">operationId: getAutomation</span>
</div>

Reads the portal automation and notification posture snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.portal.getAutomation()`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalSnapshot" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `portal_access_denied`, `permission_denied` | The principal cannot access the automation snapshot. |
| `503` | `*_unavailable` | The automation snapshot source is unavailable. |

</section>
