# Step 05 CP05-4 session-gateway route preflight owner 质量审计 - 2026-04-07

- 波次：`Wave B`
- Step：`Step 05`
- 子项：`CP05-4`

## 1. 审计范围

- `services/session-gateway/src/device_registration.rs`
- `services/session-gateway/src/lib.rs`
- `services/session-gateway/src/session.rs`
- `services/session-gateway/src/websocket_route.rs`
- `services/session-gateway/tests/lib_structure_test.rs`

## 2. 审计结论

- 通过。
- 这轮改动保持了 TDD 顺序：
  - 先新增结构红测并确认失败。
  - 再引入最小 owner seam 让测试转绿。
- 这轮改动没有绕开现有边界，而是继续沿着已经建立的 `SessionDeviceRegistration` seam 向前收口，边界方向与前序增量一致。

## 3. 正向证据

- 入口 raw glue 已收口：
  - `lib.rs` 的 realtime HTTP 三条入口不再直接调用 `ensure_route_session_current(...)`。
  - `session.rs` 的 `heartbeat_presence(...)` 与 `disconnect_session(...)` 不再直接拼 route-current + register。
  - `websocket_route.rs` 不再手工拼同样的 preflight。
- owner seam 清晰：
  - `SessionDeviceRegistration::prepare_active_device_route(...)`
  - `SessionDeviceRegistration::ensure_route_session_current(...)`
  - `AppState::prepare_active_device_route(...)`
- 回归覆盖到位：
  - 结构测试通过。
  - `session-gateway` 全量测试通过。
  - `rustfmt --check` 通过。

## 4. 风险与残留

- `resume_session(...)` 仍直接调用 `state.register_device(...)`，这是刻意保留：
  - resume 语义允许 session takeover。
  - 它不是此前重复 route preflight glue 的一部分。
- 本轮只证明 `session-gateway` 这个 seam 已收口，不证明 `CP05-4` 在 repo 级已经完成。
- 仍需继续做 `Step 05` 的 repo 级 closure review，确认是否还有别的 owner seam 残留。

## 5. 审计证据

- Red
  - `cargo test -p session-gateway --test lib_structure_test test_session_gateway_route_preflight_owner_moves_out_of_entrypoints --offline --target-dir target-cp054p-red-route-preflight`
- Green
  - `cargo test -p session-gateway --test lib_structure_test test_session_gateway_route_preflight_owner_moves_out_of_entrypoints --offline --target-dir target-cp054p-green-route-preflight`
  - `cargo test -p session-gateway --test lib_structure_test test_session_gateway_websocket_upgrade_module_stays_pure_axum_adapter --offline --target-dir target-cp054p-green-websocket-structure`
  - `cargo test -p session-gateway --test lib_structure_test test_session_gateway_device_registration_owner_moves_out_of_lib_impl --offline --target-dir target-cp054p-green-device-registration-structure`
- Regression
  - `cargo test -p session-gateway --offline --target-dir target-cp054p-reg-session-gateway`
- Formatting
  - `rustfmt --edition 2024 --check services/session-gateway/src/lib.rs services/session-gateway/src/device_registration.rs services/session-gateway/src/session.rs services/session-gateway/src/websocket_route.rs services/session-gateway/tests/lib_structure_test.rs`

## 6. 审计判定

- 认定本轮为 `CP05-4` 的有效质量增量。
- 不认定 `CP05-4`、`Step 05`、`91 / 95 / 97`、`Wave B / 93` 已整体通过。

