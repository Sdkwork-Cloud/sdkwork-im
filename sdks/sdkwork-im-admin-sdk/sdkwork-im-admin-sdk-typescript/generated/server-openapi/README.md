# @sdkwork/im-admin-backend-sdk

Generated TypeScript transport package for the IM admin backend.

## Package Role

This package is the generator-owned transport layer for the checked-in IM admin OpenAPI contract.
Use it when you need direct access to generated HTTP operations and root-exported transport types.

For business-facing admin integrations, prefer the composed package
`@sdkwork/im-admin-sdk`, which keeps the transport layer behind the stable
`ImAdminSdkClient` facade.

## Installation

```bash
npm install @sdkwork/im-admin-backend-sdk
# or
yarn add @sdkwork/im-admin-backend-sdk
# or
pnpm add @sdkwork/im-admin-backend-sdk
```

## Quick Start

```typescript
import { ImAdminBackendClient } from '@sdkwork/im-admin-backend-sdk';

const client = new ImAdminBackendClient({
  baseUrl: 'https://your-admin-origin.example.com',
  timeout: 30000,
});

client.setAuthToken('operator-session-token');

const tenants = await client.tenants.listTenants();
console.log(tenants);
```

## Authentication Modes

This admin backend surface is bearer-token based.

```typescript
const client = new ImAdminBackendClient({
  baseUrl: 'https://your-admin-origin.example.com',
});

client.setAuthToken('operator-session-token');
```

## Endpoint Targeting

- Configure `baseUrl` to the origin that serves the checked-in `/api/admin/*` contract for the
  current environment.
- In packaged installs, that target is the unified public origin that fronts the admin gateway.
- In direct backend development, point `baseUrl` to the IM admin backend origin that already
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
- `client.storage`

## Package Boundary

- Use only the package root entrypoint: `@sdkwork/im-admin-backend-sdk`.
- Do not import `generated/server-openapi/src/*` private generator paths from downstream code.
- Keep business orchestration in the composed package `@sdkwork/im-admin-sdk`
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
