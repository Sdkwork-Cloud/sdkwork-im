# sdkwork-craw-chat-sdk-management-typescript

This workspace contains the TypeScript management SDK pair for Craw Chat operator-console and
management integrations.

## Packages

- generated package: `@sdkwork/craw-chat-management-backend-sdk`
  - path: `generated/server-openapi`
- composed package: `@sdkwork/craw-chat-sdk-management`
  - path: `composed`

## Package Boundary

The manual and composed layers must consume the generated transport package only through the
package root `@sdkwork/craw-chat-management-backend-sdk`.
Do not import `generated/server-openapi/src/*` from manual sources or public declaration surfaces.

## Client Surface

The preferred consumer entrypoint is `CrawChatSdkManagementClient`, which exposes:

- `auth`
- `users`
- `marketing`
- `tenants`
- `access`
- `routing`
- `catalog`
- `usage`
- `billing`
- `operations`

`CrawChatSdkManagementClient.create({ backendClient })` and
`CrawChatSdkManagementClient.create({ backendConfig })` resolve to the same composed facade. The
generated package remains the transport-only layer; business consumers should prefer the composed
package.

## Endpoint Targeting

- Configure `backendConfig.baseUrl` to the deployed surface that serves the checked-in
  `/api/admin/*` contract.
- In packaged installs, that is the unified `craw-chat-server` / `web-gateway` public origin.
- In direct admin-backend development, use the backend origin that already owns `/api/admin/*` for
  that environment.

## Commands

- generate or refresh the TypeScript workspace:
  - `./bin/sdk-gen.sh`
- assemble workspace metadata:
  - `./bin/sdk-assemble.sh`
- verify the management SDK workspace:
  - `./bin/sdk-verify.sh`

The TypeScript verification path performs:

- generated package build to `generated/server-openapi/dist`
- generated package `npm pack --dry-run` verification
- composed package typecheck and build
- composed smoke test at `composed/test/craw-chat-sdk-management-client.test.mjs`

The generated package manifest is stabilized so `build` and `prepublishOnly` delegate to the
workspace-owned wrapper command under `sdks/sdkwork-craw-chat-sdk-management/bin/` instead of the
raw generator template.

## Contract Source

The package line is generated from:

- `../openapi/craw-chat-management.openapi.json`
- `../openapi/craw-chat-management.sdkgen.json`

The root management workspace owns authority refresh, verification, and assembly.
