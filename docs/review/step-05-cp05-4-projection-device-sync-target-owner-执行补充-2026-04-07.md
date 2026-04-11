# Step 05 CP05-4 projection device-sync target owner 执行补充

## 1. 当前上下文

- 波次：`Wave B`
- Step：`Step 05`
- 当前子项：`CP05-4`
- 本轮目标：把 `projection-service` 内部 conversation 级 device-sync target 解析收回到单一 owner seam，消除 `message / mutation / handoff / member governance` 多处重复的 active principal -> registered device fanout 逻辑。

## 2. 本轮实际落地

- `services/projection-service/src/lib.rs`
  - 新增 `device_sync_fanout_targets_for_conversation(...)`，由 `projection-service` 统一解析：
    - 当前会话 active members
    - 显式 fallback principal
    - principal -> registered device target 列表
  - 新增 `active_conversation_principal_ids(...)`，把 conversation active principal 解析封装到单一位置。
  - `fan_out_message_to_device_sync_feeds(...)` 改为消费 `device_sync_fanout_targets_for_conversation(...)`。
  - `fan_out_message_mutation_to_device_sync_feeds(...)` 改为消费 `device_sync_fanout_targets_for_conversation(...)`。
  - `fan_out_agent_handoff_status_to_device_sync_feeds(...)` 改为消费 `device_sync_fanout_targets_for_conversation(...)`。
  - `fan_out_member_governance_to_device_sync_feeds(...)` 改为消费 `device_sync_fanout_targets_for_conversation(...)`，保留“active member 为空时必须补 affected principal fallback”的既有语义。
  - `fan_out_read_cursor_to_device_sync_feeds(...)` 改为复用既有 `realtime_fanout_targets_for_principals(...)`，去掉本地 raw device loop。
- `services/projection-service/tests/lib_structure_test.rs`
  - 新增结构测试，要求 `projection-service` 暴露 conversation 级 device-sync target owner seam，并限制 raw `registered_devices(...)` fanout loop 不再散落在多处 handler。
- `services/projection-service/tests/timeline_projection_test.rs`
  - 新增行为测试，验证 conversation active members 与显式 fallback principal 的 device target 会被统一解析、去重并稳定排序。

## 3. TDD 过程

### 3.1 Red

- `cargo test -p projection-service --test lib_structure_test test_projection_service_exposes_conversation_device_sync_target_owner_seam --offline`
- `cargo test -p projection-service --test timeline_projection_test test_device_sync_fanout_targets_for_conversation_include_active_members_and_fallback_devices --offline`

### 3.2 Green

- `rustfmt --edition 2024 services/projection-service/src/lib.rs services/projection-service/tests/lib_structure_test.rs services/projection-service/tests/timeline_projection_test.rs`
- `$env:CARGO_TARGET_DIR='target-cp054e-structure'; cargo test -p projection-service --test lib_structure_test test_projection_service_exposes_conversation_device_sync_target_owner_seam --offline`
- `$env:CARGO_TARGET_DIR='target-cp054e-behavior'; cargo test -p projection-service --test timeline_projection_test test_device_sync_fanout_targets_for_conversation_include_active_members_and_fallback_devices --offline`

### 3.3 Regression

- `rustfmt --edition 2024 --check services/projection-service/src/lib.rs services/projection-service/tests/lib_structure_test.rs services/projection-service/tests/timeline_projection_test.rs`
- `$env:CARGO_TARGET_DIR='target-cp054e-full'; cargo test -p projection-service --offline`

## 4. 本轮结论

- `projection-service` 已经对 conversation 级 device-sync target 解析形成单一 owner seam。
- 本轮只闭合了 `CP05-4` 里的一个真实增量，不代表 `CP05-4` 整体完成。
- 当前仍未完成：
  - `CP05-4` 总闭环
  - `Step 05`
  - `91 / 95 / 97` 针对 `Step 05` 的总通过
  - `Wave B / 93` 总验收
