# Step 05 CP05-4 session-gateway route preflight owner 执行补充 - 2026-04-07

- 波次：`Wave B`
- Step：`Step 05`
- 子项：`CP05-4`
- 本轮增量序号：`第 16 个真实增量`

## 1. 本轮为什么做这个增量

- 上一轮真实收口后，`docs/review/step-05-执行卡-2026-04-07.md` 与 repo 搜索都指向同一个剩余 blocker：
  - `services/session-gateway/src/lib.rs`
  - `services/session-gateway/src/session.rs`
  - `services/session-gateway/src/websocket_route.rs`
- 这三个入口仍重复拼接 `ensure_route_session_current(...) + register_device(...)`。
- 这说明 `session-gateway` 虽然已经把 `device_registration` owner 从 `lib.rs` 拆出，但 route preflight 组合编排还停留在 service edge，`CP05-4` 不能判定继续实质推进。

## 2. 本轮实际完成了什么

- 新增 route preflight 结构红测，锁定入口不能继续保留 raw route-current glue。
- 在 `SessionDeviceRegistration` 中新增统一 owner seam：
  - `prepare_active_device_route(...)`
  - 私有 `ensure_route_session_current(...)`
- 在 `AppState` 中新增委托边界：
  - `prepare_active_device_route(...)`
- 将以下入口改为消费统一 seam，而不是继续手工拼装：
  - `sync_realtime_subscriptions(...)`
  - `list_realtime_events(...)`
  - `ack_realtime_events(...)`
  - `heartbeat_presence(...)`
  - `disconnect_session(...)`
  - `prepare_realtime_websocket_route(...)`
- 更新结构测试，使 `websocket_route` 不再要求直接包含 `ensure_route_session_current(...)` 与 `state.register_device(...)`。

## 3. 改了哪些代码 / 文档 / 测试

- 代码
  - `services/session-gateway/src/device_registration.rs`
  - `services/session-gateway/src/lib.rs`
  - `services/session-gateway/src/session.rs`
  - `services/session-gateway/src/websocket_route.rs`
- 测试
  - `services/session-gateway/tests/lib_structure_test.rs`
    - 新增 `test_session_gateway_route_preflight_owner_moves_out_of_entrypoints`
    - 调整 websocket route 结构约束
- review 文档
  - 本文件
  - `docs/review/step-05-cp05-4-session-gateway-route-preflight-owner-质量审计-2026-04-07.md`
  - `docs/review/step-05-cp05-4-session-gateway-route-preflight-owner-架构兑现-2026-04-07.md`

## 4. 做了哪些验证

- Red
  - `cargo test -p session-gateway --test lib_structure_test test_session_gateway_route_preflight_owner_moves_out_of_entrypoints --offline --target-dir target-cp054p-red-route-preflight`
- Green
  - `cargo test -p session-gateway --test lib_structure_test test_session_gateway_route_preflight_owner_moves_out_of_entrypoints --offline --target-dir target-cp054p-green-route-preflight`
  - `cargo test -p session-gateway --test lib_structure_test test_session_gateway_websocket_upgrade_module_stays_pure_axum_adapter --offline --target-dir target-cp054p-green-websocket-structure`
  - `cargo test -p session-gateway --test lib_structure_test test_session_gateway_device_registration_owner_moves_out_of_lib_impl --offline --target-dir target-cp054p-green-device-registration-structure`
- Full regression
  - `cargo test -p session-gateway --offline --target-dir target-cp054p-reg-session-gateway`
- Formatting
  - `rustfmt --edition 2024 services/session-gateway/src/lib.rs services/session-gateway/src/device_registration.rs services/session-gateway/src/session.rs services/session-gateway/src/websocket_route.rs services/session-gateway/tests/lib_structure_test.rs`
  - `rustfmt --edition 2024 --check services/session-gateway/src/lib.rs services/session-gateway/src/device_registration.rs services/session-gateway/src/session.rs services/session-gateway/src/websocket_route.rs services/session-gateway/tests/lib_structure_test.rs`
- Repo spot check
  - `rg -n "ensure_route_session_current\\(" services crates adapters tools -g "*.rs"`
  - 结果显示 `session-gateway` 入口层已不再保留该 raw glue，仅剩 owner seam 与 cluster/test 内部实现。

## 5. 当前结论

- 这轮是 `CP05-4` 的有效真实增量。
- `session-gateway` 的 route preflight owner 已经从分散入口收口到单一 owner seam。
- 但本轮不构成 `CP05-4`、`Step 05`、`91 / 95 / 97`、`Wave B / 93` 的整体通过结论。

## 6. 当前还差什么

- `CP05-4` 仍需 repo 级继续排查剩余 service-edge owner seam，不能因为当前 blocker 已清掉就把整个子项判定完成。
- `Step 05` 仍未闭环，不能进入 `Step 06`。
- `91 / 95 / 97` 对 `Step 05` 的总判定仍是未通过。
- `Wave B / 93` 仍阻塞。

