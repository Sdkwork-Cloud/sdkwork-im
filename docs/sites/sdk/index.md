# SDK Overview

The repository currently defines three SDK families with different consumers, contracts, and release truths:

- `sdkwork-im-sdk`
  App-facing runtime integrations generated from the live Craw Chat OpenAPI 3.x contract.
- `sdkwork-control-plane-sdk`
  Admin app boundary and control-plane integrations.
- `sdkwork-im-admin-sdk`
  IM admin and `/api/admin/*` operator integrations.

For application integration, start from the app SDK family and the TypeScript package
`@sdkwork/im-sdk`. That remains the strongest checked-in consumer SDK and the reference
contract the other language lanes converge toward.

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
| Control-plane workflows | [Control-Plane SDK](/sdk/control-plane-sdk) |
| Operator-console `/api/admin/*` workflows | [IM Admin SDK](/sdk/im-admin-sdk) |

## SDK Family Matrix

For day-to-day engineering, treat the checked-in SDK workspaces and their `.sdkwork-assembly.json`
files as the repository truth. App, Control-Plane, and IM Admin families all have materialized
TypeScript and Flutter workspaces in-repo, and all of them remain unpublished.

### Release Snapshot

The current release catalog under `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
reports `state = generated_pending_publication`. Every tracked artifact in that catalog currently
records `generationStatus = generated` and `releaseStatus = not_published`.

The repository truth is therefore stronger than "planned SDK structure": `sdkwork-im-sdk`,
`sdkwork-control-plane-sdk`, and `sdkwork-im-admin-sdk` all exist as checked-in workspaces, and
`sdkwork-im-admin-sdk` has materialized TypeScript and Flutter package workspaces.

| Family | Audience | Best entry page | Contract source | Current repo state |
| --- | --- | --- | --- | --- |
| `sdkwork-im-sdk` | Product and app integrations | [App SDK](/sdk/app-sdk) | App OpenAPI authority under `sdks/sdkwork-im-sdk/openapi/` | TypeScript and Flutter consumer lines materialized; additional language workspaces generated in-repo; publication pending |
| `sdkwork-control-plane-sdk` | Governance and control-plane tooling | [Control-Plane SDK](/sdk/control-plane-sdk) | Control-plane authority under `sdks/sdkwork-control-plane-sdk/openapi/` | TypeScript and Flutter lines materialized; publication pending |
| `sdkwork-im-admin-sdk` | IM admin and `/api/admin/*` tooling | [IM Admin SDK](/sdk/im-admin-sdk) | IM admin authority under `sdks/sdkwork-im-admin-sdk/openapi/` | TypeScript and Flutter lines materialized; publication pending |

Generated symbols must be consumed through package root entrypoints only.

App SDK consumers target `local-minimal-node` during direct local development and the unified
`craw-chat-server` / `web-gateway` public origin in packaged installs.

Control-plane SDK consumers can target `control-plane-api` directly during standalone governance
development, but packaged installs should switch to the unified gateway public origin.

IM admin SDK consumers target the deployed surface that serves `/api/admin/*`; in packaged
installs that is also the unified gateway public origin.

| API group | SDK family | Current boundary |
| --- | --- | --- |
| App Runtime (`/api/v1/*`) | `sdkwork-im-sdk` | Checked-in app OpenAPI authority plus materialized TypeScript and Flutter consumer packages |
| Control Plane Governance (`/api/v1/control/*`) | `sdkwork-control-plane-sdk` | Checked-in control-plane authority plus materialized TypeScript and Flutter consumer packages |
| Operator Console Admin API (`/api/admin/*`) | `sdkwork-im-admin-sdk` | Checked-in authority plus materialized TypeScript and Flutter generated/composed packages exist; `/api/admin/*` authority stays in this family while the admin console consumes the unified `@sdkwork/control-plane-sdk` boundary |

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
  [Control-Plane SDK](/sdk/control-plane-sdk).

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
| TypeScript | Tier A | `@sdkwork/im-sdk` with generated transport assembled under `src/generated/**` | `ImSdkClient` | [TypeScript SDK](/sdk/typescript-sdk) |
| Flutter | Tier A | `im_sdk` above generated `im_sdk_generated` | `ImSdkClient` | [Flutter SDK](/sdk/flutter-sdk) |
| Rust | Tier A | `im-sdk` above generated `sdkwork-im-sdk-generated` | `ImSdkClient` | [Rust SDK](/sdk/rust-sdk) |
| Java | Tier B | Generated artifact `com.sdkwork:im-sdk-generated`; semantic reserve under `composed` | `ImSdkClient` target | [Java SDK](/sdk/java-sdk) |
| C# | Tier B | Generated package `Sdkwork.Im.Sdk.Generated`; semantic reserve under `composed` | `ImSdkClient` target | [C# SDK](/sdk/csharp-sdk) |
| Swift | Tier B | Generated package `ImSdkGenerated`; semantic reserve under `composed` | `ImSdkClient` target | [Swift SDK](/sdk/swift-sdk) |
| Kotlin | Tier B | Generated artifact `com.sdkwork:im-sdk-generated`; semantic reserve under `composed` | `ImSdkClient` target | [Kotlin SDK](/sdk/kotlin-sdk) |
| Go | Tier B | Generated module `github.com/sdkwork/im-sdk-generated`; semantic reserve under `composed` | `ImSdkClient` target | [Go SDK](/sdk/go-sdk) |
| Python | Tier B | Generated package `sdkwork-im-sdk-generated`; semantic reserve under `composed` | `ImSdkClient` target | [Python SDK](/sdk/python-sdk) |

For Java, C#, Swift, Kotlin, Go, and Python, the real checked-in transport entrypoint is still the
generated `ImTransportClient` surface in that language's generated package. Rust now ships
`ImSdkClient` under `im-sdk`, while the remaining languages still reserve `ImSdkClient` as the
future semantic client name for a later manual layer.

## Tier Model

### Tier A

Tier A languages are the semantic-SDK baseline. They are the languages the workspace is actively
driving toward the TypeScript standard of:

- one business-facing client
- a documented generated-versus-semantic boundary
- workflow-first app guidance instead of route-group-only guidance

Today, TypeScript is fully in that shape. Flutter is a checked-in consumer SDK with known parity
gaps. Rust now also ships a checked-in semantic client under `composed`, with auth/portal and
websocket live runtime still trailing the TypeScript baseline.

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

- official consumer package: `@sdkwork/im-sdk`
- primary client: `ImSdkClient`
- synchronous construction: `new ImSdkClient({...})`
- flat config with `baseUrl`, `apiBaseUrl`, `websocketBaseUrl`, `authToken`, and
  `webSocketFactory`
- message-first outbound APIs through `sdk.createXxxMessage(...)` and `sdk.send(...)`
- payload-first domain receive APIs through `sdk.connect(...)` and durable replay through `sdk.sync`
- exact route-aligned transport modules available directly on `ImSdkClient`
- generated transport clients named `ImTransportClient` in the non-TypeScript generated packages

## Choose The Right Surface

| Need | Best fit |
| --- | --- |
| Application integration that needs the richest checked-in semantics | `ImSdkClient` in `@sdkwork/im-sdk` |
| Route-aligned Dart integration above a generated package | `ImSdkClient` in `im_sdk` |
| Transport-standardized JVM/.NET/Swift/Go/Python work | The generated transport artifact documented on the language page |
| Generated-versus-semantic rules before extending a language | [Generator Boundary](/sdk/generator-boundary) |
| Exact generated DTOs and route groups | `ImTransportClient` or the generated transport artifact for that language; in TypeScript use the route-aligned modules on `ImSdkClient` |

## Contract Source

The app SDK family is generated from the Craw Chat app OpenAPI 3.x contract exported by the
running service at `/openapi/craw-chat-app.openapi.yaml`.

For every official language:

- the checked-in authority snapshot lives under `sdks/sdkwork-im-sdk/openapi/craw-chat-app.openapi.yaml`
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
- [Control-Plane SDK](/sdk/control-plane-sdk)
- [IM Admin SDK](/sdk/im-admin-sdk)
- [Language Support](/sdk/language-support)
- [App API Overview](/api-reference/app-api)
- [Control Plane API Overview](/api-reference/control-plane-api)
