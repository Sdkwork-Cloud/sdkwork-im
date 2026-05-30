# `GET /im/v3/api/rtc/sessions/{rtcSessionId}/artifacts/recording`

<p class="api-page-intro">
  Exact request and response contract for <strong>RTC Signaling</strong> in the <strong>IM Standard API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/im/rtc"><code>RTC Signaling</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/im-api"><code>IM Standard API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, AppContext projection, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/im/v3/api/rtc/sessions/{rtcSessionId}/artifacts/recording</code>
  <span class="api-op-id">operationId: getRtcRecordingArtifact</span>
</div>

Returns recording artifact metadata when available.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.rtc`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `rtc.artifact` capability.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RtcRecordingArtifact`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `rtc_session_id` | `string` | Yes | RTC session identifier. |

### Response `200`

<ApiSchemaTable schema="RtcRecordingArtifact" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to access the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the read or handshake flow. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
