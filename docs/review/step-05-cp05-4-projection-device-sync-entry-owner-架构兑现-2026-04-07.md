# Step 05 / CP05-4 projection device-sync entry owner 架构兑现 - 2026-04-07

## 1. 对应架构能力

- `docs/架构/09-实施计划.md`
  - `Wave B / Step 05 / CP05-4`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
  - device-sync entry 默认字段不应长期停留在多个 projection handler 内联装配
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
  - device-sync 事件应先形成统一 entry draft/build，再进入 feed append 流程
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
  - `origin_event / actor / payload / occurred_at` 等权威字段应由单一 owner 维护
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`
  - 该 seam 应映射到 `projection-service` 内部 owner module，而不是散落在 `lib.rs` 多个 handler

## 2. 本轮架构兑现

- `projection-service` 新增 `device_sync.rs`，开始统一拥有 device-sync entry draft/build seam。
- `services/projection-service/src/lib.rs` 只保留 target 解析与 append 编排，不再直接内联拼装五类 `DeviceSyncFeedEntry`。
- owner / consumer 边界变为：
  - owner：`services/projection-service/src/device_sync.rs`
  - consumer：`services/projection-service/src/lib.rs`

## 3. 当前决议

- 认定本轮为 `CP05-4` 的有效架构兑现增量。
- 不认定 `CP05-4`、`Step 05`、`91 / 95 / 97`、`Wave B / 93` 已完成。

## 4. 回写对象

- `docs/架构/09-实施计划.md`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`
