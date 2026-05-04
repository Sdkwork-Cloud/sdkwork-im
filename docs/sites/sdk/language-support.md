# Language Support

This page measures language support by workspace boundary, generated transport package, semantic
ownership, and verified delivery status. It does not treat "supported" as a synonym for
"already published."

This page separates two questions:

1. Which SDK workspaces are usable in the checked-in repository right now?
2. What does the historical release catalog say about publication state?

## Current Verified Baseline

The current checked-in baseline was revalidated on 2026-04-16 from the live Craw Chat OpenAPI 3.x
export.

- all nine official language workspaces regenerated successfully from the live schema
- the root SDK verification matrix passed for TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin,
  Go, and Python
- the docs site verification also passed after regenerating the operation pages

Read the matrix below as a statement about the real checked-in repo contract, not an assumption
about registry publication or cross-language parity.

## Additional SDK Families

The repository now also carries separate TypeScript and Flutter workspaces for the Admin and
IM Admin SDK families:

| Audience | Workspace | Current state |
| --- | --- | --- |
| Admin control plane | `sdks/sdkwork-control-plane-sdk` | TypeScript and Flutter workspaces materialized, locally verifiable, publication pending |
| IM admin operator surface | `sdks/sdkwork-im-admin-sdk` | TypeScript and Flutter workspaces materialized, locally verifiable, publication pending |

Use [Control-Plane SDK](/sdk/control-plane-sdk) for `/api/v1/control/*` governance flows and
[IM Admin SDK](/sdk/im-admin-sdk) for `/api/admin/*` operator-console flows.

## SDK Family Workspace Snapshot

| Audience | Language | Workspace | Current state |
| --- | --- | --- | --- |
| App | TypeScript | `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript` | Workspace materialized, locally verifiable, publication still pending |
| App | Flutter | `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter` | Workspace materialized, locally verifiable, publication still pending |
| Admin | TypeScript | `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript` | Workspace materialized, locally verifiable, publication still pending |
| Admin | Flutter | `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter` | Workspace materialized, locally verifiable, publication still pending |
| IM Admin | TypeScript | `sdks/sdkwork-im-admin-sdk/sdkwork-im-admin-sdk-typescript` | Workspace materialized, locally verifiable, publication still pending |
| IM Admin | Flutter | `sdks/sdkwork-im-admin-sdk/sdkwork-im-admin-sdk-flutter` | Workspace materialized, locally verifiable, publication still pending |

## Fastest Onboarding

Use the overview and quick-start pages before you read the full parity matrix:

- family rules: [App SDK Overview](/sdk/app-sdk)
- shared setup rules: [Auth and Client Init](/sdk/auth-and-client-init)
- TypeScript bootstrap: [TypeScript Quick Start](/sdk/typescript-quick-start)
- Flutter bootstrap: [Flutter Quick Start](/sdk/flutter-quick-start)
- Rust bootstrap: [Rust Quick Start](/sdk/rust-quick-start)
- capability routing: [Module Map](/sdk/module-map)

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
| TypeScript | Tier A | `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript` | `@sdkwork/im-sdk` | `ImSdkClient` ships today | `generated/server-openapi`, assembled into `src/generated/**` | Full app runtime plus portal auth, portal snapshots, rich IM and AI message builders, payload-first live domain streams, durable replay helpers, and RTC signal helpers from one package |
| Flutter | Tier A | `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter` | `im_sdk` above generated `im_sdk_generated` | `ImSdkClient` ships today | `generated/server-openapi` | Route-aligned app runtime modules from `ImSdkClient`, including `sdk.auth`, `sdk.portal`, `sdk.connect(...)`, and a delivered WebSocket adapter in `im_sdk`; `im_sdk` re-exports `im_sdk_generated`, whose `ImTransportClient` also mounts `client.auth` and `client.portal`, while the TypeScript message-first builder surface remains absent |
| Rust | Tier A | `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust` | `im-sdk` above generated `sdkwork-im-sdk-generated` | `ImSdkClient` ships today | `generated/server-openapi` | Route-aligned Rust helpers ship in `im-sdk`; generated transport remains available for auth, portal, and DTO-level fallback, while websocket live runtime and the TypeScript message-first receive surface remain absent |
| Java | Tier B | `sdks/sdkwork-im-sdk/sdkwork-im-sdk-java` | Generated artifact `com.sdkwork:im-sdk-generated` | `ImSdkClient` is a target semantic name only | `generated/server-openapi` | Transport-standardized workspace with generated artifact verified and semantic reserve under `composed` |
| C# | Tier B | `sdks/sdkwork-im-sdk/sdkwork-im-sdk-csharp` | Generated package `Sdkwork.Im.Sdk.Generated` | `ImSdkClient` is a target semantic name only | `generated/server-openapi` | Transport-standardized workspace with generated package verified and semantic reserve under `composed` |
| Swift | Tier B | `sdks/sdkwork-im-sdk/sdkwork-im-sdk-swift` | Generated package `ImSdkGenerated` | `ImSdkClient` is a target semantic name only | `generated/server-openapi` | Transport-standardized workspace with generated package verified and semantic reserve under `composed` |
| Kotlin | Tier B | `sdks/sdkwork-im-sdk/sdkwork-im-sdk-kotlin` | Generated artifact `com.sdkwork:im-sdk-generated` | `ImSdkClient` is a target semantic name only | `generated/server-openapi` | Transport-standardized workspace with generated artifact verified and semantic reserve under `composed` |
| Go | Tier B | `sdks/sdkwork-im-sdk/sdkwork-im-sdk-go` | Generated module `github.com/sdkwork/im-sdk-generated` | `ImSdkClient` is a target semantic name only | `generated/server-openapi` | Transport-standardized workspace with generated module verified and semantic reserve under `composed` |
| Python | Tier B | `sdks/sdkwork-im-sdk/sdkwork-im-sdk-python` | Generated package `sdkwork-im-sdk-generated` | `ImSdkClient` is a target semantic name only | `generated/server-openapi` | Transport-standardized workspace with generated package verified and semantic reserve under `composed` |

Across Java, C#, Swift, Kotlin, Go, and Python, the current repo-standard transport client is the
generated `ImTransportClient` surface in each language. Rust now joins TypeScript and Flutter with
a checked-in semantic client, while the remaining languages still treat `ImSdkClient` as the target
semantic name rather than the current instantiable entrypoint.

## Feature Snapshot

| Capability | TypeScript | Flutter | Rust | Java / C# / Swift / Kotlin / Go / Python | Notes |
| --- | --- | --- | --- | --- | --- |
| Portal auth (`login`, `me`) | Yes | Yes | Generated fallback only | Not shipped | Flutter ships `sdk.auth`; Rust currently uses generated auth access through `ImTransportClient` |
| Portal snapshot reads | Yes | Yes | Generated fallback only | Not shipped | Flutter ships `sdk.portal`; Rust currently uses generated portal access through `ImTransportClient` |
| Session and presence | Yes | Yes | Yes | Transport only | Rust ships route-aligned session and presence modules on `ImSdkClient` |
| Realtime HTTP coordination | Yes | Yes | Yes | Transport only | Rust ships HTTP coordination helpers but not a websocket live runtime |
| Message-first builders (`createXxxMessage`, `send`, `decodeMessage`) | Yes | No | No | Not shipped | TypeScript remains the only checked-in message-first baseline |
| Realtime WebSocket adapter | Yes | Yes | Not shipped | Not shipped | TypeScript and Flutter ship handwritten live runtimes behind `sdk.connect(...)` |
| Generated transport from live schema | Yes | Yes | Yes | Yes | Every official language is wired to the same live schema source |

In Flutter, the checked-in client surface now includes `sdk.auth`, `sdk.portal`, `sdk.connect(...)`,
`client.auth`, and `client.portal` on `ImSdkClient` plus the re-exported generated transport. The
remaining parity gap is the message-first builder family.

## Verification Signals

When you need to verify the real repo contract instead of relying on prose alone, check:

- `.sdkwork-assembly.json` for package-layer ownership, `manifestPath`, and stable `generatedAt`
- `node ./sdks/sdkwork-im-sdk/bin/verify-sdk.mjs` for the app SDK family verification entrypoint
- the dedicated language guides for the exact consumer-facing examples

## Practical Selection Rules

- Choose TypeScript when you need the strongest checked-in consumer SDK, one package, and
  `sdk.connect(...)`, `sdk.sync.catchUp(...)`, `sdk.createXxxMessage(...)`, or portal helpers.
- Choose Flutter when you are integrating route-aligned app-runtime flows, including `sdk.auth`,
  `sdk.portal`, and `sdk.connect(...)`, and you do not need the TypeScript message-first builders.
- Choose Rust when you want a transport-standardized SDK in a Tier A language that is being pushed
  toward a future semantic `im_sdk` layer.
- Choose Java, C#, Swift, Kotlin, Go, or Python when a verified generated transport artifact is the
  immediate need and a manual semantic layer can land later under `composed`.
- Choose raw HTTP plus the API reference when you need a language surface that is not yet exposed by
  the current checked-in semantic SDK of your target language.

## Practical Interpretation

The checked-in workspaces and `.sdkwork-assembly.json` files answer "what engineers can use in this
repo today." The release catalog answers "what has been versioned and published." Those are related
but different questions.

- app TypeScript exposes one public package `@sdkwork/im-sdk`; the generator-owned workspace
  remains internal under `generated/server-openapi` and assembles into `src/generated/**`.
- admin TypeScript uses generated `@sdkwork/control-plane-backend-sdk` and composed `@sdkwork/control-plane-sdk`.
- IM admin TypeScript uses generated `@sdkwork/im-admin-backend-sdk` and composed `@sdkwork/im-admin-sdk`.
- admin Flutter uses generated `control_plane_backend_sdk` and composed `control_plane_sdk`.
- IM admin Flutter uses generated `im_admin_backend_sdk` and composed `im_admin_sdk`.
- admin TypeScript and Flutter are available as checked-in workspaces.
- IM admin TypeScript and Flutter are available as checked-in workspaces.
- IM admin SDK consumers target the deployed `/api/admin/*` surface; in packaged installs that surface is also reached through the unified gateway public origin.

## Release Catalog Snapshot

| Artifact | Audience | Language | Generation state | Release state |
| --- | --- | --- | --- | --- |
| `app-typescript` | app | typescript | `generated` | `not_published` |
| `app-flutter` | app | flutter | `generated` | `not_published` |
| `app-rust` | app | rust | `generated` | `not_published` |
| `app-java` | app | java | `generated` | `not_published` |
| `app-csharp` | app | csharp | `generated` | `not_published` |
| `app-swift` | app | swift | `generated` | `not_published` |
| `app-kotlin` | app | kotlin | `generated` | `not_published` |
| `app-go` | app | go | `generated` | `not_published` |
| `app-python` | app | python | `generated` | `not_published` |
| `admin-typescript` | admin | typescript | `generated` | `not_published` |
| `admin-flutter` | admin | flutter | `generated` | `not_published` |
| `im-admin-typescript` | im-admin | typescript | `generated` | `not_published` |
| `im-admin-flutter` | im-admin | flutter | `generated` | `not_published` |

The current release catalog and the checked-in workspaces agree that every tracked language line is
generated locally and remains unpublished.

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

- Read [App SDK Overview](/sdk/app-sdk) when you want the family rules before choosing a language.
- Read [Auth and Client Init](/sdk/auth-and-client-init) when you want the shared bootstrap contract before picking a language.
- Read [TypeScript Quick Start](/sdk/typescript-quick-start) when you want the shortest path into the primary production package.
- Read [Flutter Quick Start](/sdk/flutter-quick-start) when you want the current Dart package and HTTP-first integration path.
- Read [Rust Quick Start](/sdk/rust-quick-start) when you want the current Rust generated-crate contract and workspace boundary.
- Read [TypeScript SDK](/sdk/typescript-sdk) when you need the broadest checked-in app SDK surface.
- Read [Flutter SDK](/sdk/flutter-sdk) when you need the real Dart export surface and current parity limits.
- Read [Control-Plane TypeScript SDK](/sdk/control-plane-typescript-sdk) when you need governance or control-plane flows from a browser or Node.js consumer.
- Read [Control-Plane Flutter SDK](/sdk/control-plane-flutter-sdk) when you need governance or control-plane flows from Dart or Flutter.
- Read [Rust SDK](/sdk/rust-sdk) when you are evaluating the first non-TypeScript Tier A target.
- Read [Java SDK](/sdk/java-sdk) when you need the current JVM transport-standardized workspace contract.
- Read [C# SDK](/sdk/csharp-sdk) when you need the current .NET transport-standardized workspace contract.
- Read [Swift SDK](/sdk/swift-sdk) when you need the current Apple-platform transport-standardized workspace contract.
- Read [Kotlin SDK](/sdk/kotlin-sdk) when you need the current Kotlin JVM transport-standardized workspace contract.
- Read [Go SDK](/sdk/go-sdk) when you need the current Go transport-standardized workspace contract.
- Read [Python SDK](/sdk/python-sdk) when you need the current Python transport-standardized workspace contract.
- Read [Generator Boundary](/sdk/generator-boundary) when you need the generated/server-openapi versus composed rule set.
- Read [Control-Plane SDK](/sdk/control-plane-sdk) when your consumer boundary is governance or control-plane rather than the public app runtime.
