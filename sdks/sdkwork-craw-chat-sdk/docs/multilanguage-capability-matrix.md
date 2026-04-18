# Multilanguage Capability Matrix

This matrix is the maintainer-facing view of the current SDK family. It records actual package
names, current maturity, and the boundaries that verification is enforcing today.

## Tier A

Tier A languages are the semantic-SDK reference set:

- TypeScript
- Flutter
- Rust

Tier A means the language is either the current checked-in semantic baseline or the next language
being driven toward that baseline.

## Tier B

Tier B languages are transport-standardized first:

- Java
- C#
- Swift
- Kotlin
- Go
- Python

Tier B means generated transport, naming, docs, and verification are standardized, while the
business-facing semantic layer still lives in the manual `composed` reserve.

## Matrix

| Language | Tier | Current public package or transport artifact | Primary client status | Generated boundary | Semantic boundary | Current verified state |
| --- | --- | --- | --- | --- | --- | --- |
| TypeScript | Tier A | `@sdkwork/craw-chat-sdk` | `CrawChatSdkClient` ships today | `generated/server-openapi`, assembled into `src/generated/**` | root `src/**` outside `src/generated/**` | Root verification passes; TypeScript remains the semantic baseline |
| Flutter | Tier A | `craw_chat_sdk` plus generated `backend_sdk` | `CrawChatClient` ships today | `generated/server-openapi` | `composed` | Live-schema generation and workspace verification pass |
| Rust | Tier A | generated crate `sdkwork-craw-chat-backend-sdk`; semantic target `craw_chat_sdk` | `CrawChatSdkClient` target | `generated/server-openapi` | `composed` | Live-schema generation and workspace verification pass |
| Java | Tier B | generated artifact `com.sdkwork:craw-chat-backend-sdk` | `CrawChatSdkClient` target only | `generated/server-openapi` | `composed` | Live-schema generation and workspace verification pass |
| C# | Tier B | generated package `Sdkwork.CrawChat.BackendSdk` | `CrawChatSdkClient` target only | `generated/server-openapi` | `composed` | Live-schema generation and workspace verification pass |
| Swift | Tier B | generated package `CrawChatBackendSdk` | `CrawChatSdkClient` target only | `generated/server-openapi` | `composed` | Live-schema generation passes after workspace normalization; workspace verification passes |
| Kotlin | Tier B | generated artifact `com.sdkwork:craw-chat-backend-sdk` | `CrawChatSdkClient` target only | `generated/server-openapi` | `composed` | Live-schema generation and workspace verification pass |
| Go | Tier B | generated module `github.com/sdkwork/craw-chat-backend-sdk` | `CrawChatSdkClient` target only | `generated/server-openapi` | `composed` | Live-schema generation and workspace verification pass |
| Python | Tier B | generated package `sdkwork-craw-chat-backend-sdk` | `CrawChatSdkClient` target only | `generated/server-openapi` | `composed` | Live-schema generation and workspace verification pass |

## Maintainer Reading Rules

- Read the TypeScript and Flutter public package names as shipped repo contracts today.
- Read Rust as a Tier A target with a verified generated transport crate and reserved semantic
  boundary.
- Read Java, C#, Swift, Kotlin, Go, and Python as verified transport-standardized workspaces.
- Do not claim a handwritten semantic SDK for a language until the `composed` boundary contains the
  actual package manifest and public entrypoint.
