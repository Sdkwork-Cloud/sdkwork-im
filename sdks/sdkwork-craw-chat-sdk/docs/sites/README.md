# Public Docs Sync Standards

This directory documents how `sdkwork-craw-chat-sdk` workspace changes must stay aligned with the
public VitePress documentation under `docs/sites`.

Use it when package names, client names, generation boundaries, language parity notes, or SDK
wording change.

## Public Docs Ownership Map

These public pages carry the SDK contract that downstream consumers actually read:

- `docs/sites/index.md`
  Home-page SDK callouts and first-touch package naming.
- `docs/sites/sdk/index.md`
  Top-level SDK naming model across languages.
- `docs/sites/sdk/app-sdk.md`
  Family-wide contract, release notes, and live-schema source-of-truth guidance.
- `docs/sites/sdk/typescript-sdk.md`
  Official TypeScript package, client, runtime, and internal authoring-boundary guidance.
- `docs/sites/sdk/flutter-sdk.md`
  Official Flutter package contract, generated transport boundary, and parity-gap guidance.
- `docs/sites/sdk/rust-sdk.md`
  Tier A Rust target status, generated crate naming, and semantic reserve guidance.
- `docs/sites/sdk/java-sdk.md`
  Tier B Java transport-standardized guidance.
- `docs/sites/sdk/csharp-sdk.md`
  Tier B C# transport-standardized guidance.
- `docs/sites/sdk/swift-sdk.md`
  Tier B Swift transport-standardized guidance and normalized package naming.
- `docs/sites/sdk/kotlin-sdk.md`
  Tier B Kotlin transport-standardized guidance.
- `docs/sites/sdk/go-sdk.md`
  Tier B Go transport-standardized guidance.
- `docs/sites/sdk/python-sdk.md`
  Tier B Python transport-standardized guidance.
- `docs/sites/sdk/generator-boundary.md`
  Cross-language generated-versus-semantic ownership rules.
- `docs/sites/sdk/language-support.md`
  Cross-language support matrix, Tier A and Tier B guidance, and naming model.
- `docs/sites/api-reference/app-api.md`
  API overview bridge from HTTP routes to official SDK package names.
- `docs/sites/api-reference/app/conversations.md`
- `docs/sites/api-reference/app/device-sync.md`
- `docs/sites/api-reference/app/membership-and-read-state.md`
- `docs/sites/api-reference/app/messages.md`
- `docs/sites/api-reference/app/media.md`
- `docs/sites/api-reference/app/portal-and-auth.md`
- `docs/sites/api-reference/app/rtc.md`
- `docs/sites/api-reference/app/session-and-realtime.md`
- `docs/sites/api-reference/app/streams.md`
  App-domain API pages that explain which official SDK packages cover each HTTP surface.

## Contract Markers That Must Stay Visible

When the public docs talk about SDK consumption, prefer the real package and client names:

- TypeScript official consumer package: `@sdkwork/craw-chat-sdk`
- TypeScript primary client: `CrawChatSdkClient`
- TypeScript assembled generated boundary: `src/generated/**`
- TypeScript generator-owned authoring boundary: `generated/server-openapi`
- Flutter official consumer package: `craw_chat_sdk`
- Flutter primary client: `CrawChatClient`
- Flutter generated transport package: `backend_sdk`
- Rust generated transport crate: `sdkwork-craw-chat-backend-sdk`
- Java generated transport artifact: `com.sdkwork:craw-chat-backend-sdk`
- C# generated transport package: `Sdkwork.CrawChat.BackendSdk`
- Swift generated transport package: `CrawChatBackendSdk`
- Kotlin generated transport artifact: `com.sdkwork:craw-chat-backend-sdk`
- Go generated transport module: `github.com/sdkwork/craw-chat-backend-sdk`
- Python generated transport package: `sdkwork-craw-chat-backend-sdk`
- Tier A: current semantic baseline languages
- Tier B: transport-standardized languages with manual semantic reserve
- live service schema endpoint: `/openapi/craw-chat-app.openapi.yaml`

If these identifiers move in the workspace, the corresponding public docs must move in the same
change.

## Wording Rules

- Do not describe the TypeScript SDK as a two-package consumer model when the public contract is
  `@sdkwork/craw-chat-sdk`.
- Do not tell normal TypeScript consumers to import low-level transport types from
  `@sdkwork/craw-chat-backend-sdk`.
- Do not describe Flutter as a neutral generated-versus-composed package choice when
  `craw_chat_sdk` is already the official app-facing package.
- Do not fall back to vague phrases such as "generated client" or "composed client" when the real
  package name or class name is known.
- Describe realtime WebSocket transport as a manual-owned extension boundary, not as generated SDK
  delivery.
- Describe the current Flutter portal gap through `craw_chat_sdk` and `backend_sdk`, not through
  directory nicknames alone.

## Sync Workflow

1. Update the workspace README and internal SDK standards docs first when package or generation
   rules move.
2. Update the public SDK pages and app API pages listed in this file inside the same change.
3. Regenerate operation pages when the API overview source blocks change:

```powershell
node .\scripts\generate-operation-pages.mjs
```

4. Run the public docs verification entrypoint from `docs/sites`:

```powershell
node .\scripts\docs-verify.mjs
```

Equivalent targeted checks are:

```powershell
node .\scripts\verify-api-docs.mjs
node .\scripts\verify-sdk-docs.mjs
```

5. Run the SDK workspace verification flow when public docs moved because SDK contracts or
   generation boundaries changed:

```powershell
node ..\sdks\sdkwork-craw-chat-sdk\bin\verify-sdk.mjs
```

6. Build or preview VitePress when the local environment allows it:

```powershell
npm run docs:build
npm run docs:dev
```

## Change Triggers That Require Public Docs Review

- package rename, import-path rename, or client-class rename
- new generated route groups or removed route groups
- changes to the live schema export or regeneration pipeline
- Tier A or Tier B maturity changes
- Flutter, Rust, or TypeScript parity changes
- portal/auth surface changes
- realtime ownership-boundary changes

## Related Internal Docs

- internal docs map: `../README.md`
- package standards: `../package-standards.md`
- generation pipeline: `../generation-pipeline.md`
- verification matrix: `../verification-matrix.md`
- realtime extension boundary: `../realtime-extension-boundary.md`
