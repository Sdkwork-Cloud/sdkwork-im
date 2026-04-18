# CLI and Scripts

This page summarizes the user-visible command entry points that are already present in the
repository. Use it when you need the supported shell entrypoint rather than the HTTP API or SDK
surface.

## Local Lifecycle Scripts

| Script family | Description |
| --- | --- |
| `install-local.*` | Build the local node and prepare runtime directories |
| `init-config-local.*` | Create or refresh local profile config |
| `start-local.*` | Start the local node |
| `status-local.*` | Show current runtime status |
| `restart-local.*` | Restart the local node |
| `stop-local.*` | Stop the local node |
| `deploy-local.*` | Delegate Docker deployment to the bootstrap script |

## Runtime-management Scripts

| Script family | Description |
| --- | --- |
| `inspect-runtime-local.*` | Inspect managed state files |
| `repair-runtime-local.*` | Repair missing managed state files |
| `list-runtime-backups-local.*` | List backup snapshots |
| `archive-runtime-backup-local.*` | Archive one backup snapshot |
| `prune-runtime-archives-local.*` | Prune expired archived backups |
| `preview-runtime-restore-local.*` | Preview a restore without applying it |
| `restore-runtime-local.*` | Restore a backup snapshot |

## Local Verification Tools

| Script or binary | Description |
| --- | --- |
| `open-chat-test.*` | Open the local chat verification entry path |
| `chat-cli.*` | CLI for health, token generation, conversation creation, messaging, timeline reads, and interactive chat sessions |
| `chat-cli-local.*` | Local-profile forwarding wrapper for chat CLI workflows |
| `chat-window.*` | Interactive terminal window backed by `chat-cli` |
| `chat-window-gui.*` | Windows GUI wrapper around the chat-window workflow |

## Smoke

| Script | Description |
| --- | --- |
| `tools/smoke/local_stack_smoke.ps1` | PowerShell smoke verification |
| `tools/smoke/local_stack_smoke.sh` | Bash smoke verification |

## Docs Site Commands

The VitePress docs site lives under `docs/sites`.

| Command | Working directory | Purpose |
| --- | --- | --- |
| `npm run docs:dev` | `docs/sites` | Start the local VitePress dev server |
| `npm run docs:build` | `docs/sites` | Build the static docs site |
| `npm run docs:preview` | `docs/sites` | Preview the built site |
| `npm run docs:verify` | `docs/sites` | Run docs runtime checks, generate operation pages, and verify required docs references |

## SDK Workspace Commands

The SDK workspaces under `sdks/` have their own root command wrappers.

### App SDK Derived Specs And Verification

The app-facing SDK workspace is `sdks/sdkwork-craw-chat-sdk`.

Its checked-in authority and derived generator inputs live under `openapi/`:

- `openapi/craw-chat-app.openapi.yaml`
- `openapi/craw-chat-app.sdkgen.yaml`
- `openapi/craw-chat-app.flutter.sdkgen.yaml`

Run from the repository root:

```powershell
node .\sdks\sdkwork-craw-chat-sdk\bin\prepare-openapi-source.mjs
node .\sdks\sdkwork-craw-chat-sdk\bin\verify-sdk.mjs
node .\sdks\sdkwork-craw-chat-sdk\bin\verify-sdk.mjs --with-dart
```

That path validates the root workspace, language packages, and final assembly output. Successful
verification updates `.sdkwork-assembly.json`, which records package `manifestPath` values, the
`generated` / `composed` layer split, and the stable `generatedAt` timestamp used for release-facing
inspection.

### Admin SDK Derived Specs And Verification

The admin control-plane SDK workspace is `sdks/sdkwork-craw-chat-sdk-admin`.

### Admin SDK Contract Refresh

Run from the repository root when you need to refresh the checked-in admin authority contract:

```powershell
node .\sdks\sdkwork-craw-chat-sdk-admin\bin\fetch-openapi-source.mjs
node .\sdks\sdkwork-craw-chat-sdk-admin\bin\prepare-openapi-source.mjs
```

That flow refreshes:

- `openapi/admin-control-plane.openapi.yaml`
- `openapi/admin-control-plane.sdkgen.yaml`

### Admin SDK Verification And Assembly

Run from the repository root:

```powershell
node .\sdks\sdkwork-craw-chat-sdk-admin\bin\verify-sdk.mjs --language typescript --language flutter
node .\sdks\sdkwork-craw-chat-sdk-admin\bin\verify-sdk.mjs --language flutter --with-dart
```

The root verification path runs workspace automation checks, language verification, and final
assembly refresh. Successful verification updates `.sdkwork-assembly.json`, which records the
workspace package inventory, generated package `manifestPath` values, the `generated` / `composed`
layer split, and the stable `generatedAt` timestamp used for release-facing inspection.

## Help Conventions

- PowerShell lifecycle and runtime-management scripts use `-Help`
- many Bash wrappers use `--help`
- `chat-cli` uses `--help`

Examples:

```powershell
./bin/start-local.ps1 -Help
./bin/deploy-local.ps1 -Help
./bin/inspect-runtime-local.ps1 -Help
./bin/chat-window.ps1 -Help
```

```powershell
./bin/chat-cli.ps1 --help
```

## Integration Guidance

1. External systems should integrate with the documented HTTP APIs, not treat `chat-cli` as an SDK.
2. Automated Docker bring-up should use `deploy-local.*` or `bootstrap-local.ps1` instead of
   inlining a different local contract.
3. Runtime repair and restore should use the managed scripts and binary entrypoints before editing
   `state/*.json` manually.

## What To Read Next

- [Deployment](/deployment/index)
- [Runtime Operations](/deployment/runtime-operations)
- [Quick Start](/getting-started/quick-start)
