# `GET /api/v1/conversations/{conversation_id}`

<p class="api-page-intro">
  Exact request and response contract for <strong>Conversations and Handoff</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/conversations"><code>Conversations and Handoff</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Shared bearer, trusted-header, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/conversations/{conversation_id}</code>
  <span class="api-op-id">operationId: getConversationSummary</span>
</div>

Reads the conversation summary projection.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.conversations`</span></div>
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
