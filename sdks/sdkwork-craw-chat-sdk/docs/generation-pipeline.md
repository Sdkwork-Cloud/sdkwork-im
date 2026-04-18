# SDK Generation Pipeline

This document describes the required generation flow for `sdkwork-craw-chat-sdk`.

The core rule is that SDK generation must use the running service export as the authoritative source
contract, not a stale local YAML file read in isolation.

## Authority Source

The live service schema export is:

- endpoint: `/openapi/craw-chat-app.openapi.yaml`
- checked-in authority snapshot: `openapi/craw-chat-app.openapi.yaml`

Generation wrappers refresh the checked-in authority snapshot from the live service schema before
deriving language-specific generator inputs.

## Required Generation Sequence

1. Ensure the target local service is reachable.
2. For supported loopback targets, start `local-minimal-node` automatically when necessary through
   `bin/start-local.ps1` or `bin/start-local.sh`.
3. Refresh `openapi/craw-chat-app.openapi.yaml` from `/openapi/craw-chat-app.openapi.yaml`.
4. Normalize the derived generator inputs:
   - `openapi/craw-chat-app.sdkgen.yaml`
   - `openapi/craw-chat-app.flutter.sdkgen.yaml`
5. Prepare language-specific generated output roots when local workspace constraints require it.
   TypeScript uses `bin/prepare-generated-output.mjs` to stage only the conflicting
   generator-owned entries before regeneration while preserving `custom/`.
6. Generate language outputs.
7. Normalize public auth surfaces back to the Craw Chat bearer-only contract.
8. Run workspace verification before treating generation as complete.

## TypeScript Pipeline

The TypeScript path must preserve the single-package standard:

1. Stage conflicting generator-owned entries in
   `sdkwork-craw-chat-sdk-typescript/generated/server-openapi` into
   `.sdkwork/tmp/prepare-generated-output/typescript/**` before invoking the external generator.
   Preserve `custom/` because it remains the scaffold/manual extension root.
2. Generate raw transport into `sdkwork-craw-chat-sdk-typescript/generated/server-openapi`.
3. Normalize generator-owned auth surface, generated package manifest, and local package task
   entrypoint.
4. Verify generated-package build, packaging, cleanup, install safety, concurrency, and
   determinism.
5. Assemble the publishable root package so generated transport lands under `src/generated/**`.
6. Re-run root package verification and smoke tests.

## Flutter Pipeline

The Flutter path must preserve the official consumer-package standard:

1. Generate transport into `sdkwork-craw-chat-sdk-flutter/generated/server-openapi`.
   That generator-owned transport package remains `backend_sdk`.
2. Normalize the public auth surface to bearer-only behavior.
3. Verify generated model shape, auth alignment, public API boundary, composed parity, and metadata.
4. Keep any manual realtime or wrapper extensions outside generated output.

## Generator Boundary Rules

- Do not hand-edit generated files in `generated/server-openapi`.
- Do not hand-edit TypeScript assembled generated files in `src/generated/**`.
- Fix the authority contract, wrapper inputs, or normalization step and regenerate.
- Keep manual adapters, wrappers, and business helpers outside generated output.

## Primary Commands

Generation:

```powershell
.\bin\generate-sdk.ps1 -Languages typescript,flutter
```

Verification:

```powershell
node .\bin\verify-sdk.mjs --language typescript --language flutter
```

## Related Docs

- package standards: `package-standards.md`
- verification matrix: `verification-matrix.md`
- public docs sync standard: `sites/README.md`
- workspace overview: `../README.md`
