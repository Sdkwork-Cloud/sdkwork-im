# `POST /app/v3/api/rtc/provider_callbacks`

<p class="api-page-intro">
  Exact request and response contract for <strong>Provider Health</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/provider-health"><code>Provider Health</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, AppContext projection, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/app/v3/api/rtc/provider_callbacks</code>
  <span class="api-op-id">operationId: mapRtcProviderCallback</span>
</div>

Maps a provider callback into an internal RTC domain event.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / provider health</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RtcCallbackEvent`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="RtcCallbackRequest" />

### Response `200`

<ApiSchemaTable schema="RtcCallbackEvent" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `503` | `*_unavailable` | The provider health source is unavailable. |

</section>
