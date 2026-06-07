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

### Scripted Chat Validation

`open-chat-test.*` is the supported wrapper for scripted local chat verification. PowerShell uses
`-ScriptedValidation`; Bash and CMD-compatible flows use `--scripted-validation`.

The scripted contract keeps these names stable for automation:

- `ValidationMessage`
- `watchFrameTypes`
- `realtime.connected`
- `event.window`
- `OwnerPassword`
- `GuestPassword`

Use `open-chat-test` for end-to-end local validation before treating CLI or SDK samples as current.

## Server Lifecycle Scripts

| Script family | Description |
| --- | --- |
| `install-server.*` | Create the `craw-chat-server` install/config/data/log/run directory skeleton and stage canonical payload examples |
| `init-config-server.*` | Materialize the standard `server.yaml`, `server.env`, and related config overlays for one instance |
| `init-storage-server.*` | Validate the PostgreSQL storage contract and write the storage readiness report |
| `verify-server.*` | Validate `server.yaml`, PostgreSQL wiring, readiness inputs, and optional release-gate bundle integrity |
| `start-server.*` | Resolve the canonical `craw-chat-server` binary, start the unified gateway, and perform health waits |
| `status-server.*` | Show generated service contract paths, storage-report status, and optional release-gate summary |
| `restart-server.*` | Restart the managed `craw-chat-server` instance |
| `stop-server.*` | Stop the managed `craw-chat-server` instance |

## Server Service And Release Tools

| Script family | Description |
| --- | --- |
| `install-service-server.*` | Render generated `systemd`, `launchd`, and Windows Service wrapper contracts for one instance |
| `uninstall-service-server.*` | Remove generated or installed service-manager wiring for one instance |
| `plan-release-server.*` | Translate the machine-readable server release-gate bundle into an operator-facing release plan with the same semantic contract judgment used by `verify-server.*` and `status-server.*` |

## Server Release Helpers

The server release scripts now share helper-level release contract logic instead of re-implementing
their own bundle parsing:

- `verify-server-release-contracts.mjs`
  - canonical semantic audit of the server `release-gate` bundle
  - resolves the dependent `package-catalog`, `release-execution`, `release-provenance`,
    `release-checklist`, and per-platform `acceptance-manifest` files
  - returns `contractsValid`, `semanticIssueCount`, and `semanticIssues`
- `plan-release-server-contracts.mjs`
  - derives the selected platform release plan from the same machine-readable bundle
  - keeps `contractsValid` aligned with the semantic audit used by `verify-server.*`
  - emits platform plan fields such as `stagingReadmePath`, `checksumCommandExample`, and `status`

This means:

- `verify-server.*` validates runtime readiness plus semantic bundle integrity
- `status-server.*` exposes the same `releaseContracts` semantic summary alongside generated service
  contract paths
- `plan-release-server.*` will not report a healthy plan if the underlying bundle has semantic drift

## Smoke

| Script | Description |
| --- | --- |
| `tools/smoke/local_stack_smoke.ps1` | PowerShell smoke verification |
| `tools/smoke/local_stack_smoke.sh` | Bash smoke verification |

## Docs Site Commands

The VitePress docs site lives under `docs/sites`.

Install docs dependencies first:

```bash
npm ci
```

Run `npm ci` inside `docs/sites` before any `docs:*` task.

| Command | Working directory | Purpose |
| --- | --- | --- |
| `npm run docs:generate` | `docs/sites` | Standardize source API markdown and regenerate operation pages |
| `npm run docs:dev` | `docs/sites` | Standardize source API markdown, regenerate operation pages, then start the local VitePress dev server |
| `npm run docs:build` | `docs/sites` | Standardize source API markdown, regenerate operation pages, verify API plus SDK docs, then build the static docs site |
| `npm run docs:preview` | `docs/sites` | Preview the previously built site |
| `npm run docs:verify` | `docs/sites` | Standardize source API markdown, regenerate operation pages, and run API plus SDK docs verification |

The `npm run docs:*` scripts route through `scripts/run-docs-task.mjs` and keep an
`npm_node_execpath` fallback when the shell cannot resolve a bare `node` command.

Direct wrappers are also available when you want to bypass shell-specific `npm run` behavior:

| Wrapper | Working directory | Purpose |
| --- | --- | --- |
| `scripts/run-docs-task.cmd verify` | `docs/sites` | Windows CMD entry for docs verify/build/dev/generate/preview tasks |
| `powershell -ExecutionPolicy Bypass -File scripts/run-docs-task.ps1 verify` | `docs/sites` | PowerShell entry for docs verify/build/dev/generate/preview tasks |
| `sh scripts/run-docs-task.sh verify` | `docs/sites` | POSIX shell entry for docs verify/build/dev/generate/preview tasks |

Notes:

- `docs:generate`, `docs:verify`, `docs:build`, and `docs:dev` first standardize the overview API markdown and then regenerate operation pages. This keeps generated operation references aligned with the maintained source overview documents.
- `docs:verify` and `docs:build` also run API and SDK docs verification before invoking VitePress build-time work.
- `docs:preview` serves the already built site and does not mutate docs content.
- In Windows shells where `npm run` loses the normal `PATH`, the `npm_node_execpath` fallback keeps the task runnable; the direct `.cmd` and `.ps1` wrappers remain the cleanest explicit entrypoints.
- `docs:build`, `docs:dev`, and `docs:preview` require `vitepress` plus an `esbuild` child process. In restricted Windows shells that block child-process execution, use `docs:verify` in-place and run the VitePress commands from a normal local terminal.

The SDK workspaces under `sdks/` have their own root command wrappers.

### IM Standard SDK Verification

The IM standard SDK workspace is `sdks/sdkwork-im-sdk`.

Its checked-in authority and derived generator inputs live under `openapi/`:

- `openapi/craw-chat-im.openapi.yaml`
- `openapi/craw-chat-im.sdkgen.yaml`
- `openapi/craw-chat-im.flutter.sdkgen.yaml`

Run from the repository root:

```powershell
node .\sdks\sdkwork-im-sdk\bin\prepare-openapi-source.mjs
node .\sdks\sdkwork-im-sdk\bin\verify-sdk.mjs
node .\sdks\sdkwork-im-sdk\bin\verify-sdk.mjs --with-dart
```

That path validates the root workspace, language packages, and final assembly output. Successful
verification updates `.sdkwork-assembly.json`, which records package `manifestPath` values, the
`generated` / `composed` layer split, and the stable `generatedAt` timestamp used for release-facing
inspection.

### App API And Backend SDK Boundary Materialization

The app and backend SDK boundaries are materialized from the current OpenAPI authorities by:

```powershell
node .\sdks\materialize-im-v3-openapi-boundaries.mjs
```

That command consolidates backend management, control, and admin APIs into
`sdkwork-im-backend-sdk`, and moves non-management backend paths into `sdkwork-im-app-sdk`.

### App API SDK Verification

The app-business generated HTTP SDK workspace is `sdks/sdkwork-im-app-sdk`.

Run from the repository root:

```powershell
node .\sdks\sdkwork-im-app-sdk\bin\verify-sdk.mjs
node .\sdks\sdkwork-im-app-sdk\bin\generate-sdk.mjs --language typescript
```

The app SDK owns `/app/v3/api/*` and must not contain backend, admin, or control routes.

### Backend SDK Verification

The backend management SDK workspace is `sdks/sdkwork-im-backend-sdk`.

Run from the repository root:

```powershell
node .\sdks\sdkwork-im-backend-sdk\bin\verify-sdk.mjs
node .\sdks\sdkwork-im-backend-sdk\bin\generate-sdk.mjs --language typescript
```

The backend SDK owns `/backend/v3/api/*`, including `/backend/v3/api/control/*` and
`/backend/v3/api/admin/*`. Do not use a separate admin or control SDK family.

### RTC SDK Verification

The RTC provider-standard SDK workspace is `../../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk`.

Run from the repository root:

```powershell
node ../../../../sdkwork-rtc\sdks\sdkwork-rtc-sdk\bin\verify-sdk.mjs
```

Run full smoke only when required language toolchains are available:

```powershell
node ../../../../sdkwork-rtc\sdks\sdkwork-rtc-sdk\bin\smoke-sdk.mjs
```

The RTC SDK is independent from OpenAPI-generated HTTP SDK families.

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
./bin/install-server.ps1 -Help
./bin/start-server.ps1 -Help
./bin/verify-server.ps1 -Help
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
4. Formal server installs should preserve the canonical startup contract
   `craw-chat-server --config <config-root>/server.yaml` rather than inventing a parallel service
   command line.

## What To Read Next

- [Deployment](/deployment/index)
- [Runtime Operations](/deployment/runtime-operations)
- [Quick Start](/getting-started/quick-start)
