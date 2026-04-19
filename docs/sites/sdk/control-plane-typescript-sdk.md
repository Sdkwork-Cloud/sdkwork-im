# Control-Plane TypeScript SDK

## Package Boundary

The control-plane TypeScript SDK is split into two layers:

- generated transport package: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/generated/server-openapi`
  Published package name: `@sdkwork/control-plane-backend-sdk`
- composed product package: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed`
  Published package name: `@sdkwork/control-plane-sdk`

Use the composed package for admin integrations. It exposes the consumer-facing
`ControlPlaneSdkClient`, keeps generated transport ownership isolated, and remains the only
supported SDK boundary for `apps/craw-chat-admin`.

## Create The Client

```ts
import { ControlPlaneSdkClient } from '@sdkwork/control-plane-sdk';

const sdk = await ControlPlaneSdkClient.create({
  baseUrl: 'https://admin.example.com',
  authToken: '<token>',
});
```

`ControlPlaneSdkClient.create()` accepts the flat admin-facing form directly:

- `baseUrl`
- `authToken`
- `headers`
- `timeout`
- `fetch`

If the host already owns transport creation, you can still pass `backendClient`.

## Control-plane Modules

The generated control-plane contract is surfaced through stable semantic modules:

- `sdk.meta`
  Runtime health and liveness.
- `sdk.protocol`
  Governance and registry snapshots.
- `sdk.providers`
  Provider bindings, history, diff, preview, rollback, and registry.
- `sdk.social`
  Direct-chat, friendship, external-connection, and block-policy control-plane flows.
- `sdk.socialRuntime`
  Shared-channel queue inventory, repair, reclaim, release, requeue, and targeted takeover flows.
- `sdk.nodes`
  Node activation, drain, and route migration.

## Module To API Reference

| TypeScript surface | API reference |
| --- | --- |
| `sdk.meta` | [/api-reference/control-plane/protocol#get-control-healthz](/api-reference/control-plane/protocol#get-control-healthz) |
| `sdk.protocol` | [/api-reference/control-plane/protocol](/api-reference/control-plane/protocol) |
| `sdk.providers` | [/api-reference/control-plane/providers](/api-reference/control-plane/providers) |
| `sdk.social` | [/api-reference/control-plane/social](/api-reference/control-plane/social) |
| `sdk.socialRuntime` | [/api-reference/control-plane/social-runtime](/api-reference/control-plane/social-runtime) |
| `sdk.nodes` | [/api-reference/control-plane/nodes](/api-reference/control-plane/nodes) |

Example:

```ts
const health = await sdk.meta.health();
const governance = await sdk.protocol.getGovernance();
const bindings = await sdk.providers.getBindings({ tenantId: 'tenant-northstar' });
await sdk.nodes.activate('node-east-1');
```

## Browser Admin Surface

The same formal package also re-exports the current manual-owned browser admin helpers for
`/api/admin/*` routes. This keeps `apps/craw-chat-admin` on one package boundary without pretending
that those routes are already part of the generated OpenAPI authority.

Those helpers are intentionally outside the generated control-plane reference at
[Control Plane API Overview](/api-reference/control-plane-api), because they target browser-facing
`/api/admin/*` routes rather than `/api/v1/control/*`.

Representative helpers include:

- `loginAdminUser`
- `getAdminMe`
- `listTenants`
- `saveTenant`
- `listStorageProviders`
- `saveTenantStorageConfig`
- `validateTenantStorageConfig`

Example:

```ts
import {
  loginAdminUser,
  listTenants,
  listStorageProviders,
  saveTenantStorageConfig,
} from '@sdkwork/control-plane-sdk';

const session = await loginAdminUser({
  email: 'operator@example.com',
  password: 'secret',
});

const tenants = await listTenants(session.token);
const providers = await listStorageProviders(session.token);

await saveTenantStorageConfig('tenant-northstar', {
  binding: {
    providerPluginId: 'object-storage-aws',
    enabled: true,
  },
  config: {
    bucketOrContainer: 'tenant-northstar-assets',
    region: 'us-east-1',
  },
});
```

These helpers are manual-owned, TypeScript-specific, and exist because the current operator shell is
a browser TypeScript consumer. They are not part of the generated `/api/v1/control/*` authority.

## Verification

```powershell
node .\sdks\sdkwork-control-plane-sdk\bin\verify-typescript-workspace.mjs
node .\sdks\sdkwork-control-plane-sdk\bin\verify-sdk.mjs --language typescript
```
