# `GET /healthz`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>Protocol Governance</strong> in the <strong>Control Plane API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane/protocol">Back to Protocol Governance</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/healthz</code>
  <span class="api-op-id">operationId: getControlPlaneHealthz</span>
</div>

Returns the liveness state of the control-plane process.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Open endpoint</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / protocol-governance</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Not required</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ControlPlaneHealthResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ControlPlaneHealthResponse" />

</section>
