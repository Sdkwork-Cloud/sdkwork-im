# @sdkwork/craw-chat-admin-sdk

Composed TypeScript SDK for the Craw Chat admin control plane.

This package sits above the generated `@sdkwork/craw-chat-admin-backend-sdk` transport layer and exposes the consumer-facing `CrawChatAdminSdkClient`.

Ownership:

- `generated/server-openapi`
  generator-owned transport derived from the runtime OpenAPI contract
- `composed`
  manual ergonomic layer with stable module names and usage examples

## Usage

```ts
import { CrawChatAdminSdkClient } from '@sdkwork/craw-chat-admin-sdk';

const sdk = await CrawChatAdminSdkClient.create({
  baseUrl: 'https://admin.example.com',
  authToken: '<token>',
});

const health = await sdk.meta.health();
const registry = await sdk.protocol.getRegistry();
```

Preferred create options are flat:

- `baseUrl`
- `authToken`
- `headers`
- `timeout`
- `fetch`

## Modules

- `meta`
  Runtime health and liveness endpoints.
- `protocol`
  Governance and registry snapshots.
- `providers`
  Provider bindings, policy history, preview, rollback, and registry access.
- `social`
  Social graph mutation and snapshot flows.
- `socialRuntime`
  Shared-channel runtime queues, repair, and targeted recovery operations.
- `nodes`
  Node activation, drain, and route migration workflows.

## Verification

```powershell
node .\..\..\bin\verify-typescript-public-api-boundary.mjs
node .\..\..\bin\verify-typescript-usage-surface.mjs
node .\..\..\bin\build-typescript-generated-package.mjs
node .\..\..\bin\verify-typescript-generated-package.mjs
node .\bin\run-tsc.mjs -p tsconfig.build.json --noEmit
node .\bin\run-tsc.mjs -p tsconfig.build.json
node .\test\craw-chat-admin-client.test.mjs
```
