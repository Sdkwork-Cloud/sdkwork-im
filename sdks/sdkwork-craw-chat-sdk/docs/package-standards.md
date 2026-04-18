# SDK Package Standards

This document defines the package-contract rules that apply inside
`sdks/sdkwork-craw-chat-sdk`.

Use it when you need the exact naming, ownership, and public-entrypoint standards for the Craw Chat
app SDK family.

## TypeScript Standard

The TypeScript SDK uses one official consumer package per workspace:

- SDK root directory: `sdkwork-craw-chat-sdk`
- published package: `@sdkwork/craw-chat-sdk`
- runtime targets: browser and Node.js
- primary client: `CrawChatSdkClient`

### TypeScript Ownership Boundary

- generated OpenAPI transport is authored under `sdkwork-craw-chat-sdk-typescript/generated/server-openapi`
- assembled generated transport lives under `sdkwork-craw-chat-sdk-typescript/src/generated/**`
- handwritten business modules live under `sdkwork-craw-chat-sdk-typescript/src/**` outside `src/generated/**`
- manual authoring happens in `sdkwork-craw-chat-sdk-typescript/composed`

### TypeScript Public Contract

App consumers should import only from `@sdkwork/craw-chat-sdk`.

That package must continue to expose:

- `CrawChatSdkClient`
- `SdkworkBackendClient`
- `createGeneratedBackendClient`
- `generated`

The public realtime contract must continue to expose:

- payload-first live domain streams: `live.messages`, `live.data`, `live.signals`, `live.events`,
  and `live.lifecycle`
- no legacy flat callbacks such as `live.onMessage(...)`, `live.onSignal(...)`, or
  `live.onData(...)`
- resolved live lifecycle states limited to `connected`, `error`, and `closed`

Do not document TypeScript as a two-package consumer model.
`generated/server-openapi` and `composed` are authoring boundaries, not the downstream package
selection model.

## Flutter Standard

The Flutter SDK uses an official consumer package plus a generated transport package:

- official app-facing package: `craw_chat_sdk`
- primary client: `CrawChatClient`
- official consumer entrypoint: `package:craw_chat_sdk/craw_chat_sdk.dart`
- generator-owned transport package: `backend_sdk`
- transport entrypoint: `package:backend_sdk/backend_sdk.dart`

### Flutter Ownership Boundary

- generated transport is owned by `sdkwork-craw-chat-sdk-flutter/generated/server-openapi`
- manual consumer logic is owned by `sdkwork-craw-chat-sdk-flutter/composed`
- `craw_chat_sdk` re-exports `backend_sdk`

### Flutter Public Contract

For most Flutter app integrations, document `craw_chat_sdk` first.

Only direct transport consumers should start from `backend_sdk`.
Do not describe Flutter as a neutral generated-versus-composed package choice when the official
consumer package is already known.

## Shared Naming Rules

- Prefer real package names over directory nicknames.
- Prefer public client class names over generic phrases like "composed client".
- Prefer public entrypoints over internal source paths.
- Keep generated code under stable `generated` naming and keep manual business code under the real
  business package name.

## Source Of Truth Links

- workspace overview: `../README.md`
- internal docs map: `README.md`
- public docs sync standard: `sites/README.md`
- generation pipeline: `generation-pipeline.md`
- verification matrix: `verification-matrix.md`
- realtime extension boundary: `realtime-extension-boundary.md`
