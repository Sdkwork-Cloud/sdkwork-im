# Step 05 CP05-4 local-minimal-node route preflight owner 执行补充 - 2026-04-07

## 1. 当前定位

- `Wave B / Step 05 / CP05-4`
- 本轮继续做这个子项，因为 [`services/local-minimal-node/src/node/session.rs`](<workspace-root>\craw-chat\services\local-minimal-node\src\node\session.rs) 仍在多个 session / realtime / websocket 入口保留本地 `bind_device(...)` 胶水，非 takeover 路径没有直接对齐到 `LocalNodeDeviceRegistration` owner seam。

## 2. 本轮实际完成

- 在 [`services/local-minimal-node/src/node/device_registration.rs`](<workspace-root>\craw-chat\services\local-minimal-node\src\node\device_registration.rs) 新增 `LocalNodeDeviceRegistration::prepare_active_device_route(...)`。
- 在 [`services/local-minimal-node/src/node.rs`](<workspace-root>\craw-chat\services\local-minimal-node\src\node.rs) 新增 `AppState::prepare_active_device_route(...)` delegate。
- 在 [`services/local-minimal-node/src/node/session.rs`](<workspace-root>\craw-chat\services\local-minimal-node\src\node\session.rs) 删除本地 `bind_device(...)` helper。
- `heartbeat_presence(...)`、`disconnect_session(...)`、`register_device(...)`、`sync_realtime_subscriptions(...)`、`list_realtime_events(...)`、`ack_realtime_events(...)`、`realtime_websocket(...)` 改为统一消费 `state.prepare_active_device_route(...)`。
- `resume_session(...)` 保留直接调用 `state.bind_device_registration(..., allow_session_takeover = true)`，继续承载 session takeover 语义，不把 takeover 语义误收进普通 preflight seam。
- 在 [`services/local-minimal-node/tests/lib_structure_test.rs`](<workspace-root>\craw-chat\services\local-minimal-node\tests\lib_structure_test.rs) 新增 `test_local_minimal_node_route_preflight_owner_moves_out_of_session_entrypoints`。

## 3. 验证

- Red
  - `cargo test -p local-minimal-node --test lib_structure_test test_local_minimal_node_route_preflight_owner_moves_out_of_session_entrypoints --offline --target-dir target-cp054q-red-route-preflight`
- Green
  - `cargo test -p local-minimal-node --test lib_structure_test test_local_minimal_node_route_preflight_owner_moves_out_of_session_entrypoints --offline --target-dir target-cp054q-green-route-preflight`
- Regression
  - `cargo test -p local-minimal-node --offline --target-dir target-cp054q-reg-local-minimal-node`
- Format
  - `rustfmt --edition 2024 services/local-minimal-node/src/node.rs services/local-minimal-node/src/node/device_registration.rs services/local-minimal-node/src/node/session.rs services/local-minimal-node/tests/lib_structure_test.rs`
  - `rustfmt --edition 2024 --check services/local-minimal-node/src/node.rs services/local-minimal-node/src/node/device_registration.rs services/local-minimal-node/src/node/session.rs services/local-minimal-node/tests/lib_structure_test.rs`

## 4. 当前判断

- 本轮是 `CP05-4` 的有效增量。
- `local-minimal-node` 的非 takeover route preflight 已从 session entrypoint 本地胶水收口到 `device_registration` owner seam。
- `CP05-4` 仍未闭环。
- `Step 05` 仍未闭环。
- `91 / 95 / 97` 仍不能整体判定通过。
- `Wave B / 93` 仍阻塞。
