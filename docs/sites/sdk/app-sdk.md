# App SDK

## Workspace Layout

- Root workspace: `sdks/sdkwork-craw-chat-sdk`
- Authority contract: `sdks/sdkwork-craw-chat-sdk/openapi/craw-chat-app.openapi.yaml`
- Derived sdkgen input: `sdks/sdkwork-craw-chat-sdk/openapi/craw-chat-app.sdkgen.yaml`
- Language workspaces:
  - `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-typescript`
  - `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-flutter`

## Scope

The app SDK workspace is intended to generate app-facing HTTP SDK support for:

- sessions and presence
- realtime HTTP coordination
- device registration and sync feed
- inbox, conversations, membership, and read cursor
- messages and mutation flows
- media upload, lookup, and attachment
- stream lifecycle and frame transport
- RTC lifecycle, signals, and participant credentials

It intentionally excludes:

- control-plane governance APIs
- ops, audit, and diagnostics routes
- IoT routes
- provider-health-only routes

## Contract Source

The canonical route surface still comes from
`services/local-minimal-node/src/node/build.rs`. The app SDK workspace then stores two checked-in
contract files:

- `craw-chat-app.openapi.yaml`
  The OpenAPI 3.0.3 authority contract.
- `craw-chat-app.sdkgen.yaml`
  The generator-compatible derived contract.
- `craw-chat-app.flutter.sdkgen.yaml`
  The Flutter-compatible derived contract used for Dart generation.

For this family, the checked-in OpenAPI authority is real and should be documented as such.

## Auth Model

The app SDK models public bearer auth only:

- `Authorization: Bearer <token>`
- signed with `CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET`

Trusted headers are still valid for tests and embedded internal flows, but they are not the public
consumer contract for generated SDK packages.

## Realtime Boundary

The authority contract includes `GET /api/v1/realtime/ws` for visibility, but the current SDK
generation round covers HTTP only.

- generated support covers resume, subscription sync, event pull, and ack flow
- websocket transport notes remain manual-owned
- the current workspace docs explicitly call out close code `4001` and `session.disconnect` as
  transport considerations rather than generated adapter behavior

## Regeneration

From the workspace root:

```powershell
.\bin\generate-sdk.ps1 -Languages typescript,flutter
```

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\generate-sdk.ps1 -Languages typescript,flutter
```

```bash
./bin/generate-sdk.sh --language typescript --language flutter
```

Per-language forwarding wrappers are also present inside each language workspace.

## Assembly Metadata

The workspace refreshes `.sdkwork-assembly.json` as part of root verification and final assembly.

That release-facing metadata file records:

- the authority and derived spec paths
- one language entry per workspace
- each generated package `manifestPath`
- the full `packages` layer list so automation can distinguish `generated` and `composed`
- a `generatedAt` timestamp that stays stable when assembly content is unchanged

This lets release tooling, docs verification, and workspace audits discover package manifests and
published entrypoints without walking the whole repository tree.

## Verification

From the workspace root:

```powershell
node .\bin\verify-sdk.mjs
```

If the machine has a healthy Dart toolchain, opt into native Dart verification explicitly:

```powershell
node .\bin\verify-sdk.mjs --with-dart
```

Root verification refreshes `.sdkwork-assembly.json` after the TypeScript and Flutter checks pass.

## Language Guides

- [TypeScript SDK](/sdk/typescript-sdk)
- [Flutter SDK](/sdk/flutter-sdk)
- TypeScript package docs live under `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-typescript`

## Current Release Status

| Artifact | Language | Generation state | Release state |
| --- | --- | --- | --- |
| `sdkwork-craw-chat-sdk-typescript` | TypeScript | `template_only_pending_generation` | `not_published` |
| `sdkwork-craw-chat-sdk-flutter` | Flutter | `template_only_pending_generation` | `not_published` |

The current documentation standard therefore distinguishes between:

- a real checked-in OpenAPI and workspace contract
- a real generation wrapper layout
- a release wave that still does not publish packages
