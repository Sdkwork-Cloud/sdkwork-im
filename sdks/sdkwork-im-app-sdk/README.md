# SDKWork IM App SDK

`sdkwork-im-app-sdk` is the `/app/v3/api` SDK family for developers building instant messaging
applications on top of Craw Chat.

This workspace is intentionally separate from `sdkwork-im-sdk`:

- `sdkwork-im-sdk` targets the IM open-platform contract exported at `/im/v3/openapi.json`.
- `sdkwork-im-app-sdk` targets the app-development contract exported at `/app/v3/openapi.json`.
- Identity, token refresh, account, tenant, and organization context are supplied by the upstream
  platform. This SDK only consumes propagated tenant, actor, and device context.

## Contract Files

- `openapi/craw-chat-app-api.openapi.yaml`
  Authority OpenAPI 3.x contract for `/app/v3/api`.
- `openapi/craw-chat-app-api.sdkgen.yaml`
  Default generator-compatible derived input.
- `openapi/craw-chat-app-api.flutter.sdkgen.yaml`
  Flutter-compatible derived input with primitive component refs expanded.

## Generation

Primary Node entrypoint:

```powershell
node .\bin\generate-sdk.mjs --language typescript --language flutter
```

PowerShell:

```powershell
.\bin\generate-sdk.ps1 -Languages typescript,flutter
```

Bash:

```bash
./bin/generate-sdk.sh --language typescript --language flutter
```

Defaults:

- base URL: `http://127.0.0.1:18090`
- schema URL: `/app/v3/openapi.json`
- API prefix: `/app/v3/api`
- SDK name: `sdkwork-im-app-sdk`
- SDK target/type: `app`
- standard profile: `sdkwork-v3`

Generated output is written under language-specific `sdkwork-im-app-sdk-*` directories. Do not edit
generated output by hand; update the OpenAPI contract or generator inputs and regenerate.

## Verification

```powershell
node .\bin\verify-sdk.mjs
```

The verifier checks the `/app/v3/api` OpenAPI surface, dual-token `AuthToken` and `AccessToken`
security, problem-detail errors, generated language manifests, and the TypeScript `SdkworkAppClient`
surface.

## Release Snapshot Boundary

This workspace inherits the current SDK release snapshot from
`artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`.

- `state = template_only_pending_generation`
- `generationStatus = generated`
- `releaseStatus = not_published`
- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`
