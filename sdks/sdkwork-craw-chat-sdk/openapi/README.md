# SDKWork Craw Chat SDK OpenAPI Sources

This directory stores the OpenAPI source documents for the `sdkwork-craw-chat-sdk` workspace.

## Files

- `craw-chat-app.openapi.yaml`
  Authority OpenAPI 3.x contract for the app-facing Craw Chat SDK surface.
- `craw-chat-app.sdkgen.yaml`
  Generator-compatible derived input consumed by `sdkwork-sdk-generator`.
- `craw-chat-app.flutter.sdkgen.yaml`
  Flutter-compatible derived input that expands primitive component refs before Dart generation.

## Rules

- The authority contract is the source of truth and is refreshed from the live service export before generation.
- The derived contract exists for generator compatibility and normalization only.
- Generated SDK packages must never edit either file in place.
- The live service schema endpoint is `/openapi/craw-chat-app.openapi.yaml`.
- The service export resolves schema content from `CRAW_CHAT_APP_OPENAPI_SCHEMA_PATH` first, then the workspace authority file, then the embedded fallback bundled into `local-minimal-node`.
- Root regeneration wrappers refresh the checked-in authority snapshot from the live service export, then regenerate the derived contracts before SDK generation.
- The checked-in authority file remains the last successful live schema snapshot for review and diffability inside the repo.

## Related Docs

- workspace overview: `../README.md`
- internal docs map: `../docs/README.md`
- generation pipeline: `../docs/generation-pipeline.md`
- package standards: `../docs/package-standards.md`
