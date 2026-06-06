# Server 版本 Service 托管标准

`craw-chat-server` 对 operator 暴露一个正式 service 身份。应用目录、配置、日志和运行状态统一使用 app code `chat`。

## 跨平台目标

- Linux: `systemd`
- macOS: `launchd`
- Windows: Windows Service `CrawChatServer`

统一托管启动命令：

```text
craw-chat-server --config <config-root>/chat.toml
```

## 标准路径

Linux:

- Install root: `/opt/sdkwork/chat`
- Config root: `/etc/sdkwork/chat`
- Config file: `/etc/sdkwork/chat/chat.toml`
- Env file: `/etc/sdkwork/chat/server.env`
- Data root: `/var/lib/sdkwork/chat`
- Log root: `/var/log/sdkwork/chat`
- Run root: `/run/sdkwork/chat`

Windows Service:

- Install root: `%ProgramFiles%/sdkwork/chat`
- Config root: `%ProgramData%/sdkwork/chat`
- Config file: `%ProgramData%/sdkwork/chat/chat.toml`
- Env file: `%ProgramData%/sdkwork/chat/server.env`
- Data root: `%ProgramData%/sdkwork/chat/Data`
- Log root: `%ProgramData%/sdkwork/chat/Logs`
- Run root: `%ProgramData%/sdkwork/chat/Run`

macOS service:

- Install root: `/usr/lib/sdkwork/chat`
- Config root: `/Library/Application Support/sdkwork/chat`
- Config file: `/Library/Application Support/sdkwork/chat/chat.toml`
- Env file: `/Library/Application Support/sdkwork/chat/server.env`
- Data root: `/Library/Application Support/sdkwork/chat/Data`
- Log root: `/Library/Logs/sdkwork/chat`
- Run root: `/Library/Application Support/sdkwork/chat/Run`

Default instance paths do not append `default`. Non-default instances append `instances/<name>` under config/data/log/run roots only when the platform script explicitly supports multi-instance mode.

## Service templates

- systemd template: `deployments/systemd/craw-chat-server.service`
- launchd template: `deployments/launchd/com.sdkwork.crawchat.server.plist`
- Windows Service wrapper template: `deployments/windows-service/CrawChatServer.xml`
- launchd label: `com.sdkwork.crawchat.server`
- Windows Service name: `CrawChatServer`

`install-service-server` renders instance-specific files under `<config-root>/generated/`:

- `craw-chat-server.service`
- `com.sdkwork.crawchat.server.plist`
- `CrawChatServer.xml`
- `install-CrawChatServer.ps1`
- `uninstall-CrawChatServer.ps1`

## Runtime contract

- Foreground start, systemd, launchd, and Windows Service must share the same `chat.toml` config source.
- `SDKWORK_CHAT_CONFIG_FILE` may override the config path.
- `SDKWORK_CHAT_LOG_DIR` must point at the platform log root.
- Server deployments default to PostgreSQL and Redis.
- Desktop deployments default to SQLite and Redis disabled.
- Browser-visible variables must not expose database URLs, Redis URLs, or password files.

## Unified Gateway Contract

- The externally exposed service entry remains `web-gateway`.
- Standard operator endpoints: `/healthz`, `/readyz`, `/openapi.json`, `/openapi/index.json`, and `/docs`.
- Upstream operational service schema proxies remain rooted at `/openapi/services/<service-id>.openapi.json`.
- Per-service rendered docs remain rooted at `/docs/services/<service-id>`.
