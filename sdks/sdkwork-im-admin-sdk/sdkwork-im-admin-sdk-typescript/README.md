# sdkwork-im-admin-sdk-typescript

TypeScript language workspace for `sdkwork-im-admin-sdk`.

## Package Layers

- generated package: `@sdkwork/im-admin-backend-sdk`
- composed package: `@sdkwork/im-admin-sdk`

The preferred consumer entrypoint is `ImAdminSdkClient`.

## Verification

- `./bin/sdk-verify.sh`
- `./bin/sdk-verify.ps1`
- root workspace command `node ./sdks/sdkwork-im-admin-sdk/bin/verify-sdk.mjs`

## Package Boundary

- Consume generated transport symbols only through `@sdkwork/im-admin-backend-sdk`.
- Do not import `generated/server-openapi/src/*` private source paths from manual or downstream code.

## Contract Inputs

- `../openapi/im-admin.openapi.json`
- `../openapi/im-admin.sdkgen.json`

## Smoke Coverage

- composed smoke test at `composed/test/im-admin-sdk-client.test.mjs`
