# Step 05 / CP05-4 projection device-sync session state owner 架构兑现 - 2026-04-07

## 1. 对应架构能力

- `docs/架构/09-实施计划.md`
  - `Wave B / Step 05 / CP05-4`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
  - presence / device-sync 的 session state 不应长期停留在 service edge 手工拼装
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
  - `resume / heartbeat / disconnect / presence_snapshot` 应先经过 projection-owned session sync state seam，再进入 presence runtime
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
  - device scope 与 actor scope 下的 `registered_devices / latest_sync_seq` 应由单一 owner 维护
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`
  - 该 seam 应映射到 `projection-service::access`，而不是散落在 `local-minimal-node/session.rs`

## 2. 本轮架构兑现

- `projection-service` 新增 `DeviceSyncSessionState` 与 `device_sync_session_state_from_auth_context(...)`。
- `local-minimal-node/session.rs` 现在只消费 projection-owned session sync state seam，再把结果交给 `DevicePresenceRuntime`。
- owner / consumer 边界变为：
  - owner：`services/projection-service/src/access.rs`
  - consumer：`services/local-minimal-node/src/node/session.rs`

## 3. 当前决议

- 认定本轮为 `CP05-4` 的有效架构兑现增量。
- 不认定 `CP05-4`、`Step 05`、`91 / 95 / 97`、`Wave B / 93` 已完成。

## 4. 回写对象

- `docs/架构/09-实施计划.md`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`
