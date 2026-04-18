# Language Support

This page measures language support by workspace boundary, generated transport package, semantic
ownership, and verified delivery status. It does not treat "supported" as a synonym for
"already published."

## Current Verified Baseline

The current checked-in baseline was revalidated on 2026-04-16 from the live Craw Chat OpenAPI 3.x
export.

- all nine official language workspaces regenerated successfully from the live schema
- the root SDK verification matrix passed for TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin,
  Go, and Python
- the docs site verification also passed after regenerating the operation pages

Read the matrix below as a statement about the real checked-in repo contract, not an assumption
about registry publication or cross-language parity.

## Tier Definitions

### Tier A

Tier A languages are the semantic-SDK baseline. They are expected to converge on:

- one business-facing client
- a documented generated-versus-semantic boundary
- workflow-first app guidance instead of route-group-only guidance

Today that means:

- TypeScript is the full checked-in baseline.
- Flutter is a checked-in consumer SDK with known parity gaps.
- Rust is the next Tier A target, with the generated transport verified and the semantic boundary
  reserved under `composed`.

### Tier B

Tier B languages are transport-standardized first. The workspace guarantees:

- live-schema generation from `/openapi/craw-chat-app.openapi.yaml`
- generated output under `generated/server-openapi`
- a manual semantic reserve under `composed`
- naming, verification, and docs alignment

Tier B does not imply the language already ships TypeScript-level message builders, `sdk.connect(...)`,
or other handwritten live runtime ergonomics.

The verified guarantee is narrower and more precise: the language workspace is wired into the live
schema generation flow, its generated/manual boundaries are standardized, and its checked-in docs
and verification scripts agree on what is actually delivered today.

## App SDK Language Matrix

| Language | Tier | Workspace | Official consumer package or current transport artifact | Current primary client status | Generated boundary | Current delivery |
| --- | --- | --- | --- | --- | --- | --- |
| TypeScript | Tier A | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-typescript` | `@sdkwork/craw-chat-sdk` | `CrawChatSdkClient` ships today | `generated/server-openapi`, assembled into `src/generated/**` | Full app runtime plus portal auth, portal snapshots, rich IM and AI message builders, payload-first live domain streams, durable replay helpers, and RTC signal helpers from one package |
| Flutter | Tier A | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-flutter` | `craw_chat_sdk` above generated `backend_sdk` | `CrawChatClient` ships today | `generated/server-openapi` | Route-aligned app runtime modules from `CrawChatClient`, including `sdk.auth` and `sdk.portal`; `craw_chat_sdk` re-exports `backend_sdk`, whose `SdkworkBackendClient` also mounts `client.auth` and `client.portal`, while websocket live runtime and the TypeScript message-first builder surface remain absent |
| Rust | Tier A | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-rust` | Generated crate `sdkwork-craw-chat-backend-sdk`; semantic target `craw_chat_sdk` is reserved | `CrawChatSdkClient` is the target semantic client | `generated/server-openapi` | Live-schema transport generation and workspace verification pass; semantic crate still lives in the manual `composed` reserve |
| Java | Tier B | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-java` | Generated artifact `com.sdkwork:craw-chat-backend-sdk` | `CrawChatSdkClient` is a target semantic name only | `generated/server-openapi` | Transport-standardized workspace with generated artifact verified and semantic reserve under `composed` |
| C# | Tier B | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-csharp` | Generated package `Sdkwork.CrawChat.BackendSdk` | `CrawChatSdkClient` is a target semantic name only | `generated/server-openapi` | Transport-standardized workspace with generated package verified and semantic reserve under `composed` |
| Swift | Tier B | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-swift` | Generated package `CrawChatBackendSdk` | `CrawChatSdkClient` is a target semantic name only | `generated/server-openapi` | Transport-standardized workspace with generated package verified and semantic reserve under `composed` |
| Kotlin | Tier B | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-kotlin` | Generated artifact `com.sdkwork:craw-chat-backend-sdk` | `CrawChatSdkClient` is a target semantic name only | `generated/server-openapi` | Transport-standardized workspace with generated artifact verified and semantic reserve under `composed` |
| Go | Tier B | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-go` | Generated module `github.com/sdkwork/craw-chat-backend-sdk` | `CrawChatSdkClient` is a target semantic name only | `generated/server-openapi` | Transport-standardized workspace with generated module verified and semantic reserve under `composed` |
| Python | Tier B | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-python` | Generated package `sdkwork-craw-chat-backend-sdk` | `CrawChatSdkClient` is a target semantic name only | `generated/server-openapi` | Transport-standardized workspace with generated package verified and semantic reserve under `composed` |

Across Rust, Java, C#, Swift, Kotlin, Go, and Python, the current repo-standard transport client
is the generated `SdkworkBackendClient` surface in each language. `CrawChatSdkClient` remains the
semantic target name, not the checked-in app-facing client you can instantiate today.

## Feature Snapshot

| Capability | TypeScript | Flutter | Rust | Java / C# / Swift / Kotlin / Go / Python | Notes |
| --- | --- | --- | --- | --- | --- |
| Portal auth (`login`, `me`) | Yes | Yes | Not shipped | Not shipped | Flutter now ships `sdk.auth` on `CrawChatClient` and `client.auth` on `SdkworkBackendClient` |
| Portal snapshot reads | Yes | Yes | Not shipped | Not shipped | Flutter now ships `sdk.portal` on `CrawChatClient` and `client.portal` on `SdkworkBackendClient` |
| Session and presence | Yes | Yes | Transport only | Transport only | Non-TypeScript languages are transport-standardized first |
| Realtime HTTP coordination | Yes | Yes | Transport only | Transport only | Flutter is HTTP coordination only today |
| Message-first builders (`createXxxMessage`, `send`, `decodeMessage`) | Yes | No | Targeted for semantic phase | Not shipped | TypeScript remains the only checked-in message-first baseline |
| Realtime WebSocket adapter | Yes | No | Not shipped | Not shipped | TypeScript ships the handwritten live runtime behind `sdk.connect(...)` |
| Generated transport from live schema | Yes | Yes | Yes | Yes | Every official language is wired to the same live schema source |

In Flutter, the checked-in client surface now includes `sdk.auth`, `sdk.portal`, `client.auth`,
and `client.portal`. The remaining parity gap is the missing websocket live runtime and the
message-first builder family.

## Practical Selection Rules

- Choose TypeScript when you need the strongest checked-in consumer SDK, one package, and
  `sdk.connect(...)`, `sdk.sync.catchUp(...)`, `sdk.createXxxMessage(...)`, or portal helpers.
- Choose Flutter when you are integrating route-aligned app-runtime HTTP flows, including
  `sdk.auth` and `sdk.portal`, and you do not need a delivered websocket live runtime or the
  TypeScript message-first builders. Its realtime story is HTTP coordination only today.
- Choose Rust when you want a transport-standardized SDK in a Tier A language that is being pushed
  toward a future semantic `craw_chat_sdk` layer.
- Choose Java, C#, Swift, Kotlin, Go, or Python when a verified generated transport artifact is the
  immediate need and a manual semantic layer can land later under `composed`.
- Choose raw HTTP plus the API reference when you need a language surface that is not yet exposed by
  the current checked-in semantic SDK of your target language.

## How To Use This Page

Use this page to answer three questions in order:

1. Does the language workspace exist and generate from the live schema?
2. Does that language currently ship a semantic SDK, or only the generated transport boundary?
3. Even if the workspace and transport exist, is the package already published, or is it still a
   repo-standard contract?

That order prevents a common mistake: treating a repo workspace or manifest as if it already
proved registry publication or cross-language feature parity.

## What "Supported Language" Means Here

In this documentation, language support means:

- the language is an official workspace entry
- the root wrappers can generate and verify that language from the live schema
- package naming and generated/manual boundaries are documented
- the maturity tier is explicit

It does not mean:

- a package has been published to npm, pub.dev, crates.io, Maven Central, NuGet, Swift Package
  Index, Go proxy, or PyPI
- every language has reached TypeScript parity
- every language already ships a manual semantic client above the generated transport layer

That distinction keeps the site precise and trustworthy.

## What To Read Next

- Read [TypeScript SDK](/sdk/typescript-sdk) when you need the broadest checked-in app SDK surface.
- Read [Flutter SDK](/sdk/flutter-sdk) when you need the real Dart export surface and current parity limits.
- Read [Rust SDK](/sdk/rust-sdk) when you are evaluating the first non-TypeScript Tier A target.
- Read [Java SDK](/sdk/java-sdk) when you need the current JVM transport-standardized workspace contract.
- Read [C# SDK](/sdk/csharp-sdk) when you need the current .NET transport-standardized workspace contract.
- Read [Swift SDK](/sdk/swift-sdk) when you need the current Apple-platform transport-standardized workspace contract.
- Read [Kotlin SDK](/sdk/kotlin-sdk) when you need the current Kotlin JVM transport-standardized workspace contract.
- Read [Go SDK](/sdk/go-sdk) when you need the current Go transport-standardized workspace contract.
- Read [Python SDK](/sdk/python-sdk) when you need the current Python transport-standardized workspace contract.
- Read [Generator Boundary](/sdk/generator-boundary) when you need the generated/server-openapi versus composed rule set.
- Read [Admin SDK](/sdk/admin-sdk) when your consumer boundary is governance or control-plane rather than the public app runtime.
