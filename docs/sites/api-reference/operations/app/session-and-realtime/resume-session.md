# `POST /api/v1/sessions/resume`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Session and Realtime</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/session-and-realtime">Back to Session and Realtime</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/sessions/resume</code>
  <span class="api-op-id">operationId: resumeSession</span>
</div>

Resumes the current device session and returns the active presence snapshot with replay cursors.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token or trusted headers</span></div>
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
