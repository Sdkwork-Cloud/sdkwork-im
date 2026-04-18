# `GET /api/v1/devices/{device_id}/sync-feed`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Device Sync</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/device-sync">Back to Device Sync</a>
</div>

<section class="api-op api-op-single">

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
