# Step 08 - AI / Agent / IoT 统一扩展层落地

## 1. 目标与范围

本 step 用于把 AI、Agent、IoT 从“后挂模块”正式纳入系统的一等主体模型与一等实时能力模型。

本 step 必须覆盖：

- `agent` 作为一等主体
- `device` 作为一等主体
- tool call / tool result
- agent handoff
- device session / presence / twin
- MQTT / device stream / command stream

### 1.1 执行输入

- step 06 的流式与 RTC 一等能力模型
- step 07 的 capability、registry 与 rollout 治理基础
- `134` 关于统一主体模型的架构结论
- `150` 关于 provider/plugin、设备管理与接入体系的冻结标准
- 当前 `automation-service`、通知链路与潜在 MQTT 接入点

### 1.2 本步非目标

- 不在本 step 内自建推理平台、模型编排平台或大规模设备平台
- 不在本 step 内覆盖所有厂商硬件协议，但必须冻结统一协议插件标准
- 不在本 step 内重做普通 IM 主链路

### 1.3 最小输出

- `agent` 与 `device` 的主体模型
- tool call / tool result 与 device telemetry / command 基线
- Agent / Device 与消息、流主链路的统一接入方式
- `MQTT`、开源 `小智协议`、设备管理与接入体系的统一抽象
- 至少一条 Agent 路径和一条 Device 路径的自动化验证

## 2. 架构对齐

本 step 重点对齐：

- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
- `docs/架构/134-AI-Agent-IoT统一实时通信模型设计-2026-04-06.md`
- `docs/架构/143-统一协议总纲与分层设计-2026-04-06.md`
- `docs/架构/145-CCP数据协议与版本兼容安全设计-2026-04-06.md`
- `docs/架构/150-插件化提供商体系与设备接入设计-2026-04-08.md`

## 3. 当前现状与问题

当前架构已经明确：

- `user / agent / device / bot / system` 是统一主体模型
- 流式输出是系统内建能力
- MQTT 和设备接入是目标能力之一

但当前代码层仍需补齐：

- 统一的 agent 事件与生命周期模型
- tool call / tool result 的结构化事件载体
- device session、device twin、uplink / downlink stream 的明确落点
- Agent 与 IoT 对消息主链路和流主链路的共用方式
- `MQTT / 小智协议` 的统一 `IotProtocolAdapter`
- 设备管理与接入体系的 owner boundary

## 4. 设计

### 4.1 Agent 模型

Agent 必须具备：

- 主体标识与 sender 表达
- 对话上下文绑定
- 流式输出能力
- tool call / tool result 生命周期
- handoff 与人工接管能力

### 4.2 Device 模型

Device 必须具备：

- 设备身份
- 设备会话
- 设备在线状态
- telemetry 上行流
- command 下行流
- twin 或 shadow 状态

此外还必须冻结设备管理与接入体系：

- product / model
- device registry
- credential / certificate / token
- tenant / owner binding
- access policy / ban policy

### 4.3 统一实时语义

统一规则：

- Agent 输出走 `stream` 和 `message` 共用骨架
- 设备遥测走 `stream`
- 设备命令回执可写回 `message / event / stream`
- Agent 与 device 都服从统一 sender / actor / scope / capability

### 4.4 协作上下文

本 step 还要为未来协作上下文补齐扩展位：

- topic
- thread
- card
- task reference
- document reference
- workflow reference
- AI summary

### 4.5 IoT 协议插件与设备接入

IoT 必须采用双层抽象：

- `DeviceAccessProvider`：负责注册、鉴权、会话、绑定、封禁、device twin
- `IotProtocolAdapter`：负责 `MQTT`、开源 `小智协议` 等协议的编解码与上下行归一化

首批协议固定为：

- `MQTT`
- `小智协议`

## 5. 实施落地规划

### 5.1 任务拆解

1. 建立 `contract-agent`、`contract-iot`
2. 建立 `domain-agent`、`domain-device`
3. 建立 `app-agent`、`app-iot`
4. 建立 `runtime-agent`、`runtime-iot`
5. 建立 `runtime-device-management` 或等价 owner seam，承接设备管理与接入体系
6. 让 `stream`、`message`、`notification` 支持 agent / device 事件接入
7. 建立 `interface-mqtt`、`interface-xiaozhi` 与设备鉴权、签名、topic / frame 归一化逻辑

### 5.2 重点路径

重点涉及：

- `services/automation-service/`
- `services/notification-service/`
- `services/streaming-service/`
- `services/control-plane-api/`
- `crates/im-domain-core/src/automation.rs`
- 新增或重构的 `runtime-agent`、`runtime-iot`、`runtime-device-management`、`interface-mqtt`、`interface-xiaozhi`

### 5.3 Agent 落地顺序

推荐顺序：

1. 人与 Agent 对话
2. Agent 流式输出物化
3. tool call / tool result 结构化事件
4. handoff / 协作上下文

### 5.4 IoT 落地顺序

推荐顺序：

1. 设备鉴权与设备会话
2. telemetry 上行
3. command 下行
4. twin / shadow
5. `小智协议` 映射到统一设备模型
6. 人 / Agent / 设备协同场景

## 6. 测试计划

建议重点测试：

- 人与 Agent 对话测试
- Agent 流式输出测试
- tool call / tool result 生命周期测试
- 设备鉴权与 MQTT 接入测试
- `小智协议` 适配与设备接入测试
- telemetry / command 测试
- device twin 状态同步测试
- 统一 sender / actor 模型测试

建议补充以下场景：

- human -> agent -> human 回路
- human -> agent -> device -> human 回路
- device -> telemetry -> timeline / alert 回路

## 7. 结果验证

本 step 完成后，需要验证：

- Agent 和 Device 已成为系统一等主体
- AI 扩展不会破坏普通 IM 主链路
- IoT 接入不是外挂协议，而是 CCP 的一个 binding / capability 组合
- `MQTT / 小智协议` 都通过统一 `IotProtocolAdapter` 接入，设备管理与接入体系清晰
- Agent 与 Device 可以通过消息和流共享同一套实时能力骨架

## 8. 检查点

- `CP08-1`：`agent`、`device` 的契约、主体模型和设备管理基础模型已落地
- `CP08-2`：Agent 流式输出与 tool call 生命周期已可验证
- `CP08-3`：`MQTT / 小智协议`、设备接入与 telemetry / command 主路径已打通
- `CP08-4`：Agent / Device 已复用统一 sender / actor / capability 机制

### 8.1 推荐 review 产物

- `docs/review/step-08-执行卡-YYYY-MM-DD.md`
- `docs/review/step-08-ai-agent-iot统一扩展-YYYY-MM-DD.md`
- `docs/review/step-08-agent主体验证-YYYY-MM-DD.md`
- `docs/review/step-08-device接入验证-YYYY-MM-DD.md`

### 8.2 推荐并行车道

- `08-A`：AI / Agent 主体建模、能力挂载、实时输出桥接
- `08-B`：IoT / Device 接入、设备管理、会话映射、协议适配
- `08-C`：sender / actor / scope 统一语义与兼容验证
- 收口要求：Agent 与 Device 都必须进入统一主体模型，任何车道不得为 AI / IoT 单独再造一套实时协议。
- 车道编排参考：[`94-Step并行执行编排与车道拆分建议`](./94-Step并行执行编排与车道拆分建议.md)

### 8.3 架构能力闭环判定

- 至少需要一条 Agent 路径和一条 Device 路径跑通统一主链路，且不会破坏普通 IM 语义。
- 如果 Agent / Device 只是旁路模块，或 AI / IoT 通过独立协议栈接入，或设备管理与接入体系仍散落在协议适配层，本 step 不算闭环。
- 闭环验收以 [`95-架构能力闭环验收标准`](./95-架构能力闭环验收标准.md) 中 Step 08 条目为准。

### 8.4 快速并行执行建议

- 先冻结统一主体模型中的 `actor` / `sender` / `scope` 扩展规则，再并行做 Agent、Device、兼容验证三条线。
- 推荐至少同时维护一条 Agent 实链路和一条 Device 实链路，避免只做概念建模。
- 一旦发现 AI / IoT 需要独立协议才能落地，必须先回到架构文档修正，而不是旁路实现。

### 8.5 完成后必须回写的架构文档

- 强制范围：本文件 `## 2. 架构对齐` 中列出的全部架构文档。
- 回写重点：Agent、Device、统一主体模型、协议兼容与安全边界是否真正接入 IM 主链路，而不是形成旁路架构。
- 必备证据：`docs/review/step-08-架构兑现-YYYY-MM-DD.md` 与 `docs/review/step-08-架构回写决议-YYYY-MM-DD.md`。

## 9. 风险与回滚

### 9.1 风险

- AI 与 IoT 若走两套体系，会再次破坏统一架构
- Agent / device 若绕开统一 sender 和 capability 模型，后续权限治理会失控
- MQTT 接入若不做边界归一化，会出现新的协议孤岛

### 9.2 回滚

- 可以先让 agent / device 以扩展 payload 落到主链路，再逐步抽出完整模块
- MQTT 先以最小能力绑定接入，避免一次性覆盖复杂设备场景
- handoff、twin 等高级能力先以 feature flag 开启

## 10. 完成定义

满足以下条件时，本 step 完成：

- Agent 与 Device 均具备一等主体地位
- AI / Agent / IoT 的主要实时链路已跑通
- 扩展能力没有破坏普通 IM 的体验与稳定性

## 11. 下一步准入条件

进入 step 09 前必须确认：

- 新扩展能力已经能够接入统一的存储、投影和观测体系

## 12. 2026-04-08 As-Built（IoT MQTT protocol adapter baseline）

- 新增 crate
  - `adapters/iot-mqtt`
- 当前 `iot-mqtt` 已实现最小 `IotProtocolAdapter` baseline：
  - `protocol_key`
  - `decode_uplink`
  - `encode_downlink`
  - `provider_health_snapshot`
- 协议归一化边界已冻结：
  - uplink -> `device.telemetry`
  - downlink -> `device.command`
  - `topic / qos / brokerEndpoint` 只停留在 adapter metadata
- 新增验证
  - `adapters/iot-mqtt/tests/adapter_contract_test.rs`
  - `services/local-minimal-node/tests/iot_provider_docs_test.rs`
- Step 08 / `08-B`
  - IoT 线已从“只有协议矩阵”推进到“第一条真实 protocol adapter baseline”
  - 下一轮优先 `iot-xiaozhi` adapter、`DeviceAccessProvider` baseline 与 IoT provider external surface

## 13. 2026-04-08 As-Built（xiaozhi external source alignment）
- `xiaozhi` 官方源码真源冻结为：`https://github.com/78/xiaozhi-esp32.git`
- 仓库内标准落位冻结为：`external/xiaozhi-esp32`
- 标准命令冻结为：`git submodule add https://github.com/78/xiaozhi-esp32.git external/xiaozhi-esp32`
- Step 08 的这一轮要求不再只是保留 `xiaozhi` 名称，而是必须反复阅读官方源码，对齐真实接入行为。
- 抽象边界固定为双层：
  - `DeviceAccessProvider`：设备注册、鉴权、会话、绑定、设备管理与接入体系
  - `IotProtocolAdapter`：`xiaozhi` 协议编解码、上下行映射、消息归一化
- 在完成真实源码对齐前，不能宣称 `xiaozhi` 已进入运行时交付；当前仅完成文档标准冻结与 external 目录标准。

## 14. 2026-04-08 As-Built（IoT local device access provider baseline）
- 新增 crate
  - `adapters/iot-access-local`
- 当前 `iot-access-local` 已实现最小 `DeviceAccessProvider` baseline：
  - `register_device`
  - `bind_owner`
  - `disable_device`
  - `provider_health_snapshot`
- 设备管理与接入体系基线已经冻结：
  - local device registry
  - owner binding
  - disable / ban baseline
  - 默认协议分配 `mqtt / xiaozhi`
- Step 08 / `08-D`
  - IoT 线已不再只有协议 adapter，开始拥有设备管理与接入体系的真实 provider baseline
  - 下一轮优先 `iot-xiaozhi` adapter、`DeviceAccessProvider` 注入 `local-minimal-node / session-gateway` 与 IoT provider external surface

## 15. 2026-04-08 As-Built（local-minimal-node DeviceAccessProvider injection）
- `local-minimal-node` 已新增真实 provider 注入入口：
  - `build_default_app_with_device_access_provider`
  - `build_default_app_with_runtime_dir_and_device_access_provider`
- 默认运行时已真实装配 `iot-access-local`
- `LocalNodeDeviceRegistration` 已真实消费 `DeviceAccessProvider`
- `/im/v3/api/devices/register` 已真实调用：
  - `register_device`
  - `bind_owner`
- 当前第一次注册冻结参数：
  - `product_id = local-minimal-device`
  - `credential_kind = device_route`
- provider 调用只在“projection 中尚无该 device”时触发，避免 heartbeat / realtime route preflight 重复触发外部设备接入逻辑
- Step 08 / `08-E`
  - IoT 线已从“只有 DeviceAccessProvider baseline”推进到“local-minimal-node 真实运行时注入闭环”
  - 下一轮优先 `session-gateway` 注入与 IoT provider external HTTP surface

## 16. 2026-04-09 As-Built（session-gateway DeviceAccessProvider injection）
- `session-gateway` 已新增真实 provider 注入入口：
  - `build_app_with_device_access_provider`
  - `build_app_with_cluster_and_device_access_provider`
- 默认 runtime 已真实装配 `im-adapter-iot-access-local::LocalDeviceAccessProvider`
- `SessionDeviceRegistration` 现在是 `session-gateway` 中 `DeviceAccessProvider` 的唯一消费点
- `POST /im/v3/api/device/sessions/resume` 已真实调用：
  - `register_device`
  - `bind_owner`
- provider 调用顺序已冻结为：
  - `resume`
  - `DeviceSyncState::has_registered_device`
  - 首次注册时 `register_device / bind_owner`
  - presence / realtime / session-state / route bind
- 当前 `session-gateway` 冻结的 provider 请求常量为：
  - `product_id = session-gateway-device`
  - `credential_kind = device_route`
- heartbeat / realtime route preflight 不再重复触发 provider 注册/绑定
- Step 08 / `08-F`
  - `local-minimal-node + session-gateway` 的 `DeviceAccessProvider` 运行时注入已形成真实闭环
  - 下一轮优先 IoT provider external HTTP surface

## 17. 2026-04-09 As-Built（IoT access provider health HTTP surface）
- `local-minimal-node` 已新增：
  - `GET /app/v3/api/iot/access/provider_health`
- 该路由已真实返回当前注入的：
  - `DeviceAccessProvider::provider_health_snapshot()`
- 当前默认 `iot-access-local` 已具备直接 HTTP 可见性：
  - `pluginId = iot-access-local`
  - `details.providerKind = local`
  - `details.assignedProtocols = mqtt,xiaozhi`
- Step 08 / `08-G`
  - IoT provider external HTTP surface 已迈出第一条真实 access health 路径
  - 但 protocol surface 仍未闭环，下一轮优先 IoT protocol external HTTP surface
