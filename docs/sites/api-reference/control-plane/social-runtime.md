# Control Plane Social Runtime

<p class="api-page-intro">
  Social runtime endpoints back <code>sdk.socialRuntime</code> in the admin SDKs. They expose
  pending, delivered, and dead-letter shared-channel sync inventories, plus operational controls for
  claim, release, republish, repair, reclaim, requeue, and targeted takeover flows.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane-api"><code>Control Plane</code> Back to Control Plane overview</a>
  <a href="/sdk/admin-sdk"><code>Admin SDK</code> See the cross-language client surface</a>
</div>

The checked-in admin control-plane authority intentionally leaves current runtime repair and
inventory responses open-ended. Queue-control inputs, route semantics, and permissions are stable;
response bodies should be treated as opaque JSON and consumed through the generated admin SDK
surfaces.

<a id="get-pending-shared-channel-sync-inventory"></a>
<section class="api-op">

## `GET /api/v1/control/social/runtime/pending-shared-channel-sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/social/runtime/pending-shared-channel-sync</code>
  <span class="api-op-id">operationId: getPendingSharedChannelSyncInventory</span>
</div>

Read the pending shared-channel sync queue.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-runtime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialSharedChannelSyncPendingInventoryResponse`</span></div>
</div>

### Response `200`

`SocialSharedChannelSyncPendingInventoryResponse` is currently modeled as an open-ended runtime
inventory payload in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The queue read request is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested inventory source does not exist. |
| `409` | `*_conflict` | Current runtime state blocks the read. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="get-delivery-state-shared-channel-sync-inventory"></a>
<section class="api-op">

## `GET /api/v1/control/social/runtime/delivery-state-shared-channel-sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/social/runtime/delivery-state-shared-channel-sync</code>
  <span class="api-op-id">operationId: getSharedChannelSyncDeliveryStateInventory</span>
</div>

Read merged shared-channel sync delivery state.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-runtime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialSharedChannelSyncDeliveryStateInventoryResponse`</span></div>
</div>

### Response `200`

`SocialSharedChannelSyncDeliveryStateInventoryResponse` is currently modeled as an open-ended
runtime inventory payload in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The queue read request is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested inventory source does not exist. |
| `409` | `*_conflict` | Current runtime state blocks the read. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="get-delivered-shared-channel-sync-inventory"></a>
<section class="api-op">

## `GET /api/v1/control/social/runtime/delivered-shared-channel-sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/social/runtime/delivered-shared-channel-sync</code>
  <span class="api-op-id">operationId: getDeliveredSharedChannelSyncInventory</span>
</div>

Read the delivered shared-channel sync ledger.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-runtime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialSharedChannelSyncDeliveredInventoryResponse`</span></div>
</div>

### Response `200`

`SocialSharedChannelSyncDeliveredInventoryResponse` is currently modeled as an open-ended runtime
inventory payload in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The queue read request is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested inventory source does not exist. |
| `409` | `*_conflict` | Current runtime state blocks the read. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="get-dead-letter-shared-channel-sync-inventory"></a>
<section class="api-op">

## `GET /api/v1/control/social/runtime/dead-letter-shared-channel-sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/social/runtime/dead-letter-shared-channel-sync</code>
  <span class="api-op-id">operationId: getDeadLetterSharedChannelSyncInventory</span>
</div>

Read the dead-letter shared-channel sync queue.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-runtime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialSharedChannelSyncDeadLetterInventoryResponse`</span></div>
</div>

### Response `200`

`SocialSharedChannelSyncDeadLetterInventoryResponse` is currently modeled as an open-ended runtime
inventory payload in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The queue read request is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested inventory source does not exist. |
| `409` | `*_conflict` | Current runtime state blocks the read. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="requeue-dead-letter-shared-channel-sync"></a>
<section class="api-op">

## `POST /api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync</code>
  <span class="api-op-id">operationId: requeueDeadLetterSharedChannelSync</span>
</div>

Requeue all dead-letter shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-runtime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialSharedChannelSyncDeadLetterRequeueResponse`</span></div>
</div>

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

`SocialSharedChannelSyncDeadLetterRequeueResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The requeue request is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | The requested dead-letter queue does not exist. |
| `409` | `*_conflict` | Current runtime state blocks the requeue. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="requeue-dead-letter-shared-channel-sync-targeted"></a>
<section class="api-op">

## `POST /api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync-targeted`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync-targeted</code>
  <span class="api-op-id">operationId: requeueDeadLetterSharedChannelSyncTargeted</span>
</div>

Requeue selected dead-letter shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-runtime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialSharedChannelSyncDeadLetterTargetedRequeueResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SocialSharedChannelSyncDeadLetterTargetedRequeueRequest" />

### Response `200`

`SocialSharedChannelSyncDeadLetterTargetedRequeueResponse` is currently modeled as an open-ended
runtime operation payload in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The targeted requeue payload is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | One or more targeted request keys do not exist. |
| `409` | `*_conflict` | Current runtime state blocks the requeue. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="repair-social-runtime-snapshot"></a>
<section class="api-op">

## `POST /api/v1/control/social/runtime/repair-derived-snapshot`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/social/runtime/repair-derived-snapshot</code>
  <span class="api-op-id">operationId: repairSocialRuntimeSnapshot</span>
</div>

Repair the persisted social runtime derived snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-runtime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialRuntimeRepairResponse`</span></div>
</div>

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

`SocialRuntimeRepairResponse` is currently modeled as an open-ended runtime repair payload in the
checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The repair request is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | A referenced runtime snapshot or queue does not exist. |
| `409` | `*_conflict` | Current runtime state blocks the repair. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="repair-shared-channel-sync"></a>
<section class="api-op">

## `POST /api/v1/control/social/runtime/repair-shared-channel-sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/social/runtime/repair-shared-channel-sync</code>
  <span class="api-op-id">operationId: repairSharedChannelSync</span>
</div>

Repair shared-channel sync backlog state.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-runtime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialSharedChannelSyncRepairResponse`</span></div>
</div>

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

`SocialSharedChannelSyncRepairResponse` is currently modeled as an open-ended runtime repair
payload in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The repair request is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | A referenced shared-channel sync backlog or queue does not exist. |
| `409` | `*_conflict` | Current runtime state blocks the repair. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="claim-pending-shared-channel-sync-targeted"></a>
<section class="api-op">

## `POST /api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted</code>
  <span class="api-op-id">operationId: claimPendingSharedChannelSyncTargeted</span>
</div>

Claim selected pending shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-runtime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialSharedChannelSyncPendingClaimResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SocialSharedChannelSyncPendingTargetedClaimRequest" />

### Response `200`

`SocialSharedChannelSyncPendingClaimResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The targeted claim payload is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | One or more targeted request keys do not exist. |
| `409` | `*_conflict` | Current runtime ownership blocks the claim. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="release-pending-shared-channel-sync-targeted"></a>
<section class="api-op">

## `POST /api/v1/control/social/runtime/release-pending-shared-channel-sync-targeted`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/social/runtime/release-pending-shared-channel-sync-targeted</code>
  <span class="api-op-id">operationId: releasePendingSharedChannelSyncTargeted</span>
</div>

Release selected pending shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-runtime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialSharedChannelSyncPendingReleaseResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SocialSharedChannelSyncPendingTargetedReleaseRequest" />

### Response `200`

`SocialSharedChannelSyncPendingReleaseResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The targeted release payload is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | One or more targeted request keys do not exist. |
| `409` | `*_conflict` | Current runtime ownership blocks the release. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="reclaim-stale-pending-shared-channel-sync"></a>
<section class="api-op">

## `POST /api/v1/control/social/runtime/reclaim-stale-pending-shared-channel-sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/social/runtime/reclaim-stale-pending-shared-channel-sync</code>
  <span class="api-op-id">operationId: reclaimStalePendingSharedChannelSync</span>
</div>

Reclaim stale shared-channel sync pending ownership.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-runtime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialSharedChannelSyncPendingStaleReclaimResponse`</span></div>
</div>

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

`SocialSharedChannelSyncPendingStaleReclaimResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The reclaim request is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | The requested queue or ownership records do not exist. |
| `409` | `*_conflict` | Current runtime ownership blocks the reclaim. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="republish-pending-shared-channel-sync-targeted"></a>
<section class="api-op">

## `POST /api/v1/control/social/runtime/republish-pending-shared-channel-sync-targeted`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/social/runtime/republish-pending-shared-channel-sync-targeted</code>
  <span class="api-op-id">operationId: republishPendingSharedChannelSyncTargeted</span>
</div>

Republish selected pending shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-runtime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialSharedChannelSyncTargetedRepublishResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SocialSharedChannelSyncTargetedRepublishRequest" />

### Response `200`

`SocialSharedChannelSyncTargetedRepublishResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The targeted republish payload is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | One or more targeted request keys do not exist. |
| `409` | `*_conflict` | Current runtime ownership blocks the republish. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="takeover-pending-shared-channel-sync-targeted"></a>
<section class="api-op">

## `POST /api/v1/control/social/runtime/takeover-pending-shared-channel-sync-targeted`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/social/runtime/takeover-pending-shared-channel-sync-targeted</code>
  <span class="api-op-id">operationId: takeoverPendingSharedChannelSyncTargeted</span>
</div>

Take over selected pending shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-runtime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialSharedChannelSyncPendingTakeoverResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SocialSharedChannelSyncPendingTargetedTakeoverRequest" />

### Response `200`

`SocialSharedChannelSyncPendingTakeoverResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The targeted takeover payload is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | One or more targeted request keys do not exist. |
| `409` | `*_conflict` | Current runtime ownership blocks the takeover. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
