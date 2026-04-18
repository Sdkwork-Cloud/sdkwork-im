# SDKWork Craw Chat SDK OpenAPI Sources

This directory stores the OpenAPI source documents for the `sdkwork-craw-chat-sdk` workspace.

Files:

- `craw-chat-app.openapi.yaml`
  Authority OpenAPI 3.x contract for the app-facing Craw Chat SDK surface.
- `craw-chat-app.sdkgen.yaml`
  Generator-compatible derived input consumed by `sdkwork-sdk-generator`.
- `craw-chat-app.flutter.sdkgen.yaml`
  Flutter-compatible derived input that expands primitive component refs before Dart generation.

Rules:

- The authority contract is the source of truth.
- The derived contract exists for generator compatibility and normalization only.
- The derived contracts also carry `x-sdkwork-sdk-surface` plus per-operation `x-sdkwork-*` ownership hints so generator layers can consume stable service, surface-group, and transport metadata directly from the checked-in sdkgen inputs.
- Generated SDK packages must never edit either file in place.
- Root regeneration wrappers refresh the derived contract before generation.
- Offline generation is supported from the checked-in authority snapshot.
