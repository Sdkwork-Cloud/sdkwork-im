# SDKWork Craw Chat SDK Swift Workspace

This workspace is the Swift lane of the `sdkwork-craw-chat-sdk` multi-language delivery standard for Craw Chat.

## Current Standard

- Maturity tier: `Tier B`
- Target semantic client: `CrawChatSdkClient`
- Current generated transport package: `CrawChatBackendSdk`
- Reserved semantic package: `CrawChatSdk`
- Generator-owned boundary: `generated/server-openapi`
- Manual-owned semantic boundary: `composed`

The checked-in Swift delivery is transport-first. `generated/server-openapi` contains the generated transport package from the live OpenAPI 3.x schema, while `composed` stays manual-owned for the future `CrawChatSdkClient` semantic facade and Swift-specific conveniences.

## Workspace Layout

- `generated/server-openapi`
  Generator-owned Swift transport package. See the generated README in that directory for raw package usage.
- `composed`
  Manual-owned semantic reserve for the app-facing Swift SDK.
- `bin/`
  Thin forwarding wrappers to the root generation and verification entrypoints.
- `README.md`
  Manual-owned workspace contract for the Swift lane.

Do not hand-edit generated files under `generated/server-openapi`.

## Generate

The workspace wrapper delegates to the root generator, which refreshes the live service schema before regenerating the Swift transport package.

```powershell
.\bin\sdk-gen.ps1
```

```bash
./bin/sdk-gen.sh
```

## Verify

Use the workspace verifier to confirm README alignment, package-boundary ownership, and assembly metadata correctness.

```powershell
.\bin\sdk-verify.ps1
```

```bash
./bin/sdk-verify.sh
```
