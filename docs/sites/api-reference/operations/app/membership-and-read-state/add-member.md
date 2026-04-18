# `POST /api/v1/conversations/{conversation_id}/members/add`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Membership and Read State</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/membership-and-read-state">Back to Membership and Read State</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/conversations/{conversation_id}/members/add</code>
  <span class="api-op-id">operationId: addMember</span>
</div>

Adds a principal to the conversation roster.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / membership</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation-bound write access.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ConversationMember`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `conversation_id` | `string` | Yes | Conversation identifier. |

### Request Body

<ApiSchemaTable schema="AddConversationMemberRequest" />

### Response `200`

<ApiSchemaTable schema="ConversationMember" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `403` | `conversation_permission_denied` | The caller cannot add members to this conversation. |
| `409` | `member_already_exists` | The target principal is already present in the conversation. |

</section>
