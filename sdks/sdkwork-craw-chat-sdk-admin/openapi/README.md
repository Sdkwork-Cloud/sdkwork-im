# Admin OpenAPI Contracts

This directory contains the checked-in control-plane authority contract and the derived admin sdkgen
contract used by `sdkwork-craw-chat-sdk-admin`.

## Files

- `craw-chat-control-plane.openapi.json`
  - exact authority snapshot exported from `control-plane-api`
- `craw-chat-control-plane.sdkgen.json`
  - derived sdkgen contract with `x-sdkwork-sdk-surface` metadata for SDK assembly

## Refresh Workflow

Refresh the checked-in authority snapshot from the live service implementation:

```bash
./bin/generate-sdk.sh
```

If you already exported the control-plane OpenAPI JSON to a local file, normalize it into the
checked-in authority contract:

```bash
node ./bin/refresh-openapi-source.mjs \
  --source-file ./path/to/exported-control-plane.openapi.json
```

Regenerate the derived sdkgen contract from that authority snapshot:

```bash
node ./bin/prepare-openapi-source.mjs \
  --base ./openapi/craw-chat-control-plane.openapi.json \
  --derived ./openapi/craw-chat-control-plane.sdkgen.json
```

Run the workspace verifier after refreshing:

```bash
node ./bin/verify-sdk.mjs
```
