# Step 05 CP05-4 local-minimal-node disconnect lifecycle owner 架构兑现 - 2026-04-07

## 1. 对应能力

- `docs/架构/09-实施计划.md`
  - `Wave B / Step 05 / CP05-4` owner seam 收口补完本地 profile 侧最后一块 disconnect lifecycle
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
  - local profile 的 app-facing disconnect path 不再自己编排 route/runtime/platform glue
- `docs/架构/134-AI-Agent-IoT统一实时通信模型设计-2026-04-06.md`
  - device-scoped lifecycle owner 在本地 profile 中继续统一 register / route / disconnect
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
  - local profile disconnect 时序继续回到 owner seam 内部串联 route / realtime / presence / platform
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
  - tenant / actor / device / session scope 继续是本地 profile disconnect 的唯一权威输入
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`
  - `local-minimal-node` crate 的 app delegate -> owner 映射继续强化

## 2. 已兑现

- `session-gateway` 与 `local-minimal-node` 的 session entrypoint 已都不再保留 raw disconnect lifecycle glue。
- `LocalNodeDeviceRegistration` 现在同时拥有：
  - register/bind
  - route preflight
  - disconnect lifecycle
  - platform operational view refresh
- `CP05-4` 的已知 owner seam blocker 已清空。

## 3. 未兑现

- `Step 05` 仍未完成 step-wide `91 / 95 / 97` 闭环审计。
- `Wave B / 93` 仍未进入执行条件。

## 4. 证据

- 代码证据：
  - `services/local-minimal-node/src/node/device_registration.rs`
  - `services/local-minimal-node/src/node.rs`
  - `services/local-minimal-node/src/node/session.rs`
- 测试证据：
  - `test_local_minimal_node_disconnect_lifecycle_owner_moves_out_of_session_entrypoints`
  - `cargo test -p local-minimal-node --offline --target-dir target-cp054s-reg-local-minimal-node`
- 扫描证据：
  - `rg -n "disconnect_fence_matches_session\\(|clear_device_subscriptions\\(|release_device_route\\(|mark_device_disconnected\\(|platform::refresh_node_operational_view\\(&state\\)" services/session-gateway/src/session.rs services/local-minimal-node/src/node/session.rs -g "*.rs"`

