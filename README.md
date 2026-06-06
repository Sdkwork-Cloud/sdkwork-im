# Craw Chat

Current docs site source: [docs/sites](./docs/sites). SDK workspace index:
[sdks/README.md](./sdks/README.md).

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
- `sdkwork-rtc-signaling-service` (owned by `D:\sdkwork-opensource\sdkwork-rtc`)
- `media-service`
- `notification-service`
- `automation-service`
- `audit-service`
- `ops-service`
- `projection-service`
- `control-plane-api`

其中 `local-minimal-node` 是当前默认的本地最小可运行集成形态，用于单机验证完整主链路。

当前部署 profile 现状：

- `local-minimal`
  - 当前唯一完整闭环 profile
  - 覆盖本地脚本、Docker Compose、smoke 与 runtime 运维入口
- `local-default`
  - 当前已冻结名称与 compose/template 入口
  - 现阶段仍复用 `local-minimal` 服务合同，作为后续默认本地开发拓扑的扩展位

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
./bin/install-local.ps1 -ProfileName local-default
./bin/init-config-local.ps1 -ProfileName local-default
```

Bash:

```bash
./bin/install-local.sh
./bin/init-config-local.sh
./bin/install-local.sh --profile local-default
./bin/init-config-local.sh --profile local-default
```

Windows CMD:

```cmd
bin\install-local.cmd
bin\init-config-local.cmd
bin\install-local.cmd --profile local-default
bin\init-config-local.cmd --profile local-default
```

### 3. 启动服务

PowerShell:

```powershell
./bin/start-local.ps1
./bin/start-local.ps1 -ProfileName local-default
```

Bash:

```bash
./bin/start-local.sh
./bin/start-local.sh --profile local-default
```

Windows CMD:

```cmd
bin\start-local.cmd
bin\start-local.cmd --profile local-default
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
./bin/status-local.ps1 -ProfileName local-default
./bin/restart-local.ps1
./bin/restart-local.ps1 -ProfileName local-default
./bin/stop-local.ps1
./bin/stop-local.ps1 -ProfileName local-default
```

Bash:

```bash
./bin/status-local.sh
./bin/status-local.sh --profile local-default
./bin/restart-local.sh
./bin/restart-local.sh --profile local-default
./bin/stop-local.sh
./bin/stop-local.sh --profile local-default
```

Windows CMD:

```cmd
bin\status-local.cmd --profile local-default
bin\restart-local.cmd --profile local-default
bin\stop-local.cmd --profile local-default
```

`local-default` 当前会优先写入 `.runtime/local-default/config/local-default.env`，但仍复用 `.runtime/local-minimal` 运行目录合同。

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

统一 `bin/` 入口：

```powershell
./bin/deploy-local.ps1 -ProfileName local-minimal
./bin/deploy-local.ps1 -ProfileName local-default -SmokeBaseUrl http://127.0.0.1:28090
```

```bash
bash bin/deploy-local.sh --profile local-default --smoke-base-url http://127.0.0.1:28090
```

当前阶段 `local-default` 已是受支持的部署 profile 名称，但仍复用 `local-minimal` 的 compose 服务合同与 smoke 链路。
Docker smoke 现在使用 `x-sdkwork-*` AppContext 投影头；`local-minimal` compose 只保留 `CRAW_CHAT_FRIEND_REQUEST_CURSOR_HS256_SECRET` 作为好友请求 cursor 的业务签名 secret。

Docker Compose:

```bash
docker compose -f deployments/docker-compose/local-minimal.yml up -d --build
```

## 文档入口

- 架构总览：[docs/架构/README.md](./docs/架构/README.md)
- 部署说明：[docs/部署/README.md](./docs/部署/README.md)
- CLI 聊天验证与兼容矩阵：[docs/部署/CLI聊天验证与兼容矩阵.md](./docs/部署/CLI聊天验证与兼容矩阵.md)
- 兼容矩阵与 SDK/CLI/operator 验证索引：[docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md](./docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md)
- SDK 总览：[sdks/README.md](./sdks/README.md)
- Release bundle 归档约定：[artifacts/releases/README.md](./artifacts/releases/README.md)
- 多环境 profile 与模板：[docs/部署/多环境Profile与配置模板.md](./docs/部署/多环境Profile与配置模板.md)
- 本地最小安装与运行：[docs/部署/本地最小安装与运行.md](./docs/部署/本地最小安装与运行.md)
- 性能与灾备演练场景与高阶门禁模板：[docs/部署/性能与灾备演练场景.md](./docs/部署/性能与灾备演练场景.md)
  - 包含 `tools/perf/step-11-pre-release-tier-gate.json` 与 `tools/perf/step-11-capacity-tier-gate.json`
  - Step 11 catalog 入口为 `tools/perf/step-11-scenario-catalog.json`，未来高阶 `artifactRoot` 归档根目录为 `artifacts/perf/step-11/pre-release` 与 `artifacts/perf/step-11/capacity`，当前仍仅用于模板化归档定位
  - 对应 schema 为 `tools/perf/schemas/step-11-scenario-catalog.schema.json` 与 `tools/perf/schemas/step-11-tier-gate.schema.json`
  - 高阶 gate 还冻结 `collectionSummary`、`evidenceSlots`、`pending_collection`、`checksumSha256` 等 evidence-slot 契约字段，当前仍待真实采集回填
  - `collectionSummary` 公开 `totalSlots`、`requiredSlots`、`optionalSlots`、`collectedSlots`、`pendingSlots`、`skippedOptionalSlots` 六个统计字段
  - 当前冻结值为 `totalSlots = 7`、`requiredSlots = 7`、`optionalSlots = 0`、`collectedSlots = 0`、`pendingSlots = 7`、`skippedOptionalSlots = 0`
  - evidence slot 元数据还包含 `artifactPath`、`suggestedRelativePath`、`collectedAt`、`sizeBytes` 等回填字段，用于后续真实证据采集
  - evidence slot 语义字段还包含 `scenarioFamily`、`required`、`reportId`，用于区分场景槽位与报告槽位
  - 最小示例值统一冻结为 `scenarioFamily = connection` / `scenarioFamily = failover`、`required = true`、`reportId = capacity_report` / `reportId = recovery_report`
  - `reportId` 与 `artifactKind` 的对应关系包括 `capacity_report -> report_markdown`、`recovery_report -> report_markdown`
  - `reportId` 与建议路径的对应关系包括 `capacity_report -> reports/capacity-report.md`、`recovery_report -> reports/recovery-report.md`
  - `reportId` 与 `requiredSections` 的对应关系包括 `capacity_report -> input_scale / throughput_summary / tail_latency_summary`、`recovery_report -> recovery_window / rto_rpo_summary / operator_follow_up`
  - `suggestedRelativePath` 示例包括 `connection/metrics.json`、`failover/drill.json`、`reports/capacity-report.md`、`reports/recovery-report.md`
  - Capacity Tier 额外示例路径包括 `connection/capacity.json`、`restore-recovery/recovery.json`、`failover/recovery.json`
  - Capacity Tier 剩余 capacity 路径示例包括 `message/capacity.json`、`stream/capacity.json`
  - evidence slot 还冻结主键 `id`，代表值包括 `connection_metrics`、`connection_capacity`、`failover_recovery`
  - Capacity Tier 其余代表性 slot id 还包括 `message_capacity`、`stream_capacity`、`restore_recovery_recovery`
  - Pre-Release Tier collected slot examples now include `message_metrics` and `stream_metrics`
  - Pre-Release Tier collected path examples now include `message/metrics.json` and `stream/metrics.json`
  - Pre-Release Tier current state is now `evidence_collected_gate_blocked`
  - Capacity Tier current state remains `template_only_pending_execution`
  - Only Capacity Tier still waits for real collection; Pre-Release Tier already carries all seven truthful local artifacts.
  - evidence slot 还公开 `artifactKind`，代表值包括 `metrics_json`、`drill_json`、`capacity_json`、`recovery_json`、`report_markdown`
  - 机器契约还冻结 `requiredFields` / `requiredSections`，示例值包括 `runId`、`connectP95Ms`、`input_scale`、`operator_follow_up`
  - 额外字段示例包括 `messageTps`、`frameP95Ms`、`recovery_window`、`rto_rpo_summary`
  - report section 代表值还包括 `throughput_summary`、`tail_latency_summary`、`recovery_window`、`operator_follow_up`
  - 更细一级字段示例还包括 `fanoutP95Ms`、`streamFramesPerSecond`、`previewDiffAccuracy`、`rollbackActivationSeconds`
  - drill / rollback 字段示例还包括 `drainCompletionSeconds`、`restoreRtoSeconds`、`compatibilityMatrixPassRate`、`postRollbackProtocolErrorRate`
  - `artifactKind` 与代表字段/section 的对应关系包括 `metrics_json -> connectP95Ms / messageTps / frameP95Ms`、`drill_json -> drainCompletionSeconds / rollbackActivationSeconds`、`capacity_json -> fanoutP95Ms / streamFramesPerSecond`、`recovery_json -> restoreRtoSeconds / previewDiffAccuracy`、`report_markdown -> throughput_summary / rto_rpo_summary`
  - `artifactKind` 与建议路径的对应关系包括 `metrics_json -> connection/metrics.json / message/metrics.json`、`drill_json -> failover/drill.json / restore-recovery/drill.json`、`capacity_json -> connection/capacity.json / message/capacity.json`、`recovery_json -> failover/recovery.json / restore-recovery/recovery.json`、`report_markdown -> reports/capacity-report.md / reports/recovery-report.md`
  - `artifactKind` 与代表性 `slot id` 的对应关系包括 `metrics_json -> connection_metrics / message_metrics`、`drill_json -> failover_drill / restore_recovery_drill`、`capacity_json -> connection_capacity / message_capacity`、`recovery_json -> failover_recovery / restore_recovery_recovery`、`report_markdown -> capacity_report / recovery_report`
  - `artifactKind` 与 `requiredFields / requiredSections` 的对应关系包括 `metrics_json -> runId / connectionCount / successCount`、`drill_json -> runId / drainCompletionSeconds / takeoverDurationMs`、`capacity_json -> runId / peakActiveConnections / messageTps`、`recovery_json -> runId / restoreRtoSeconds / staleSessionRejectionRate`、`report_markdown -> input_scale / throughput_summary / operator_follow_up`
  - `requiredScenarioFamilies = connection / message / stream / drain-rebalance / restore-recovery / failover / upgrade-rollback`
  - `requiredScenarioFamilies = connection / message / stream / restore-recovery / failover`
  - `requiredReports = capacity_report / recovery_report`
  - `requiredOutputs` 以 `scenarioFamily -> artifactKind -> requiredFields` tuple 冻结最小输出契约，代表项包括 `connection -> metrics_json -> runId / connectionCount / successCount`、`restore-recovery -> recovery_json -> runId / restoreRtoSeconds / dataLossRpoEvents / previewDiffAccuracy`
  - `operatorDocPath = docs/部署/性能与灾备演练场景.md`，`scenarioCatalogPath = tools/perf/step-11-scenario-catalog.json`
  - `profile = local-default / capacity-dedicated`
  - `reviewBackwrite = docs/step/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md / docs/review/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md / docs/架构/09AR-pre-release-capacity-tier-gates-implementation-plan-2026-04-09.md / docs/架构/150AR-pre-release-capacity-tier-gates-design-2026-04-09.md`
  - `scenarioFamily` 与 `artifactKind` 的对应关系包括 `connection -> metrics_json / capacity_json`、`failover -> drill_json / recovery_json`、`restore-recovery -> drill_json / recovery_json`
  - `scenarioFamily` 与 `requiredFields / requiredSections` 的对应关系包括 `connection -> runId / connectP95Ms`、`failover -> runId / takeoverDurationMs`、`restore-recovery -> runId / restoreRtoSeconds / previewDiffAccuracy`
  - `scenarioFamily` 与 slot id 的对应关系包括 `connection -> connection_metrics / connection_capacity`、`failover -> failover_drill / failover_recovery`、`restore-recovery -> restore_recovery_drill / restore_recovery_recovery`
  - 代表性 `slot id` 与 `artifactKind` 的对应关系包括 `connection_metrics -> metrics_json`、`failover_drill -> drill_json`、`restore_recovery_recovery -> recovery_json`
  - 代表性 `slot id` 与 `requiredFields / requiredSections` 的对应关系包括 `connection_metrics -> runId / connectP95Ms`、`failover_drill -> runId / takeoverDurationMs`、`capacity_report -> input_scale / throughput_summary / tail_latency_summary`
  - `scenarioFamily` 与建议路径的对应关系包括 `connection -> connection/metrics.json / connection/capacity.json`、`failover -> failover/drill.json / failover/recovery.json`、`restore-recovery -> restore-recovery/drill.json / restore-recovery/recovery.json`
  - 代表性 `slot id` 与建议路径的对应关系包括 `connection_metrics -> connection/metrics.json`、`failover_drill -> failover/drill.json`、`restore_recovery_recovery -> restore-recovery/recovery.json`
  - 默认命名关系为 `artifactPath = artifactRoot + "/" + suggestedRelativePath`
  - 在真实采集前，`artifactPath`、`collectedAt`、`sizeBytes`、`checksumSha256` 继续保持 `null`
  - 当前状态为 `template_only_pending_execution`，默认预发布 profile 为 `local-default`，目标容量环境为 `capacity-dedicated`
- local-default发布后验证样本：[docs/部署/local-default发布后验证样本.md](./docs/部署/local-default发布后验证样本.md)
- local-default发布后验证执行记录模板：[docs/部署/local-default发布后验证执行记录模板.md](./docs/部署/local-default发布后验证执行记录模板.md)
- Step 执行索引：[docs/step/README.md](./docs/step/README.md)
- Review 输出索引：[docs/review/README.md](./docs/review/README.md)

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

当前仓库采用 `AGPL-3.0-or-later` + 商业授权策略：

- 非商业使用、评估、研究、教育和源码协作可在 `AGPL-3.0-or-later` 下进行。
- Commercial use requires a separate commercial license；包括生产部署、paid SaaS、托管服务、商业分发、OEM/白标、专有产品集成、转售或其他商业化使用。
- 商业用途必须先购买或取得单独的 commercial authorization，具体以 [COMMERCIAL-LICENSE.md](./COMMERCIAL-LICENSE.md) 为准。
- AGPL-3.0-or-later 正文入口见 [LICENSE](./LICENSE)。

## SDKWork Documentation Contract

Domain: communication
Capability: chat
Package type: app
Status: ACTIVE

### Public API

Public exports are declared in `specs/component.spec.json` under `contracts.publicExports`.

### Required SDK Surface

- None declared in `specs/component.spec.json`.

### Configuration

Configuration keys and runtime entrypoints are declared in `specs/component.spec.json`.

### SaaS/Private/Local Behavior

This module follows the canonical standards linked from `specs/component.spec.json`, including deployment and runtime configuration rules where applicable.

### Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this module.

### Extension Points

Extension points are limited to declared public exports, runtime entrypoints, SDK clients, events, and config keys.

### Verification

- `cargo test --manifest-path apps/craw-chat/Cargo.toml`
- `node scripts/dev/sdkwork-chat-database-naming-standard.test.mjs`

### Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`.
