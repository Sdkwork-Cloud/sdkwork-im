# Quick Start

This is the shortest verified path to a working local Craw Chat app node.

## 1. Initialize Local Config

### PowerShell

```powershell
./bin/install-local.ps1
./bin/init-config-local.ps1
```

```powershell
./bin/install-local.ps1 -ProfileName local-default
./bin/init-config-local.ps1 -ProfileName local-default
```

### Bash

```bash
./bin/install-local.sh
./bin/init-config-local.sh
```

```bash
./bin/install-local.sh --profile local-default
./bin/init-config-local.sh --profile local-default
```

### Windows CMD

```cmd
bin\install-local.cmd
bin\init-config-local.cmd
```

```cmd
bin\install-local.cmd --profile local-default
bin\init-config-local.cmd --profile local-default
```

## 2. Review the Generated Runtime Config

| Profile | Primary config file | Effective runtime directory |
| --- | --- | --- |
| `local-minimal` | `.runtime/local-minimal/config/local-minimal.env` | `.runtime/local-minimal` |
| `local-default` | `.runtime/local-default/config/local-default.env` | Falls back to `.runtime/local-minimal` unless you override `CRAW_CHAT_RUNTIME_DIR` |

Minimum expected keys:

```dotenv
CRAW_CHAT_BIND_ADDR=127.0.0.1:18090
CRAW_CHAT_RUNTIME_DIR=.runtime/local-minimal
CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET=<generated-or-manually-set-secret>
```

If `CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET` is missing, `start-local.*` refuses to start the public
app node.

## 3. Start the Local Node

### PowerShell

```powershell
./bin/start-local.ps1
./bin/start-local.ps1 -ProfileName local-default
```

Useful flags:

```powershell
./bin/start-local.ps1 -Foreground
./bin/start-local.ps1 -BindAddress 127.0.0.1:28090
./bin/start-local.ps1 -Release
./bin/start-local.ps1 -Help
```

### Bash

```bash
./bin/start-local.sh
./bin/start-local.sh --profile local-default
```

### Windows CMD

```cmd
bin\start-local.cmd
bin\start-local.cmd --profile local-default
```

`start-local.ps1` builds the node if needed, resolves config and runtime paths, writes PID and log
paths, starts the process, and waits up to 30 seconds for `/healthz` to return `200`.

## 4. Verify Health

```bash
curl http://127.0.0.1:18090/healthz
curl http://127.0.0.1:18090/readyz
```

Typical runtime files:

- `logs/local-minimal-node.out.log`
- `logs/local-minimal-node.err.log`
- `pids/local-minimal-node.pid`

## 5. Check Status, Restart, and Stop

### PowerShell

```powershell
./bin/status-local.ps1
./bin/restart-local.ps1
./bin/stop-local.ps1
```

```powershell
./bin/status-local.ps1 -ProfileName local-default
./bin/restart-local.ps1 -ProfileName local-default
./bin/stop-local.ps1 -ProfileName local-default
```

### Bash

```bash
./bin/status-local.sh
./bin/restart-local.sh
./bin/stop-local.sh
```

`status-local.ps1` also prints the next runtime-management commands for inspect, repair, backup,
archive, preview, and restore.

## 6. Run Smoke Verification

### PowerShell

```powershell
powershell -ExecutionPolicy Bypass -File tools\smoke\local_stack_smoke.ps1
```

```powershell
powershell -ExecutionPolicy Bypass -File tools\smoke\local_stack_smoke.ps1 -BaseUrl http://127.0.0.1:28090
```

### Docker bootstrap path

```powershell
./bin/deploy-local.ps1 -ProfileName local-minimal
./bin/deploy-local.ps1 -ProfileName local-default -SmokeBaseUrl http://127.0.0.1:28090
```

```bash
bash bin/deploy-local.sh --profile local-default --smoke-base-url http://127.0.0.1:28090
```

The smoke script waits for health, generates a signed local bearer token, creates a conversation,
posts a message, and verifies the resulting conversation summary.

## 7. Use the Local Verification Tools

```powershell
./bin/open-chat-test.ps1
./bin/chat-cli.ps1 --help
./bin/chat-window.ps1 -Help
```

These scripts are for local verification and demo workflows. They are not substitutes for the
generated SDK families.

## 8. First Manual API Call

The easiest path is to reuse the smoke script, but if you call the HTTP API directly, use:

- `Authorization: Bearer <HS256 JWT>`
- The same signing secret configured in `CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET`
- Claims that at least identify tenant and subject

The public app surface is documented in [App API Overview](/api-reference/app-api).

::: tip Recommended first-run profile
If you only need a stable local verification path, use `local-minimal` first. The `local-default`
name is already wired into scripts and config, but it still resolves to the current
`local-minimal` runtime contract.
:::
