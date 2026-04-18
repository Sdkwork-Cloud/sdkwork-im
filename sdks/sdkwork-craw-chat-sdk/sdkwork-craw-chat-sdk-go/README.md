# SDKWork Craw Chat SDK Go Workspace

This workspace is the Go lane of the `sdkwork-craw-chat-sdk` multi-language delivery standard for Craw Chat.

## Current Standard

- Maturity tier: `Tier B`
- Target semantic client: `CrawChatSdkClient`
- Current generated transport module: `github.com/sdkwork/craw-chat-backend-sdk`
- Reserved semantic module: `github.com/sdkwork/craw-chat-sdk`
- Generator-owned boundary: `generated/server-openapi`
- Manual-owned semantic boundary: `composed`

The checked-in Go delivery is transport-first. `generated/server-openapi` is the publishable transport module generated from the live OpenAPI 3.x schema, while `composed` is the manual-owned reserve for the future `CrawChatSdkClient` facade and higher-level Go helpers.

## Workspace Layout

- `generated/server-openapi`
  Generator-owned Go transport module. Use its local README for raw transport imports and API usage.
- `composed`
  Manual-owned semantic reserve for the future Go app-facing SDK module.
- `bin/`
  Thin forwarding wrappers to the root generation and verification entrypoints.
- `README.md`
  Manual-owned workspace contract for the Go lane.

Do not edit files under `generated/server-openapi` manually.

## Generate

The workspace wrapper forwards to the root generator, which refreshes the latest live schema before regenerating the Go transport module.

```powershell
.\bin\sdk-gen.ps1
```

```bash
./bin/sdk-gen.sh
```

## Verify

The workspace verifier confirms README language markers, boundary ownership, and assembly metadata consistency.

```powershell
.\bin\sdk-verify.ps1
```

```bash
./bin/sdk-verify.sh
```
