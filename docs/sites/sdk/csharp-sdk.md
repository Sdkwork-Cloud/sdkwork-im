# C# SDK

The C# workspace is a Tier B member of the `craw-chat-sdk` business SDK family and currently ships
as a transport-standardized .NET lane.

## Current Delivery Reality

This page describes the current checked-in repo contract validated on 2026-04-16. It does not
itself claim NuGet publication or a shipped semantic `.NET` client.

Today the verified C# entrypoint is the generated package under `generated/server-openapi`. The
future semantic .NET package remains reserved under `composed`.

## Package Contract

| Concern | Value |
| --- | --- |
| Maturity tier | Tier B |
| Generated transport package | `Sdkwork.CrawChat.BackendSdk` |
| Raw generated namespace | `Sdkwork.CrawChat.BackendSdk` |
| Raw generated client | `SdkworkBackendClient` |
| Reserved semantic package | `Sdkwork.CrawChat.Sdk` |
| Target business client | `CrawChatSdkClient` |
| Generator-owned boundary | `generated/server-openapi` |
| Manual semantic boundary | `composed` |

## What Ships Today

- live-schema generation from the Craw Chat OpenAPI 3.x contract
- verified generated package naming and assembly metadata
- a stable ownership split between `generated/server-openapi` and `composed`
- a raw generated transport client named `SdkworkBackendClient`

For install commands, generated namespace usage, and raw API examples, use
`generated/server-openapi/README.md` as the exact transport reference.

## Raw Generated Client

If you are integrating C# today, start from the generated package and `SdkworkBackendClient`.

- generated package: `Sdkwork.CrawChat.BackendSdk`
- raw generated client: `SdkworkBackendClient`
- reserved semantic package: `Sdkwork.CrawChat.Sdk`

That is the current checked-in .NET surface that the workspace verifies today.

## API Reference Map

Use `generated/server-openapi/README.md` together with `SdkworkBackendClient` when you need the
exact C# route-group names and DTO entrypoints. Use the map below to jump from transport concern
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

This keeps the C# page precise: the repo-standard delivery today is transport-first, so the API
reference plus `generated/server-openapi/README.md` remains the exact route authority until a
future semantic `CrawChatSdkClient` is implemented under `composed`.

## What Is Not Shipped Yet

- no checked-in semantic package that already exposes `CrawChatSdkClient`
- no handwritten message-first SDK layer above generated route groups
- no delivered websocket live runtime surface comparable to the TypeScript SDK

Treat this page as a repo contract for current .NET delivery, not as proof of full semantic parity.

## When To Use `composed`

Use `composed` only when you are intentionally implementing the future semantic .NET layer:

- `CrawChatSdkClient`
- higher-level business wrappers
- message-first helpers
- live runtime orchestration above transport-level coordination

Do not hand-edit generated C# files under `generated/server-openapi`.

## Generate And Verify

Root workspace:

```powershell
powershell -ExecutionPolicy Bypass -File .\sdks\sdkwork-craw-chat-sdk\bin\generate-sdk.ps1 -Languages csharp
node .\sdks\sdkwork-craw-chat-sdk\bin\verify-sdk.mjs --language csharp
```

C# workspace wrappers:

```powershell
.\sdks\sdkwork-craw-chat-sdk\sdkwork-craw-chat-sdk-csharp\bin\sdk-gen.ps1
.\sdks\sdkwork-craw-chat-sdk\sdkwork-craw-chat-sdk-csharp\bin\sdk-verify.ps1
```

```bash
./sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-csharp/bin/sdk-gen.sh
./sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-csharp/bin/sdk-verify.sh
```

## When To Choose C#

- Choose C# when you need a verified .NET transport package generated from the Craw Chat app API.
- Choose C# when your application or service can work directly against generated request or
  response models and route-group methods.
- Choose TypeScript when you need the current message-first and live-runtime semantic baseline.

## What To Read Next

- Read [Language Support](/sdk/language-support) when you need the full cross-language maturity
  matrix and current .NET delivery status.
- Read [Generator Boundary](/sdk/generator-boundary) when you need the exact ownership split
  between `generated/server-openapi` and `composed`.
- Read [TypeScript SDK](/sdk/typescript-sdk) when you need the current semantic baseline rather
  than the C# transport-first workspace.
- Read [Portal and Auth](/api-reference/app/portal-and-auth), [Conversations](/api-reference/app/conversations),
  and [Messages](/api-reference/app/messages) when you need the exact HTTP contract behind the
  generated C# transport.
- Read [Session and Realtime](/api-reference/app/session-and-realtime) and [RTC](/api-reference/app/rtc)
  when you need the route-level transport contract for live coordination and RTC workflows.
