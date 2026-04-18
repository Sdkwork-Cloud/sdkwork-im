# SDKWork Craw Chat SDK Java Workspace

This workspace is the Java lane of the `sdkwork-craw-chat-sdk` multi-language delivery standard for Craw Chat.

## Current Standard

- Maturity tier: `Tier B`
- Target semantic client: `CrawChatSdkClient`
- Current generated transport artifact: `com.sdkwork:craw-chat-backend-sdk`
- Reserved semantic artifact: `com.sdkwork:craw-chat-sdk`
- Generator-owned boundary: `generated/server-openapi`
- Manual-owned semantic boundary: `composed`

The checked-in Java delivery is transport-first. `generated/server-openapi` contains the publishable transport artifact generated from the live OpenAPI 3.x schema, while `composed` remains the manual-owned reserve for the future `CrawChatSdkClient` facade and Java-specific business helpers.

## Workspace Layout

- `generated/server-openapi`
  Generator-owned Java transport artifact. Use `generated/server-openapi/README.md` when you need the raw OpenAPI transport surface.
- `composed`
  Manual-owned semantic reserve for the eventual Java app-facing SDK package.
- `bin/`
  Thin forwarding wrappers back to the root generation and verification entrypoints.
- `README.md`
  Manual-owned workspace contract for the Java lane.

Do not hand-edit files under `generated/server-openapi`.

## Generate

The workspace wrapper delegates to the root generator, which refreshes the latest live schema before regenerating the Java transport artifact.

```powershell
.\bin\sdk-gen.ps1
```

```bash
./bin/sdk-gen.sh
```

## Verify

The workspace verifier checks README ownership, assembly metadata, and the required language markers for the Java lane.

```powershell
.\bin\sdk-verify.ps1
```

```bash
./bin/sdk-verify.sh
```
