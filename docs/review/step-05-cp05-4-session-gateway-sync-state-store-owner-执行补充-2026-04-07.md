# Step 05 / CP05-4 / session-gateway sync-state store owner 执行补充

## 当前定位

- 波次：`Wave B`
- Step：`Step 05`
- 子项：`CP05-4`
- 本轮前置真实状态：
  - `services/session-gateway/src/session.rs` 已接管 `resume / get_presence_me / heartbeat / disconnect`
  - 但 `services/session-gateway/src/lib.rs` 的 `AppState` 仍直接持有 `registered_devices` / `latest_sync_sequences`
  - `AppState::device_sync_session_state(...)` 仍直接读取 `registered_devices(...)` 与 `latest_device_sync_seq(...)`
  - 这说明 session sync-state 的底层 owner 仍停留在 `AppState`

## 本轮为什么继续做这个子项

- `CP05-4` 的目标不是把 handler 挪文件，而是把主链路 side-effect / sync-state 的真实 owner 收敛到单一边界。
- 上一轮只完成了 handler 级收口，底层 storage owner 仍未独立，`Step 05` 不能据此宣告闭环。
- 如果继续把 raw storage 留在 `AppState`，后续 session / presence / websocket 路径仍会把 `lib.rs` 当事实 owner，违背 Step 05 的 owner 收口要求。

## 本轮实际完成

- 新增 `services/session-gateway/src/session_state.rs`
  - 新增 `SessionSyncState`
  - 新增 `DeviceSyncSessionState`
  - 新增 `register_device(...)`
  - 新增 `device_sync_session_state(...)`
  - 将 `registered_devices(...)` / `latest_device_sync_seq(...)` 作为该模块私有 helper
- 修改 `services/session-gateway/src/lib.rs`
  - 新增 `mod session_state;`
  - `AppState` 改为持有 `session_state: SessionSyncState`
  - `AppState::register_device(...)` 改为委托 `self.session_state.register_device(...)`
  - `AppState::device_sync_session_state(...)` 改为委托 `self.session_state.device_sync_session_state(...)`
  - 移除 `AppState` 中的 raw storage field 与 raw read helper
- 修改 `services/session-gateway/src/session.rs`
  - `DeviceSyncSessionState` 改为从 `session_state` 模块引入
  - session/presence handler 继续只消费 owner seam，不感知底层 map
- 修改 `services/session-gateway/tests/lib_structure_test.rs`
  - 新增 `test_session_gateway_sync_state_owner_moves_out_of_lib_impl`
  - 约束 `lib.rs` 不再直接持有 sync-state storage 与 raw read helper
  - 约束 `session_state.rs` 成为新的 owner 模块

## 本轮验证

- Red
  - `cargo test -p session-gateway --test lib_structure_test test_session_gateway_sync_state_owner_moves_out_of_lib_impl --offline --target-dir target-cp054m-red-sync-owner`
- Green
  - `cargo test -p session-gateway --test lib_structure_test test_session_gateway_sync_state_owner_moves_out_of_lib_impl --offline --target-dir target-cp054m-green-sync-owner`
  - `cargo test -p session-gateway --test lib_structure_test test_session_gateway_session_paths_use_device_sync_session_state_owner_seam --offline --target-dir target-cp054m-green-session-owner`
  - `cargo test -p session-gateway --test lib_structure_test test_session_gateway_session_surface_moves_out_of_lib_impl --offline --target-dir target-cp054m-green-session-structure`
- Regression
  - `cargo test -p session-gateway --offline --target-dir target-cp054m-reg-session-gateway`
- Format
  - `rustfmt --edition 2024 services/session-gateway/src/lib.rs services/session-gateway/src/session.rs services/session-gateway/src/session_state.rs services/session-gateway/tests/lib_structure_test.rs`
  - `rustfmt --edition 2024 --check services/session-gateway/src/lib.rs services/session-gateway/src/session.rs services/session-gateway/src/session_state.rs services/session-gateway/tests/lib_structure_test.rs`

## 当前判断

- 本轮是 `CP05-4` 的第十三个真实增量。
- `session-gateway` 内部这条 sync-state storage seam 已从 `AppState` 收口到独立 owner 模块。
- 但 `CP05-4` 仍未整体闭环。
- `Step 05` 仍未闭环。
- `91 / 95 / 97` 仍不能整体判定 `Step 05` 通过。
- `Wave B / 93` 仍阻塞。

## 下一步建议

- 基于当前仓库继续做 `CP05-4` 的 repo 级残留 seam 复核，而不是跳到 `Step 06`。
- 优先查找剩余仍由 edge / app 层持有的 owner 边界，尤其是跨 `notification-service`、`projection-service`、`local-minimal-node`、`session-gateway` 的共享状态装配点。
