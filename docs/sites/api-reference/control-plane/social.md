# Control Plane Social Graph Control

<p class="api-page-intro">
  Social graph control endpoints back <code>sdk.social</code> in the admin SDKs. They let operators
  bind direct chats, establish external collaboration topology, manage friendship aggregates, apply
  shared-channel policies, and enforce user blocks.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane-api"><code>Control Plane</code> Back to Control Plane overview</a>
  <a href="/sdk/admin-sdk"><code>Admin SDK</code> See the cross-language client surface</a>
</div>

The checked-in admin control-plane authority keeps current social response payloads open-ended on
purpose. Mutation inputs, route semantics, and permission boundaries are stable; response bodies
should be treated as opaque JSON and consumed through the generated admin SDK surfaces.

<a id="bind-direct-chat"></a>
<section class="api-op">

## `POST /api/v1/control/social/direct-chats/bindings`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/social/direct-chats/bindings</code>
  <span class="api-op-id">operationId: bindDirectChat</span>
</div>

Bind a direct chat to a conversation.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-graph</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialDirectChatCommitResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="BindDirectChatRequest" />

### Response `200`

`SocialDirectChatCommitResponse` is currently modeled as an open-ended social commit payload in the
checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The direct-chat binding payload is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | The referenced direct chat or conversation aggregate does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the mutation. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>
<a id="submit-friend-request"></a>
<section class="api-op">

## `POST /api/v1/control/social/friend-requests`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/social/friend-requests</code>
  <span class="api-op-id">operationId: submitFriendRequest</span>
</div>

Submit a friend request event.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-graph</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialFriendRequestCommitResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SubmitFriendRequestRequest" />

### Response `200`

`SocialFriendRequestCommitResponse` is currently modeled as an open-ended social commit payload in
the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The friend-request payload is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | A referenced user or aggregate does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the mutation. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>
<a id="get-friend-request-snapshot"></a>
<section class="api-op">

## `GET /api/v1/control/social/friend-requests/{request_id}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/social/friend-requests/{request_id}</code>
  <span class="api-op-id">operationId: getFriendRequestSnapshot</span>
</div>

Read a friend request snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-graph</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialFriendRequestSnapshotResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `request_id` | `string` | Yes | Friend request aggregate identifier. |

### Response `200`

`SocialFriendRequestSnapshotResponse` is currently modeled as an open-ended social snapshot payload
in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The friend request identifier is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested friend request aggregate does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the read. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>
<a id="activate-friendship"></a>
<section class="api-op">

## `POST /api/v1/control/social/friendships`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/social/friendships</code>
  <span class="api-op-id">operationId: activateFriendship</span>
</div>

Activate a friendship event.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-graph</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialFriendshipCommitResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="ActivateFriendshipRequest" />

### Response `200`

`SocialFriendshipCommitResponse` is currently modeled as an open-ended social commit payload in the
checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The friendship activation payload is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | A referenced user, direct chat, or friendship aggregate does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the mutation. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>
<a id="get-friendship-snapshot"></a>
<section class="api-op">

## `GET /api/v1/control/social/friendships/{friendship_id}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/social/friendships/{friendship_id}</code>
  <span class="api-op-id">operationId: getFriendshipSnapshot</span>
</div>

Read a friendship snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-graph</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialFriendshipSnapshotResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `friendship_id` | `string` | Yes | Friendship aggregate identifier. |

### Response `200`

`SocialFriendshipSnapshotResponse` is currently modeled as an open-ended social snapshot payload in
the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The friendship identifier is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested friendship aggregate does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the read. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>
<a id="apply-shared-channel-policy"></a>
<section class="api-op">

## `POST /api/v1/control/social/shared-channel-policies`

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
<a id="get-shared-channel-policy-snapshot"></a>
<section class="api-op">

## `GET /api/v1/control/social/shared-channel-policies/{policy_id}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/social/shared-channel-policies/{policy_id}</code>
  <span class="api-op-id">operationId: getSharedChannelPolicySnapshot</span>
</div>

Read a shared-channel policy snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-graph</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialSharedChannelPolicySnapshotResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `policy_id` | `string` | Yes | Shared-channel policy aggregate identifier. |

### Response `200`

`SocialSharedChannelPolicySnapshotResponse` is currently modeled as an open-ended social snapshot
payload in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The shared-channel policy identifier is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested shared-channel policy aggregate does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the read. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>
<a id="block-user"></a>
<section class="api-op">

## `POST /api/v1/control/social/user-blocks`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/social/user-blocks</code>
  <span class="api-op-id">operationId: blockUser</span>
</div>

Block a user in the social graph.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-graph</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialUserBlockCommitResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="BlockUserRequest" />

### Response `200`

`SocialUserBlockCommitResponse` is currently modeled as an open-ended social commit payload in the
checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The user-block payload is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | A referenced user, direct chat, or block aggregate does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the mutation. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>
<a id="get-user-block-snapshot"></a>
<section class="api-op">

## `GET /api/v1/control/social/user-blocks/{block_id}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/social/user-blocks/{block_id}</code>
  <span class="api-op-id">operationId: getUserBlockSnapshot</span>
</div>

Read a user block snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-graph</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialUserBlockSnapshotResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `block_id` | `string` | Yes | User block aggregate identifier. |

### Response `200`

`SocialUserBlockSnapshotResponse` is currently modeled as an open-ended social snapshot payload in
the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The user block identifier is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested user block aggregate does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the read. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>
<a id="get-direct-chat-snapshot"></a>
<section class="api-op">

## `GET /api/v1/control/social/direct-chats/{direct_chat_id}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/social/direct-chats/{direct_chat_id}</code>
  <span class="api-op-id">operationId: getDirectChatSnapshot</span>
</div>

Read a direct chat snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-graph</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialDirectChatSnapshotResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `direct_chat_id` | `string` | Yes | Direct chat aggregate identifier. |

### Response `200`

`SocialDirectChatSnapshotResponse` is currently modeled as an open-ended social snapshot payload in
the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The direct chat identifier is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested direct chat aggregate does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the read. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>
<a id="establish-external-connection"></a>
<section class="api-op">

## `POST /api/v1/control/social/external-connections`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/social/external-connections</code>
  <span class="api-op-id">operationId: establishExternalConnection</span>
</div>

Establish an external collaboration connection.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-graph</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialExternalConnectionCommitResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="EstablishExternalConnectionRequest" />

### Response `200`

`SocialExternalConnectionCommitResponse` is currently modeled as an open-ended social commit payload
in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The external-connection payload is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | A referenced external tenant or aggregate does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the mutation. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>
<a id="get-external-connection-snapshot"></a>
<section class="api-op">

## `GET /api/v1/control/social/external-connections/{connection_id}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/social/external-connections/{connection_id}</code>
  <span class="api-op-id">operationId: getExternalConnectionSnapshot</span>
</div>

Read an external connection snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-graph</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialExternalConnectionSnapshotResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `connection_id` | `string` | Yes | External connection aggregate identifier. |

### Response `200`

`SocialExternalConnectionSnapshotResponse` is currently modeled as an open-ended social snapshot
payload in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The external connection identifier is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested external connection aggregate does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the read. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>
<a id="bind-external-member-link"></a>
<section class="api-op">

## `POST /api/v1/control/social/external-member-links`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/api/v1/control/social/external-member-links</code>
  <span class="api-op-id">operationId: bindExternalMemberLink</span>
</div>

Bind an external member link.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-graph</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialExternalMemberLinkCommitResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="BindExternalMemberLinkRequest" />

### Response `200`

`SocialExternalMemberLinkCommitResponse` is currently modeled as an open-ended social commit payload
in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The external-member-link payload is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | A referenced external connection or actor does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the mutation. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>
<a id="get-external-member-link-snapshot"></a>
<section class="api-op">

## `GET /api/v1/control/social/external-member-links/{link_id}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/control/social/external-member-links/{link_id}</code>
  <span class="api-op-id">operationId: getExternalMemberLinkSnapshot</span>
</div>

Read an external member link snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-craw-chat-sdk-admin` / social-graph</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SocialExternalMemberLinkSnapshotResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `link_id` | `string` | Yes | External member link aggregate identifier. |

### Response `200`

`SocialExternalMemberLinkSnapshotResponse` is currently modeled as an open-ended social snapshot
payload in the checked-in admin control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The external member link identifier is invalid. |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested external member link aggregate does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the read. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>
