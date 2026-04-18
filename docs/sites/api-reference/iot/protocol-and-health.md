# IoT Protocol and Health

<p class="api-page-intro">
  These endpoints expose IoT provider health and protocol translation between external device
  payloads and the internal stream model.
</p>

<a id="get-iot-access-provider-health"></a>
<section class="api-op">

## `GET /api/v1/iot/access/provider-health`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/iot/access/provider-health</code>
  <span class="api-op-id">operationId: getIotAccessProviderHealth</span>
</div>

Returns the IoT access provider health snapshot.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>No standalone published SDK family</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderHealthSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ProviderHealthSnapshot" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `503` | `*_unavailable` | The provider health source is unavailable. |

</section>
<a id="get-iot-protocol-provider-health"></a>
<section class="api-op">

## `GET /api/v1/iot/protocol/provider-health`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/iot/protocol/provider-health</code>
  <span class="api-op-id">operationId: getIotProtocolProviderHealth</span>
</div>

Returns the IoT protocol adapter health snapshot.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>No standalone published SDK family</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderHealthSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ProviderHealthSnapshot" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `503` | `*_unavailable` | The provider health source is unavailable. |

</section>
<a id="ingest-iot-uplink"></a>
<section class="api-op">

## `POST /api/v1/iot/protocol/uplink`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/iot/protocol/uplink</code>
  <span class="api-op-id">operationId: ingestIotProtocolUplink</span>
</div>

Decodes an external uplink payload into an internal telemetry stream frame.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>No standalone published SDK family</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Registered bound device actor.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 StreamFrame`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="IotProtocolUplinkRequest" />

### Response `200`

<ApiSchemaTable schema="StreamFrame" />

### Security

- Caller must satisfy the runtime's uplink pre-checks and device read access rules.


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `device_id_missing`, `device_id_mismatch`, `invalid_request` | The uplink payload or bound device id is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `device_permission_denied` | The caller is not an authorized device actor. |
| `404` | `device_not_found` | The target device is not registered. |
| `503` | `*_unavailable` | The IoT protocol adapter is unavailable. |

</section>
<a id="ingest-iot-downlink"></a>
<section class="api-op">

## `POST /api/v1/iot/protocol/downlink`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/iot/protocol/downlink</code>
  <span class="api-op-id">operationId: ingestIotProtocolDownlink</span>
</div>

Encodes a platform JSON payload into the device protocol and writes the result into the device
command stream.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>No standalone published SDK family</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Registered device scope with `device.command.send`.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 IotProtocolDownlinkResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="IotProtocolDownlinkRequest" />

### Response `200`

<ApiSchemaTable schema="IotProtocolDownlinkResponse" />

### Security

- Requires `device.command.send`.


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `device_id_missing`, `device_id_mismatch`, `invalid_request` | The downlink payload or device id is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `device_permission_denied` | The caller lacks `device.command.send` or device ownership. |
| `404` | `device_not_found` | The target device is not registered. |
| `503` | `*_unavailable` | The IoT protocol adapter is unavailable. |

</section>
