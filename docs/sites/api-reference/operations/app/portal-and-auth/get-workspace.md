# `GET /api/v1/portal/workspace`

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
  <code>/api/v1/portal/workspace</code>
  <span class="api-op-id">operationId: getWorkspace</span>
</div>

Reads the current authenticated tenant workspace summary used by the portal shell.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.portal.getWorkspace()`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalWorkspaceView`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalWorkspaceView" />

### Example Response

```json
{
  "name": "Nebula Commerce IM",
  "slug": "nebula-commerce-im",
  "tier": "Enterprise",
  "region": "CN-East / Multi-AZ",
  "supportPlan": "Platinum",
  "seats": 84,
  "activeBrands": 12,
  "uptime": "99.983%"
}
```

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `portal_access_denied`, `permission_denied` | The principal cannot access the workspace summary. |
| `503` | `*_unavailable` | The workspace snapshot source is unavailable. |

</section>
