# `GET /api/v1/conversations/{conversation_id}`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Conversations and Handoff</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/conversations">Back to Conversations and Handoff</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/conversations/{conversation_id}</code>
  <span class="api-op-id">operationId: getConversationSummary</span>
</div>

Reads the conversation summary projection.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token or trusted headers</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / conversations</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Active conversation member.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ConversationSummaryView`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `conversation_id` | `string` | Yes | Conversation identifier. |

### Response `200`

<ApiSchemaTable schema="ConversationSummaryView" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `404` | `conversation_summary_not_found` | The conversation summary is not available. |

</section>
