# `GET /api/v1/notifications/{notification_id}`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Notifications</strong> in the <strong>Platform API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/platform/notifications">Back to Notifications</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/notifications/{notification_id}</code>
  <span class="api-op-id">operationId: getNotification</span>
</div>

Reads a single notification task by identifier.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token or trusted headers</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / notifications</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Current recipient scope.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 NotificationTask`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `notification_id` | `string` | Yes | Notification task identifier. |

### Response `200`

<ApiSchemaTable schema="NotificationTask" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller is not allowed to read the target notification scope. |
| `404` | `notification_not_found` | The requested notification task does not exist. |

</section>
