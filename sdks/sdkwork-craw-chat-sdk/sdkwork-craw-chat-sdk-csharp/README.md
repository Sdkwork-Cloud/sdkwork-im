# SDKWork Craw Chat SDK C# Workspace

This workspace is the C# lane of the `sdkwork-craw-chat-sdk` multi-language delivery standard for Craw Chat.

## Current Standard

- Maturity tier: `Tier B`
- Target semantic client: `CrawChatSdkClient`
- Current generated transport package: `Sdkwork.CrawChat.BackendSdk`
- Reserved semantic package: `Sdkwork.CrawChat.Sdk`
- Generator-owned boundary: `generated/server-openapi`
- Manual-owned semantic boundary: `composed`

The checked-in C# delivery is transport-first. `generated/server-openapi` is the publishable transport package generated from the live OpenAPI 3.x schema, while `composed` is reserved for the future `CrawChatSdkClient` semantic facade and higher-level chat workflows.

## Workspace Layout

- `generated/server-openapi`
  Generator-owned C# transport package. Use its local README for raw HTTP/OpenAPI usage.
- `composed`
  Manual-owned semantic reserve for the app-facing C# SDK layer.
- `bin/`
  Thin forwarding wrappers to the root generator and verification pipeline.
- `README.md`
  Manual-owned workspace contract for the C# lane.

Do not edit generated files under `generated/server-openapi` by hand.

## Generate

The wrapper forwards to the root generator, which refreshes the latest live schema before building the C# transport package.

```powershell
.\bin\sdk-gen.ps1
```

```bash
./bin/sdk-gen.sh
```

## Verify

The workspace verifier checks the README contract, generated/composed boundary ownership, and assembly metadata alignment.

```powershell
.\bin\sdk-verify.ps1
```

```bash
./bin/sdk-verify.sh
```
