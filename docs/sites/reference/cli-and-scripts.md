# CLI and Scripts

This page summarizes the user-visible command entry points that are already present in the
repository.

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
