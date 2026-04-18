# `POST /api/v1/control/social/shared-channel-policies`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Social Graph Control</strong> in the <strong>Control Plane API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane/social">Back to Social Graph Control</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/social/shared-channel-policies</code>
  <span class="api-op-id">operationId: applySharedChannelPolicy</span>
</div>

Apply a shared-channel policy.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-graph</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialSharedChannelPolicyCommitResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="ApplySharedChannelPolicyRequest" />

### Response `200`

`SocialSharedChannelPolicyCommitResponse` is currently modeled as an open-ended social commit
payload in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The shared-channel policy payload is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | A referenced policy, channel, or connection aggregate does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the mutation. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>
