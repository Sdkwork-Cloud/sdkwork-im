# Craw Chat

`craw-chat` 是一个面向即时通信场景的 Rust 多 crate 工作区，当前聚焦于可快速安装、可本地运行、可逐步演进为分布式部署的 IM 服务端实现。

当前仓库已经包含以下核心能力：

- 会话创建与成员治理
- 消息发送、编辑、撤回与时间线查询
- 收件箱与已读游标
- 实时事件投递、ACK、补偿与 WebSocket 接入
- 通用流式数据传输
- RTC 会话与自定义信令
- 媒体上传与资源绑定
- 通知、自动化、审计、运维诊断
- 本地最小运行时持久化、修复、备份与恢复
- 跨平台安装、启动、停止、重启、聊天测试脚本

## 仓库结构

```text
craw-chat/
├─ adapters/       # 本地内存、本地磁盘等适配器
├─ crates/         # 核心领域、事件、契约、鉴权、时间工具
├─ services/       # 会话、实时、流、RTC、媒体、通知等服务
├─ tools/          # chat-cli 等测试与辅助工具
├─ bin/            # Windows / PowerShell / Bash 启动与运维脚本
├─ deployments/    # Docker 与本地引导脚本
├─ docs/           # 架构、部署、review、实施文档
└─ scripts/        # 本地运行与 smoke 脚本
```

## 主要服务

- `local-minimal-node`
- `conversation-runtime`
- `session-gateway`
- `streaming-service`
- `rtc-signaling-service`
- `media-service`
- `notification-service`
- `automation-service`
- `audit-service`
- `ops-service`
- `projection-service`
- `control-plane-api`

其中 `local-minimal-node` 是当前默认的本地最小可运行集成形态，用于单机验证完整主链路。

## 快速开始

### 1. 安装依赖

需要本机具备：

- Rust 工具链
- `cargo`
- PowerShell 7+ 或 Bash
- 可选：Docker / Docker Compose

### 2. 本地初始化

PowerShell:

```powershell
./bin/install-local.ps1
./bin/init-config-local.ps1
```

Bash:

```bash
./bin/install-local.sh
./bin/init-config-local.sh
```

Windows CMD:

```cmd
bin\install-local.cmd
bin\init-config-local.cmd
```

### 3. 启动服务

PowerShell:

```powershell
./bin/start-local.ps1
```

Bash:

```bash
./bin/start-local.sh
```

Windows CMD:

```cmd
bin\start-local.cmd
```

默认监听地址：

- `http://127.0.0.1:18090`

健康检查：

```bash
curl http://127.0.0.1:18090/healthz
```

### 4. 状态、停止、重启

PowerShell:

```powershell
./bin/status-local.ps1
./bin/restart-local.ps1
./bin/stop-local.ps1
```

Bash:

```bash
./bin/status-local.sh
./bin/restart-local.sh
./bin/stop-local.sh
```

## 聊天验证

仓库内置了命令行聊天测试工具和多窗口启动脚本。

启动聊天测试窗口：

PowerShell:

```powershell
./bin/open-chat-test.ps1
```

Bash:

```bash
./bin/open-chat-test.sh
```

Windows CMD:

```cmd
bin\open-chat-test.cmd
```

直接调用 CLI：

PowerShell:

```powershell
./bin/chat-cli.ps1 --help
./bin/chat-window.ps1 --help
```

Bash:

```bash
./bin/chat-cli.sh --help
./bin/chat-window.sh --help
```

## Docker 与引导

PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -File deployments\scripts\bootstrap-local.ps1
```

Docker Compose:

```bash
docker compose -f deployments/docker-compose/local-minimal.yml up -d --build
```

## 文档入口

- 架构总览：[docs/架构/README.md](./docs/架构/README.md)
- 部署说明：[docs/部署/README.md](./docs/部署/README.md)
- 本地最小安装与运行：[docs/部署/本地最小安装与运行.md](./docs/部署/本地最小安装与运行.md)
- Review 输出目录：[docs/review](./docs/review)

## 构建与测试

格式检查与构建：

```bash
cargo build
cargo test
```

针对 workspace 根执行：

```bash
cargo test --workspace
```

## 当前约束

- 业务请求体中不再显式传递 `tenantId`
- 租户与调用主体从认证上下文解析
- 消息发送者统一使用 `sender` 结构建模
- 当前 `local-minimal` profile 用于开发与单机验证
- 后续将继续向更细粒度、可替换、可插拔的 crate 架构演进

## 许可证

当前仓库代码默认沿用工作区中的 `MIT` 声明，具体以各 crate 与后续仓库配置为准。
