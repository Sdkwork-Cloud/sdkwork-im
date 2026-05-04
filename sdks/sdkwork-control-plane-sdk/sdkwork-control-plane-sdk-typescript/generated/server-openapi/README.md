# @sdkwork/control-plane-backend-sdk

Generated TypeScript transport package for the control-plane API.

## Package Role

This package is the generator-owned transport layer for the checked-in control-plane OpenAPI
contract. Use it when you need direct access to generated HTTP operations and root-exported
transport types.

For business-facing integrations, prefer the composed package `@sdkwork/control-plane-sdk`,
which keeps the transport package behind a stable control-plane-oriented facade.

## Installation

```bash
npm install @sdkwork/control-plane-backend-sdk
# or
yarn add @sdkwork/control-plane-backend-sdk
# or
pnpm add @sdkwork/control-plane-backend-sdk
```

## Quick Start

```typescript
import { ControlPlaneBackendClient } from '@sdkwork/control-plane-backend-sdk';

const client = new ControlPlaneBackendClient({
  baseUrl: 'http://127.0.0.1:18081',
  timeout: 30000,
});

client.setAuthToken('your-control-plane-api-token');

const governance = await client.protocol.getProtocolGovernance();
```

## Authentication Modes

Choose exactly one authentication mode per client instance.

### Mode A: API Key

Recommended for service-to-service control-plane automation.

```typescript
const client = new ControlPlaneBackendClient({ baseUrl: 'http://127.0.0.1:18081' });
client.setAuthToken('your-control-plane-api-token');
// Sends: Authorization: Bearer <authToken>
```

### Mode B: Dual Token

Use this when the target deployment expects a rotated bearer token later in the client lifecycle.

```typescript
const client = new ControlPlaneBackendClient({ baseUrl: 'http://127.0.0.1:18081' });
client.setAuthToken('rotated-control-plane-token');
```

This generated client currently standardizes on bearer-token auth through `setAuthToken(...)`.

## Endpoint Targeting

- For standalone governance development, point `baseUrl` to the direct `control-plane-api`
  origin, typically `http://127.0.0.1:18081`.
- For packaged installs, point `baseUrl` to the unified `craw-chat-server` or `web-gateway`
  public origin.
- Keep one deployment model per client configuration. Do not mix direct control-plane and unified
  gateway assumptions in the same client instance.

## Surface Groups

- `client.meta` - health probe operations
- `client.protocol` - protocol governance and contract inspection operations
- `client.providers` - provider-binding and provider runtime operations
- `client.social` - social control-plane operations
- `client.socialRuntime` - shared-channel runtime repair and queue operations
- `client.nodes` - node lifecycle operations

## Package Boundary

- Use only the package root entrypoint: `@sdkwork/control-plane-backend-sdk`.
- Do not import `generated/server-openapi/src/*` private generator paths from downstream code.
- Keep business orchestration in the composed package `@sdkwork/control-plane-sdk` instead of
  re-exporting generated internals.

## Regeneration Contract

- Generator-owned files are tracked in `.sdkwork/sdkwork-generator-manifest.json`.
- Each run also writes `.sdkwork/sdkwork-generator-changes.json` so automation can inspect
  created, updated, deleted, unchanged, scaffolded, and backed-up files for the latest generation.
- Apply mode also writes `.sdkwork/sdkwork-generator-report.json` with the full execution report,
  including `schemaVersion`, `generator`, stable artifact paths, and the execution handoff
  commands that match CLI `--json` output.
- `bin/sdk-gen.sh` and `bin/sdk-gen.bat` delegate to `bin/sdk-gen-core.mjs`, which resolves npm
  from the active Node.js runtime instead of assuming a bare `npm` binary on PATH.
- Put hand-written wrappers, adapters, and orchestration in `custom/`.
- Files scaffolded under `custom/` are created once and preserved across regenerations.
