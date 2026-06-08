# Server 版本安装与初始化

`craw-chat-server` 是 SDKWork Chat 的正式 server 安装入口。当前应用 app code 是 `chat`，发布包名是 `sdkwork-chat`，对外路径是 `/sdkwork/chat`。

统一启动入口：

```text
craw-chat-server --config <config-root>/chat.toml
```

## 命令面

- `install-server`
- `init-config-server`
- `init-storage-server`
- `verify-server`
- `plan-release-server`
- `install-service-server`
- `start-server`
- `stop-server`
- `restart-server`
- `status-server`

## 首次安装流程

1. 运行 `install-server`
2. 运行 `init-config-server`
3. 配置 PostgreSQL 和 Redis
4. 运行 `init-storage-server`
5. 运行 `verify-server`
6. 运行 `install-service-server`
7. 运行 `start-server`

PowerShell 示例：

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\install-server.ps1 -InstanceName default
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\init-config-server.ps1 -InstanceName default
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\init-storage-server.ps1 -InstanceName default -Mode verify-only
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\verify-server.ps1 -InstanceName default
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\install-service-server.ps1 -InstanceName default
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\start-server.ps1 -InstanceName default -Release
```

Bash 示例：

```bash
bash bin/install-server.sh --instance default
bash bin/init-config-server.sh --instance default
bash bin/init-storage-server.sh --instance default --mode verify-only
bash bin/verify-server.sh --instance default
bash bin/install-service-server.sh --instance default
bash bin/start-server.sh --instance default --release
```

## 标准路径矩阵

Linux archive:

- Install root: `/opt/sdkwork/chat`
- Config root: `/etc/sdkwork/chat`
- Config file: `/etc/sdkwork/chat/chat.toml`
- Env file: `/etc/sdkwork/chat/server.env`
- PostgreSQL helper config: `/etc/sdkwork/chat/postgresql.yaml`
- PostgreSQL password file: `/etc/sdkwork/chat/database.secret`
- Data root: `/var/lib/sdkwork/chat`
- Log root: `/var/log/sdkwork/chat`
- Run root: `/run/sdkwork/chat`

macOS service:

- Install root: `/usr/lib/sdkwork/chat`
- Config root: `/Library/Application Support/sdkwork/chat`
- Data root: `/Library/Application Support/sdkwork/chat/Data`
- Log root: `/Library/Logs/sdkwork/chat`
- Run root: `/Library/Application Support/sdkwork/chat/Run`

Windows Service:

- Install root: `%ProgramFiles%/sdkwork/chat`
- Config root: `%ProgramData%/sdkwork/chat`
- Data root: `%ProgramData%/sdkwork/chat/Data`
- Log root: `%ProgramData%/sdkwork/chat/Logs`
- Run root: `%ProgramData%/sdkwork/chat/Run`

Desktop user data is separate from server data. Desktop defaults to SQLite at `~/.sdkwork/chat/data/chat.sqlite` or `%USERPROFILE%/.sdkwork/chat/data/chat.sqlite`.

## Release payload contract

Server archives must include:

- `bin/craw-chat-server` or `bin/craw-chat-server.exe`
- `config/chat.toml.example`
- `config/server.env.example`
- `config/postgresql.yaml.example`
- `bin/*server*` lifecycle scripts
- `service/linux/craw-chat-server.service`
- `service/macos/com.sdkwork.crawchat.server.plist`
- `service/windows/CrawChatServer.xml`
- `web/sdkwork-chat-pc/dist`
- `INSTALL.md`
- `install-manifest.json`

Packages must not include `.env`, `.env.postgres`, `.env.release.local`, local SQLite databases, generated runtime state, `node_modules`, Git metadata, `database.secret`, or other secrets.

## Database contract

Server release packages default to PostgreSQL:

```env
SDKWORK_CHAT_DATABASE_ENGINE=postgresql
SDKWORK_CHAT_DATABASE_HOST=db.example.com
SDKWORK_CHAT_DATABASE_PORT=5432
SDKWORK_CHAT_DATABASE_NAME=sdkwork_chat_prod
SDKWORK_CHAT_DATABASE_SCHEMA=sdkwork_chat_prod
SDKWORK_CHAT_DATABASE_USERNAME=sdkwork_chat_prod
SDKWORK_CHAT_DATABASE_PASSWORD_FILE=/etc/sdkwork/chat/database.secret
SDKWORK_CHAT_DATABASE_SSL_MODE=require
```

Desktop packages default to SQLite:

```toml
[database]
engine = "sqlite"
file = "~/.sdkwork/chat/data/chat.sqlite"
max_connections = 1
```

## Service package matrix

- Linux archive forms: `tar.gz`
  - initialization entrypoints: `install-server.sh`, `init-config-server.sh`, `init-storage-server.sh`, `install-service-server.sh`
  - checksum command: `sha256sum -b <artifact> >> SHA256SUMS`
- macOS archive forms: `tar.gz`
  - initialization entrypoints: `install-server.sh`, `init-config-server.sh`, `init-storage-server.sh`, `install-service-server.sh`
  - checksum command: `shasum -a 256 <artifact> >> SHA256SUMS`
- Windows archive forms: `zip`
  - initialization entrypoints: `install-server.ps1`, `init-config-server.ps1`, `init-storage-server.ps1`, `install-service-server.ps1`
  - Windows package is wrapper-required and includes `bin/CrawChatServer.exe`
  - checksum command: `Get-FileHash -Algorithm SHA256 <artifact>`

## Unified gateway endpoints

After `start-server`, the unified `web-gateway` port is the operator-facing entry for:

- `/healthz`
- `/readyz`
- `/openapi.json`
- `/openapi/index.json`
- `/docs`

The gateway also exposes upstream operational service schemas at `/openapi/services/<service-id>.openapi.json` and rendered docs at `/docs/services/<service-id>`.

## Release audit commands

`verify-server`, `plan-release-server`, and `status-server` can audit release bundle contracts with a release gate manifest.

The same server release bundle freezes these machine-readable manifests:

- `artifacts/releases/wave-d-2026-04-08/server/package-catalog.json`
- `artifacts/releases/wave-d-2026-04-08/server/release-execution.json`
- `artifacts/releases/wave-d-2026-04-08/server/release-provenance.json`
- `artifacts/releases/wave-d-2026-04-08/server/release-gate.json`
- platform staging acceptance manifests named `acceptance-manifest.json`

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\verify-server.ps1 `
  -InstanceName default `
  -OutputFormat json `
  -ReleaseGatePath .\artifacts\releases\wave-d-2026-04-08\server\release-gate.json
```

```bash
bash bin/verify-server.sh \
  --instance default \
  --output-format json \
  --release-gate-path artifacts/releases/wave-d-2026-04-08/server/release-gate.json
```

The runtime readiness and release-contract validity surfaces must report the same package IDs, startup command, service manager, staged payload entries, and checksum manifests.
