# Authentication and Errors

<p class="api-page-intro">
  This page defines the security model and error envelope shared across the Craw Chat HTTP APIs.
  Individual operation pages document only endpoint-specific authorization requirements or conflict
  conditions on top of these defaults.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/portal-and-auth"><code>App</code> Portal bearer-auth flows and portal-session reads</a>
  <a href="/api-reference/app-api"><code>App</code> User-facing runtime domains and their operation groups</a>
  <a href="/api-reference/control-plane-api"><code>Control Plane</code> Administrative endpoints that use the same bearer model with <code>control.read</code> and <code>control.write</code></a>
</div>

## How To Use This Page

Use this page for the shared contract first:

1. Start here when you need bearer-token rules, trusted-header semantics, or the common error
   envelope.
2. Switch to operation pages when your next question is endpoint-specific permission, conflict, or
   resource-not-found behavior.
3. Switch to the SDK docs only when your next question is language surface, package naming, or
   publication state.

## Security Schemes

### `BearerAuth`

Public deployments of `craw-chat-server` / `web-gateway`, `local-minimal-node`, and
`control-plane-api` accept bearer tokens on non-health endpoints. In the packaged server flow, the
unified gateway fronts the public HTTP bind and forwards requests to the same bearer-authenticated
app and governance surfaces documented here.

| Item | Value |
| --- | --- |
| Header | `Authorization: Bearer <jwt>` |
| Signing secret | `CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET` |
| Algorithm | `HS256` |
| Resolver | `resolve_public_bearer_auth_context()` |

Supported claim aliases:

| Semantic field | Accepted claim names |
| --- | --- |
| Tenant | `tenant_id`, `tenantId` |
| Actor ID | `sub`, `actor_id`, `actorId`, `user_id`, `userId` |
| Actor kind | `actor_kind`, `actorKind`, `principal_type`, `principalType` |
| Session ID | `sid`, `session_id`, `sessionId` |
| Device ID | `did`, `device_id`, `deviceId` |
| Permissions | `permissions`, `perms`, `scope`, `scp` |

#### Temporal claim validation

Public bearer verification also validates temporal claims with a `60s` clock-skew allowance.

- `nbf` must not be in the future (beyond skew).
- `exp` must not be expired.
- `iat` must not be in the future (beyond skew).
- When `CRAW_CHAT_PUBLIC_BEARER_REQUIRE_EXP` is enabled, `exp` is required.
- When `CRAW_CHAT_PUBLIC_BEARER_MAX_TTL_SECONDS` is set to a positive value, token TTL cannot
  exceed that maximum.
- When `CRAW_CHAT_PUBLIC_BEARER_REQUIRED_ISS` is non-empty, token `iss` must match exactly.
- When `CRAW_CHAT_PUBLIC_BEARER_REQUIRED_AUD` is non-empty, token `aud` must include that value
  (string or array form).

These controls harden public HTTP surfaces against replay windows that are too large for commercial
traffic.

### `TrustedHeaders`

Trusted headers are intended for internal service-to-service wiring, tests, and explicitly trusted
networks. They are not part of the public SDK contract.

| Header | Meaning |
| --- | --- |
| `x-tenant-id` | Tenant identifier |
| `x-actor-id`, `x-user-id` | Actor identifier |
| `x-actor-kind` | Actor kind. Defaults to `user` when omitted in trusted flows |
| `x-session-id` | Session identifier |
| `x-device-id` | Device identifier |
| `x-permissions`, `x-scope`, `x-scopes` | Permission set |

## Permission Model

### Control Plane permissions

| Permission | Grants |
| --- | --- |
| `control.read` | Read protocol registry, governance state, provider registry, bindings, and policy history |
| `control.write` | Mutate provider policies and execute node lifecycle operations |

### Platform and operator permissions

| Permission | Grants |
| --- | --- |
| `audit.read` | Read audit records and export bundles |
| `audit.write` | Write audit anchors |
| `ops.read` | Read operator health, cluster, lag, and diagnostics endpoints |
| `device.telemetry.read` | Read device telemetry streams |
| `device.command.send` | Send device protocol downlinks or commands |
| `conversation.shared_channel.sync` | Execute shared-channel linked-member sync. Reserved for system actor `control-plane-sync`. |

## Security-Specific Error Codes

| HTTP | `code` | When it appears |
| --- | --- | --- |
| `401` | `jwt_exp_required` | `exp` is missing while `CRAW_CHAT_PUBLIC_BEARER_REQUIRE_EXP` or `CRAW_CHAT_PUBLIC_BEARER_MAX_TTL_SECONDS` requires it. |
| `401` | `jwt_ttl_exceeded` | Public bearer lifetime exceeds `CRAW_CHAT_PUBLIC_BEARER_MAX_TTL_SECONDS`. |
| `401` | `jwt_issuer_invalid` | Public bearer `iss` does not match `CRAW_CHAT_PUBLIC_BEARER_REQUIRED_ISS`. |
| `401` | `jwt_audience_invalid` | Public bearer `aud` does not satisfy `CRAW_CHAT_PUBLIC_BEARER_REQUIRED_AUD`. |
| `401` | `jwt_not_yet_valid`, `jwt_expired`, `jwt_issued_at_invalid`, `jwt_temporal_claim_invalid` | Temporal claims are invalid for current wall-clock time. |
| `403` | `shared_channel_sync_permission_denied` | Missing permission `conversation.shared_channel.sync` for shared-channel sync endpoint. |
| `403` | `shared_channel_sync_actor_invalid` | Caller actor is not the system actor `control-plane-sync`. |
| `429` | `shared_channel_sync_rate_limited` | Shared-channel sync exceeded per-tenant rate limit window. |

## Error Envelope

### App, Platform, and IoT APIs

The application-facing APIs return a compact error object:

```json
{
  "code": "conversation_not_found",
  "message": "conversation summary not found: conv_demo_001"
}
```

<ApiSchemaTable schema="ApiError" />

### Control Plane APIs

The control plane adds a `status` discriminator that mirrors the control-plane error category:

```json
{
  "status": "forbidden",
  "code": "permission_denied",
  "message": "missing required permission: control.write"
}
```

`status` can be one of:

- `unauthorized`
- `forbidden`
- `invalid`
- `conflict`
- `not_found`
- `unavailable`

## Common HTTP Statuses

| HTTP | Meaning |
| --- | --- |
| `200` | Success |
| `400` | Validation failure, malformed request body, or invalid query value |
| `401` | Missing or invalid bearer token, or incomplete trusted header set |
| `403` | Permission denied or principal-to-resource binding violation |
| `404` | Referenced conversation, message, media asset, node, or policy version does not exist |
| `409` | Version conflict, membership conflict, route migration conflict, or invalid lifecycle transition |
| `501` | Requested provider capability is not implemented |
| `503` | Provider, registry, or runtime dependency is unavailable |

## Client Guidance

1. Branch on `code` for application handling. Do not depend on the exact wording of `message`.
2. For control-plane clients, use both the HTTP status code and the response `status` field.
3. Treat operation pages as the source for endpoint-specific conflicts and resource-not-found cases.
4. Use the SDK pages as the source for language-surface questions, and this page as the source for
   shared auth and error semantics.

## What To Read Next

- [App API Overview](/api-reference/app-api)
- [Control Plane API Overview](/api-reference/control-plane-api)
- [SDK Overview](/sdk/index)
