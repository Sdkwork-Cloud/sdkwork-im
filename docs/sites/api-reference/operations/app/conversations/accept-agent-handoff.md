# `POST /api/v1/conversations/{conversation_id}/agent-handoff/accept`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Conversations and Handoff</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/conversations">Back to Conversations and Handoff</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/conversations/{conversation_id}/agent-handoff/accept</code>
  <span class="api-op-id">operationId: acceptAgentHandoff</span>
</div>

Accepts the handoff from the target side. No JSON request body is required.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / conversations</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Active conversation member with `handoff.accept` authority.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 AgentHandoffStateView`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `conversation_id` | `string` | Yes | Conversation identifier. |

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

<ApiSchemaTable schema="AgentHandoffStateView" />


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
