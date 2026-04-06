# Docker部署Smoke与跨平台一致性标准-2026-04-05

## 1. 目标

本标准用于约束 `craw-chat` 的 Docker 本地部署入口在 Windows PowerShell、Windows CMD、Linux/macOS Bash 三类入口上的行为一致性，避免出现“某个平台只是拉起容器但没有完成真实可用性验证”的假成功。

标准适用于以下入口：

- `bin/deploy-local.ps1`
- `bin/deploy-local.sh`
- `bin/deploy-local.cmd`
- `deployments/scripts/bootstrap-local.ps1`
- `tools/smoke/local_stack_smoke.ps1`
- `tools/smoke/local_stack_smoke.sh`

## 2. 平台一致性要求

### 2.1 预检必须对等

任一平台的 Docker 部署入口都必须在执行 `docker compose up` 前完成以下检查：

- `docker --version`
- `docker info`
- `docker compose version`

禁止只在部分平台校验 compose 插件可用性。否则在插件缺失时，不同平台会出现错误提示质量不一致的问题，影响快速安装部署。

### 2.2 Smoke 必须对等

任一平台的 Docker 部署入口都不得仅以 `/healthz` 作为“部署成功”的唯一判定标准。部署完成后必须执行真实 smoke，用于验证：

- 服务已经可接收 HTTP 请求
- Conversation 创建接口可用
- 消息发送接口可用
- 会话聚合读取接口可用
- 消息写入后的汇总状态正确回读

PowerShell 与 Bash 的 smoke 脚本允许实现细节不同，但验收语义必须一致。

### 2.3 Windows CMD 参数别名必须与公开文档一致

若文档已对外公开某个标准化参数别名，Windows `*.cmd` 包装层必须将其归一化为对应的 PowerShell 参数，不得要求操作者只在 Windows 上改用另一套写法。

例如：

- `deploy-local.cmd --skip-smoke` 必须等价于 `deploy-local.ps1 -SkipSmoke`
- `start-local.cmd --foreground` 必须等价于 `start-local.ps1 -Foreground`
- `install-local.cmd --bind-addr <host:port>` 必须等价于 `install-local.ps1 -BindAddress <host:port>`

这条规则适用于所有通过 `_cmd-forward-powershell.cmd` 转发的生命周期脚本。

## 3. Smoke 最低标准

`tools/smoke/local_stack_smoke.*` 至少必须完成以下步骤：

1. 等待 `http://127.0.0.1:18090/healthz` 或自定义 `base-url` 对应的 `/healthz` 返回成功。
2. 通过带授权头的请求创建一个测试会话。
3. 向测试会话发送一条测试消息。
4. 回读会话详情并校验 `lastSummary` 等关键字段与发送内容一致。
5. 任一步骤失败即以非零退出码或显式异常终止。

说明：

- 默认健康检查地址必须显式固化在脚本中，便于资产测试和部署标准保持一致。
- Smoke 脚本不是演示脚本，必须作为部署成败判定的一部分。

## 4. 失败诊断标准

当以下任一阶段失败时，部署脚本必须自动输出 compose 诊断信息：

- `docker compose up -d --build` 失败
- smoke 验证失败

最小诊断输出必须包括：

- `docker compose -f <compose-file> ps`
- `docker compose -f <compose-file> logs --tail 200`

要求：

- 诊断输出必须自动触发，不能要求操作者手工补命令。
- 诊断输出失败不得覆盖原始部署失败结论。
- 错误文本必须明确区分“compose 拉起失败”和“smoke 校验失败”。

## 5. 依赖要求

### 5.1 Bash 入口

`bin/deploy-local.sh` 与 `tools/smoke/local_stack_smoke.sh` 的依赖要求如下：

- 必须存在 `docker`
- 必须存在 `docker compose` 插件
- smoke 需要 `curl` 或 `wget` 至少一种

若 `curl` 与 `wget` 均不存在，脚本必须明确报错，不得输出成功。

### 5.2 PowerShell 入口

`bin/deploy-local.ps1` 与 `deployments/scripts/bootstrap-local.ps1` 的依赖要求如下：

- 必须存在 `docker`
- 必须可访问 Docker daemon
- 必须存在 `docker compose` 插件
- smoke 使用 PowerShell 自带 HTTP 能力进行校验

## 6. 成功判定标准

仅当以下条件全部满足时，才允许输出“profile is ready”或等价成功信息：

- Docker CLI 可用
- Docker daemon 可用
- Docker compose 插件可用
- compose 服务拉起成功
- smoke 校验成功

若调用方显式指定 `--skip-smoke` 或 `-SkipSmoke`，脚本必须明确提示“已跳过 smoke 验证”，不得伪装成完整验收通过。

## 7. 后续扩展约束

后续若扩展到 SaaS、私有化、分布式多节点 profile，本标准继续适用，且需在此基础上补充：

- 多服务依赖拓扑 readiness 检查
- 消息链路端到端写读验证
- WebSocket/实时链路 smoke
- 多副本滚动升级后的最小业务回归 smoke
