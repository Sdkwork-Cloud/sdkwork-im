# `POST /app/v3/api/devices/{deviceId}/twin/reported`

<p class="api-page-intro">
  Exact request and response contract for <strong>Device Twin</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/device-twin"><code>Device Twin</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, AppContext projection, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/app/v3/api/devices/{deviceId}/twin/reported</code>
  <span class="api-op-id">operationId: updateDeviceTwinReported</span>
</div>

Updates the reported twin state produced by the device runtime.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.device.twin.reported.create(deviceId, body)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Registered device owner or authorized device actor.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 DeviceTwinView`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `deviceId` | `string` | Yes | Registered device identifier. |

### Request Body

<ApiSchemaTable schema="UpdateDeviceTwinReportedRequest" />

### Response `200`

<ApiSchemaTable schema="DeviceTwinView" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `device_id_missing`, `device_id_mismatch`, `invalid_request` | The device twin payload or bound device id is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `device_permission_denied`, `permission_denied` | The caller is not allowed to mutate the device twin. |
| `404` | `device_not_found` | The target device is not registered. |
| `409` | `device_twin_conflict`, `conflict` | Current device twin state blocks the mutation. |
| `503` | `*_unavailable` | The device twin source is unavailable. |

</section>
