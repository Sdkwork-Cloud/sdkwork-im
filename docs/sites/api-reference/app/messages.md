# Messages

<p class="api-page-intro">
  Message endpoints expose timeline reads, regular message submission, system-channel publish, and
  message mutation operations such as edit and recall.
</p>

<a id="post-message"></a>
<section class="api-op">

## `POST /api/v1/conversations/{conversation_id}/messages`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/conversations/{conversation_id}/messages</code>
  <span class="api-op-id">operationId: postMessage</span>
</div>

Posts a regular conversation message.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / messages</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation-bound write access.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PostMessageResult`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `conversation_id` | `string` | Yes | Conversation identifier. |

### Request Body

<ApiSchemaTable schema="PostMessageRequest" />

### Response `200`

<ApiSchemaTable schema="PostMessageResult" />

### Example Request

```json
{
  "clientMsgId": "msg-client-001",
  "summary": "Greeting",
  "text": "hello world"
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
<a id="get-timeline"></a>
<section class="api-op">

## `GET /api/v1/conversations/{conversation_id}/messages`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/conversations/{conversation_id}/messages</code>
  <span class="api-op-id">operationId: getTimeline</span>
</div>

Returns the projection-backed message timeline for the conversation.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / messages</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Active conversation member.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 TimelineListResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `conversation_id` | `string` | Yes | Conversation identifier. |

### Response `200`

<ApiSchemaTable schema="TimelineListResponse" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `conversation_permission_denied`, `device_permission_denied`, `permission_denied` | The caller is not allowed to access the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the read or handshake flow. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="publish-system-channel-message"></a>
<section class="api-op">

## `POST /api/v1/conversations/{conversation_id}/system-channel/publish`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/conversations/{conversation_id}/system-channel/publish</code>
  <span class="api-op-id">operationId: publishSystemChannelMessage</span>
</div>

Publishes a system message to the conversation's system channel.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / messages</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation-bound write access.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PostMessageResult`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `conversation_id` | `string` | Yes | Conversation identifier. |

### Request Body

Uses the same request schema as regular message submission.

<ApiSchemaTable schema="PostMessageRequest" />

### Response `200`

<ApiSchemaTable schema="PostMessageResult" />


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
<a id="edit-message"></a>
<section class="api-op">

## `POST /api/v1/messages/{message_id}/edit`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/messages/{message_id}/edit</code>
  <span class="api-op-id">operationId: editMessage</span>
</div>

Edits a previously posted message.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / messages</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation-bound write access.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 MessageMutationResult`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `message_id` | `string` | Yes | Message identifier. |

### Request Body

<ApiSchemaTable schema="EditMessageRequest" />

### Response `200`

<ApiSchemaTable schema="MessageMutationResult" />


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
<a id="recall-message"></a>
<section class="api-op">

## `POST /api/v1/messages/{message_id}/recall`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/messages/{message_id}/recall</code>
  <span class="api-op-id">operationId: recallMessage</span>
</div>

Recalls a message. This operation does not require a JSON request body.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / messages</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation-bound write access.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 MessageMutationResult`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `message_id` | `string` | Yes | Message identifier. |

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

<ApiSchemaTable schema="MessageMutationResult" />


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
