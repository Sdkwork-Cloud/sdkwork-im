# SDKWork Craw Chat SDK Rust Workspace

This workspace is the Rust lane of the `sdkwork-craw-chat-sdk` multi-language delivery standard for Craw Chat.

## Current Standard

- Maturity tier: `Tier A`
- Target semantic client: `CrawChatSdkClient`
- Current generated transport crate: `sdkwork-craw-chat-backend-sdk`
- Reserved semantic crate: `craw_chat_sdk`
- Generator-owned boundary: `generated/server-openapi`
- Manual-owned semantic boundary: `composed`

The checked-in Rust delivery is transport-first. `generated/server-openapi` contains the publishable transport crate generated from the latest live OpenAPI 3.x schema, while `composed` is the manual-owned boundary where the future `craw_chat_sdk` facade and `CrawChatSdkClient` semantics will be implemented.

## Workspace Layout

- `generated/server-openapi`
  Generator-owned Rust transport crate. Use the generated README in this directory for raw transport usage.
- `composed`
  Manual-owned semantic reserve for the Rust business SDK surface, higher-level helpers, and future realtime abstractions.
- `bin/`
  Thin forwarding wrappers that delegate to the root generation and verification pipeline.
- `README.md`
  Manual-owned workspace contract for the Rust lane.

Do not hand-edit generator output inside `generated/server-openapi`.

## Generate

Both workspace wrappers forward to the root pipeline, which refreshes the live service schema before generation.

```powershell
.\bin\sdk-gen.ps1
```

```bash
./bin/sdk-gen.sh
```

## Verify

Use the workspace verifier to confirm directory ownership, README contract, and assembly metadata alignment.

```powershell
.\bin\sdk-verify.ps1
```

```bash
./bin/sdk-verify.sh
```
