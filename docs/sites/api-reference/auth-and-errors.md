# Authentication and Errors

<p class="api-page-intro">
  This page defines the security model and error envelope shared across the Craw Chat HTTP APIs.
  Individual operation pages document only endpoint-specific authorization requirements or conflict
  conditions on top of these defaults.
</p>

## Security Schemes

### `BearerAuth`

Public deployments of `local-minimal-node` and `control-plane-api` accept bearer tokens on
non-health endpoints.

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

### `TrustedHeaders`

Trusted headers are intended for internal service-to-service wiring, tests, and explicitly trusted
networks. They are not the public SDK contract.

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
