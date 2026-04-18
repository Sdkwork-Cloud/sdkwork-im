# App SDK

The app SDK family is the primary integration surface for embedding Craw Chat conversation,
presence, media, stream, and RTC capabilities into web, mobile, and backend applications.

## Audience And Scope

Use this family when you need app-facing product features such as:

- session resume and disconnect
- presence heartbeat and current-presence reads
- realtime subscription sync, event pull, and event acknowledgement
- device registration and device sync feed reads
- inbox, conversations, members, read cursors, and timeline navigation
- messages, media, streams, and RTC

Do not use this family for:

- control-plane governance
- node lifecycle administration
- audit, ops, or diagnostics-only routes
- internal trusted-header-only workflows as a public integration model

## Workspace Layout

- root workspace: `sdks/sdkwork-craw-chat-sdk`
- authority contract: `sdks/sdkwork-craw-chat-sdk/openapi/craw-chat-app.openapi.yaml`
- derived generator input: `sdks/sdkwork-craw-chat-sdk/openapi/craw-chat-app.sdkgen.yaml`
- language workspaces:
  - `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-typescript`
  - `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-flutter`
  - `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-rust`

## Package Layers

| Language | Preferred public package or crate | Generated transport package or crate | Public entrypoint |
| --- | --- | --- | --- |
| TypeScript | `@sdkwork/craw-chat-sdk` | `@sdkwork/craw-chat-backend-sdk` | `CrawChatClient.create()` |
| Flutter | `craw_chat_sdk` | `backend_sdk` | `CrawChatClient.create()` |
| Rust | `craw-chat-sdk` | `sdkwork-craw-chat-backend-sdk` | `CrawChatClient::new_with_base_url()` |

For all three languages:

- `generated/server-openapi` is generator-owned transport output
- `composed` is the manual-owned integration layer you should import from
- the composed layer exposes `CrawChatClient`, semantic modules, and builder helpers

## Client Surface

The composed client surfaces are aligned by capability:

- `session`
- `presence`
- `realtime`
- `devices`
- `inbox`
- `conversations`
- `messages`
- `media`
- `streams`
- `rtc`

Convenience builders are also available for common text-message, text-stream-frame, and JSON RTC
signal payload construction.

## Contract Source

The app SDK family is generated from a checked-in authority contract:

- authority contract: `openapi/craw-chat-app.openapi.yaml`
- default generator input: `openapi/craw-chat-app.sdkgen.yaml`
- Flutter-specific generator input: `openapi/craw-chat-app.flutter.sdkgen.yaml`

The canonical route surface still originates from `services/local-minimal-node/src/node/build.rs`,
but the SDK workspace uses the checked-in OpenAPI authority as its explicit consumer contract.

## Auth Model

The public app SDK contract is bearer-token based.

- SDK consumers should use `Authorization: Bearer <token>`
- public auth is signed with `CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET`
- trusted headers remain internal and test-oriented, not the public SDK contract

## Realtime Boundary

The app SDK family includes the realtime HTTP coordination surface:

- replace or sync subscriptions
- pull event windows
- acknowledge consumed events

The authority contract also documents `GET /api/v1/realtime/ws`, but this round does not claim a
full manual WebSocket adapter in the composed SDK layers. The docs may describe the transport and
recovery contract without overstating SDK implementation.

## Ownership Rules

- change the authority contract or generator inputs when you need transport changes
- regenerate instead of hand-editing generated files
- keep manual composition in the `composed` layer
- route consumers to `composed`, not to generated private source paths

## Regeneration

From the app SDK workspace root:

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\generate-sdk.ps1 -Languages typescript,flutter,rust
```

```bash
./bin/generate-sdk.sh --language typescript --language flutter --language rust
```

Per-language forwarding wrappers also exist inside each language workspace for focused generation
and verification.

## Reading Path

- start with [Language Support](/sdk/language-support)
- then pick a quick start:
  [TypeScript](/sdk/typescript-quick-start),
  [Flutter](/sdk/flutter-quick-start),
  [Rust](/sdk/rust-quick-start)
- use [Auth and Client Init](/sdk/auth-and-client-init) for shared setup rules
- use [Module Map](/sdk/module-map) to route yourself to the correct capability page

## Publication Boundary

These docs describe the implemented local SDK workspaces in this repository. Public package
publication remains separate from local workspace presence and should not be inferred from the
existence of the packages or crates alone.
