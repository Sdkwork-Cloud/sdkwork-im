# Rust SDK

The Rust workspace is the first non-TypeScript language in the Tier A target set for the Craw Chat
app SDK family, but the checked-in Rust delivery is still transport-first today.

## Current Delivery Reality

This page describes the current checked-in repo contract validated on 2026-04-16. It is a
statement of the repo contract, not a claim that crates.io publication or TypeScript-level semantic
parity already exists.

Today the real Rust consumption boundary is the generated transport crate under
`generated/server-openapi`. The future semantic crate stays reserved under `composed`.

## Package Contract

| Concern | Value |
| --- | --- |
| Maturity tier | Tier A |
| Generated transport crate package | `sdkwork-craw-chat-backend-sdk` |
| Generated Rust import crate | `sdkwork_craw_chat_backend_sdk` |
| Raw generated client | `SdkworkBackendClient` |
| Reserved semantic crate package | `craw-chat-sdk` |
| Reserved semantic crate import | `craw_chat_sdk` |
| Target business client | `CrawChatSdkClient` |
| Generator-owned boundary | `generated/server-openapi` |
| Manual semantic boundary | `composed` |

## What Ships Today

- live-schema generation from the same Craw Chat OpenAPI 3.x export as every official language
- verified transport crate naming and assembly metadata
- a stable generated-versus-semantic split between `generated/server-openapi` and `composed`
- a raw generated transport client named `SdkworkBackendClient`

For exact transport installation and raw usage examples, start from
`generated/server-openapi/README.md`.

## Raw Generated Client

If you are integrating Rust today, start with the generated transport crate and
`SdkworkBackendClient`.

- package name: `sdkwork-craw-chat-backend-sdk`
- Rust import crate: `sdkwork_craw_chat_backend_sdk`
- current raw client: `SdkworkBackendClient`

This is the verified checked-in Rust entrypoint today. The semantic crate target `craw_chat_sdk`
and business client `CrawChatSdkClient` remain future work above the transport layer.

## API Reference Map

Use `generated/server-openapi/README.md` together with `SdkworkBackendClient` when you need the
exact Rust route-group names and DTO entrypoints. Use the map below to jump from transport concern
to the matching HTTP reference:

| Transport concern | Generated transport focus today | Exact API reference |
| --- | --- | --- |
| Auth and portal shell reads | auth and portal route groups on `SdkworkBackendClient` | [Portal and Auth](/api-reference/app/portal-and-auth) |
| Conversation lifecycle and handoff | conversation route groups on `SdkworkBackendClient` | [Conversations](/api-reference/app/conversations) |
| Membership and read cursors | conversation membership and read-state route groups | [Membership and Read State](/api-reference/app/membership-and-read-state) |
| Message send payloads and timeline schemas | message route groups and DTOs | [Messages](/api-reference/app/messages) |
| Upload and attachment lifecycle | media route groups and DTOs | [Media](/api-reference/app/media) |
| Session, presence, and realtime coordination | session, presence, and realtime route groups | [Session and Realtime](/api-reference/app/session-and-realtime) |
| Device registration and sync feeds | device route groups | [Device Sync](/api-reference/app/device-sync) |
| RTC lifecycle and signaling-side HTTP operations | rtc route groups | [RTC](/api-reference/app/rtc) |
| Stream ingestion and checkpoints | stream route groups | [Streams](/api-reference/app/streams) |

This keeps the Rust page precise: the repo-standard delivery today is transport-first, so the API
reference plus `generated/server-openapi/README.md` remains the exact route authority until a
future semantic `CrawChatSdkClient` is implemented under `composed`.

## What Is Not Shipped Yet

- no checked-in semantic Rust crate that already exposes `CrawChatSdkClient`
- no handwritten message-first business layer comparable to TypeScript
- no delivered websocket live runtime abstraction above the generated transport boundary

That is why this page treats Rust as a repo contract and transport-first workspace today, not as a
finished semantic app SDK.

## When To Use `composed`

Use `composed` only when you are intentionally implementing the next Rust semantic layer:

- business-facing `CrawChatSdkClient`
- ergonomic wrappers above raw route groups
- message-first helpers
- realtime orchestration above transport-level coordination

Do not hand-edit generated Rust files under `generated/server-openapi`.

## Generate And Verify

Root workspace:

```powershell
powershell -ExecutionPolicy Bypass -File .\sdks\sdkwork-craw-chat-sdk\bin\generate-sdk.ps1 -Languages rust
node .\sdks\sdkwork-craw-chat-sdk\bin\verify-sdk.mjs --language rust
```

Rust workspace wrappers:

```powershell
.\sdks\sdkwork-craw-chat-sdk\sdkwork-craw-chat-sdk-rust\bin\sdk-gen.ps1
.\sdks\sdkwork-craw-chat-sdk\sdkwork-craw-chat-sdk-rust\bin\sdk-verify.ps1
```

```bash
./sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-rust/bin/sdk-gen.sh
./sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-rust/bin/sdk-verify.sh
```

## When To Choose Rust

- Choose Rust when you need a verified transport-standardized SDK in the first non-TypeScript Tier
  A target language.
- Choose Rust when you are building service-side or systems-side integration and can work directly
  against the generated transport boundary.
- Choose TypeScript instead when you need the checked-in message-first, portal-ready, live-runtime
  semantic SDK today.

## What To Read Next

- Read [Language Support](/sdk/language-support) when you need the full cross-language maturity
  matrix and Tier A versus Tier B boundaries.
- Read [Generator Boundary](/sdk/generator-boundary) when you need the exact ownership split
  between `generated/server-openapi` and `composed`.
- Read [TypeScript SDK](/sdk/typescript-sdk) when you need the current semantic baseline that Rust
  is converging toward.
- Read [Portal and Auth](/api-reference/app/portal-and-auth), [Conversations](/api-reference/app/conversations),
  and [Messages](/api-reference/app/messages) when you need the exact HTTP contract behind the
  generated Rust transport.
- Read [Session and Realtime](/api-reference/app/session-and-realtime) and [RTC](/api-reference/app/rtc)
  when you need the route-level transport contract for live coordination and RTC workflows.
