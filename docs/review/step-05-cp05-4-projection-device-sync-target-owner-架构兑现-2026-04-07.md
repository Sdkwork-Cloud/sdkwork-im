# Step 05 CP05-4 projection device-sync target owner 架构兑现

## 1. 对应架构能力

- `docs/架构/09-实施计划.md`
  - `Wave B / Step 05 / CP05-4` 对 projection / notification / multi-device sync owner seam 的持续收口。
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
  - 多设备 sync target 解析应由 service owner 统一持有，而不是散落在 consumer/handler 中。
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
  - device-sync feed fanout 应把 target 解析与 side-effect 组装放在统一 owner，保证跨事件类型时序一致。
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
  - principal / device target 解析属于权限与作用域边界的一部分，不能由多个 handler 各自复制。
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`
  - `projection-service` 负责 projection 与 multi-device sync 相关 target seam，crate 内部 handler 作为 consumer。

## 2. 本轮兑现内容

- `projection-service` 新增 conversation 级 `device_sync_fanout_targets_for_conversation(...)`，把：
  - active conversation principals
  - fallback principal
  - principal -> registered device target
  收敛到单一 owner seam。
- `fan_out_message_to_device_sync_feeds(...)`
- `fan_out_message_mutation_to_device_sync_feeds(...)`
- `fan_out_agent_handoff_status_to_device_sync_feeds(...)`
- `fan_out_member_governance_to_device_sync_feeds(...)`
  均已改为消费同一 seam。
- `fan_out_read_cursor_to_device_sync_feeds(...)` 也已改为复用 `projection-service` 自己已有的 `realtime_fanout_targets_for_principals(...)`，不再保留 raw device loop。

## 3. 偏差检查

- 未发现偏离 `Step 05 / CP05-4` 架构意图的新增临时兼容层。
- 未把 owner 再下沉回 `local-minimal-node` 或其它 consumer。
- 本轮仍属于 `CP05-4` 的局部兑现，不代表 `CP05-4` 整体架构目标已经全部完成。

## 4. 后续状态

- 已兑现：
  - `projection-service` conversation 级 device-sync target owner seam
- 未兑现：
  - `CP05-4` 其余 projection / notification / multi-device sync owner seam
  - `Step 05` 总体 `91 / 95 / 97`
  - `Wave B / 93`
