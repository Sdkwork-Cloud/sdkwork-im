# sdkwork-craw-chat-sdk-admin-flutter

This workspace owns the Flutter package surface for the Craw Chat control-plane API SDK family.

## Layout

- `generated/server-openapi`
  Generator-owned Flutter HTTP SDK output materialized from the checked-in OpenAPI contract.
- `composed`
  Manual-owned consumer package `craw_chat_sdk_admin` built above the generated HTTP layer.
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
`package:craw_chat_admin_backend_sdk/craw_chat_admin_backend_sdk.dart`; it does not import
`generated/server-openapi/lib/src` private paths directly.

## Consumer Package

The primary admin-facing Flutter package is `composed/pubspec.yaml`:

- package name: `craw_chat_sdk_admin`
- library entrypoint: `composed/lib/craw_chat_sdk_admin.dart`
- main client: `CrawChatAdminClient`
- exposed domains:
- `cluster`
- `protocol`
- `providers`
- `social`
- `system`

The generated backend transport package is:

- package name: `craw_chat_admin_backend_sdk`
- library entrypoint: `generated/server-openapi/lib/craw_chat_admin_backend_sdk.dart`

## Generate

From this workspace:

```powershell
.\bin\sdk-gen.ps1
```

```bash
./bin/sdk-gen.sh
```

These scripts forward to the root `sdkwork-craw-chat-sdk-admin/bin/generate-sdk.*` wrapper, refresh
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

These scripts forward to the root `sdkwork-craw-chat-sdk-admin/bin/verify-sdk.mjs` wrapper. The
forwarded verification path includes Flutter workspace verification, package metadata checks,
public API boundary checks, and composed-surface checks for `CrawChatAdminClient`.

## Endpoint Targeting

- For standalone governance development, point baseUrl directly at control-plane-api, which defaults to http://127.0.0.1:18081.
- For packaged installs, point the same client at the unified craw-chat-server / web-gateway public origin.
- Do not mix direct control-plane origins and packaged single-port gateway assumptions in the same client instance.

## Current Workspace Status

The Flutter workspace is materialized end to end:

- generated transport package: `craw_chat_admin_backend_sdk`
- composed product package: `craw_chat_sdk_admin`
- public client surface: `CrawChatAdminClient`
- source-level verification: enabled

Publication and version assignment are still pending, but this workspace is no longer
template-only.
