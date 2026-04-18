# `GET /api/v1/iot/protocol/provider-health`

<p class="api-page-intro">
  Exact request and response contract for <strong>Protocol and Health</strong> in the <strong>IoT API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/iot/protocol-and-health"><code>Protocol and Health</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/iot-api"><code>IoT API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Shared bearer, trusted-header, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/iot/protocol/provider-health</code>
  <span class="api-op-id">operationId: getIotProtocolProviderHealth</span>
</div>

Returns the IoT protocol adapter health snapshot.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token or trusted headers</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk` / iot</span></div>
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
