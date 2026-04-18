# `GET /api/v1/iot/access/provider-health`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Protocol and Health</strong> in the <strong>IoT API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/iot/protocol-and-health">Back to Protocol and Health</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/iot/access/provider-health</code>
  <span class="api-op-id">operationId: getIotAccessProviderHealth</span>
</div>

Returns the IoT access provider health snapshot.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>No standalone published SDK family</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderHealthSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ProviderHealthSnapshot" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `503` | `*_unavailable` | The provider health source is unavailable. |

</section>
