# Step 05 CP05-4 local-minimal-node route preflight owner 架构兑现 - 2026-04-07

## 1. 对应架构文档

- [`docs/架构/09-实施计划.md`](<workspace-root>\craw-chat\docs\架构\09-实施计划.md)
- [`docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`](<workspace-root>\craw-chat\docs\架构\130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md)
- [`docs/架构/134-AI-Agent-IoT统一实时通信模型设计-2026-04-06.md`](<workspace-root>\craw-chat\docs\架构\134-AI-Agent-IoT统一实时通信模型设计-2026-04-06.md)
- [`docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`](<workspace-root>\craw-chat\docs\架构\136-关键业务链路与跨Plane时序设计-2026-04-06.md)
- [`docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`](<workspace-root>\craw-chat\docs\架构\139-权限能力模型与协议演进设计-2026-04-06.md)
- [`docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`](<workspace-root>\craw-chat\docs\架构\147-CCP到Crate与接口模块落地映射设计-2026-04-06.md)

## 2. 已兑现能力

- `local-minimal-node` 的普通 session / realtime / websocket device route preflight 不再留在 [`services/local-minimal-node/src/node/session.rs`](<workspace-root>\craw-chat\services\local-minimal-node\src\node\session.rs) 本地 helper。
- `LocalNodeDeviceRegistration` 现在同时承接 device registration owner 与普通 route preflight owner，consumer 只保留 delegate。
- `resume_session(...)` 的 takeover 语义保持单独入口，没有被普通 preflight seam 混淆。

## 3. 未兑现能力

- `CP05-4` 仍未完成 repo 级 closure。
- `Step 05` 仍未满足 `95` 的整体闭环标准。
- 还不能执行 `Wave B / 93` 总验收。

## 4. 实现与架构是否偏离

- 判定：`实现更具体`
- 原因：本轮没有偏离 `device-scoped registration / route / sync-state` 先进入 owner seam、再进入 runtime 的方向，只是把 `local-minimal-node` 普通 preflight 的落点补齐到了与 `session-gateway` 同类的 as-built 边界。

## 5. docs/架构 回写

- 已完成回写。
- 本轮追加到 `09 / 130 / 134 / 136 / 139 / 147` 的 as-built 记录，用于说明 `local-minimal-node` 普通 route preflight seam 的实际落点。

## 6. 证据

- 代码
  - [`services/local-minimal-node/src/node.rs`](<workspace-root>\craw-chat\services\local-minimal-node\src\node.rs)
  - [`services/local-minimal-node/src/node/device_registration.rs`](<workspace-root>\craw-chat\services\local-minimal-node\src\node\device_registration.rs)
  - [`services/local-minimal-node/src/node/session.rs`](<workspace-root>\craw-chat\services\local-minimal-node\src\node\session.rs)
- 测试
  - `cargo test -p local-minimal-node --test lib_structure_test test_local_minimal_node_route_preflight_owner_moves_out_of_session_entrypoints --offline --target-dir target-cp054q-green-route-preflight`
  - `cargo test -p local-minimal-node --offline --target-dir target-cp054q-reg-local-minimal-node`
- 文档
  - 本文
  - [`docs/review/step-05-cp05-4-local-minimal-node-route-preflight-owner-执行补充-2026-04-07.md`](<workspace-root>\craw-chat\docs\review\step-05-cp05-4-local-minimal-node-route-preflight-owner-执行补充-2026-04-07.md)
  - [`docs/review/step-05-cp05-4-local-minimal-node-route-preflight-owner-质量审计-2026-04-07.md`](<workspace-root>\craw-chat\docs\review\step-05-cp05-4-local-minimal-node-route-preflight-owner-质量审计-2026-04-07.md)
