# SDK Overview

The Craw Chat repository currently exposes two SDK families:

- `sdkwork-craw-chat-sdk`
  App-facing runtime integrations generated from the live Craw Chat OpenAPI 3.x contract.
- `sdkwork-craw-chat-sdk-admin`
  Admin and control-plane integrations.

For application integration, start from the app SDK family and the TypeScript package
`@sdkwork/craw-chat-sdk`. That remains the strongest checked-in consumer SDK and the reference
contract for future semantic SDKs in other languages.

## Start Here

| Need | Start with |
| --- | --- |
| Public app runtime integration | [App SDK](/sdk/app-sdk) |
| Shared auth and client bootstrap rules | [Auth and Client Init](/sdk/auth-and-client-init) |
| Fastest browser or Node onboarding | [TypeScript Quick Start](/sdk/typescript-quick-start) |
| Fastest Flutter onboarding | [Flutter Quick Start](/sdk/flutter-quick-start) |
| Fastest Rust onboarding | [Rust Quick Start](/sdk/rust-quick-start) |
| Real TypeScript package, imports, and examples | [TypeScript SDK](/sdk/typescript-sdk) |
| Current language parity, tiers, and transport status | [Language Support](/sdk/language-support) |
| Generated-versus-semantic ownership rules | [Generator Boundary](/sdk/generator-boundary) |
| Route-level HTTP semantics | [App API Overview](/api-reference/app-api) |
| Admin and control-plane workflows | [Admin SDK](/sdk/admin-sdk) |

## Choose By Scenario

Use this rule of thumb before reading the rest of the matrix:

- If you need the richest app-facing SDK today, start with TypeScript and
  [TypeScript SDK](/sdk/typescript-sdk).
- If you are building Flutter UI integration, start with [Flutter SDK](/sdk/flutter-sdk) and treat
  websocket live runtime plus message-first builders as current parity gaps.
- If you want the shortest path from zero to a running integration, use the quick-start sequence:
  [TypeScript](/sdk/typescript-quick-start),
  [Flutter](/sdk/flutter-quick-start),
  [Rust](/sdk/rust-quick-start), then
  [Module Map](/sdk/module-map).
- If you need generated transport only for JVM, .NET, Swift, Go, Python, or current Rust service
  integration, start from the language page for that transport-standardized workspace.
- If you are building control-plane or governance tooling, skip the app SDK family and start with
  [Admin SDK](/sdk/admin-sdk).

## Quick Starts And Module Docs

Use the fast-start set when you are wiring a new client instead of evaluating overall workspace
policy:

| Need | Best page |
| --- | --- |
| Shared base URL, token, and client creation rules | [Auth and Client Init](/sdk/auth-and-client-init) |
| Package-to-capability lookup | [Module Map](/sdk/module-map) |
| TypeScript bootstrap | [TypeScript Quick Start](/sdk/typescript-quick-start) |
| Flutter bootstrap | [Flutter Quick Start](/sdk/flutter-quick-start) |
| Rust bootstrap | [Rust Quick Start](/sdk/rust-quick-start) |
| Capability-focused deep dives | [/sdk/modules/session-and-presence](/sdk/modules/session-and-presence), [/sdk/modules/realtime](/sdk/modules/realtime), [/sdk/modules/conversations](/sdk/modules/conversations), [/sdk/modules/messages](/sdk/modules/messages), [/sdk/modules/media](/sdk/modules/media), [/sdk/modules/streams](/sdk/modules/streams), [/sdk/modules/rtc](/sdk/modules/rtc) |
| Scenario walkthroughs | [/sdk/examples/session-bootstrap](/sdk/examples/session-bootstrap), [/sdk/examples/conversation-workflow](/sdk/examples/conversation-workflow), [/sdk/examples/message-and-media](/sdk/examples/message-and-media), [/sdk/examples/stream-and-rtc](/sdk/examples/stream-and-rtc) |

## API Reference Router

Use the SDK guides for package choice and runtime ergonomics. Jump to the route-level reference
when you need exact HTTP payloads, DTOs, or operation semantics:

| Need | Exact API reference |
| --- | --- |
| Auth, current session, and portal shell snapshots | [Portal and Auth](/api-reference/app/portal-and-auth) |
| Conversation creation, timeline entrypoints, and app conversation lifecycle | [Conversations](/api-reference/app/conversations) |
| Message payload schemas and send-route semantics | [Messages](/api-reference/app/messages) |
| Upload preparation, completion, download, and attachment flows | [Media](/api-reference/app/media) |
| Session lifecycle, live subscriptions, and realtime coordination | [Session and Realtime](/api-reference/app/session-and-realtime) |
| RTC session lifecycle and signaling-side HTTP operations | [RTC](/api-reference/app/rtc) |
| Stream open, append, checkpoint, complete, and abort flows | [Streams](/api-reference/app/streams) |
| Control-plane protocol registry and governance rules | [Protocol Governance](/api-reference/control-plane/protocol) |
| Provider policy, routing, and provider governance | [Provider Governance](/api-reference/control-plane/providers) |
| Drain, activation, and node lifecycle operations | [Node Operations](/api-reference/control-plane/nodes) |

## App SDK Language Matrix

Read the matrix this way:

- `Official consumer package`
  The package normal application code should import today.
- generated transport artifact
  The verified transport-level package or module that exists today when a semantic package is not
  yet shipped for that language.

| Language | Tier | Current public surface | Primary client | Best current reading |
| --- | --- | --- | --- | --- |
| TypeScript | Tier A | `@sdkwork/craw-chat-sdk` with generated transport assembled under `src/generated/**` | `CrawChatSdkClient` | [TypeScript SDK](/sdk/typescript-sdk) |
| Flutter | Tier A | `craw_chat_sdk` above generated `backend_sdk` | `CrawChatClient` | [Flutter SDK](/sdk/flutter-sdk) |
| Rust | Tier A target | Generated crate `sdkwork-craw-chat-backend-sdk`; semantic crate target `craw_chat_sdk` remains under `composed` | `CrawChatSdkClient` target | [Rust SDK](/sdk/rust-sdk) |
| Java | Tier B | Generated artifact `com.sdkwork:craw-chat-backend-sdk`; semantic reserve under `composed` | `CrawChatSdkClient` target | [Java SDK](/sdk/java-sdk) |
| C# | Tier B | Generated package `Sdkwork.CrawChat.BackendSdk`; semantic reserve under `composed` | `CrawChatSdkClient` target | [C# SDK](/sdk/csharp-sdk) |
| Swift | Tier B | Generated package `CrawChatBackendSdk`; semantic reserve under `composed` | `CrawChatSdkClient` target | [Swift SDK](/sdk/swift-sdk) |
| Kotlin | Tier B | Generated artifact `com.sdkwork:craw-chat-backend-sdk`; semantic reserve under `composed` | `CrawChatSdkClient` target | [Kotlin SDK](/sdk/kotlin-sdk) |
| Go | Tier B | Generated module `github.com/sdkwork/craw-chat-backend-sdk`; semantic reserve under `composed` | `CrawChatSdkClient` target | [Go SDK](/sdk/go-sdk) |
| Python | Tier B | Generated package `sdkwork-craw-chat-backend-sdk`; semantic reserve under `composed` | `CrawChatSdkClient` target | [Python SDK](/sdk/python-sdk) |

For every non-TypeScript language without a shipped semantic client, the real checked-in transport
entrypoint is the generated `SdkworkBackendClient` surface in that language's generated package,
while `CrawChatSdkClient` remains the target semantic client name for a later manual layer.

## Tier Model

### Tier A

Tier A languages are the semantic-SDK baseline. They are the languages the workspace is actively
driving toward the TypeScript standard of:

- one business-facing client
- a documented generated-versus-semantic boundary
- workflow-first app guidance instead of route-group-only guidance

Today, TypeScript is fully in that shape. Flutter is a checked-in consumer SDK with known parity
gaps. Rust is the next Tier A target, with the generated transport verified and the manual semantic
boundary reserved under `composed`.

### Tier B

Tier B languages are transport-standardized first. The workspace guarantees:

- the language is generated from the same live schema
- the generated transport lands under `generated/server-openapi`
- a manual semantic reserve exists under `composed`
- package naming, verification, and docs stay aligned

Tier B does not claim TypeScript-level live runtime, message builder, or RTC semantic parity until
those handwritten layers actually exist.

## TypeScript Standard

The TypeScript app SDK remains the reference implementation:

- official consumer package: `@sdkwork/craw-chat-sdk`
- primary client: `CrawChatSdkClient`
- synchronous construction: `new CrawChatSdkClient({...})`
- flat config with `baseUrl`, `apiBaseUrl`, `websocketBaseUrl`, `authToken`, and
  `webSocketFactory`
- message-first outbound APIs through `sdk.createXxxMessage(...)` and `sdk.send(...)`
- payload-first domain receive APIs through `sdk.connect(...)` and durable replay through `sdk.sync`
- low-level generated transport available from the same package through `SdkworkBackendClient` and
  `generated`

## Choose The Right Surface

| Need | Best fit |
| --- | --- |
| Application integration that needs the richest checked-in semantics | `CrawChatSdkClient` in `@sdkwork/craw-chat-sdk` |
| Route-aligned Dart integration above a generated package | `CrawChatClient` in `craw_chat_sdk` |
| Transport-standardized JVM/.NET/Swift/Go/Python work | The generated transport artifact documented on the language page |
| Generated-versus-semantic rules before extending a language | [Generator Boundary](/sdk/generator-boundary) |
| Exact generated DTOs and route groups | `SdkworkBackendClient`, `generated`, or the generated transport artifact for that language |

## Contract Source

The app SDK family is generated from the Craw Chat app OpenAPI 3.x contract exported by the
running service at `/openapi/craw-chat-app.openapi.yaml`.

For every official language:

- the checked-in authority snapshot lives under `sdks/sdkwork-craw-chat-sdk/openapi/craw-chat-app.openapi.yaml`
- the generator-owned transport boundary lives under `generated/server-openapi`
- the handwritten semantic boundary lives under `composed`, except for the TypeScript single-package
  assembly that exposes the generated layer under `src/generated/**`

## What To Read Next

- [App SDK](/sdk/app-sdk)
- [TypeScript SDK](/sdk/typescript-sdk)
- [Flutter SDK](/sdk/flutter-sdk)
- [Rust SDK](/sdk/rust-sdk)
- [TypeScript Quick Start](/sdk/typescript-quick-start)
- [Flutter Quick Start](/sdk/flutter-quick-start)
- [Rust Quick Start](/sdk/rust-quick-start)
- [Auth and Client Init](/sdk/auth-and-client-init)
- [Module Map](/sdk/module-map)
- [Java SDK](/sdk/java-sdk)
- [C# SDK](/sdk/csharp-sdk)
- [Swift SDK](/sdk/swift-sdk)
- [Kotlin SDK](/sdk/kotlin-sdk)
- [Go SDK](/sdk/go-sdk)
- [Python SDK](/sdk/python-sdk)
- [Generator Boundary](/sdk/generator-boundary)
