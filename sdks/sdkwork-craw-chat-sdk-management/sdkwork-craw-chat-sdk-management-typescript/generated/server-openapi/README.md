# @sdkwork/craw-chat-management-backend-sdk

Generated TypeScript transport package for the Craw Chat operator-console management backend.

## Package Role

This package is the generator-owned transport layer for the checked-in management OpenAPI contract.
Use it when you need direct access to generated HTTP operations and root-exported transport types.

For business-facing management integrations, prefer the composed package
`@sdkwork/craw-chat-sdk-management`, which keeps the transport layer behind a stable management
client facade.

## Installation

```bash
npm install @sdkwork/craw-chat-management-backend-sdk
# or
yarn add @sdkwork/craw-chat-management-backend-sdk
# or
pnpm add @sdkwork/craw-chat-management-backend-sdk
```

## Quick Start

```typescript
import { SdkworkBackendClient } from '@sdkwork/craw-chat-management-backend-sdk';

const client = new SdkworkBackendClient({
  baseUrl: 'https://your-management-origin.example.com',
  timeout: 30000,
});

client.setApiKey('your-management-api-key');

const tenantPage = await client.tenants.getApiAdminTenants();
```

## Authentication Modes

Choose exactly one authentication mode per client instance.

### Mode A: API Key

Recommended for service-to-service management automation.

```typescript
const client = new SdkworkBackendClient({
  baseUrl: 'https://your-management-origin.example.com',
});

client.setApiKey('your-management-api-key');
// Sends: Authorization: Bearer <apiKey>
```

### Mode B: Dual Token

Use this when the target deployment expects a bearer token plus a delegated access token.

```typescript
const client = new SdkworkBackendClient({
  baseUrl: 'https://your-management-origin.example.com',
});

client.setAuthToken('your-auth-token');
client.setAccessToken('your-access-token');
// Sends:
// Authorization: Bearer <authToken>
// Access-Token: <accessToken>
```

Do not combine `setApiKey(...)` with `setAuthToken(...)` and `setAccessToken(...)` on the
same client instance.

## Endpoint Targeting

- Configure `baseUrl` to the origin that serves the checked-in `/api/admin/*` contract for the
  current environment.
- In packaged installs, that target is the unified `craw-chat-server` or `web-gateway` public
  origin.
- In direct backend development, point `baseUrl` to the management backend origin that already
  owns the `/api/admin/*` surface for that environment.

## Surface Groups

- `client.auth`
- `client.users`
- `client.marketing`
- `client.tenants`
- `client.access`
- `client.routing`
- `client.catalog`
- `client.usage`
- `client.billing`
- `client.operations`

## Package Boundary

- Use only the package root entrypoint: `@sdkwork/craw-chat-management-backend-sdk`.
- Do not import `generated/server-openapi/src/*` private generator paths from downstream code.
- Keep business orchestration in the composed package `@sdkwork/craw-chat-sdk-management`
  instead of re-exporting generated internals.

## Regeneration Contract

- Generator-owned files are tracked in `.sdkwork/sdkwork-generator-manifest.json`.
- Each run also writes `.sdkwork/sdkwork-generator-changes.json` so automation can inspect
  created, updated, deleted, unchanged, scaffolded, and backed-up files for the latest generation.
- Apply mode also writes `.sdkwork/sdkwork-generator-report.json` with the full execution report,
  including `schemaVersion`, `generator`, stable artifact paths, and the execution handoff
  commands that match CLI `--json` output.
- Put hand-written wrappers, adapters, and orchestration in `custom/`.
- Files scaffolded under `custom/` are created once and preserved across regenerations.
