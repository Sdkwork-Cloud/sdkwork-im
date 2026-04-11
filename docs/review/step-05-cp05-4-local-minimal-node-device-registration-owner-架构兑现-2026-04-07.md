# Step 05 CP05-4 local-minimal-node device registration owner 架构兑现 - 2026-04-07

## 1. 对应能力

- `09`：`Wave B / Step 05 / CP05-4` owner 收口继续推进
- `130`：本地 node 的 app-facing 入口不应长期保留 device registration edge glue
- `134`：device-scoped registration / route / sync-state 需要先进入稳定 owner seam，再进入 runtime
- `136`：session / command / projection 入口请求时序应先命中 device registration seam，再进入 presence / realtime / projection / cluster runtime
- `139`：device scope 与 route/session authority 应集中在单一权限边界
- `147`：`local-minimal-node` 模块映射需要把 device registration owner 从 `access.rs` 拆开

## 2. 已兑现

- `services/local-minimal-node/src/node/device_registration.rs` 成为本地 device registration owner
- `AppState` 退化为委托边界，`access.rs` 不再持有 route bind / register orchestration 事实实现
- `build.rs` 对 owner seam 的运行时装配统一收口到共享依赖集合

## 3. 未兑现

- `CP05-4` 仍未整体闭环
- `Step 05` 仍未整体闭环
- `91 / 95 / 97 / Wave B / 93` 仍未通过
- `session-gateway` 的 realtime entry route-affinity seam 仍未从重复入口 glue 收口到单一 owner

## 4. 回写对象

- `docs/架构/09-实施计划.md`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
- `docs/架构/134-AI-Agent-IoT统一实时通信模型设计-2026-04-06.md`
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`
