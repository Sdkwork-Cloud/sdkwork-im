# Craw Chat 架构文档索引

本文档集用于冻结 `craw-chat` 的产品定位、总体架构、连接层设计、存储路线、模块边界、协议模型、安全隔离与实施路线。

当前推荐方向已经收敛为：

`消费级消息内核 + 企业协作上下文 + AI / Agent 扩展平台 + IoT 智能硬件接入 + 连接优先的分层弹性扩容体系`

执行口径补充：

- `As-Built` 以 `152CJ` 为准。
- Step 实施入口以 `docs/step/100-*` 与 `docs/step/101-*` 为准。
- 若本 README 的历史阶段状态与 `152CJ`、`100-*`、`101-*` 冲突，以后者为准。

## 一、阅读顺序

如需快速建立完整架构认知，建议按以下顺序阅读：

1. [130-连接优先的AI时代即时通讯架构蓝图-2026-04-06](./130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md)
2. [131-连接管理与分层弹性扩容架构设计-2026-04-06](./131-连接管理与分层弹性扩容架构设计-2026-04-06.md)
3. [132-存储架构与自主演进路线设计-2026-04-06](./132-存储架构与自主演进路线设计-2026-04-06.md)
4. [133-代码结构治理与crate拆分标准-2026-04-06](./133-代码结构治理与crate拆分标准-2026-04-06.md)
5. [134-AI-Agent-IoT统一实时通信模型设计-2026-04-06](./134-AI-Agent-IoT统一实时通信模型设计-2026-04-06.md)
6. [135-行业对标与终局能力矩阵-2026-04-06](./135-行业对标与终局能力矩阵-2026-04-06.md)
7. [136-关键业务链路与跨Plane时序设计-2026-04-06](./136-关键业务链路与跨Plane时序设计-2026-04-06.md)
8. [137-部署拓扑与容量规划设计-2026-04-06](./137-部署拓扑与容量规划设计-2026-04-06.md)
9. [138-高可用与灾备恢复设计-2026-04-06](./138-高可用与灾备恢复设计-2026-04-06.md)
10. [139-权限能力模型与协议演进设计-2026-04-06](./139-权限能力模型与协议演进设计-2026-04-06.md)
11. [140-可观测性与SLO治理设计-2026-04-06](./140-可观测性与SLO治理设计-2026-04-06.md)
12. [141-数据生命周期与归档成本治理设计-2026-04-06](./141-数据生命周期与归档成本治理设计-2026-04-06.md)
13. [142-控制面与配置治理设计-2026-04-06](./142-控制面与配置治理设计-2026-04-06.md)
14. [143-统一协议总纲与分层设计-2026-04-06](./143-统一协议总纲与分层设计-2026-04-06.md)
15. [144-CCP传输绑定与握手协商设计-2026-04-06](./144-CCP传输绑定与握手协商设计-2026-04-06.md)
16. [145-CCP数据协议与版本兼容安全设计-2026-04-06](./145-CCP数据协议与版本兼容安全设计-2026-04-06.md)
17. [146-CCP协议注册表与多端SDK兼容治理设计-2026-04-06](./146-CCP协议注册表与多端SDK兼容治理设计-2026-04-06.md)
18. [147-CCP到Crate与接口模块落地映射设计-2026-04-06](./147-CCP到Crate与接口模块落地映射设计-2026-04-06.md)
19. [148-CCP控制面注册表与协议发布治理设计-2026-04-06](./148-CCP控制面注册表与协议发布治理设计-2026-04-06.md)
20. [149-多Cell多Region协议升级与灾备兼容设计-2026-04-06](./149-多Cell多Region协议升级与灾备兼容设计-2026-04-06.md)
21. [150-插件化提供商体系与设备接入设计-2026-04-08](./150-插件化提供商体系与设备接入设计-2026-04-08.md)
22. [02-架构标准与总体设计](./02-架构标准与总体设计.md)
23. [03-模块规划与边界](./03-模块规划与边界.md)
24. [04-技术选型与可插拔策略](./04-技术选型与可插拔策略.md)
25. [05-数据模型与数据库设计](./05-数据模型与数据库设计.md)
26. [06-Gateway-API-与协议设计](./06-Gateway-API-与协议设计.md)
27. [07-缓存-流式通信-RTC-通知设计](./07-缓存-流式通信-RTC-通知设计.md)
28. [08-安全-多租户-SaaS-私有化-部署设计](./08-安全-多租户-SaaS-私有化-部署设计.md)
29. [09-实施计划](./09-实施计划.md)

## 二、优先级规则

为避免架构口径冲突，本文档集采用以下优先级：

- `130-149` 为当前阶段的主基线文档，优先级最高。
- `01-09` 为基础说明文档，必须与 `130-149` 保持一致。
- `11-24` 与 `26-129` 为专项标准文档，只在各自主题范围内细化规则，不能推翻总纲。
- 若专项标准与总纲出现冲突，以 `130-149` 为准，并回收修正文档。

## 三、当前冻结结论

当前版本的架构冻结结论如下：

- 采用 `Link Plane / Route Plane / Messaging Plane / Stream-AI Plane / Projection Plane / Storage Plane` 六大 plane。
- 采用 `Control Plane / Ops Plane` 作为横切治理层。
- 连接层优先建设，支持单机 `10 万级长连接` 的落地目标，并为更高密度保留空间。
- 消息与流并列为一等能力，流不是 AI 附属功能。
- AI、Agent、Device、Bot、System 都是一等主体。
- 对外 API 不接受客户端伪造的 `tenantId`、`senderId` 等权威字段，统一从认证上下文推导。
- 消息发送者统一采用 `sender` 结构，而不是平铺 `senderId`。
- 协议路线采用“标准传输 + 自有 CCP 应用协议族”，不自研新的 TCP 级传输协议。
- 冻结 `hello / auth_bind / session_resume / envelope / schema version / capability negotiation` 协议骨架。
- 编码与协议解耦，第一阶段以 `JSON` 为主，设备侧可选 `CBOR`。
- 协议治理采用 `registry + compatibility matrix + SDK facade layering` 模式，而不是各端各自兼容。
- `CCP` 必须单独形成协议基础设施 crate 族，不能重新混回 `interface-*` 和 `services/*`。
- 协议注册表、release channel、rollout policy、kill switch 必须进入控制面治理。
- 协议升级必须按 `cell` 为最小发布域、按 `region` 为汇聚与灾备域推进。
- 第一阶段存储基线采用 `PostgreSQL + Redis + S3-compatible Object Storage`。
- RTC、对象存储、IoT 必须统一进入 provider/plugin 体系，不能在 service 内散落厂商私有分支。
- RTC 当前冻结支持 `火山引擎 / 阿里云 / 腾讯云`，全局默认 provider 为 `火山引擎`。
- 对象存储统一通过 `S3` 标准接入，覆盖 `阿里云 / 腾讯云 / 火山引擎 / AWS / Google / Microsoft`。
- 用户中心统一收敛为 `用户模块 plugin`，只允许 `本地实现` 与 `外部系统集成` 两种形态，默认 `本地实现`。
- IoT 当前冻结支持 `MQTT` 与开源 `小智协议`，并要求设备管理与接入体系成为主链能力。
- 第一阶段不自建通用分布式数据库，但必须从第一天冻结自有的 `MessageLog / StreamStore / RouteStore / ProjectionRebuild` 抽象。
- 可观测性必须按 plane 建立统一的指标、追踪、日志和 diagnostics 体系，并围绕 SLO 运转。
- 数据治理必须明确热、温、冷、归档分层与 retention / legal hold 规则。
- 控制面必须独立负责配置、能力开关、配额、drain / rebalance 编排和租户治理，不能侵入热写路径。
- 控制面还必须负责 provider registry、默认 provider、tenant provider policy 和 rollout / kill switch。
- 代码结构必须按 crate 和模块边界治理，禁止继续堆积超大 `lib.rs`。

## 四、文档目录

### 4.1 架构总纲与关键专项

- [130-连接优先的AI时代即时通讯架构蓝图-2026-04-06](./130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md)
- [131-连接管理与分层弹性扩容架构设计-2026-04-06](./131-连接管理与分层弹性扩容架构设计-2026-04-06.md)
- [132-存储架构与自主演进路线设计-2026-04-06](./132-存储架构与自主演进路线设计-2026-04-06.md)
- [133-代码结构治理与crate拆分标准-2026-04-06](./133-代码结构治理与crate拆分标准-2026-04-06.md)
- [134-AI-Agent-IoT统一实时通信模型设计-2026-04-06](./134-AI-Agent-IoT统一实时通信模型设计-2026-04-06.md)
- [135-行业对标与终局能力矩阵-2026-04-06](./135-行业对标与终局能力矩阵-2026-04-06.md)
- [136-关键业务链路与跨Plane时序设计-2026-04-06](./136-关键业务链路与跨Plane时序设计-2026-04-06.md)
- [137-部署拓扑与容量规划设计-2026-04-06](./137-部署拓扑与容量规划设计-2026-04-06.md)
- [138-高可用与灾备恢复设计-2026-04-06](./138-高可用与灾备恢复设计-2026-04-06.md)
- [139-权限能力模型与协议演进设计-2026-04-06](./139-权限能力模型与协议演进设计-2026-04-06.md)
- [140-可观测性与SLO治理设计-2026-04-06](./140-可观测性与SLO治理设计-2026-04-06.md)
- [141-数据生命周期与归档成本治理设计-2026-04-06](./141-数据生命周期与归档成本治理设计-2026-04-06.md)
- [142-控制面与配置治理设计-2026-04-06](./142-控制面与配置治理设计-2026-04-06.md)
- [143-统一协议总纲与分层设计-2026-04-06](./143-统一协议总纲与分层设计-2026-04-06.md)
- [144-CCP传输绑定与握手协商设计-2026-04-06](./144-CCP传输绑定与握手协商设计-2026-04-06.md)
- [145-CCP数据协议与版本兼容安全设计-2026-04-06](./145-CCP数据协议与版本兼容安全设计-2026-04-06.md)
- [146-CCP协议注册表与多端SDK兼容治理设计-2026-04-06](./146-CCP协议注册表与多端SDK兼容治理设计-2026-04-06.md)
- [147-CCP到Crate与接口模块落地映射设计-2026-04-06](./147-CCP到Crate与接口模块落地映射设计-2026-04-06.md)
- [148-CCP控制面注册表与协议发布治理设计-2026-04-06](./148-CCP控制面注册表与协议发布治理设计-2026-04-06.md)
- [149-多Cell多Region协议升级与灾备兼容设计-2026-04-06](./149-多Cell多Region协议升级与灾备兼容设计-2026-04-06.md)
- [150-插件化提供商体系与设备接入设计-2026-04-08](./150-插件化提供商体系与设备接入设计-2026-04-08.md)
- [09AR-pre-release-capacity-tier-gates-implementation-plan-2026-04-09](./09AR-pre-release-capacity-tier-gates-implementation-plan-2026-04-09.md)
- [150AR-pre-release-capacity-tier-gates-design-2026-04-09](./150AR-pre-release-capacity-tier-gates-design-2026-04-09.md)
- [09AS-chat-cli-token-only-contract-implementation-plan-2026-04-09](./09AS-chat-cli-token-only-contract-implementation-plan-2026-04-09.md)
- [150AS-chat-cli-token-only-contract-design-2026-04-09](./150AS-chat-cli-token-only-contract-design-2026-04-09.md)
- [09AT-chat-cli-lowercase-bearer-normalization-contract-implementation-plan-2026-04-09](./09AT-chat-cli-lowercase-bearer-normalization-contract-implementation-plan-2026-04-09.md)
- [150AT-chat-cli-lowercase-bearer-normalization-contract-design-2026-04-09](./150AT-chat-cli-lowercase-bearer-normalization-contract-design-2026-04-09.md)
- [09AU-chat-cli-provided-token-claims-boundary-implementation-plan-2026-04-09](./09AU-chat-cli-provided-token-claims-boundary-implementation-plan-2026-04-09.md)
- [150AU-chat-cli-provided-token-claims-boundary-design-2026-04-09](./150AU-chat-cli-provided-token-claims-boundary-design-2026-04-09.md)
- [09AV-chat-cli-cmd-help-pass-through-contract-implementation-plan-2026-04-09](./09AV-chat-cli-cmd-help-pass-through-contract-implementation-plan-2026-04-09.md)
- [150AV-chat-cli-cmd-help-pass-through-contract-design-2026-04-09](./150AV-chat-cli-cmd-help-pass-through-contract-design-2026-04-09.md)
- [09AW-open-chat-test-cmd-gnu-flag-contract-implementation-plan-2026-04-09](./09AW-open-chat-test-cmd-gnu-flag-contract-implementation-plan-2026-04-09.md)
- [150AW-open-chat-test-cmd-gnu-flag-contract-design-2026-04-09](./150AW-open-chat-test-cmd-gnu-flag-contract-design-2026-04-09.md)
- [09AX-chat-window-cmd-gnu-flag-contract-implementation-plan-2026-04-09](./09AX-chat-window-cmd-gnu-flag-contract-implementation-plan-2026-04-09.md)
- [150AX-chat-window-cmd-gnu-flag-contract-design-2026-04-09](./150AX-chat-window-cmd-gnu-flag-contract-design-2026-04-09.md)
- [09AY-open-chat-test-cmd-validation-message-special-char-contract-implementation-plan-2026-04-09](./09AY-open-chat-test-cmd-validation-message-special-char-contract-implementation-plan-2026-04-09.md)
- [150AY-open-chat-test-cmd-validation-message-special-char-contract-design-2026-04-09](./150AY-open-chat-test-cmd-validation-message-special-char-contract-design-2026-04-09.md)
- [09AZ-chat-window-cmd-message-prefix-special-char-contract-implementation-plan-2026-04-09](./09AZ-chat-window-cmd-message-prefix-special-char-contract-implementation-plan-2026-04-09.md)
- [150AZ-chat-window-cmd-message-prefix-special-char-contract-design-2026-04-09](./150AZ-chat-window-cmd-message-prefix-special-char-contract-design-2026-04-09.md)
- [09BA-chat-window-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09](./09BA-chat-window-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md)
- [150BA-chat-window-cmd-help-gnu-surface-contract-design-2026-04-09](./150BA-chat-window-cmd-help-gnu-surface-contract-design-2026-04-09.md)
- [09BB-open-chat-test-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09](./09BB-open-chat-test-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md)
- [150BB-open-chat-test-cmd-help-gnu-surface-contract-design-2026-04-09](./150BB-open-chat-test-cmd-help-gnu-surface-contract-design-2026-04-09.md)
- [09BC-chat-window-gui-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09](./09BC-chat-window-gui-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md)
- [150BC-chat-window-gui-cmd-help-gnu-surface-contract-design-2026-04-09](./150BC-chat-window-gui-cmd-help-gnu-surface-contract-design-2026-04-09.md)
- [09BD-chat-window-gui-cmd-label-special-char-contract-implementation-plan-2026-04-09](./09BD-chat-window-gui-cmd-label-special-char-contract-implementation-plan-2026-04-09.md)
- [150BD-chat-window-gui-cmd-label-special-char-contract-design-2026-04-09](./150BD-chat-window-gui-cmd-label-special-char-contract-design-2026-04-09.md)
- [09BE-chat-window-gui-cmd-gnu-flag-contract-implementation-plan-2026-04-09](./09BE-chat-window-gui-cmd-gnu-flag-contract-implementation-plan-2026-04-09.md)
- [150BE-chat-window-gui-cmd-gnu-flag-contract-design-2026-04-09](./150BE-chat-window-gui-cmd-gnu-flag-contract-design-2026-04-09.md)

### 4.2 基础架构说明

- [01-产品设计与需求范围](./01-产品设计与需求范围.md)
- [02-架构标准与总体设计](./02-架构标准与总体设计.md)
- [03-模块规划与边界](./03-模块规划与边界.md)
- [04-技术选型与可插拔策略](./04-技术选型与可插拔策略.md)
- [05-数据模型与数据库设计](./05-数据模型与数据库设计.md)
- [06-Gateway-API-与协议设计](./06-Gateway-API-与协议设计.md)
- [07-缓存-流式通信-RTC-通知设计](./07-缓存-流式通信-RTC-通知设计.md)
- [08-安全-多租户-SaaS-私有化-部署设计](./08-安全-多租户-SaaS-私有化-部署设计.md)
- [09-实施计划](./09-实施计划.md)
- [10-实施进度-2026-04-05](./10-实施进度-2026-04-05.md)

### 4.3 协议与实时标准

- [11-流式中止能力设计](./11-流式中止能力设计.md)
- [12-RTC-自定义信令设计](./12-RTC-自定义信令设计.md)
- [13-通用流帧传输标准](./13-通用流帧传输标准.md)
- [14-实时订阅与断线补偿标准](./14-实时订阅与断线补偿标准.md)
- [15-会话成员实时广播标准](./15-会话成员实时广播标准.md)
- [16-消息变更实时广播标准](./16-消息变更实时广播标准.md)
- [17-会话关联流实时广播标准](./17-会话关联流实时广播标准.md)
- [18-流完成与中止实时广播标准](./18-流完成与中止实时广播标准.md)
- [19-实时事件窗口确认与裁剪标准](./19-实时事件窗口确认与裁剪标准.md)
- [20-WebSocket实时传输绑定标准](./20-WebSocket实时传输绑定标准.md)
- [21-跨节点实时路由与设备连接归属标准](./21-跨节点实时路由与设备连接归属标准.md)
- [22-路由归属观测与节点排空标准](./22-路由归属观测与节点排空标准.md)
- [23-节点排空与路由迁移控制标准](./23-节点排空与路由迁移控制标准.md)
- [24-实时确认点持久化与恢复标准](./24-实时确认点持久化与恢复标准.md)

### 4.4 Review 与修复文档

- [25-实现Review与修复计划-2026-04-05](./25-实现Review与修复计划-2026-04-05.md)
- [26-消息变更授权与通知收件人标准-2026-04-05](./26-消息变更授权与通知收件人标准-2026-04-05.md)
- [27-外部认证与Trusted-Identity边界标准-2026-04-05](./27-外部认证与Trusted-Identity边界标准-2026-04-05.md)
- [28-多入口服务外部认证收敛与控制面保护-2026-04-05](./28-多入口服务外部认证收敛与控制面保护-2026-04-05.md)
- [29-剩余独立服务公网认证收口与Public-Builder补齐-2026-04-05](./29-剩余独立服务公网认证收口与Public-Builder补齐-2026-04-05.md)
- [30-审计与运维接口最小权限标准-2026-04-05](./30-审计与运维接口最小权限标准-2026-04-05.md)

## 五、实施原则

- 先冻结文档，再按文档驱动重构和实现。
- 先做边界与正确性，再做性能压榨和极限优化。
- 先做连接、消息、流、路由四条主链，再扩 AI、IoT 与更复杂协作能力。
- 先用成熟组件承载数据，再按价值密度演进自研热路径。
- 每次代码变更都必须对照本目录中的基线文档和专项标准执行。

## 六、2026-04-08 当前实施状态

- `00-13` 已全部闭环，当前仓库已完成 `Wave D / Step 13` 的发布就绪与持续迭代收口。
- `Wave D / 93` 已通过，总体验收状态从“按 step 推进”切换为“持续优化模式”。
- 当前可作为基线交付的能力包括：
  - `local-minimal` / `local-default` profile 入口
  - 部署、启动、状态、恢复、聊天验证等 operator 入口
  - CLI / SDK facade / compatibility matrix / control-plane baseline
  - 性能、HA、DR、rollback 的本地量化基线与演练证据
- 当前明确不纳入本轮发布承诺的事项包括：
  - 多语言 SDK 的正式生成与发布流水线
  - `Pre-Release Tier` 与 `Capacity Tier` 的持续量化门禁；当前只冻结模板态门禁文件：`tools/perf/step-11-pre-release-tier-gate.json`、`tools/perf/step-11-capacity-tier-gate.json`
  - 对应公共 catalog 入口为 `tools/perf/step-11-scenario-catalog.json`，未来高阶 `artifactRoot` 归档根目录冻结为 `artifacts/perf/step-11/pre-release` 与 `artifacts/perf/step-11/capacity`
  - 对应 schema 入口为 `tools/perf/schemas/step-11-scenario-catalog.schema.json` 与 `tools/perf/schemas/step-11-tier-gate.schema.json`
  - 模板态高阶 gate 还显式冻结 `collectionSummary`、`evidenceSlots`、`pending_collection`、`checksumSha256` 等 evidence-slot 契约字段
  - `collectionSummary` 公开 `totalSlots`、`requiredSlots`、`optionalSlots`、`collectedSlots`、`pendingSlots`、`skippedOptionalSlots` 六个统计字段
  - 当前冻结值为 `totalSlots = 7`、`requiredSlots = 7`、`optionalSlots = 0`、`collectedSlots = 0`、`pendingSlots = 7`、`skippedOptionalSlots = 0`
  - evidence slot 元数据还包含 `artifactPath`、`suggestedRelativePath`、`collectedAt`、`sizeBytes` 等回填字段
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
  - 多 `cell` / 多 `region` 的正式 rollout orchestration
- 本轮 review 与验收证据已归档于：
  - `docs/review/step-13-执行卡-2026-04-08.md`
  - `docs/review/step-13-release-readiness-2026-04-08.md`
  - `docs/review/step-13-go-no-go清单-2026-04-08.md`
  - `docs/review/step-13-next-wave-backlog-2026-04-08.md`
  - `docs/review/step-13-架构兑现-2026-04-08.md`
  - `docs/review/step-13-架构回写决议-2026-04-08.md`
  - `docs/review/wave-d-93-总验收-2026-04-08.md`
# 2026-04-09 Step 12 Addendum

- [09BF-start-local-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09](./09BF-start-local-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md)
- [150BF-start-local-cmd-help-gnu-surface-contract-design-2026-04-09](./150BF-start-local-cmd-help-gnu-surface-contract-design-2026-04-09.md)
- [09BG-status-local-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09](./09BG-status-local-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md)
- [150BG-status-local-cmd-help-gnu-surface-contract-design-2026-04-09](./150BG-status-local-cmd-help-gnu-surface-contract-design-2026-04-09.md)
- [09BH-user-module-runtime-provider-selection-implementation-plan-2026-04-09](./09BH-user-module-runtime-provider-selection-implementation-plan-2026-04-09.md)
- [150BH-user-module-runtime-provider-selection-design-2026-04-09](./150BH-user-module-runtime-provider-selection-design-2026-04-09.md)
- [09BI-install-deploy-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09](./09BI-install-deploy-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md)
- [150BI-install-deploy-cmd-help-gnu-surface-contract-design-2026-04-09](./150BI-install-deploy-cmd-help-gnu-surface-contract-design-2026-04-09.md)
- [09BJ-user-module-external-missing-catalog-unavailable-contract-implementation-plan-2026-04-09](./09BJ-user-module-external-missing-catalog-unavailable-contract-implementation-plan-2026-04-09.md)
- [150BJ-user-module-external-missing-catalog-unavailable-contract-design-2026-04-09](./150BJ-user-module-external-missing-catalog-unavailable-contract-design-2026-04-09.md)
## 2026-04-09 Addendum

- [09BK-user-module-provider-health-http-surface-implementation-plan-2026-04-09](./09BK-user-module-provider-health-http-surface-implementation-plan-2026-04-09.md)
- [150BK-user-module-provider-health-http-surface-design-2026-04-09](./150BK-user-module-provider-health-http-surface-design-2026-04-09.md)
## 2026-04-09 Addendum

- [09BL-local-minimal-ops-provider-bindings-runtime-visibility-implementation-plan-2026-04-09](./09BL-local-minimal-ops-provider-bindings-runtime-visibility-implementation-plan-2026-04-09.md)
- [150BL-local-minimal-ops-provider-bindings-runtime-visibility-design-2026-04-09](./150BL-local-minimal-ops-provider-bindings-runtime-visibility-design-2026-04-09.md)
- [09BM-local-minimal-ops-provider-bindings-http-surface-implementation-plan-2026-04-09](./09BM-local-minimal-ops-provider-bindings-http-surface-implementation-plan-2026-04-09.md)
- [150BM-local-minimal-ops-provider-bindings-http-surface-design-2026-04-09](./150BM-local-minimal-ops-provider-bindings-http-surface-design-2026-04-09.md)
## 2026-04-09 Addendum

- [09BN-step11-tier-gate-doc-state-alignment-implementation-plan-2026-04-09](./09BN-step11-tier-gate-doc-state-alignment-implementation-plan-2026-04-09.md)
- [150BN-step11-tier-gate-doc-state-alignment-design-2026-04-09](./150BN-step11-tier-gate-doc-state-alignment-design-2026-04-09.md)
## 2026-04-09 Addendum

- [09BO-step11-tier-artifact-root-materialization-implementation-plan-2026-04-09](./09BO-step11-tier-artifact-root-materialization-implementation-plan-2026-04-09.md)
- [150BO-step11-tier-artifact-root-materialization-design-2026-04-09](./150BO-step11-tier-artifact-root-materialization-design-2026-04-09.md)
## 2026-04-09 Addendum

- [09BP-step11-tier-machine-readable-evidence-index-implementation-plan-2026-04-09](./09BP-step11-tier-machine-readable-evidence-index-implementation-plan-2026-04-09.md)
- [150BP-step11-tier-machine-readable-evidence-index-design-2026-04-09](./150BP-step11-tier-machine-readable-evidence-index-design-2026-04-09.md)
## 2026-04-09 Addendum

- [09BQ-step11-pre-release-failover-collected-evidence-implementation-plan-2026-04-09](./09BQ-step11-pre-release-failover-collected-evidence-implementation-plan-2026-04-09.md)
- [150BQ-step11-pre-release-failover-collected-evidence-design-2026-04-09](./150BQ-step11-pre-release-failover-collected-evidence-design-2026-04-09.md)
- [09BR-step11-pre-release-restore-recovery-collected-evidence-implementation-plan-2026-04-09](./09BR-step11-pre-release-restore-recovery-collected-evidence-implementation-plan-2026-04-09.md)
- [150BR-step11-pre-release-restore-recovery-collected-evidence-design-2026-04-09](./150BR-step11-pre-release-restore-recovery-collected-evidence-design-2026-04-09.md)
- [09BS-step11-pre-release-drain-rebalance-collected-evidence-implementation-plan-2026-04-09](./09BS-step11-pre-release-drain-rebalance-collected-evidence-implementation-plan-2026-04-09.md)
- [150BS-step11-pre-release-drain-rebalance-collected-evidence-design-2026-04-09](./150BS-step11-pre-release-drain-rebalance-collected-evidence-design-2026-04-09.md)
- [09BT-step11-pre-release-upgrade-rollback-collected-evidence-implementation-plan-2026-04-09](./09BT-step11-pre-release-upgrade-rollback-collected-evidence-implementation-plan-2026-04-09.md)
- [150BT-step11-pre-release-upgrade-rollback-collected-evidence-design-2026-04-09](./150BT-step11-pre-release-upgrade-rollback-collected-evidence-design-2026-04-09.md)
- [09BU-step11-pre-release-connection-metrics-collected-evidence-implementation-plan-2026-04-09](./09BU-step11-pre-release-connection-metrics-collected-evidence-implementation-plan-2026-04-09.md)
- [150BU-step11-pre-release-connection-metrics-collected-evidence-design-2026-04-09](./150BU-step11-pre-release-connection-metrics-collected-evidence-design-2026-04-09.md)
- [09BW-step11-pre-release-message-metrics-collected-evidence-implementation-plan-2026-04-09](./09BW-step11-pre-release-message-metrics-collected-evidence-implementation-plan-2026-04-09.md)
- [150BW-step11-pre-release-message-metrics-collected-evidence-design-2026-04-09](./150BW-step11-pre-release-message-metrics-collected-evidence-design-2026-04-09.md)
- [09BX-step11-pre-release-stream-metrics-collected-evidence-implementation-plan-2026-04-09](./09BX-step11-pre-release-stream-metrics-collected-evidence-implementation-plan-2026-04-09.md)
- [150BX-step11-pre-release-stream-metrics-collected-evidence-design-2026-04-09](./150BX-step11-pre-release-stream-metrics-collected-evidence-design-2026-04-09.md)
- [09BV-step11-closure-claim-supersession-implementation-plan-2026-04-09](./09BV-step11-closure-claim-supersession-implementation-plan-2026-04-09.md)
- [150BV-step11-closure-claim-supersession-design-2026-04-09](./150BV-step11-closure-claim-supersession-design-2026-04-09.md)
## 2026-04-09 Addendum

- [09CA-shell-process-identity-portability-implementation-plan-2026-04-09](./09CA-shell-process-identity-portability-implementation-plan-2026-04-09.md)
- [150CA-shell-process-identity-portability-design-2026-04-09](./150CA-shell-process-identity-portability-design-2026-04-09.md)
- [09CB-runtime-lifecycle-profile-selection-implementation-plan-2026-04-09](./09CB-runtime-lifecycle-profile-selection-implementation-plan-2026-04-09.md)
- [150CB-runtime-lifecycle-profile-selection-design-2026-04-09](./150CB-runtime-lifecycle-profile-selection-design-2026-04-09.md)
- [09CC-lifecycle-profile-doc-contract-alignment-implementation-plan-2026-04-09](./09CC-lifecycle-profile-doc-contract-alignment-implementation-plan-2026-04-09.md)
- [150CC-lifecycle-profile-doc-contract-alignment-design-2026-04-09](./150CC-lifecycle-profile-doc-contract-alignment-design-2026-04-09.md)
- [09CD-start-local-ps1-health-timeout-test-stability-implementation-plan-2026-04-09](./09CD-start-local-ps1-health-timeout-test-stability-implementation-plan-2026-04-09.md)
- [150CD-start-local-ps1-health-timeout-test-stability-design-2026-04-09](./150CD-start-local-ps1-health-timeout-test-stability-design-2026-04-09.md)
## 2026-04-09 Addendum

- [09CE-restore-runtime-cmd-expected-preview-fingerprint-implementation-plan-2026-04-09](./09CE-restore-runtime-cmd-expected-preview-fingerprint-implementation-plan-2026-04-09.md)
- [150CE-restore-runtime-cmd-expected-preview-fingerprint-design-2026-04-09](./150CE-restore-runtime-cmd-expected-preview-fingerprint-design-2026-04-09.md)
## 2026-04-09 Addendum

- [09CF-inspect-runtime-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09](./09CF-inspect-runtime-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md)
- [150CF-inspect-runtime-cmd-help-gnu-surface-contract-design-2026-04-09](./150CF-inspect-runtime-cmd-help-gnu-surface-contract-design-2026-04-09.md)
## 2026-04-09 Addendum

- [09CG-start-local-ps1-health-timeout-window-recalibration-implementation-plan-2026-04-09](./09CG-start-local-ps1-health-timeout-window-recalibration-implementation-plan-2026-04-09.md)
- [150CG-start-local-ps1-health-timeout-window-recalibration-design-2026-04-09](./150CG-start-local-ps1-health-timeout-window-recalibration-design-2026-04-09.md)
## 2026-04-09 Addendum

- [09CH-repair-runtime-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09](./09CH-repair-runtime-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md)
- [150CH-repair-runtime-cmd-help-gnu-surface-contract-design-2026-04-09](./150CH-repair-runtime-cmd-help-gnu-surface-contract-design-2026-04-09.md)
## 2026-04-09 Addendum

- [09CI-open-chat-test-detached-gui-start-process-fallback-implementation-plan-2026-04-09](./09CI-open-chat-test-detached-gui-start-process-fallback-implementation-plan-2026-04-09.md)
- [150CI-open-chat-test-detached-gui-start-process-fallback-design-2026-04-09](./150CI-open-chat-test-detached-gui-start-process-fallback-design-2026-04-09.md)
## 2026-04-09 Addendum

- 读“当前应用信息”时优先看 `152CJ-current-architecture-as-built-alignment-2026-04-09`。
- 读“关系域 / 空间域 / 会话域 / DDD 命名标准”时优先看 `150CJ-im-social-space-conversation-ddd-design-2026-04-09`。
- 读“行业对标与终局能力映射”时优先看 `151CJ-im-benchmark-model-alignment-2026-04-09`。
- 回到主干文档时，以 `02 / 03 / 05 / 09 / 130 / 135 / 143 / 146 / 148` 的“当前实现 / 目标态 / 口径规则”增补为准，禁止混读。

- [09CJ-im-social-space-conversation-ddd-implementation-plan-2026-04-09](./09CJ-im-social-space-conversation-ddd-implementation-plan-2026-04-09.md)
- [150CJ-im-social-space-conversation-ddd-design-2026-04-09](./150CJ-im-social-space-conversation-ddd-design-2026-04-09.md)
- [151CJ-im-benchmark-model-alignment-2026-04-09](./151CJ-im-benchmark-model-alignment-2026-04-09.md)
- [152CJ-current-architecture-as-built-alignment-2026-04-09](./152CJ-current-architecture-as-built-alignment-2026-04-09.md)
