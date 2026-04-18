# `POST /api/v1/iot/protocol/downlink`

<p class="api-page-intro">
  Exact request and response contract for <strong>Protocol and Health</strong> in the <strong>IoT API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/iot/protocol-and-health"><code>Protocol and Health</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/iot-api"><code>IoT API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Shared bearer, trusted-header, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

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
