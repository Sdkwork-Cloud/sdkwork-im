# SDKWork IM Backend SDK

`sdkwork-im-backend-sdk` is the `/backend/v3/api` SDK family for backend management, operator,
control-plane, admin-console, and service-side Craw Chat integration.

It is intentionally separate from both public IM and app-development SDKs:

- `sdkwork-im-sdk` targets `/im/v3/api`.
- `sdkwork-im-app-sdk` targets `/app/v3/api`.
- `sdkwork-im-backend-sdk` targets `/backend/v3/api`.

Control-plane and admin are modules in this workspace, not separate SDK families. The backend
authority explicitly owns:

- `/backend/v3/api/ops/*`
- `/backend/v3/api/audit/*`
- `/backend/v3/api/automation/*`
- `/backend/v3/api/control/*`
- `/backend/v3/api/admin/*`

Identity, token refresh, account, tenant, and organization context are supplied by the upstream
platform. This SDK only consumes propagated backend context and does not expose login or client
device route lifecycle APIs.

## Contract Files

- `openapi/craw-chat-backend-api.openapi.yaml`
  Authority OpenAPI 3.x contract for `/backend/v3/api`.
- `openapi/craw-chat-backend-api.sdkgen.yaml`
  Generator-compatible derived input.

## Generation

Primary Node entrypoint:

```powershell
node .\bin\generate-sdk.mjs --language typescript
```

PowerShell:

```powershell
.\bin\generate-sdk.ps1 -Languages typescript
```

Bash:

```bash
./bin/generate-sdk.sh --language typescript
```

Defaults:

- base URL: `http://127.0.0.1:18090`
- schema URL: `/backend/v3/openapi.json`
- API prefix: `/backend/v3/api`
- SDK name: `sdkwork-im-backend-sdk`
- SDK target/type: `backend`
- standard profile: `sdkwork-v3`

Generated output is written under language-specific `sdkwork-im-backend-sdk-*` directories. Do not
edit generated output by hand.

The repository-wide boundary materialization command is:

```powershell
node ..\materialize-im-v3-openapi-boundaries.mjs
```

Run it before generation when control-plane, admin, app-business, or backend grouping changes. It
consolidates control/admin authority into this workspace and keeps non-management HTTP APIs in the
app SDK family.

## Verification

```powershell
node .\bin\verify-sdk.mjs
```

The verifier checks the `/backend/v3/api` OpenAPI surface, dual-token `AuthToken` and `AccessToken`
security, problem-detail errors, generated language manifests, and the TypeScript
`SdkworkBackendClient` surface.

## Release Snapshot Boundary

This workspace inherits the current SDK release snapshot from
`artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`.

- `state = template_only_pending_generation`
- `generationStatus = generated`
- `releaseStatus = not_published`
- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`
