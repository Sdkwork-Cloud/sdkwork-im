# `POST /api/v1/devices/register`

<p class="api-page-intro">
  Exact request and response contract for <strong>Device Sync</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/device-sync"><code>Device Sync</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Shared bearer, trusted-header, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/devices/register</code>
  <span class="api-op-id">operationId: registerDevice</span>
</div>

Registers a device for the current principal and establishes the active routing record used by
realtime delivery and projection sync.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token or trusted headers</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/craw-chat-sdk` / `sdk.generated.device.register(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; `deviceId` must match the bound auth context when present.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RegisteredDeviceView`</span></div>
</div>

### Request Body

`application/json`

<ApiSchemaTable schema="RegisterDeviceRequest" />

### Response `200`

<ApiSchemaTable schema="RegisteredDeviceView" />

### Example Request

```json
{
  "deviceId": "device-web-01"
}
```

### Example Response

```json
{
  "tenantId": "tenant-demo",
  "principalId": "user-alice",
  "deviceId": "device-web-01",
  "registeredAt": "2026-04-09T10:00:00Z"
}
```


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
