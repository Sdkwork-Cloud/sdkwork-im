# Server 版本 Service 托管标准

`craw-chat-server` 对 operator 暴露一个正式 service 身份，不要求 operator 直接管理内部业务服务集合。

## 跨平台目标

- Linux: `systemd`
- macOS: `launchd`
- Windows Service: `CrawChatServer`

## 当前阶段

- 已提供 `systemd` unit 模板：`deployments/systemd/craw-chat-server.service`
- 已提供 `launchd` plist 模板：`deployments/launchd/com.sdkwork.crawchat.server.plist`
- 已提供 Windows Service wrapper 模板：`deployments/windows-service/CrawChatServer.xml`
- `launchd` 目标 label：`com.sdkwork.crawchat.server`
- Windows Service 目标名：`CrawChatServer`
- 统一托管启动命令：`craw-chat-server --config <config-root>/server.yaml`

## 标准路径

- Linux config root: `/etc/craw-chat/default`
- Linux data root: `/var/lib/craw-chat/default`
- Linux log root: `/var/log/craw-chat/default`
- Linux run root: `/var/run/craw-chat/default`

## 一阶段命令

- `install-service-server`
- `uninstall-service-server`
- `start-server`
- `stop-server`
- `restart-server`
- `status-server`

## 说明

- `install-service-server` 当前阶段负责渲染和报告 service contract。
- 生成产物位于 `<config-root>/generated/`，其中包括 `craw-chat-server.service` 与 `com.sdkwork.crawchat.server.plist`。
- Windows Service 采用 dedicated wrapper contract，而不是将前台 console 进程直接注册为原生服务。
- `install-service-server` 还会生成 `CrawChatServer.xml`、`install-CrawChatServer.ps1`、`uninstall-CrawChatServer.ps1`，供 Windows 托管层继续接管。
- service 启动后必须对齐统一端口 `web-gateway`、`/healthz`、`/readyz`、`/openapi.json`、`/docs` 等外部面。
- 手工 `start-server` 与 `systemd` 托管必须共用同一 `server.yaml` 配置源，避免前后台启动语义分叉。
## Release payload contract

- Windows Service host mode is `wrapper-required`
- release payload must ship `bin/CrawChatServer.exe` together with `deployments/windows-service/CrawChatServer.xml`
- `install-service-server` must generate the instance-specific artifacts under `<config-root>/generated/`
  - `CrawChatServer.xml`
  - `install-CrawChatServer.ps1`
  - `uninstall-CrawChatServer.ps1`
- the wrapper must not change the service identity `CrawChatServer`
- the wrapped process contract stays fixed as `craw-chat-server --config <config-root>/server.yaml`

## Unified Gateway Contract

- the externally exposed service entry must remain `web-gateway`
- the standard operator-visible endpoints remain `/healthz`, `/readyz`, `/openapi.json`, `/openapi/index.json`, and `/docs`
- per-service schema proxies remain rooted at `/openapi/services/<service-id>.openapi.json`
- per-service rendered docs remain rooted at `/docs/services/<service-id>`
- startup output must surface the aggregate OpenAPI document, the OpenAPI service index, and the per-service schema/docs endpoints on the unified gateway port
