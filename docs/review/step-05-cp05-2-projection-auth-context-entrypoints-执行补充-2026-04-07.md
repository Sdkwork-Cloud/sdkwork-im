# Step 05 CP05-2 Projection Auth Context Entrypoints 执行补充 - 2026-04-07

## 1. 当前定位

- 当前仍处于 `Wave B / Step 05 / CP05-2`。
- `CP05-1` 已闭环，`CP05-2` 在上一轮完成了 runtime-owned read query authority owner 收口，但 projection/read-model authority owner 仍未闭环。
- 本轮不能跳到 `CP05-3 / CP05-4`，也不能启动 `Wave B / 93`。

## 2. 本轮为什么继续这个缺口

- `services/projection-service/src/http.rs` 仍直接线程化 `auth.tenant_id / auth.actor_id`。
- `services/local-minimal-node/src/node/projection.rs` 仍直接消费 projection-service 的 raw query API。
- `services/local-minimal-node/src/node/session.rs` 仍直接消费 projection-service 的 raw device/query API。
- 因此，runtime read boundary 已开始收口，但 projection/read-model authority owner 仍分散在 service entrypoint，属于 `CP05-2` 的真实剩余缺口。

## 3. 本轮实际完成

- 新增 `services/projection-service/src/access.rs`，把 projection/read-model authority capture 收口到 projection-service 自己持有的 auth-context access boundary。
- 在 `TimelineProjectionService` 上新增 auth-context wrapper：
  - `ensure_active_member_from_auth_context(...)`
  - `register_device_from_auth_context(...)`
  - `registered_devices_from_auth_context(...)`
  - `latest_device_sync_seq_from_auth_context(...)`
  - `device_sync_feed_from_auth_context(...)`
  - `inbox_from_auth_context(...)`
  - `timeline_from_auth_context(...)`
  - `conversation_summary_from_auth_context(...)`
  - `read_cursor_from_auth_context(...)`
- 新增 `ProjectionAccessError`，让 authority validation 从 HTTP/local node adapter 下沉到 projection-service。
- `services/projection-service/src/http.rs` 改为统一消费上述 auth-context wrapper，不再自己拼接 raw `tenant_id / actor_id`。
- `services/local-minimal-node/src/node/projection.rs` 改为统一消费 projection-service 的 auth-context wrapper。
- `services/local-minimal-node/src/node/session.rs` 改为统一消费 projection-service 的 auth-context wrapper：
  - `registered_devices_from_auth_context(...)`
  - `latest_device_sync_seq_from_auth_context(...)`
  - `device_sync_feed_from_auth_context(...)`
- `services/local-minimal-node/src/node.rs` 增加 `ProjectionAccessError -> ApiError` 映射。
- 代码面复核结果：
  - `projection-service/http.rs` 与 `local-minimal-node node/projection.rs + node/session.rs` 已不再出现 raw `projection_service.(register_device|registered_devices|latest_device_sync_seq|device_sync_feed|timeline|inbox|conversation_summary|read_cursor)(...)` 调用。

## 4. 本轮改动文件

- 代码
  - `services/projection-service/src/access.rs`
  - `services/projection-service/src/lib.rs`
  - `services/projection-service/src/http.rs`
  - `services/local-minimal-node/src/node.rs`
  - `services/local-minimal-node/src/node/access.rs`
  - `services/local-minimal-node/src/node/projection.rs`
  - `services/local-minimal-node/src/node/session.rs`
- 测试
  - `services/projection-service/tests/lib_structure_test.rs`
  - `services/local-minimal-node/tests/lib_structure_test.rs`

## 5. TDD 证据

### 5.1 Red

- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2e-red-projection-structure'; cargo test -p projection-service --test lib_structure_test test_projection_service_access_module_exposes_auth_context_entrypoints --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2e-red-local-node-structure'; cargo test -p local-minimal-node --test lib_structure_test test_local_minimal_node_projection_paths_use_projection_service_auth_context_entrypoints --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2e-red-local-node-session'; cargo test -p local-minimal-node --test lib_structure_test test_local_minimal_node_session_projection_paths_use_projection_service_auth_context_entrypoints --offline`

### 5.2 Green / fresh verification

- `rustfmt --edition 2024 --check services/projection-service/src/access.rs services/projection-service/src/lib.rs services/projection-service/src/http.rs services/projection-service/tests/lib_structure_test.rs services/local-minimal-node/src/node.rs services/local-minimal-node/src/node/access.rs services/local-minimal-node/src/node/projection.rs services/local-minimal-node/src/node/session.rs services/local-minimal-node/tests/lib_structure_test.rs`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2e-green-projection-structure'; cargo test -p projection-service --test lib_structure_test --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2e-green-local-node-structure'; cargo test -p local-minimal-node --test lib_structure_test --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2e-projection-full'; cargo test -p projection-service --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2e-local-node-full'; cargo test -p local-minimal-node --offline`

## 6. 当前判断

- 这是 `CP05-2` 的第五个真实增量。
- 本轮已经把 projection/read-model 外部 authority owner 从 adapter 层收口到 `projection-service`。
- 但 `CP05-2` 仍未闭环，至少还剩：
  - `services/local-minimal-node/src/node/effects.rs` 仍存在 runtime read raw call，例如 `.list_members(auth.tenant_id.as_str(), conversation_id)`；
  - 下游 Step 05 authority consumer 仍未全部完成 owner 收口。
- 因此：
  - `CP05-2` 未通过
  - `Step 05` 未闭环
  - `91 / 95 / 97` 仍不能判定整体通过
  - `Wave B / 93` 仍阻塞
