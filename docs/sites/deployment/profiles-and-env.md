# Profiles and Environment

Sdkwork IM development and production routing are owned by topology v2. Use
`specs/topology.spec.json` and `configs/topology/*.env` as the only profile authority.

## Development Profiles

| Profile id | Command | Purpose |
| --- | --- | --- |
| `standalone.unified-process.development` | `pnpm dev`, `pnpm dev:browser`, `pnpm dev:desktop` | Default PostgreSQL standalone development stack |
| `self-hosted.unified-process.development` | `pnpm dev:browser:postgres:unified-process:standalone` | Current topology v2 profile-file mapping |
| `self-hosted.split-services.production` | private install templates | On-prem production bind + URL contract |
| `cloud-hosted.split-services.production` | `pnpm build` | SaaS production (`im.sdkwork.com`, `api.sdkwork.com`) |

See [Production Domain Binding](/deployment/production-domain-binding) for public URL keys.

## Application and Platform Surfaces

| Surface | Server env | Client env |
| --- | --- | --- |
| IM HTTP | `SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL` | `VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL` |
| IM WebSocket | `SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL` | `VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL` |
| Platform gateway | `SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` | `VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` |
| Ingress bind | `SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND` | — |

## Server-Only Dev

`pnpm dev:server` starts `scripts/im-server-dev.mjs`, which runs `sdkwork-im-server` with the
default split-services development profile env and managed `sdkwork-api-gateway`.

## Packaged Server Deployment

Production server installs use:

- `deployments/templates/server.env.example`
- `deployments/templates/chat.toml.example`
- `/etc/sdkwork/chat/server.env`

Do not use retired `local-minimal` / `local-default` profile names or `.runtime/local-*` config trees.

## Authentication Boundary

`sdkwork-appbase` owns login, IAM sessions, dual-token validation, users, tenants, organizations,
and the authoritative IAM context. Public clients send `Authorization: Bearer <auth-token>` and
`Access-Token: <access-token>` only.

For trusted gateway or service-to-service traffic, the gateway validates appbase dual tokens,
drops any client-supplied identity projection, and signs the private forwarded AppContext projection
with `SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET`. Protected service routes should run with
`SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE=true`.

## Security Hardening Variables

| Variable | Purpose |
| --- | --- |
| `SDKWORK_IM_BROWSER_ORIGINS` | Comma-separated explicit browser origins allowed to call the public app routes. |
| `SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE` | Set to `true` so standalone services reject unsigned AppContext projection headers. |
| `SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET` | Non-public HMAC secret shared only between the trusted gateway and internal services. |
