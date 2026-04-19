# Admin Control-Plane OpenAPI

This directory stores the checked-in OpenAPI contract inputs for `sdkwork-control-plane-sdk`.

Files:

- `control-plane.openapi.yaml`
  The normalized authority snapshot fetched from the live `control-plane-api` runtime.
- `control-plane.sdkgen.yaml`
  The derived generator input prepared from the authority snapshot.

Rules:

- fetch the latest runtime contract before regeneration
- treat the authority snapshot as the source of truth inside the SDK workspace
- do not hand-edit generated output to compensate for contract drift
- if the runtime contract changes, refresh the authority snapshot and re-run preparation

Runtime source:

- default live endpoint: `http://127.0.0.1:18081/openapi.json`
- default runtime command: `cargo run -p control-plane-api`

Refresh flow:

```powershell
node .\bin\fetch-openapi-source.mjs
node .\bin\prepare-openapi-source.mjs --base .\openapi\control-plane.openapi.yaml --derived .\openapi\control-plane.sdkgen.yaml
```
