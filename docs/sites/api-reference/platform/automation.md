# Automation

<p class="api-page-intro">
  Automation endpoints request asynchronous executions and inspect execution state after the job has
  been accepted by the runtime.
</p>

<a id="request-automation-execution"></a>
<section class="api-op">

## `POST /api/v1/automation/executions`

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
<a id="get-automation-execution"></a>
<section class="api-op">

## `GET /api/v1/automation/executions/{execution_id}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/automation/executions/{execution_id}</code>
  <span class="api-op-id">operationId: getAutomationExecution</span>
</div>

Reads an automation execution by identifier.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token or trusted headers</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / automation</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`automation.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 AutomationExecution`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `execution_id` | `string` | Yes | Automation execution identifier. |

### Response `200`

<ApiSchemaTable schema="AutomationExecution" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `automation.read`. |
| `404` | `automation_execution_not_found` | The requested automation execution does not exist. |
| `503` | `automation_store_unavailable` | Automation persistence is unavailable. |

</section>
