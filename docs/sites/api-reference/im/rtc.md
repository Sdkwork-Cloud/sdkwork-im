# RTC

<p class="api-page-intro">
  RTC endpoints manage session lifecycle, custom signaling, participant credential issuance,
  recording artifact lookup, and mapping of provider callbacks into platform events.
</p>

<div class="api-link-list">
  <a href="/api-reference/im/streams"><code>Streams</code> Stream transport and frame delivery are documented separately</a>
  <a href="/api-reference/app/provider-health"><code>App Provider Health</code> RTC provider health snapshots live under App API</a>
  <a href="/sdk/app-sdk"><code>SDK</code> <code>@sdkwork/im-sdk</code> and <code>im_sdk</code> both expose RTC helpers above these transport routes</a>
</div>

## Recommended SDK Mapping

- `sdk.rtc.create(...)` creates the RTC session
- `sdk.rtc.invite(...)`, `sdk.rtc.accept(...)`, `sdk.rtc.reject(...)`, and `sdk.rtc.end(...)` drive lifecycle transitions
- `sdk.rtc.postJsonSignal(...)` sends common JSON signaling payloads over the RTC signaling route
- `live.signals.onRtcSession(...)` receives inbound signaling events from the live runtime
- `sdk.rtc.issueParticipantCredential(...)` issues provider join credentials
- `sdk.rtc.getRecordingArtifact(...)` fetches recording metadata

Example:

```ts
const session = await sdk.rtc.create({
  rtcSessionId: 'rtc-1',
  conversationId: 'conversation-1',
  rtcMode: 'group_call',
});

await sdk.rtc.invite(session.rtcSessionId, {
  signalingStreamId: 'rtc-signal-1',
});

await sdk.rtc.postJsonSignal(session.rtcSessionId, 'offer', {
  signalingStreamId: 'rtc-signal-1',
  payload: {
    sdp: 'v=0...',
  },
});

await sdk.rtc.issueParticipantCredential(session.rtcSessionId, {
  participantId: 'user-1',
});

await sdk.rtc.getRecordingArtifact(session.rtcSessionId);

live.signals.onRtcSession(session.rtcSessionId, (signal, context) => {
  console.log(signal.signalType, signal.payload, context.scopeId);
});
```

<a id="create-rtc-session"></a>
<section class="api-op">

## `POST /im/v3/api/rtc/sessions`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/rtc/sessions</code>
  <span class="api-op-id">operationId: createRtcSession</span>
</div>

Creates a new RTC session.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.rtc`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `rtc.create` capability when the session is bound to a conversation.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RtcSession`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="CreateRtcSessionRequest" />

### Response `200`

<ApiSchemaTable schema="RtcSession" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="invite-rtc-session"></a>
<section class="api-op">

## `POST /im/v3/api/rtc/sessions/{rtcSessionId}/invite`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/rtc/sessions/{rtcSessionId}/invite</code>
  <span class="api-op-id">operationId: inviteRtcSession</span>
</div>

Starts the invitation phase for the RTC session.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.rtc`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `rtc.invite` capability.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RtcSession`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `rtc_session_id` | `string` | Yes | RTC session identifier. |

### Request Body

<ApiSchemaTable schema="InviteRtcSessionRequest" />

### Response `200`

<ApiSchemaTable schema="RtcSession" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="accept-rtc-session"></a>
<section class="api-op">

## `POST /im/v3/api/rtc/sessions/{rtcSessionId}/accept`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/rtc/sessions/{rtcSessionId}/accept</code>
  <span class="api-op-id">operationId: acceptRtcSession</span>
</div>

Accepts the RTC session.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.rtc`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `rtc.accept` capability.</span></div>
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
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="reject-rtc-session"></a>
<section class="api-op">

## `POST /im/v3/api/rtc/sessions/{rtcSessionId}/reject`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/rtc/sessions/{rtcSessionId}/reject</code>
  <span class="api-op-id">operationId: rejectRtcSession</span>
</div>

Rejects the RTC session.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.rtc`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `rtc.reject` capability.</span></div>
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
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="end-rtc-session"></a>
<section class="api-op">

## `POST /im/v3/api/rtc/sessions/{rtcSessionId}/end`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/rtc/sessions/{rtcSessionId}/end</code>
  <span class="api-op-id">operationId: endRtcSession</span>
</div>

Ends the RTC session.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
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
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="post-rtc-signal"></a>
<section class="api-op">

## `POST /im/v3/api/rtc/sessions/{rtcSessionId}/signals`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/rtc/sessions/{rtcSessionId}/signals</code>
  <span class="api-op-id">operationId: postRtcSignal</span>
</div>

Posts a custom RTC signal event.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.rtc`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `rtc.signal` capability.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RtcSignalEvent`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `rtc_session_id` | `string` | Yes | RTC session identifier. |

### Request Body

<ApiSchemaTable schema="PostRtcSignalRequest" />

### Response `200`

<ApiSchemaTable schema="RtcSignalEvent" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="issue-rtc-participant-credential"></a>
<section class="api-op">

## `POST /im/v3/api/rtc/sessions/{rtcSessionId}/credentials`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/rtc/sessions/{rtcSessionId}/credentials</code>
  <span class="api-op-id">operationId: issueRtcParticipantCredential</span>
</div>

Issues a provider credential for a participant.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.rtc`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `rtc.issue_credential` capability.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RtcParticipantCredential`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `rtc_session_id` | `string` | Yes | RTC session identifier. |

### Request Body

<ApiSchemaTable schema="IssueRtcParticipantCredentialRequest" />

### Response `200`

<ApiSchemaTable schema="RtcParticipantCredential" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="get-rtc-recording-artifact"></a>
<section class="api-op">

## `GET /im/v3/api/rtc/sessions/{rtcSessionId}/artifacts/recording`

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
