# SDKWork Craw Chat Admin SDK Workspace

`sdkwork-craw-chat-sdk-admin` is the formal admin SDK workspace for Craw Chat.

It is a workspace, not a single package. The workspace owns:

- the checked-in authority OpenAPI 3.x contract captured from the running admin control-plane service
- the generator-compatible derived contract used by `sdkwork-sdk-generator`
- root regeneration and verification wrappers
- generated TypeScript and Flutter admin transport packages
- composed TypeScript and Flutter admin client packages
- package-level documentation for regeneration, verification, and release preparation

## Scope

This workspace standardizes the admin control-plane SDK family for:

- protocol governance, compatibility visibility, and control-plane health
- provider registry, binding policy, diff, preview, history, and rollback flows
- social graph control for direct chat, friendship, external connection, shared-channel policy, and block state
- shared-channel runtime repair, queue inventory, reclaim, requeue, republish, and takeover workflows
- node drain, activate, and route migration operations
- browser-only operator helpers currently required by `apps/craw-chat-admin` for `/api/admin/*`

This workspace does not define:

- app-facing chat and conversation SDK APIs
- fabricated websocket or realtime admin adapters that are not present in the real contract
- a second handwritten admin API package living inside the consuming application

## Contract Source

The canonical admin route surface must be fetched from the running control-plane or admin runtime before generation.

The workspace stores contract files under `openapi/`:

- `admin-control-plane.openapi.yaml`
  The normalized checked-in authority OpenAPI 3.x snapshot captured from the live runtime.
- `admin-control-plane.sdkgen.yaml`
  The derived generator-compatible input used by `sdkwork-sdk-generator`.

The authority file is the source of truth. Generated output is never the source of truth.

## Workspace Layout

```text
sdkwork-craw-chat-sdk-admin/
  openapi/
  bin/
  sdkwork-craw-chat-sdk-admin-typescript/
  sdkwork-craw-chat-sdk-admin-flutter/
  README.md
  .sdkwork-assembly.json
```

Per language workspace:

- `generated/server-openapi`
  Generator-owned output only.
- `composed`
  Manual-owned consumer-facing admin SDK built above the generated HTTP layer.
- `bin/`
  Thin forwarding scripts back to the root workspace wrappers.
- `README.md`
  Manual-owned language workspace documentation.

## Package Layers

Primary consumer packages in this workspace are:

- TypeScript composed package: `sdkwork-craw-chat-sdk-admin-typescript/composed`
  Publishes `@sdkwork/craw-chat-admin-sdk`
- Flutter composed package: `sdkwork-craw-chat-sdk-admin-flutter/composed`
  Publishes `craw_chat_admin_sdk`

Generated transport packages remain available for lower-level HTTP use:

- TypeScript generated package: `@sdkwork/craw-chat-admin-backend-sdk`
- Flutter generated package: `craw_chat_admin_backend_sdk`

The public composed client name is `CrawChatAdminSdkClient`.

## Public Client Shape

Both primary language packages standardize on the same client naming and flat creation model:

```ts
import { CrawChatAdminSdkClient } from '@sdkwork/craw-chat-admin-sdk';

const sdk = await CrawChatAdminSdkClient.create({
  baseUrl: 'https://admin.example.com',
  authToken: '<token>',
});
```

```dart
import 'package:craw_chat_admin_sdk/craw_chat_admin_sdk.dart';

final sdk = CrawChatAdminSdkClient.create(
  baseUrl: 'https://admin.example.com',
  authToken: '<token>',
);
```

Preferred create options are flat:

- `baseUrl`
- `authToken`
- `headers`
- `timeout`

Direct transport injection stays available through `backendClient`.

## Module To API Mapping

| Admin SDK module | Control-plane reference |
| --- | --- |
| `sdk.meta` | `GET /healthz` on the protocol governance page |
| `sdk.protocol` | Protocol registry and governance |
| `sdk.providers` | Provider registry, binding policies, diff, preview, history, and rollback |
| `sdk.social` | Direct chat, external collaboration, friendship, shared-channel policy, and block control |
| `sdk.socialRuntime` | Shared-channel sync queue inventory, repair, reclaim, republish, requeue, and takeover |
| `sdk.nodes` | Node drain, activate, and route migration |

Reference pages:

- `/api-reference/control-plane-api`
- `/api-reference/control-plane/protocol`
- `/api-reference/control-plane/providers`
- `/api-reference/control-plane/social`
- `/api-reference/control-plane/social-runtime`
- `/api-reference/control-plane/nodes`

## Ownership Rules

- do not hand-edit files inside `generated/server-openapi`
- fix the authority contract or wrapper inputs, then regenerate
- keep manual docs, forwarding scripts, and ergonomic client code outside generated output
- do not reintroduce a second handwritten admin transport boundary inside `apps/craw-chat-admin`

## Runtime Schema Capture Requirement

Admin SDK generation must start from the live runtime rather than from a stale handwritten YAML file.

The standard flow is:

1. start or target the approved admin control-plane runtime
2. fetch the live OpenAPI 3.x schema
3. validate that the payload is OpenAPI 3.x and contains the expected admin route groups
4. normalize unstable fields
5. write `openapi/admin-control-plane.openapi.yaml`
6. derive `openapi/admin-control-plane.sdkgen.yaml`
7. generate language-specific `generated/server-openapi` outputs
8. run verification and refresh assembly metadata

This preserves both runtime fidelity and checked-in reproducibility.

## Generate

Run from the workspace root:

```powershell
.\bin\generate-sdk.ps1 -Languages typescript,flutter
```

If local PowerShell execution policy blocks script execution, use:

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\generate-sdk.ps1 -Languages typescript,flutter
```

```bash
./bin/generate-sdk.sh --language typescript --language flutter
```

The wrapper flow is:

1. fetch and normalize the live authority contract
2. prepare the derived `sdkgen` input
3. generate the requested language workspaces
4. run language verification
5. refresh `.sdkwork-assembly.json`

## Assemble

When generated and composed manifests are already current, refresh workspace assembly metadata directly:

```powershell
.\bin\assemble-sdk.ps1 -Languages typescript,flutter
```

If local PowerShell execution policy blocks script execution, use:

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\assemble-sdk.ps1 -Languages typescript,flutter
```

```bash
./bin/assemble-sdk.sh --language typescript --language flutter
```

These root wrappers rebuild `.sdkwork-assembly.json` from the checked-in authority contract plus the
language package manifests.
Language-local `sdk-assemble` wrappers forward to this root entrypoint and pin a single language.

## Assembly Metadata

Every verified regeneration refreshes `.sdkwork-assembly.json` at the workspace root.

That file is the release-facing inventory for this SDK workspace. It records:

- the authority and derived spec paths
- one entry per language workspace
- the generated package `manifestPath` and public entrypoints
- the `packages` layer list so automation can distinguish `generated` versus `composed`
- a `generatedAt` timestamp that remains stable when the assembly content is unchanged

This keeps release preparation and workspace inspection deterministic. Review automation should be
able to answer which package is published from which manifest without scanning the entire tree.

## Verification

Run the root verification entrypoint from the workspace root:

```powershell
node .\bin\verify-sdk.mjs --language typescript --language flutter
```

Use `--with-dart` when the machine has a working Dart or Flutter toolchain and you want native
`dart pub get` plus analysis on top of the default source-level guards:

```powershell
node .\bin\verify-sdk.mjs --language flutter --with-dart
```

If local PowerShell execution policy blocks script execution, use:

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\verify-sdk.ps1 -Languages typescript,flutter
```

```bash
./bin/verify-sdk.sh --language typescript --language flutter
```

The workspace verification chain must prove:

- live schema fetch works
- the normalized authority snapshot is stable
- the derived `sdkgen` file is deterministic
- the workspace automation meta-test and assembly regression test both stay wired into root verification
- generated and composed package boundaries stay aligned
- TypeScript and Flutter workspaces both validate, including usage-surface checks and package metadata verification where applicable
- consuming apps can depend on the formal admin SDK boundary

On Windows, the Flutter verifier uses `bin/verify-flutter-dart-analysis.dart` instead of raw
`dart analyze` so analysis still works when the bundled Dart toolchain cannot safely spawn its
own helper process under the current shell or sandbox policy.

## TypeScript Workspace

The TypeScript workspace lives under `sdkwork-craw-chat-sdk-admin-typescript/`.

- generated package name: `@sdkwork/craw-chat-admin-backend-sdk`
- composed package name: `@sdkwork/craw-chat-admin-sdk`
- public client: `CrawChatAdminSdkClient`

The composed package should expose flat client configuration, semantic admin modules, and thin re-exports of generated types where useful.

The TypeScript composed package is also the single formal package boundary for `apps/craw-chat-admin`.
It intentionally co-locates:

- generated `/api/v1/control/*` modules
- manual-owned `/api/admin/*` helpers used by the browser operator shell

That keeps the consuming app on one package without pretending browser-only admin routes are already
generated from the control-plane authority.

## Flutter Workspace

The Flutter workspace lives under `sdkwork-craw-chat-sdk-admin-flutter/`.

- generated package name: `craw_chat_admin_backend_sdk`
- composed package name: `craw_chat_admin_sdk`
- public client: `CrawChatAdminSdkClient`

The Flutter package mirrors the same generated-versus-composed ownership model and domain grouping used by the TypeScript package.

## Consumer Boundary

`apps/craw-chat-admin` is the primary real consumer of this workspace.

- it must depend on `@sdkwork/craw-chat-admin-sdk`
- it must not keep a parallel `sdkwork-craw-chat-admin-admin-api` transport package
- any UI-facing helper inside the app must wrap the formal SDK rather than reimplement HTTP calls

## Release Preparation

This workspace follows the same release preparation discipline as `sdkwork-craw-chat-sdk`:

- contract capture is reviewable
- generated output is reproducible
- manual composition is isolated
- verification is explicit
- package names, client names, and directory layout stay stable across languages

## Language Workspaces

- TypeScript: [sdkwork-craw-chat-sdk-admin-typescript](./sdkwork-craw-chat-sdk-admin-typescript/README.md)
- Flutter: [sdkwork-craw-chat-sdk-admin-flutter](./sdkwork-craw-chat-sdk-admin-flutter/README.md)

## Documentation

- Site overview: `docs/sites/sdk/admin-sdk.md`
- TypeScript guide: `docs/sites/sdk/admin-typescript-sdk.md`
- Flutter guide: `docs/sites/sdk/admin-flutter-sdk.md`
- Control-plane API overview: `docs/sites/api-reference/control-plane-api.md`
