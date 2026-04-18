# Generation and Ownership

This page defines the maintenance boundary for the app SDK family. It is intended for SDK
maintainers and contributors, not just downstream consumers.

## Generator-Owned Surface

`generated/server-openapi` is generator-owned output for every supported language:

- TypeScript: `sdkwork-craw-chat-sdk-typescript/generated/server-openapi`
- Flutter: `sdkwork-craw-chat-sdk-flutter/generated/server-openapi`
- Rust: `sdkwork-craw-chat-sdk-rust/generated/server-openapi`

Do not hand-edit files in these directories to fix product behavior.

## Manual-Owned Surface

`composed` is the manual-owned SDK layer and the preferred public integration surface:

- TypeScript composed package: `@sdkwork/craw-chat-sdk`
- Flutter composed package: `craw_chat_sdk`
- Rust composed crate: `craw-chat-sdk`

This layer owns:

- `CrawChatClient`
- semantic modules
- builder helpers
- public consumer documentation and examples

## Regeneration Rule

When the transport contract or generated transport surface needs to change:

1. change the authority contract or generator inputs
2. regenerate the affected language workspaces
3. re-run the workspace verification flow
4. update manual docs only where public behavior or examples changed

Do not patch generated files by hand.

## Source-Of-Truth Hierarchy

1. `openapi/craw-chat-app.openapi.yaml`
2. derived sdkgen inputs under `openapi/`
3. root generation and verification wrappers under `bin/`
4. generated transport output under `generated/server-openapi`
5. manual composition under `composed`

## Documentation Rule

Every SDK page in this site should make it obvious whether it is documenting:

- generated transport behavior
- manual convenience layers
- or a transport contract that is documented but not wrapped manually in this round

## Related Pages

- [App SDK](/sdk/app-sdk)
- [Language Support](/sdk/language-support)
- [Auth and Client Init](/sdk/auth-and-client-init)
