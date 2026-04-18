# `POST /api/v1/automation/executions`

<p class="api-page-intro">
  Exact request and response contract for <strong>Automation</strong> in the <strong>Platform API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/platform/automation"><code>Automation</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/platform-api"><code>Platform API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Shared bearer, trusted-header, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/automation/executions</code>
  <span class="api-op-id">operationId: requestAutomationExecution</span>
</div>

Requests a new automation execution.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token or trusted headers</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / automation</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`automation.execute`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 AutomationExecution`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="RequestAutomationExecution" />

### Response `200`

<ApiSchemaTable schema="AutomationExecution" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The automation execution request is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `automation.execute`. |
| `409` | `automation_execution_conflict` | The execution id conflicts with an existing request. |
| `503` | `automation_store_unavailable`, `journal_unavailable` | Automation persistence is unavailable. |

</section>
