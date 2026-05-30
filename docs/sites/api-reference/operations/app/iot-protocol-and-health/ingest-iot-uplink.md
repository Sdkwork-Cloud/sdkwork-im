# `POST /app/v3/api/iot/protocol/uplink`

<p class="api-page-intro">
  Exact request and response contract for <strong>IoT Protocol and Health</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/iot-protocol-and-health"><code>IoT Protocol and Health</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, AppContext projection, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/app/v3/api/iot/protocol/uplink</code>
  <span class="api-op-id">operationId: ingestIotProtocolUplink</span>
</div>

Decodes an external uplink payload into an internal telemetry stream frame.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.iot`</span></div>
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
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `device_permission_denied` | The caller is not an authorized device actor. |
| `404` | `device_not_found` | The target device is not registered. |
| `503` | `*_unavailable` | The IoT protocol adapter is unavailable. |

</section>
