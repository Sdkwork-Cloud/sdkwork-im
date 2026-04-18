# `POST /api/v1/auth/login`

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
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/auth/login</code>
  <span class="api-op-id">operationId: login</span>
</div>

Signs a portal operator into the current tenant workspace and returns bearer-token session material.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalLoginResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="PortalLoginRequest" />

### Response `200`

<ApiSchemaTable schema="PortalLoginResponse" />

### Example Request

Portal credentials are provisioned per environment. Public portal docs intentionally show issued
credentials, not a built-in demo operator account.

```json
{
  "tenantId": "tenant-acme",
  "login": "ops_lead",
  "password": "your-issued-password",
  "clientKind": "portal_operator"
}
```

### Example Response

```json
{
  "accessToken": "portal-token",
  "refreshToken": "portal-refresh-token",
  "expiresAt": 1760000000,
  "user": {
    "id": "ops_lead",
    "login": "ops_lead",
    "name": "Avery Chen",
    "role": "Tenant Operations Lead",
    "email": "avery.chen@acme-commerce.example",
    "actorKind": "user",
    "clientKind": "portal_operator",
    "permissions": ["portal.access"]
  },
  "workspace": {
    "name": "Acme Commerce IM",
    "slug": "acme-commerce-im",
    "tier": "Enterprise",
    "region": "CN-East / Multi-AZ",
    "supportPlan": "Platinum",
    "seats": 84,
    "activeBrands": 12,
    "uptime": "99.983%"
  }
}
```

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The login payload is malformed. |
| `401` | `invalid_credentials`, `invalid_token` | Credentials could not be validated. |
| `403` | `portal_access_denied`, `permission_denied` | The principal cannot access the requested tenant portal. |
| `413` | `payload_too_large` | The submitted payload exceeds allowed limits. |
| `503` | `*_unavailable` | An authentication or portal dependency is unavailable. |

</section>
