# Server 版本 Service 托管标准

`sdkwork-im-server` �?operator 暴露一个正�?service 身份。应用目录、配置、日志和运行状态统一使用 app code `chat`�?

## 跨平台目�?

- Linux: `systemd`
- macOS: `launchd`
- Windows: Windows Service `SdkworkImServer`

统一托管启动命令�?

```text
sdkwork-im-server --config <config-root>/chat.toml
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

- systemd template: `deployments/systemd/sdkwork-im-server.service`
- launchd template: `deployments/launchd/com.sdkwork.SdkworkIm.server.plist`
- Windows Service wrapper template: `deployments/windows-service/SdkworkImServer.xml`
- launchd label: `com.sdkwork.SdkworkIm.server`
- Windows Service name: `SdkworkImServer`

`install-service-server` renders instance-specific files under `<config-root>/generated/`:

- `sdkwork-im-server.service`
- `com.sdkwork.SdkworkIm.server.plist`
- `SdkworkImServer.xml`
- `install-SdkworkImServer.ps1`
- `uninstall-SdkworkImServer.ps1`

## Runtime contract

- Foreground start, systemd, launchd, and Windows Service must share the same `chat.toml` config source.
- `SDKWORK_IM_CONFIG_FILE` may override the config path.
- `SDKWORK_IM_LOG_DIR` must point at the platform log root.
- Server deployments default to PostgreSQL and Redis.
- Desktop deployments default to SQLite and Redis disabled.
- Browser-visible variables must not expose database URLs, Redis URLs, or password files.

## Unified Gateway Contract

- The externally exposed service entry remains `sdkwork-im-gateway`.
- Standard operator endpoints: `/healthz`, `/readyz`, `/openapi.json`, `/openapi/index.json`, and `/docs`.
- Upstream operational service schema proxies remain rooted at `/openapi/services/<service-id>.openapi.json`.
- Per-service rendered docs remain rooted at `/docs/services/<service-id>`.
