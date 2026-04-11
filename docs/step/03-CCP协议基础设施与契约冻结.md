# Step 03 - CCP 协议基础设施与契约冻结

## 1. 目标与范围

本 step 用于把 `CCP = Craw Chat Protocol` 从架构文档真正落到工程骨架上，并冻结后续高频使用的协议基础设施与业务契约边界。

本 step 的重点是：

- 正式建立 `ccp-*` crate 族
- 正式细化 `contract-*` 业务契约层
- 把权威字段、安全边界、版本协商和传输绑定统一起来
- 为 WebSocket、HTTP、SSE、MQTT 共用同一套协议骨架

### 1.1 执行输入

- step 02 形成的 crate 骨架与依赖边界
- `143-149` 的协议总纲与专项文档
- 当前 `im-platform-contracts`、`im-auth-context`、`session-gateway` 的协议现状
- 当前关于 `tenantId`、`sender` 权威字段的既有结论

### 1.2 本步非目标

- 不在本 step 内完成所有业务领域 schema 的最终字段全集
- 不在本 step 内完成 QUIC 等后续预留 binding 的真实实现
- 不在本 step 内推动全量 SDK 与客户端同步升级

### 1.3 最小输出

- `ccp-*` crate 族骨架
- `contract-*` 与 `ccp-*` 的职责边界
- 权威字段与握手控制帧收口规则
- 可自动化验证的协议基础测试

## 2. 架构对齐

本 step 重点对齐：

- `docs/架构/143-统一协议总纲与分层设计-2026-04-06.md`
- `docs/架构/144-CCP传输绑定与握手协商设计-2026-04-06.md`
- `docs/架构/145-CCP数据协议与版本兼容安全设计-2026-04-06.md`
- `docs/架构/146-CCP协议注册表与多端SDK兼容治理设计-2026-04-06.md`
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`
- `docs/架构/148-CCP控制面注册表与协议发布治理设计-2026-04-06.md`

## 3. 当前现状与问题

当前仓库已经有 `im-platform-contracts`，也有 WebSocket、HTTP 和本地验证能力，但还存在以下缺口：

- 尚未形成 `ccp-core / ccp-control / ccp-registry / ccp-codec / ccp-binding-*` 实体 crate
- 各入口仍可能夹带自己的 envelope、握手或错误语义
- `tenantId`、`sender` 等权威字段需要更明确地从认证上下文收口
- 业务契约与协议骨架还未完成彻底分层

## 4. 设计

### 4.1 CCP crate 族

必须建立以下协议基础设施层：

- `craw-chat-ccp-core`
- `craw-chat-ccp-control`
- `craw-chat-ccp-registry`
- `craw-chat-ccp-codec`
- `craw-chat-ccp-codec-json`
- `craw-chat-ccp-codec-cbor`
- `craw-chat-ccp-binding-http`
- `craw-chat-ccp-binding-ws`
- `craw-chat-ccp-binding-sse`
- `craw-chat-ccp-binding-mqtt`

### 4.2 业务契约层

建议将当前 `im-platform-contracts` 演进为或拆分为：

- `craw-chat-contract-core`
- `craw-chat-contract-control`
- `craw-chat-contract-message`
- `craw-chat-contract-stream`
- `craw-chat-contract-rtc`
- `craw-chat-contract-agent`
- `craw-chat-contract-iot`
- `craw-chat-contract-notification`
- `craw-chat-contract-admin`

### 4.3 冻结内容

本 step 必须冻结以下协议资产：

- 统一 envelope
- `kind / type / schema / scope / route / flags / trace`
- `hello / hello_ack / auth_bind / auth_ok / session_resume / session_resumed / heartbeat / goaway / error`
- capability negotiation
- compatibility matrix 基本结构
- JSON / CBOR codec trait 与注册入口
- `ccp/http/1`、`ccp/ws/1`、`ccp/sse/1`、`ccp/mqtt/1`

### 4.4 安全边界

必须明确：

- 客户端不得提交权威 `tenantId`
- 客户端不得伪造 `sender`
- `sender` 统一采用结构化对象，而不是平铺 `senderId`
- 认证上下文负责注入 `tenant / principal / device / actor`
- binding 层只负责承载和归一化，不负责业务授权

## 5. 实施落地规划

### 5.1 任务拆解

1. 在新 workspace 中建立 `ccp-*` 骨架
2. 从现有契约和接口代码中抽出 envelope 与控制帧语义
3. 定义 codec trait、JSON codec 和 CBOR codec
4. 定义四种 binding 的通用接口和限制模型
5. 细化 `contract-*` 的业务 schema 命名空间
6. 对齐认证上下文中的权威字段注入规则

### 5.2 重点路径

重点涉及：

- `crates/im-platform-contracts/`
- `crates/im-auth-context/`
- `services/session-gateway/src/websocket.rs`
- `services/session-gateway/src/realtime.rs`
- `services/streaming-service/`
- `services/rtc-signaling-service/`
- `tools/chat-cli/`

### 5.3 迁移策略

推荐顺序：

1. 先建立 `ccp-core / ccp-control / ccp-codec`
2. 再建立 `ccp-binding-http / ws / sse / mqtt`
3. 再切换 `interface-*` 和 `services/*` 使用 binding
4. 最后补齐 registry、compatibility 和治理逻辑

### 5.4 权威字段收口

本 step 必须把以下规则写死到契约和绑定中：

- 请求体中的 `tenantId` 无效或被忽略
- `sender` 从认证上下文和会话上下文构造
- 内部应用命令使用规范化的 `actor` / `sender`
- 外部协议与内部命令之间需要显式转换层

## 6. 测试计划

建议新增或强化以下测试：

- envelope 编解码 round-trip 测试
- `hello -> auth_bind -> active` 状态机测试
- `session_resume` 正向和异常路径测试
- binding 与 codec 兼容性测试
- 权威字段覆盖防护测试
- compatibility matrix 基本约束测试

建议命令：

- `cargo test --workspace ccp`
- `cargo test -p im-platform-contracts`
- `cargo test -p session-gateway websocket_smoke_test -- --nocapture`
- `cargo test -p streaming-service http_smoke_test -- --nocapture`

## 7. 结果验证

本 step 完成后，应满足：

- 任一传输入口都能映射到同一套 CCP envelope
- 认证上下文收口后的权威字段规则不再漂移
- WebSocket、HTTP、SSE、MQTT 都有统一的协议母体
- 业务 schema 可以在不破坏协议骨架的情况下演进

## 8. 检查点

- `CP03-1`：`ccp-*` crate 族骨架已落地
- `CP03-2`：控制帧、envelope 和 capability negotiation 已冻结
- `CP03-3`：`sender`、`tenant` 等权威字段已从协议边界统一收口
- `CP03-4`：主要入口的协议测试可以跑通

### 8.1 推荐 review 产物

- `docs/review/step-03-执行卡-YYYY-MM-DD.md`
- `docs/review/step-03-CCP协议冻结决议-YYYY-MM-DD.md`
- `docs/review/step-03-contract映射矩阵-YYYY-MM-DD.md`
- `docs/review/step-03-权威字段边界审计-YYYY-MM-DD.md`

### 8.2 推荐并行车道

- `03-A`：`ccp-*` 协议骨架、envelope、握手协商
- `03-B`：`contract-*` 契约分层与传输绑定适配
- `03-C`：认证上下文、安全字段、版本兼容与示例测试资产
- 收口要求：`tenant`、`sender`、`actor` 等权威字段只能由单一 owner 收口，所有传输绑定必须复用统一协议骨架。
- 车道编排参考：[`94-Step并行执行编排与车道拆分建议`](./94-Step并行执行编排与车道拆分建议.md)

### 8.3 架构能力闭环判定

- 任一入口都必须映射到统一握手与 envelope 规则，否则只算“命名完成”，不算闭环。
- 如果客户端仍可伪造权威字段，或不同传输仍各写一套协议细节，本 step 未完成。
- 闭环验收以 [`95-架构能力闭环验收标准`](./95-架构能力闭环验收标准.md) 中 Step 03 条目为准。

### 8.4 快速并行执行建议

- 先冻结版本协商、envelope、权威字段，再并行推进协议骨架、契约分层、安全校验。
- 推荐“CCP 骨架”“contract 绑定”“认证与兼容”三车道同步推进，但最终字段裁决只能有一个 owner。
- 本步未冻结前，不允许 Step 04-06 大规模接入新协议实现，否则会造成反复返工。

### 8.5 完成后必须回写的架构文档

- 强制范围：本文件 `## 2. 架构对齐` 中列出的全部架构文档，必要时包含 `docs/架构/149-多Cell多Region协议升级与灾备兼容设计-2026-04-06.md`。
- 回写重点：握手协商、envelope、binding、权威字段、安全与兼容规则是否已经从协议设计转为统一实现约束。
- 必备证据：`docs/review/step-03-架构兑现-YYYY-MM-DD.md` 与 `docs/review/step-03-架构回写决议-YYYY-MM-DD.md`。

## 9. 风险与回滚

### 9.1 风险

- 如果先改业务再改协议，容易导致协议骨架反复返工
- 如果不把权威字段写死，后续客户端兼容会继续混乱
- 如果 binding 和业务契约不分层，`gateway` 类服务会再次膨胀

### 9.2 回滚

- 保留现有 `im-platform-contracts` 作为过渡导出层
- binding 可先以适配方式接入，不立刻替换所有旧入口
- registry 可先小步落地，再扩大治理范围

## 10. 完成定义

满足以下条件时，本 step 完成：

- `CCP` 的 crate 级落点已明确并初步实现
- 业务契约和协议基础设施完成职责拆分
- 权威字段、安全边界和握手模型已冻结
- 主要协议测试开始具备自动化验证

## 11. 下一步准入条件

进入 step 04 前必须确认：

- 连接层和路由层可以直接依赖新的 `ccp-*` 协议骨架
- 入口层不再需要各自重写一套握手和 envelope
