# Step 05 CP05-2 downstream authority consumer auth-context entrypoints 执行补充

## 1. 当前定位

- 波次：`Wave B`
- Step：`Step 05`
- 子项：`CP05-2`
- 本轮目标：收掉 `local-minimal-node` 中最后两类 downstream authority consumer seam
  - `effects.rs` 的 member fanout / realtime recipient raw runtime read
  - `access.rs` 的 conversation-bound write access raw `actor_kind` threading

## 2. 本轮为什么做这一项

- 在 projection/read-model authority owner 收口完成后，`CP05-2` 仍残留最后两处 consumer-side authority seam：
  - `services/local-minimal-node/src/node/effects.rs`
  - `services/local-minimal-node/src/node/access.rs`
- 如果不继续把这两处 seam 收回 owner boundary，`sender / tenant` authority closure 仍不能视为完成，`CP05-2` 就不能闭环。

## 3. 本轮实际完成

- `services/local-minimal-node/src/node/effects.rs`
  - 新增 `conversation_member_principal_ids_from_auth_context(...)`
  - message notification fanout 改为消费 `conversation_runtime.list_members_from_auth_context(...)`
  - realtime conversation message event 改为消费 auth-context member recipient 解析
  - membership / handoff / stream conversation recipient 解析不再直接走 raw `.list_members(...)`
- `services/local-minimal-node/src/node/membership.rs`
  - 在 membership mutation 前先解析 `base_principals`
  - 避免 leave / remove 之后再做 post-mutation raw roster read
- `services/local-minimal-node/src/node/message.rs`
  - `publish_realtime_conversation_message_event(...)` 改为传 `AuthContext`
- `services/conversation-runtime/src/runtime.rs`
  - 新增 `ensure_conversation_bound_write_allowed_from_auth_context(...)`
  - 保留原 actor_kind mismatch 防护，不放宽安全语义
- `services/local-minimal-node/src/node/access.rs`
  - conversation-bound write access 改为消费 runtime 的 auth-context write-access seam

## 4. 测试与验证

### 4.1 TDD Red

- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2f-red-local-node-effects'; cargo test -p local-minimal-node --test lib_structure_test test_local_minimal_node_effects_member_fanout_uses_runtime_auth_context_entrypoints --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2g-red-runtime-write-access'; cargo test -p conversation-runtime --test conversation_domain_structure_test test_runtime_exposes_conversation_bound_write_access_auth_context_entrypoint --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2g-red-local-node-write-access'; cargo test -p local-minimal-node --test lib_structure_test test_local_minimal_node_access_paths_use_runtime_write_access_auth_context_entrypoint --offline`

### 4.2 调试回归

- 首次把 `ensure_conversation_bound_write_allowed_from_auth_context(...)` 接到 `ensure_conversation_bound_write_allowed(...)` 后，触发真实回归：
  - `test_conversation_bound_rtc_writes_reject_bearer_actor_kind_mismatch`
  - `test_conversation_bound_stream_writes_reject_bearer_actor_kind_mismatch`
- 根因：新 auth-context seam 错误放宽了 `actor_kind` 校验
- 修复：让 runtime auth-context seam 继续委托 `ensure_conversation_bound_write_allowed_with_actor_kind(...)`

### 4.3 回归修复验证

- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2g-fix-rtc-mismatch'; cargo test -p local-minimal-node --test access_control_e2e_test test_conversation_bound_rtc_writes_reject_bearer_actor_kind_mismatch --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2g-fix-stream-mismatch'; cargo test -p local-minimal-node --test access_control_e2e_test test_conversation_bound_stream_writes_reject_bearer_actor_kind_mismatch --offline`

### 4.4 Fresh verification

- `rustfmt --edition 2024 --check services/conversation-runtime/src/runtime.rs services/conversation-runtime/tests/conversation_domain_structure_test.rs services/local-minimal-node/src/node/access.rs services/local-minimal-node/src/node/effects.rs services/local-minimal-node/src/node/message.rs services/local-minimal-node/src/node/membership.rs services/local-minimal-node/tests/lib_structure_test.rs`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2g-runtime-full-rerun'; cargo test -p conversation-runtime --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2g-local-node-full-rerun'; cargo test -p local-minimal-node --offline`
- 搜索验证
  - `rg -n "\\.list_members\\(|\\.get_agent_handoff_state\\(|\\.require_active_member\\(|ensure_conversation_bound_write_allowed_with_actor_kind\\(|\\.ensure_conversation_bound_write_allowed\\(" services/local-minimal-node/src/node services/conversation-runtime/src/runtime/http.rs`
  - `rg -n "projection_service\\.(register_device|registered_devices|latest_device_sync_seq|device_sync_feed|timeline|inbox|conversation_summary|read_cursor)\\(" services/local-minimal-node/src/node services/projection-service/src/http.rs`
  - 上述两条当前均无匹配

## 5. 当前判断

- 本轮完成后，`CP05-2` 的下游 consumer seam 已补齐：
  - message mutation
  - non-message command
  - runtime auth-context entrypoint
  - read query auth-context entrypoint
  - projection/read-model auth-context entrypoint
  - downstream effects / write-access consumer seam
- 基于当前仓库代码与搜索证据，`CP05-2` 进入闭环态。
- 但 `Step 05` 仍未闭环：
  - `CP05-3` 未完成
  - `CP05-4` 未完成
  - `91 / 95 / 97` 不能整体判定 `Step 05` 通过
  - `Wave B / 93` 仍不能启动
