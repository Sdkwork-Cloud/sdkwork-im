# `POST /api/v1/control/provider-bindings`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Provider Governance</strong> in the <strong>Control Plane API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane/providers">Back to Provider Governance</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/provider-bindings</code>
  <span class="api-op-id">operationId: upsertProviderBindingPolicy</span>
</div>

Writes a deployment-level or tenant-level provider binding policy entry.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / provider-governance</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderBindingCommitResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="UpsertProviderBindingPolicyRequest" />

### Response `200`

<ApiSchemaTable schema="ProviderBindingCommitResponse" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `409` | `provider_policy_version_conflict` | `expectedBaseVersion` does not match the latest policy version. |
| `404` | `provider_plugin_not_found` | The referenced plugin is not present in the registry. |

</section>
