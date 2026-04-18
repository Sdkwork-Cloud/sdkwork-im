# Session and Realtime

<p class="api-page-intro">
  Session and realtime endpoints cover health probes, session resume and disconnect flows, presence
  heartbeats, realtime subscription sync, event polling, ACK tracking, and WebSocket upgrade.
</p>

<a id="get-healthz"></a>
<section class="api-op">

## `GET /healthz`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/healthz</code>
  <span class="api-op-id">operationId: getHealthz</span>
</div>

Returns process liveness for the app runtime.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Open endpoint</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / session</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Not required</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 HealthResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="HealthResponse" />

</section>

<a id="get-readyz"></a>
<section class="api-op">

## `GET /readyz`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/readyz</code>
  <span class="api-op-id">operationId: getReadyz</span>
</div>

Returns process readiness for the app runtime.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Open endpoint</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / session</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Not required</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 HealthResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="HealthResponse" />

</section>

<a id="resume-session"></a>
<section class="api-op">

## `POST /api/v1/sessions/resume`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/sessions/resume</code>
  <span class="api-op-id">operationId: resumeSession</span>
</div>

Resumes the current device session and returns the active presence snapshot with replay cursors.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / session</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; device ownership and session binding are enforced where required.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SessionResumeView`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="ResumeSessionRequest" />

### Response `200`

<ApiSchemaTable schema="SessionResumeView" />

### Example Request

```json
{
  "deviceId": "device-web-01",
  "lastSeenSyncSeq": 128
}
```

### Example Response

```json
{
  "tenantId": "tenant-demo",
  "actorId": "user-alice",
  "actorKind": "user",
  "sessionId": "sess_web_01",
  "deviceId": "device-web-01",
  "resumeRequired": false,
  "resumeFromSyncSeq": 128,
  "latestSyncSeq": 132,
  "resumedAt": "2026-04-09T10:00:00Z",
  "presence": {
    "tenantId": "tenant-demo",
    "principalId": "user-alice",
    "currentDeviceId": "device-web-01",
    "devices": []
  }
}
```

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication could not be resolved. |
| `403` | `device_permission_denied` | The device does not belong to the current principal. |
| `409` | `reconnect_required` | The session must be re-established instead of resumed. |

</section>

<a id="disconnect-session"></a>
<section class="api-op">

## `POST /api/v1/sessions/disconnect`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/sessions/disconnect</code>
  <span class="api-op-id">operationId: disconnectSession</span>
</div>

Disconnects the current device session.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / session</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; device ownership and session binding are enforced where required.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PresenceSnapshotView`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="PresenceDeviceRequest" />

### Response `200`

<ApiSchemaTable schema="PresenceSnapshotView" />


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
<a id="heartbeat-presence"></a>
<section class="api-op">

## `POST /api/v1/presence/heartbeat`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/presence/heartbeat</code>
  <span class="api-op-id">operationId: heartbeatPresence</span>
</div>

Refreshes the presence heartbeat for the current device.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / session</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; device ownership and session binding are enforced where required.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PresenceSnapshotView`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="PresenceDeviceRequest" />

### Response `200`

<ApiSchemaTable schema="PresenceSnapshotView" />


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
<a id="get-presence-me"></a>
<section class="api-op">

## `GET /api/v1/presence/me`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/presence/me</code>
  <span class="api-op-id">operationId: getPresenceMe</span>
</div>

Reads the current principal's presence snapshot.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / session</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; device ownership and session binding are enforced where required.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PresenceSnapshotView`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PresenceSnapshotView" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to access the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the read or handshake flow. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="sync-realtime-subscriptions"></a>
<section class="api-op">

## `POST /api/v1/realtime/subscriptions/sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/realtime/subscriptions/sync</code>
  <span class="api-op-id">operationId: syncRealtimeSubscriptions</span>
</div>

Replaces the realtime subscription set for the current device.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / session</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; device ownership and session binding are enforced where required.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RealtimeSubscriptionSnapshot`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SyncRealtimeSubscriptionsRequest" />

### Response `200`

<ApiSchemaTable schema="RealtimeSubscriptionSnapshot" />


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
<a id="list-realtime-events"></a>
<section class="api-op">

## `GET /api/v1/realtime/events`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/realtime/events</code>
  <span class="api-op-id">operationId: listRealtimeEvents</span>
</div>

Fetches realtime events from the device event window.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / session</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; device ownership and session binding are enforced where required.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RealtimeEventWindow`</span></div>
</div>

### Query Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `afterSeq` | `uint64 \| null` | No | Continue reading after this realtime sequence. |
| `limit` | `uint64 \| null` | No | Maximum number of events to return. The current default is `100`. |

### Response `200`

<ApiSchemaTable schema="RealtimeEventWindow" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to access the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the read or handshake flow. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="ack-realtime-events"></a>
<section class="api-op">

## `POST /api/v1/realtime/events/ack`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/realtime/events/ack</code>
  <span class="api-op-id">operationId: ackRealtimeEvents</span>
</div>

Acknowledges the highest realtime event sequence consumed by the client.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / session</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; device ownership and session binding are enforced where required.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RealtimeAckState`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="AckRealtimeEventsRequest" />

### Response `200`

<ApiSchemaTable schema="RealtimeAckState" />


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
<a id="realtime-websocket"></a>
<section class="api-op">

## `GET /api/v1/realtime/ws`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/realtime/ws</code>
  <span class="api-op-id">operationId: realtimeWebsocket</span>
</div>

Upgrades the connection to WebSocket. This page documents the HTTP handshake surface only; it does
not expand the full realtime frame protocol.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / session</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; active device route is prepared before upgrade.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`101 Switching Protocols`</span></div>
</div>

### Security

- Public clients authenticate with bearer tokens. Trusted headers remain reserved for internal service wiring and test-only flows.
- Device ownership and session binding are validated before upgrade

### Headers

| Header | Required | Description |
| --- | --- | --- |
| `Sec-WebSocket-Protocol` | No | When set to `ccp.v1.json`, the runtime enters the `CcpJson` subprotocol mode. Otherwise it falls back to the legacy JSON frame mode. |

### Response `101`

| Output | Type | Description |
| --- | --- | --- |
| `Upgrade` | `header` | Returned as `websocket` when the handshake succeeds. |
| `Connection` | `header` | Returned as `Upgrade` for the switching-protocols handshake. |
| `Sec-WebSocket-Accept` | `header` | Standard RFC 6455 handshake proof derived from the client key. |
| `Sec-WebSocket-Protocol` | `header \| null` | Echoed when the server accepts a negotiated subprotocol such as `ccp.v1.json`. |

### Response Notes

- Status code is `101 Switching Protocols`.
- After the handshake, the connection leaves the request-response lifecycle and enters realtime transport mode.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `device_permission_denied` | The device is not authorized for the current principal. |
| `409` | `disconnect_fence_conflict` | Routing or session state blocks the upgrade. |

</section>
