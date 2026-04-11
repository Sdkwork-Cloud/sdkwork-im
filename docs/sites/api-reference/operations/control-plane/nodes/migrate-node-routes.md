# `POST /api/v1/control/nodes/{node_id}/routes/migrate`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Node Operations</strong> in the <strong>Control Plane API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane/nodes">Back to Node Operations</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/nodes/{node_id}/routes/migrate</code>
  <span class="api-op-id">operationId: migrateNodeRoutes</span>
</div>

Migrates owned realtime routes from the source node to a target node.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / node-operations</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RouteMigrationResult`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `node_id` | `string` | Yes | Source node identifier. |

### Request Body

<ApiSchemaTable schema="MigrateRoutesRequest" />

### Response `200`

<ApiSchemaTable schema="RouteMigrationResult" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `invalid_provider_policy` | The mutation payload is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found`, `provider_plugin_not_found` | The requested node, plugin, or target resource does not exist. |
| `409` | `*_conflict`, `provider_policy_conflict` | Current control-plane state blocks the mutation. |
| `503` | `*_unavailable` | The governance snapshot or provider runtime is unavailable. |

</section>
