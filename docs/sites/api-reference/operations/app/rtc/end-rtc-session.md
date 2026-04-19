# `POST /api/v1/rtc/sessions/{rtc_session_id}/end`

<p class="api-page-intro">
  Exact request and response contract for <strong>RTC</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/rtc"><code>RTC</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Shared bearer, trusted-header, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/rtc/sessions/{rtc_session_id}/end</code>
  <span class="api-op-id">operationId: endRtcSession</span>
</div>

Ends the RTC session.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.rtc`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `rtc.end` capability.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RtcSession`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `rtc_session_id` | `string` | Yes | RTC session identifier. |

### Request Body

<ApiSchemaTable schema="UpdateRtcSessionRequest" />

### Response `200`

<ApiSchemaTable schema="RtcSession" />


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
