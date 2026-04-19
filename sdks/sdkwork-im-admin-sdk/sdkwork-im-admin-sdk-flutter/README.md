# sdkwork-im-admin-sdk-flutter

This workspace owns the Flutter package surface for the IM admin backend SDK family.

## Layout

- `generated/server-openapi`
  Generator-owned Flutter HTTP SDK output materialized from the checked-in OpenAPI contract.
- `composed`
  Manual-owned consumer package `im_admin_sdk` built above the generated HTTP layer.
- `bin/`
  Thin forwarding scripts to the root workspace wrappers.
- `README.md`
  Manual-owned workspace documentation.

## Generation Boundary

This workspace follows the layered Flutter SDK pattern:

- generated HTTP SDK lives in `generated/server-openapi`
- composed Flutter SDK lives in `composed`
- future orchestration or realtime adapters must stay outside generated output

Do not hand-edit the generated package. Change the checked-in OpenAPI inputs or the root workspace
materializer and regenerate.

The manual `composed` layer consumes the generated package only through
`package:im_admin_backend_sdk/im_admin_backend_sdk.dart`; it does not import
`generated/server-openapi/lib/src` private paths directly.

## Consumer Package

The primary admin-facing Flutter package is `composed/pubspec.yaml`:

- package name: `im_admin_sdk`
- library entrypoint: `composed/lib/im_admin_sdk.dart`
- main client: `ImAdminSdkClient`
- exposed domains:
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
- `storage`

The generated backend transport package is:

- package name: `im_admin_backend_sdk`
- library entrypoint: `generated/server-openapi/lib/im_admin_backend_sdk.dart`

## Generate

From this workspace:

```powershell
.\bin\sdk-gen.ps1
```

```bash
./bin/sdk-gen.sh
```

These scripts forward to the root `sdkwork-im-admin-sdk/bin/generate-sdk.*` wrapper, refresh
the checked-in authority contract when needed, rematerialize the Flutter workspace, rebuild the
assembly snapshot, and then run the root verification flow.

## Verify

From this workspace:

```powershell
.\bin\sdk-verify.ps1
```

```bash
./bin/sdk-verify.sh
```

These scripts forward to the root `sdkwork-im-admin-sdk/bin/verify-sdk.mjs` wrapper. The
forwarded verification path includes Flutter workspace verification, package metadata checks,
public API boundary checks, and composed-surface checks for `ImAdminSdkClient`.

## Endpoint Targeting

- Point baseUrl at the deployed surface that serves the checked-in /api/admin/* contract.
- In packaged installs, that surface is the unified public origin that fronts the admin gateway.
- In direct backend development, use the environment-specific origin that already owns /api/admin/*.

## Current Workspace Status

The Flutter workspace is materialized end to end:

- generated transport package: `im_admin_backend_sdk`
- composed product package: `im_admin_sdk`
- public client surface: `ImAdminSdkClient`
- source-level verification: enabled

Publication and version assignment are still pending, but this workspace is no longer
template-only.
