# SDKWork Craw Chat SDK Python Workspace

This workspace is the Python lane of the `sdkwork-craw-chat-sdk` multi-language delivery standard for Craw Chat.

## Current Standard

- Maturity tier: `Tier B`
- Target semantic client: `CrawChatSdkClient`
- Current generated transport package: `sdkwork-craw-chat-backend-sdk`
- Reserved semantic package: `sdkwork-craw-chat-sdk`
- Generator-owned boundary: `generated/server-openapi`
- Manual-owned semantic boundary: `composed`

The checked-in Python delivery is transport-first. `generated/server-openapi` contains the publishable transport package generated from the live OpenAPI 3.x schema, while `composed` is the manual-owned reserve for the future `CrawChatSdkClient` facade and Python business helpers.

## Workspace Layout

- `generated/server-openapi`
  Generator-owned Python transport package. See the generated README for raw package usage.
- `composed`
  Manual-owned semantic reserve for the app-facing Python SDK package.
- `bin/`
  Thin forwarding wrappers back to the root generator and verification pipeline.
- `README.md`
  Manual-owned workspace contract for the Python lane.

Do not hand-edit generated files under `generated/server-openapi`.

## Generate

The workspace wrapper delegates to the root generator, which refreshes the live service schema before regenerating the Python transport package.

```powershell
.\bin\sdk-gen.ps1
```

```bash
./bin/sdk-gen.sh
```

## Verify

The workspace verifier checks README terminology, boundary ownership, and assembly metadata alignment for Python delivery.

```powershell
.\bin\sdk-verify.ps1
```

```bash
./bin/sdk-verify.sh
```
