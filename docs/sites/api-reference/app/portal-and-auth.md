# Portal and Auth

<p class="api-page-intro">
  Portal and auth endpoints cover tenant-portal sign-in, current-session discovery, public landing
  snapshots, authenticated workspace reads, and portal module snapshots for dashboard,
  conversations, realtime, media, automation, and governance.
</p>

<div class="api-link-list">
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Shared bearer rules and error-envelope behavior are documented separately</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the full app-runtime domain overview</a>
  <a href="/sdk/typescript-sdk"><code>TypeScript SDK</code> <code>@sdkwork/craw-chat-sdk</code> currently exposes <code>auth</code> and <code>portal</code> through root generated exports and semantic portal helpers</a>
  <a href="/sdk/flutter-sdk"><code>Flutter SDK</code> The official <code>craw_chat_sdk</code> package and underlying <code>backend_sdk</code> transport package do not yet export <code>auth</code> or <code>portal</code></a>
</div>

<a id="login"></a>
<section class="api-op">

## `POST /api/v1/auth/login`

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

<a id="me"></a>
<section class="api-op">

## `GET /api/v1/auth/me`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/auth/me</code>
  <span class="api-op-id">operationId: me</span>
</div>

Reads the current portal session and returns the resolved tenant, user, and workspace context.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalMeResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalMeResponse" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `503` | `*_unavailable` | An auth or portal dependency is unavailable. |

</section>

<a id="get-home"></a>
<section class="api-op">

## `GET /api/v1/portal/home`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/portal/home</code>
  <span class="api-op-id">operationId: getHome</span>
</div>

Reads the public tenant-portal home snapshot used for the landing experience.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalSnapshot" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `503` | `*_unavailable` | The portal snapshot source is unavailable. |

</section>

<a id="get-auth"></a>
<section class="api-op">

## `GET /api/v1/portal/auth`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/portal/auth</code>
  <span class="api-op-id">operationId: getAuth</span>
</div>

Reads the public sign-in snapshot for the tenant portal.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalSnapshot" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `503` | `*_unavailable` | The portal snapshot source is unavailable. |

</section>

<a id="get-workspace"></a>
<section class="api-op">

## `GET /api/v1/portal/workspace`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/portal/workspace</code>
  <span class="api-op-id">operationId: getWorkspace</span>
</div>

Reads the current authenticated tenant workspace summary used by the portal shell.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk`</span></div>
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

<a id="get-dashboard"></a>
<section class="api-op">

## `GET /api/v1/portal/dashboard`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/portal/dashboard</code>
  <span class="api-op-id">operationId: getDashboard</span>
</div>

Reads the authenticated dashboard snapshot for tenant operations.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalSnapshot" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `portal_access_denied`, `permission_denied` | The principal cannot access the dashboard snapshot. |
| `503` | `*_unavailable` | The dashboard snapshot source is unavailable. |

</section>

<a id="get-conversations"></a>
<section class="api-op">

## `GET /api/v1/portal/conversations`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/portal/conversations</code>
  <span class="api-op-id">operationId: getConversations</span>
</div>

Reads the portal conversations module snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalSnapshot" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `portal_access_denied`, `permission_denied` | The principal cannot access the conversations snapshot. |
| `503` | `*_unavailable` | The conversations snapshot source is unavailable. |

</section>

<a id="get-realtime"></a>
<section class="api-op">

## `GET /api/v1/portal/realtime`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/portal/realtime</code>
  <span class="api-op-id">operationId: getRealtime</span>
</div>

Reads the portal realtime posture snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalSnapshot" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `portal_access_denied`, `permission_denied` | The principal cannot access the realtime snapshot. |
| `503` | `*_unavailable` | The realtime snapshot source is unavailable. |

</section>

<a id="get-media"></a>
<section class="api-op">

## `GET /api/v1/portal/media`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/portal/media</code>
  <span class="api-op-id">operationId: getMedia</span>
</div>

Reads the portal media and RTC snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalSnapshot" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `portal_access_denied`, `permission_denied` | The principal cannot access the media snapshot. |
| `503` | `*_unavailable` | The media snapshot source is unavailable. |

</section>

<a id="get-automation"></a>
<section class="api-op">

## `GET /api/v1/portal/automation`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/portal/automation</code>
  <span class="api-op-id">operationId: getAutomation</span>
</div>

Reads the portal automation and notification posture snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk`</span></div>
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

<a id="get-governance"></a>
<section class="api-op">

## `GET /api/v1/portal/governance`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/portal/governance</code>
  <span class="api-op-id">operationId: getGovernance</span>
</div>

Reads the portal governance and compliance snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalSnapshot" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `portal_access_denied`, `permission_denied` | The principal cannot access the governance snapshot. |
| `503` | `*_unavailable` | The governance snapshot source is unavailable. |

</section>
