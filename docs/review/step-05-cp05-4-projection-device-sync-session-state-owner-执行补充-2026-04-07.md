# Step 05 / CP05-4 projection device-sync session state owner 执行补充 - 2026-04-07

## 1. 当前定位

- 波次：`Wave B`
- Step：`Step 05`
- 当前子项：`CP05-4`
- 本轮不允许把 `CP05-4`、`Step 05`、`91 / 95 / 97`、`Wave B / 93` 写成完成。

## 2. 本轮为什么做这个增量

前面已经把 projection realtime fanout target、projection device-sync target、projection device-sync entry、notification request/fanout 等 seam 收口到真实 owner，但 `services/local-minimal-node/src/node/session.rs` 仍在 session edge 分开拉取：

- `registered_devices_from_auth_context(...)`
- `latest_device_sync_seq_from_auth_context(...)`

并在 `resume / get_presence_me / heartbeat / disconnect` 四条路径上重复拼装 session sync state。这仍属于 `CP05-4` 中 `projection / sync` 与 `presence / device sync` 主链路之间的剩余连接点，因此本轮继续沿 `CP05-4` 前推，而不是跳到 `Step 06`。

## 3. 本轮实际完成

- `projection-service` 新增 projection-owned session sync state seam：
  - `services/projection-service/src/access.rs`
  - `DeviceSyncSessionState`
  - `device_sync_session_state_from_auth_context(...)`
- `services/projection-service/src/lib.rs`
  - 导出 `DeviceSyncSessionState`
- `services/local-minimal-node/src/node/session.rs`
  - 新增本地 consumer helper `device_sync_session_state(...)`
  - `resume / get_presence_me / heartbeat / disconnect` 改为统一消费 projection-owned session sync state seam
  - 删除本地 `registered_devices(...)` helper

## 4. 改动文件

- `services/projection-service/src/access.rs`
- `services/projection-service/src/lib.rs`
- `services/projection-service/tests/lib_structure_test.rs`
- `services/local-minimal-node/src/node/session.rs`
- `services/local-minimal-node/tests/lib_structure_test.rs`

## 5. 验证

### 5.1 Red

- `$env:CARGO_TARGET_DIR='target-cp054h-red-projection-structure'; cargo test -p projection-service --test lib_structure_test test_projection_service_access_module_exposes_auth_context_entrypoints --offline`
- `$env:CARGO_TARGET_DIR='target-cp054h-red-local-structure'; cargo test -p local-minimal-node --test lib_structure_test test_local_minimal_node_session_projection_paths_use_projection_service_auth_context_entrypoints --offline`

### 5.2 Green

- `$env:CARGO_TARGET_DIR='target-cp054h-green-projection-structure'; cargo test -p projection-service --test lib_structure_test test_projection_service_access_module_exposes_auth_context_entrypoints --offline`
- `$env:CARGO_TARGET_DIR='target-cp054h-green-local-structure'; cargo test -p local-minimal-node --test lib_structure_test test_local_minimal_node_session_projection_paths_use_projection_service_auth_context_entrypoints --offline`
- `$env:CARGO_TARGET_DIR='target-cp054h-reg-resume'; cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_resumes_session_and_returns_presence_snapshot --offline`
- `$env:CARGO_TARGET_DIR='target-cp054h-reg-sync'; cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_exposes_device_sync_feed_for_multi_device_resume --offline`
- `$env:CARGO_TARGET_DIR='target-cp054h-reg-disconnect'; cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_requires_fresh_resume_after_disconnect --offline`

### 5.3 Regression

- `rustfmt --edition 2024 services/projection-service/src/access.rs services/projection-service/src/lib.rs services/projection-service/tests/lib_structure_test.rs services/local-minimal-node/src/node/session.rs services/local-minimal-node/tests/lib_structure_test.rs`
- `rustfmt --edition 2024 --check services/projection-service/src/access.rs services/projection-service/src/lib.rs services/projection-service/tests/lib_structure_test.rs services/local-minimal-node/src/node/session.rs services/local-minimal-node/tests/lib_structure_test.rs`
- `$env:CARGO_TARGET_DIR='target-cp054h-reg-projection-full'; cargo test -p projection-service --offline`
- `rg -n "registered_devices_from_auth_context\\(|latest_device_sync_seq_from_auth_context\\(|fn registered_devices\\(|device_sync_session_state_from_auth_context\\(" services/local-minimal-node/src/node/session.rs services/projection-service/src/access.rs -S`

## 6. 当前结论

- 本轮是 `CP05-4` 的一个有效 projection / sync owner 增量。
- `projection-service` 现在拥有 session 级 device-sync state 的统一 auth-context seam。
- `local-minimal-node/session.rs` 已不再在四条路径上分别拼装 `registered_devices + latest_sync_seq`。
- 但 `CP05-4` 仍未闭环，`Step 05` 仍未闭环，`91 / 95 / 97` 仍不能整体判定通过，`Wave B / 93` 仍阻塞。
