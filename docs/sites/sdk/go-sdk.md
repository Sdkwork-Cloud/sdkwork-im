# Go SDK

The Go workspace is a Tier B member of the `craw-chat-sdk` business SDK family and currently ships
as a transport-standardized module.

## Current Delivery Reality

This page describes the current checked-in repo contract validated on 2026-04-16. It does not by
itself claim module proxy publication or a shipped semantic Go client.

Today the verified Go entrypoint is the generated module under `generated/server-openapi`. The
semantic Go module remains reserved under `composed`.

## Package Contract

| Concern | Value |
| --- | --- |
| Maturity tier | Tier B |
| Generated transport module | `github.com/sdkwork/craw-chat-backend-sdk` |
| Raw generated client | `SdkworkBackendClient` |
| Reserved semantic module | `github.com/sdkwork/craw-chat-sdk` |
| Target business client | `CrawChatSdkClient` |
| Generator-owned boundary | `generated/server-openapi` |
| Manual semantic boundary | `composed` |

## What Ships Today

- live-schema generation from the Craw Chat OpenAPI 3.x export
- verified generated module naming and assembly metadata
- a stable split between `generated/server-openapi` and `composed`
- a raw generated transport client named `SdkworkBackendClient`

For exact import examples and raw transport usage, use `generated/server-openapi/README.md` as the
transport reference.

## Raw Generated Client

If you are integrating Go today, start from the generated module and `SdkworkBackendClient`.

- generated module: `github.com/sdkwork/craw-chat-backend-sdk`
- raw generated client: `SdkworkBackendClient`
- reserved semantic module: `github.com/sdkwork/craw-chat-sdk`

That is the checked-in Go surface the workspace verifies today.

## API Reference Map

Use `generated/server-openapi/README.md` together with `SdkworkBackendClient` when you need the
exact Go route-group names and DTO entrypoints. Use the map below to jump from transport concern
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

This keeps the Go page precise: the repo-standard delivery today is transport-first, so the API
reference plus `generated/server-openapi/README.md` remains the exact route authority until a
future semantic `CrawChatSdkClient` is implemented under `composed`.

## What Is Not Shipped Yet

- no checked-in semantic Go module that already exposes `CrawChatSdkClient`
- no handwritten message-first business layer above generated route groups
- no delivered websocket live runtime abstraction above the generated transport module

Treat this page as a repo contract for current Go delivery, not as a claim of semantic parity.

## When To Use `composed`

Use `composed` only when you are intentionally implementing the future semantic Go layer:

- `CrawChatSdkClient`
- business wrappers above raw route groups
- message-first helpers
- live runtime orchestration above transport-level coordination

Do not hand-edit generated Go files under `generated/server-openapi`.

## Generate And Verify

Root workspace:

```powershell
powershell -ExecutionPolicy Bypass -File .\sdks\sdkwork-craw-chat-sdk\bin\generate-sdk.ps1 -Languages go
node .\sdks\sdkwork-craw-chat-sdk\bin\verify-sdk.mjs --language go
```

Go workspace wrappers:

```powershell
.\sdks\sdkwork-craw-chat-sdk\sdkwork-craw-chat-sdk-go\bin\sdk-gen.ps1
.\sdks\sdkwork-craw-chat-sdk\sdkwork-craw-chat-sdk-go\bin\sdk-verify.ps1
```

```bash
./sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-go/bin/sdk-gen.sh
./sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-go/bin/sdk-verify.sh
```

## When To Choose Go

- Choose Go when you need a verified transport module generated from the Craw Chat schema for
  services or automation.
- Choose Go when you can work directly against generated request or response models and
  transport-level route groups.
- Choose TypeScript or Flutter when you need a checked-in semantic SDK rather than a
  transport-standardized boundary.

## What To Read Next

- Read [Language Support](/sdk/language-support) when you need the full cross-language maturity
  matrix and current Go delivery status.
- Read [Generator Boundary](/sdk/generator-boundary) when you need the exact ownership split
  between `generated/server-openapi` and `composed`.
- Read [TypeScript SDK](/sdk/typescript-sdk) when you need the current semantic baseline rather
  than the Go transport-first workspace.
- Read [Portal and Auth](/api-reference/app/portal-and-auth), [Conversations](/api-reference/app/conversations),
  and [Messages](/api-reference/app/messages) when you need the exact HTTP contract behind the
  generated Go transport.
- Read [Session and Realtime](/api-reference/app/session-and-realtime) and [RTC](/api-reference/app/rtc)
  when you need the route-level transport contract for live coordination and RTC workflows.
