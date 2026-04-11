# Step 05 CP05-4 local-minimal-node disconnect lifecycle owner 执行补充 - 2026-04-07

## 1. 执行上下文

- `Wave B / Step 05 / CP05-4`
- `session-gateway` 的 disconnect lifecycle owner 已完成后，[`services/local-minimal-node/src/node/session.rs`](D:\javasource\spring-ai-plus\spring-ai-plus-business\apps\craw-chat\services\local-minimal-node\src\node\session.rs) 仍保留对等 raw glue：
  - `disconnect_fence_matches_session(...)`
  - `clear_device_subscriptions(...)`
  - `release_device_route(...)`
  - `mark_device_disconnected(...)`
  - `platform::refresh_node_operational_view(&state)`
- 本轮目标是把本地 profile 的 disconnect 生命周期也收进 `LocalNodeDeviceRegistration`，收掉 `CP05-4` 最后一类已知 owner seam blocker。

## 2. 实际落地

- [`services/local-minimal-node/src/node/device_registration.rs`](D:\javasource\spring-ai-plus\spring-ai-plus-business\apps\craw-chat\services\local-minimal-node\src\node\device_registration.rs)
  - 新增 `DisconnectActiveDeviceRouteOutcome`
  - 新增 `LocalNodeDeviceRegistration::disconnect_active_device_route(...)`
  - 统一接管 disconnect fence 判断、活动路由预处理、订阅清理、route release、disconnect 标记、disconnect signal 与 `platform::refresh_node_operational_view(...)`
- [`services/local-minimal-node/src/node.rs`](D:\javasource\spring-ai-plus\spring-ai-plus-business\apps\craw-chat\services\local-minimal-node\src\node.rs)
  - 新增 `AppState::disconnect_active_device_route(...)` delegate
- [`services/local-minimal-node/src/node/session.rs`](D:\javasource\spring-ai-plus\spring-ai-plus-business\apps\craw-chat\services\local-minimal-node\src\node\session.rs)
  - `disconnect_session(...)` 不再直连 raw disconnect glue
  - 只根据 owner outcome 返回 `presence_snapshot(...)` 或 `disconnect(...)`
- [`services/local-minimal-node/tests/lib_structure_test.rs`](D:\javasource\spring-ai-plus\spring-ai-plus-business\apps\craw-chat\services\local-minimal-node\tests\lib_structure_test.rs)
  - 新增 `test_local_minimal_node_disconnect_lifecycle_owner_moves_out_of_session_entrypoints`

## 3. 验证

- Red
  - `cargo test -p local-minimal-node --test lib_structure_test test_local_minimal_node_disconnect_lifecycle_owner_moves_out_of_session_entrypoints --offline --target-dir target-cp054s-red-local-disconnect`
- Green
  - `cargo test -p local-minimal-node --test lib_structure_test test_local_minimal_node_disconnect_lifecycle_owner_moves_out_of_session_entrypoints --offline --target-dir target-cp054s-green-local-disconnect`
- Structure regression
  - `cargo test -p local-minimal-node --test lib_structure_test --offline --target-dir target-cp054s-structure`
- Package regression
  - `cargo test -p local-minimal-node --offline --target-dir target-cp054s-reg-local-minimal-node`
- Format
  - `rustfmt --edition 2024 services/local-minimal-node/src/node.rs services/local-minimal-node/src/node/device_registration.rs services/local-minimal-node/src/node/session.rs services/local-minimal-node/tests/lib_structure_test.rs`
  - `rustfmt --edition 2024 --check services/local-minimal-node/src/node.rs services/local-minimal-node/src/node/device_registration.rs services/local-minimal-node/src/node/session.rs services/local-minimal-node/tests/lib_structure_test.rs`
- 补充扫描
  - `rg -n "disconnect_fence_matches_session\\(|clear_device_subscriptions\\(|release_device_route\\(|mark_device_disconnected\\(|platform::refresh_node_operational_view\\(&state\\)" services/session-gateway/src/session.rs services/local-minimal-node/src/node/session.rs -g "*.rs"`
  - 结果：无命中，说明两个 session entry 模块的 raw disconnect lifecycle glue 已清空

## 4. 当前结论

- `CP05-4` 的最后一类已知 owner seam blocker 已完成收口。
- `Step 05` 仍不能直接宣布完成，因为还缺一轮基于当前真实仓库状态的整步 `91 / 95 / 97` 闭环审计。
- `Wave B / 93` 仍阻塞，因为 `Wave B` 还未整体完成。
