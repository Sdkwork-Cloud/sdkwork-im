# sdkwork-craw-chat-sdk-admin TypeScript

This workspace contains the TypeScript admin SDK pair for Craw Chat control-plane integrations.

## Packages

- generated package: `@sdkwork/craw-chat-admin-backend-sdk`
  - path: `generated/server-openapi`
- composed package: `@sdkwork/craw-chat-sdk-admin`
  - path: `composed`

## Client Surface

The preferred consumer entrypoint is `CrawChatSdkAdminClient`, which exposes:

- `protocol`
- `providers`
- `cluster`
- `social`
- `system`

`CrawChatSdkAdminClient.create({ backendClient })` and
`CrawChatSdkAdminClient.create({ backendConfig })` both resolve to the same composed facade. The
generated package remains the transport-only layer; business consumers should prefer the composed
package.

## Endpoint Targeting

- For standalone governance development, configure `backendConfig.baseUrl` to the direct
  `control-plane-api` origin, typically `http://127.0.0.1:18081`.
- For packaged installs, configure `backendConfig.baseUrl` to the unified `craw-chat-server` /
  `web-gateway` public origin.
- Do not mix those deployment assumptions in the same client configuration.

## Package Boundary

The manual and composed layers must consume the generated transport package only through the
package root `@sdkwork/craw-chat-admin-backend-sdk`.
Do not import `generated/server-openapi/src/*` from manual sources or from public declaration
surfaces.

## Commands

- generate or refresh the TypeScript workspace:
  - `./bin/sdk-gen.sh`
- assemble workspace metadata:
  - `./bin/sdk-assemble.sh`
- verify the admin SDK workspace:
  - `./bin/sdk-verify.sh`

The TypeScript verification path now performs:

- generated package build to `generated/server-openapi/dist`
- generated package `npm pack --dry-run` verification
- composed package typecheck and build
- composed smoke test at `composed/test/craw-chat-sdk-admin-client.test.mjs`

The generated package manifest is stabilized so `build` and `prepublishOnly` delegate to the
workspace-owned wrapper command under `sdks/sdkwork-craw-chat-sdk-admin/bin/` instead of the raw
generator template.

## Contract Source

The package line is generated from:

- `../openapi/craw-chat-control-plane.openapi.json`
- `../openapi/craw-chat-control-plane.sdkgen.json`

The root admin workspace owns authority refresh, verification, and assembly.
