# Device Twin

<p class="api-page-intro">
  Device twin endpoints expose app-business read and mutation flows for desired and reported device
  state under <code>/app/v3/api/*</code>. They are outside the IM standardized development API and
  are generated through <code>sdkwork-im-app-sdk</code>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app-api"><code>App API</code> Return to the app-business domain overview</a>
  <a href="/api-reference/app/iot-protocol-and-health"><code>IoT</code> Device protocol ingress and egress are documented separately</a>
  <a href="/sdk/app-sdk"><code>App SDK</code> Use <code>sdkwork-im-app-sdk</code> and <code>SdkworkAppClient.device.twin</code></a>
</div>

<a id="get-device-twin"></a>
<section class="api-op">

## `GET /app/v3/api/devices/{deviceId}/twin`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/app/v3/api/devices/{deviceId}/twin</code>
  <span class="api-op-id">operationId: getDeviceTwin</span>
</div>

Reads the latest desired and reported twin state for a registered device.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.device.twin.list(deviceId)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Registered device owner or authorized device observer.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 DeviceTwinView`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `deviceId` | `string` | Yes | Registered device identifier. |

### Response `200`

<ApiSchemaTable schema="DeviceTwinView" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `device_permission_denied`, `permission_denied` | The caller is not allowed to read the device twin. |
| `404` | `device_not_found` | The target device is not registered. |
| `503` | `*_unavailable` | The device twin source is unavailable. |

</section>

<a id="update-device-twin-desired"></a>
<section class="api-op">

## `POST /app/v3/api/devices/{deviceId}/twin/desired`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/app/v3/api/devices/{deviceId}/twin/desired</code>
  <span class="api-op-id">operationId: updateDeviceTwinDesired</span>
</div>

Updates the desired twin state that the device runtime should converge toward.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.device.twin.desired.create(deviceId, body)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Registered device owner or authorized device actor.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 DeviceTwinView`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `deviceId` | `string` | Yes | Registered device identifier. |

### Request Body

<ApiSchemaTable schema="UpdateDeviceTwinDesiredRequest" />

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

<a id="update-device-twin-reported"></a>
<section class="api-op">

## `POST /app/v3/api/devices/{deviceId}/twin/reported`

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
