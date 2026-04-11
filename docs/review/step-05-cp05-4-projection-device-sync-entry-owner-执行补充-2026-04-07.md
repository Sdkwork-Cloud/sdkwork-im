# Step 05 / CP05-4 projection device-sync entry owner 执行补充 - 2026-04-07

## 1. 当前定位

- 波次：`Wave B`
- Step：`Step 05`
- 当前子项：`CP05-4`
- 本轮不允许把 `CP05-4`、`Step 05`、`91 / 95 / 97`、`Wave B / 93` 写成完成。

## 2. 本轮为什么做这个增量

前面已经把 projection realtime fanout target、projection conversation device-sync target、notification request/fanout 若干 seam 收口到真实 owner，但 `services/projection-service/src/lib.rs` 里仍然有五条 device-sync 投递路径分别手工组装 `DeviceSyncFeedEntry`：

- `fan_out_message_to_device_sync_feeds(...)`
- `fan_out_message_mutation_to_device_sync_feeds(...)`
- `fan_out_read_cursor_to_device_sync_feeds(...)`
- `fan_out_agent_handoff_status_to_device_sync_feeds(...)`
- `fan_out_member_governance_to_device_sync_feeds(...)`

这会让 `tenant_id / origin_event_id / origin_event_type / actor / payload_schema / occurred_at` 等公共字段继续在 projection handler 内部漂移，因此本轮继续沿 `CP05-4` 前推，而不是跳到 `Step 06`。

## 3. 本轮实际完成

- `projection-service` 新增 projection-owned device-sync entry owner module：
  - `services/projection-service/src/device_sync.rs`
  - `DeviceSyncEntryDraft`
  - `DeviceSyncEntryDraft::build_for_target(...)`
- `services/projection-service/src/lib.rs`
  - 新增 `append_device_sync_draft(...)`
  - message / mutation / read-cursor / handoff / member-governance 五条 device-sync fanout 路径统一消费同一个 draft/build seam
- `services/projection-service/tests/lib_structure_test.rs`
  - 新增 `test_projection_service_device_sync_entry_assembly_moves_out_of_lib_impl`
  - 锁定 `lib.rs` 不得继续保留 inline `DeviceSyncFeedEntry { ... }`

## 4. 改动文件

- `services/projection-service/src/device_sync.rs`
- `services/projection-service/src/lib.rs`
- `services/projection-service/tests/lib_structure_test.rs`

## 5. 验证

### 5.1 Red

- `$env:CARGO_TARGET_DIR='target-cp054g-red-structure'; cargo test -p projection-service --test lib_structure_test test_projection_service_device_sync_entry_assembly_moves_out_of_lib_impl --offline`

### 5.2 Green

- `$env:CARGO_TARGET_DIR='target-cp054g-green-structure'; cargo test -p projection-service --test lib_structure_test test_projection_service_device_sync_entry_assembly_moves_out_of_lib_impl --offline`
- `$env:CARGO_TARGET_DIR='target-cp054g-green-message-read'; cargo test -p projection-service --test timeline_projection_test test_device_sync_feed_projects_registered_devices_for_message_and_read_cursor_events --offline`
- `$env:CARGO_TARGET_DIR='target-cp054g-green-governance'; cargo test -p projection-service --test timeline_projection_test test_member_governance_events_project_typed_sync_feed_deltas --offline`
- `$env:CARGO_TARGET_DIR='target-cp054g-green-handoff'; cargo test -p projection-service --test timeline_projection_test test_agent_handoff_status_change_projects_device_sync_entries_for_active_members --offline`

### 5.3 Regression

- `rustfmt --edition 2024 services/projection-service/src/lib.rs services/projection-service/src/device_sync.rs services/projection-service/tests/lib_structure_test.rs`
- `rustfmt --edition 2024 --check services/projection-service/src/lib.rs services/projection-service/src/device_sync.rs services/projection-service/tests/lib_structure_test.rs`
- `$env:CARGO_TARGET_DIR='target-cp054g-reg-full'; cargo test -p projection-service --offline`
- `rg -n "DeviceSyncFeedEntry \\{|append_device_sync_draft\\(|build_for_target\\(" services/projection-service/src/lib.rs services/projection-service/src/device_sync.rs services/projection-service/tests/lib_structure_test.rs -S`

## 6. 当前结论

- 本轮是 `CP05-4` 的一个有效 projection / sync owner 增量。
- `projection-service` 现在拥有 device-sync entry draft/build 的统一装配 seam。
- `services/projection-service/src/lib.rs` 已不再保留五处 inline `DeviceSyncFeedEntry` 组装。
- 但 `CP05-4` 仍未闭环，`Step 05` 仍未闭环，`91 / 95 / 97` 仍不能整体判定通过，`Wave B / 93` 仍阻塞。
