# `GET /api/v1/realtime/ws`

<p class="api-page-intro">
  Exact request and response contract for <strong>Session and Realtime</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/session-and-realtime"><code>Session and Realtime</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Shared bearer, trusted-header, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/api/v1/realtime/ws</code>
  <span class="api-op-id">operationId: realtimeWebsocket</span>
</div>

Upgrades the connection to WebSocket. This page documents the HTTP handshake surface only; it does
not expand the full realtime frame protocol.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Bearer token</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.connect(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; active device route is prepared before upgrade.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`101 Switching Protocols`</span></div>
</div>

### Security

- Bearer token or trusted headers
- Device ownership and session binding are validated before upgrade

### Headers

| Header | Required | Description |
| --- | --- | --- |
| `Sec-WebSocket-Protocol` | No | When set to `ccp/ws/1`, the runtime enters the `CcpJson` subprotocol mode. Otherwise it falls back to the legacy JSON frame mode. |

### Response `101`

| Output | Type | Description |
| --- | --- | --- |
| `Upgrade` | `header` | Returned as `websocket` when the handshake succeeds. |
| `Connection` | `header` | Returned as `Upgrade` for the switching-protocols handshake. |
| `Sec-WebSocket-Accept` | `header` | Standard RFC 6455 handshake proof derived from the client key. |
| `Sec-WebSocket-Protocol` | `header \| null` | Echoed when the server accepts a negotiated subprotocol such as `ccp/ws/1`. |

### Response Notes

- Status code is `101 Switching Protocols`.
- After the handshake, the connection leaves the request-response lifecycle and enters realtime transport mode.
- For TypeScript consumers, the standard SDK entrypoint for that transport is `sdk.connect(...)`.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `missing_authorization`, `invalid_token` | Authentication failed. |
| `403` | `device_permission_denied` | The device is not authorized for the current principal. |
| `409` | `disconnect_fence_conflict` | Routing or session state blocks the upgrade. |

</section>
