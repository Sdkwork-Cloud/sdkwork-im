# Control Plane Protocol Governance

<p class="api-page-intro">
  Protocol governance endpoints expose the control-plane health probe, protocol registry inventory,
  and the effective governance snapshot applied to compatible clients.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane-api"><code>Control Plane</code> Back to Control Plane overview</a>
  <a href="/sdk/admin-sdk"><code>Admin SDK</code> See how `sdk.meta` and `sdk.protocol` map to this page</a>
</div>

<a id="get-control-healthz"></a>
<section class="api-op">

## `GET /healthz`

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

<a id="get-protocol-registry"></a>
<section class="api-op">

## `GET /api/v1/control/protocol-registry`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/protocol-registry</code>
  <span class="api-op-id">operationId: getProtocolRegistry</span>
</div>

Returns the active protocol registry snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / protocol-governance</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProtocolRegistryResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ProtocolRegistryResponse" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | Query or path parameters are invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `503` | `*_unavailable` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="get-protocol-governance"></a>
<section class="api-op">

## `GET /api/v1/control/protocol-governance`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/protocol-governance</code>
  <span class="api-op-id">operationId: getProtocolGovernance</span>
</div>

Returns the effective control-plane governance snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / protocol-governance</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProtocolGovernanceResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ProtocolGovernanceResponse" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | Query or path parameters are invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `503` | `*_unavailable` | The governance snapshot or provider runtime is unavailable. |

</section>
