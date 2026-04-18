# Device Sync

<p class="api-page-intro">
  Device sync endpoints register devices for the current principal and expose the projection-backed
  sync feed consumed by multi-device clients.
</p>

<a id="register-device"></a>
<section class="api-op">

## `POST /api/v1/devices/register`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/devices/register</code>
  <span class="api-op-id">operationId: registerDevice</span>
</div>

Registers a device for the current principal and establishes the active routing record used by
realtime delivery and projection sync.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / device-sync</span></div>
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
<a id="get-device-sync-feed"></a>
<section class="api-op">

## `GET /api/v1/devices/{device_id}/sync-feed`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/devices/{device_id}/sync-feed</code>
  <span class="api-op-id">operationId: getDeviceSyncFeed</span>
</div>

Reads sync-feed entries for a device after the last sequence already processed by the client.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / device-sync</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Registered device owner.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 DeviceSyncFeedResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `device_id` | `string` | Yes | Device identifier. |

### Query Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `afterSeq` | `uint64 \| null` | No | Only return items after this sync sequence. |

### Response `200`

<ApiSchemaTable schema="DeviceSyncFeedResponse" />

### Example Response

```json
{
  "items": [
    {
      "tenantId": "tenant-demo",
      "principalId": "user-alice",
      "deviceId": "device-web-01",
      "syncSeq": 132,
      "originEventId": "evt_001",
      "originEventType": "message.posted",
      "conversationId": "conv_demo_001",
      "messageId": "msg_1001",
      "messageSeq": 7,
      "summary": "hello world"
    }
  ]
}
```

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `403` | `device_permission_denied` | The requested device is not bound to the current principal. |
| `404` | `device_not_found` | The device route or device projection is missing. |

</section>
