# Step 05 CP05-4 session-gateway disconnect lifecycle owner 执行补充 - 2026-04-07

## 1. 执行上下文

- `Wave B / Step 05 / CP05-4`
- 上一轮已完成 `session-gateway` 与 `local-minimal-node` 的 route preflight owner 收口，但 [`services/session-gateway/src/session.rs`](<workspace-root>\craw-chat\services\session-gateway\src\session.rs) 仍直接编排 disconnect 生命周期 glue：
  - `disconnect_fence_matches_session(...)`
  - `clear_device_subscriptions(...)`
  - `release_device_route(...)`
  - `mark_device_disconnected(...)`
- 本轮目标是在不改变断开语义的前提下，把上述 glue 收进既有 `SessionDeviceRegistration` owner seam，继续推进 `CP05-4`。

## 2. 实际落地

- [`services/session-gateway/src/device_registration.rs`](<workspace-root>\craw-chat\services\session-gateway\src\device_registration.rs)
  - 新增 `DisconnectActiveDeviceRouteOutcome`
  - 新增 `SessionDeviceRegistration::disconnect_active_device_route(...)`
  - 统一接管 disconnect fence 判断、活动路由预处理、订阅清理、route release、disconnect 标记与断开信号发送
- [`services/session-gateway/src/lib.rs`](<workspace-root>\craw-chat\services\session-gateway\src\lib.rs)
  - 新增 `AppState::disconnect_active_device_route(...)` delegate
  - 清理 owner 下沉后不再需要的 `node_id`、`realtime_cluster` 主状态字段
- [`services/session-gateway/src/session.rs`](<workspace-root>\craw-chat\services\session-gateway\src\session.rs)
  - `disconnect_session(...)` 不再直接操作 cluster/runtime raw disconnect glue
  - 只根据 owner outcome 决定返回 `presence_snapshot(...)` 还是 `disconnect(...)`
- [`services/session-gateway/tests/lib_structure_test.rs`](<workspace-root>\craw-chat\services\session-gateway\tests\lib_structure_test.rs)
  - 新增 `test_session_gateway_disconnect_lifecycle_owner_moves_out_of_session_entrypoints`
  - 锁定 session entrypoint 不能回流 raw disconnect lifecycle glue

## 3. 验证

- Red
  - `cargo test -p session-gateway --test lib_structure_test test_session_gateway_disconnect_lifecycle_owner_moves_out_of_session_entrypoints --offline --target-dir target-cp054r-red-disconnect-owner`
- Green
  - `cargo test -p session-gateway --test lib_structure_test test_session_gateway_disconnect_lifecycle_owner_moves_out_of_session_entrypoints --offline --target-dir target-cp054r-green-disconnect-owner`
- Structure regression
  - `cargo test -p session-gateway --test lib_structure_test --offline --target-dir target-cp054r-structure`
- Package regression
  - `cargo test -p session-gateway --offline --target-dir target-cp054r-reg-session-gateway`
- Format
  - `rustfmt --edition 2024 services/session-gateway/src/lib.rs services/session-gateway/src/device_registration.rs services/session-gateway/src/session.rs services/session-gateway/tests/lib_structure_test.rs`
  - `rustfmt --edition 2024 --check services/session-gateway/src/lib.rs services/session-gateway/src/device_registration.rs services/session-gateway/src/session.rs services/session-gateway/tests/lib_structure_test.rs`

## 4. 当前结论

- 本轮只收敛了 `session-gateway` 的 disconnect lifecycle owner seam。
- `CP05-4` 仍未整体闭环，因为 [`services/local-minimal-node/src/node/session.rs`](<workspace-root>\craw-chat\services\local-minimal-node\src\node\session.rs) 还保留对等的 raw disconnect lifecycle glue。
- `Step 05` 仍未闭环。
- `91 / 95 / 97` 针对 `Step 05` 的整体验收仍未通过。
- `Wave B / 93` 仍不可执行。
