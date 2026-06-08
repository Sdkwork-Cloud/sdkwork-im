# Local Binary

The local binary workflow is the most complete and transparent deployment path in the repository.
It is not the formal packaged `craw-chat-server` install contract.
The local binary workflow is optimized for development, and it is not the formal packaged `craw-chat-server` install contract.

## Lifecycle Scripts

| Script | Purpose |
| --- | --- |
| `install-local.*` | Build `local-minimal-node`, initialize config, and prepare runtime directories |
| `init-config-local.*` | Create or refresh the selected profile config file |
| `start-local.*` | Start the node with PID, log, and health-wait management |
| `status-local.*` | Show PID, config, logs, health, and the next runtime-management commands |
| `restart-local.*` | Restart the managed local node |
| `stop-local.*` | Stop the managed local node |

## Install And Initialize

### PowerShell

```powershell
./bin/install-local.ps1
./bin/init-config-local.ps1
./bin/install-local.ps1 -Help
```

### Bash

```bash
./bin/install-local.sh
./bin/init-config-local.sh
./bin/install-local.sh --help
```

### CMD

```cmd
bin\install-local.cmd
bin\init-config-local.cmd
```

`install-local.ps1` requires `cargo`, forwards to `init-config-local.ps1`, prepares the runtime
directory, and builds `local-minimal-node` offline in debug mode by default.

## Start The Node

```powershell
./bin/start-local.ps1
./bin/start-local.ps1 -Foreground
./bin/start-local.ps1 -Release
./bin/start-local.ps1 -BindAddress 127.0.0.1:28090
./bin/start-local.ps1 -Help
```

PowerShell uses `-Help`. CMD and Bash wrappers expose shell-style help flags for their own syntax.

### What The Start Script Does

1. Runs `install-local.ps1`
2. Resolves the effective config file and runtime directory
3. Resolves the optional `CRAW_CHAT_FRIEND_REQUEST_CURSOR_HS256_SECRET`
4. Resolves `CRAW_CHAT_APP_CONTEXT_REQUIRE_SIGNATURE` and `CRAW_CHAT_APP_CONTEXT_SIGNATURE_SECRET` from the selected local profile config
5. Starts the process in foreground or background mode
6. Writes PID and log targets
7. Waits up to 30 seconds for `/healthz`

## Runtime Logs And PID Files

The managed runtime directory contains:

- `logs/local-minimal-node.out.log`
- `logs/local-minimal-node.err.log`
- `pids/local-minimal-node.pid`

## Status And Health

```powershell
./bin/status-local.ps1
./bin/status-local.ps1 -Help
```

The status script prints:

- profile name
- config path
- bind address
- health URL
- stdout and stderr log paths
- PID, if running
- next commands for inspect, repair, backup catalog, archive, preview, and restore

## Best-practice Workflow

1. Run `install-local.*`
2. Run `start-local.*`
3. Run `status-local.*`
4. Run `tools/smoke/local_stack_smoke.ps1` when you want an end-to-end sanity check
5. Use the runtime-management scripts before editing state files manually

## What To Read Next

- Switch to [Server Lifecycle](/deployment/server-lifecycle) when you need the packaged single-port
  server, unified gateway, config-root layout, or PostgreSQL-backed server install contract.
- [Runtime Operations](/deployment/runtime-operations)
- [Profiles and Environment](/deployment/profiles-and-env)
- [CLI and Scripts](/reference/cli-and-scripts)
