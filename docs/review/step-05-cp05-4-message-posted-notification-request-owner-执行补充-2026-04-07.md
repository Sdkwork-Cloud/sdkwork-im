# Step 05 / CP05-4 message-posted notification request owner 执行补充 - 2026-04-07

## 1. 当前定位

- 波次：`Wave B`
- Step：`Step 05`
- 当前子项：`CP05-4`
- 本轮不允许把 `CP05-4`、`Step 05`、`91 / 95 / 97`、`Wave B / 93` 写成完成。

## 2. 本轮为什么做这个增量

前面已经把 notification public access、message notification fanout、automation result notification、projection realtime fanout target、projection device-sync target 等 owner seam 收口到真实 owner，但 `services/local-minimal-node/src/node/effects.rs` 仍在本地拼装 `message.posted` notification 的默认字段：

- `notification_id_seed`
- `source_event_type`
- `category`
- `channel`
- `title / body`
- `payload`

这仍属于 `CP05-4` 中 notification side-effect 与 owner seam 之间的剩余连接点，因此本轮继续沿 `CP05-4` 前推，而不是跳到 `Step 06`。

## 3. 本轮实际完成

- `notification-service` 新增 runtime-owned request owner：
  - `RequestMessagePostedNotifications`
  - `NotificationRuntime::request_message_posted_notifications(...)`
- `message.posted` notification 的默认字段装配从 `local-minimal-node` 收口到 `notification-service`：
  - `source_event_type = message.posted`
  - `category = message.new / rtc.event`
  - `channel = inapp`
  - `notification_id_seed = message_id`
  - payload JSON 的 message/conversation 字段
  - `title / body = summary`
- `local-minimal-node` effects 现在只负责：
  - 提供 `auth`
  - 提供 `recipient_ids`
  - 提供 message/conversation 元数据和 summary
  - 消费 notification owner seam

## 4. 改动文件

- `services/notification-service/src/lib.rs`
- `services/notification-service/tests/lib_structure_test.rs`
- `services/notification-service/tests/notification_pipeline_test.rs`
- `services/local-minimal-node/src/node/effects.rs`
- `services/local-minimal-node/tests/lib_structure_test.rs`

## 5. 验证

### 5.1 Red

- `cargo test -p notification-service --test lib_structure_test test_notification_runtime_exposes_message_posted_notification_owner_seam --offline`
- `cargo test -p notification-service --test notification_pipeline_test test_request_message_posted_notifications_own_message_notification_defaults --offline`
- `$env:CARGO_TARGET_DIR='target-cp054f-red-local'; cargo test -p local-minimal-node --test lib_structure_test test_local_minimal_node_effects_use_notification_service_message_posted_owner_seam --offline`

### 5.2 Green

- `$env:CARGO_TARGET_DIR='target-cp054f-green-notification-structure'; cargo test -p notification-service --test lib_structure_test test_notification_runtime_exposes_message_posted_notification_owner_seam --offline`
- `$env:CARGO_TARGET_DIR='target-cp054f-green-notification-behavior'; cargo test -p notification-service --test notification_pipeline_test test_request_message_posted_notifications_own_message_notification_defaults --offline`
- `$env:CARGO_TARGET_DIR='target-cp054f-green-local'; cargo test -p local-minimal-node --test lib_structure_test test_local_minimal_node_effects_use_notification_service_message_posted_owner_seam --offline`

### 5.3 Regression

- `rustfmt --edition 2024 services/notification-service/src/lib.rs services/notification-service/tests/lib_structure_test.rs services/notification-service/tests/notification_pipeline_test.rs services/local-minimal-node/src/node/effects.rs services/local-minimal-node/tests/lib_structure_test.rs`
- `rustfmt --edition 2024 --check services/notification-service/src/lib.rs services/notification-service/tests/lib_structure_test.rs services/notification-service/tests/notification_pipeline_test.rs services/local-minimal-node/src/node/effects.rs services/local-minimal-node/tests/lib_structure_test.rs`
- `$env:CARGO_TARGET_DIR='target-cp054f-reg-notification'; cargo test -p notification-service --offline`
- `$env:CARGO_TARGET_DIR='target-cp054f-reg-local'; cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_fanouts_message_notifications_to_other_active_members_only --offline`

## 6. 当前结论

- 本轮是 `CP05-4` 的一个有效 notification owner 增量。
- `local-minimal-node` 已不再本地拼装 `message.posted` notification 默认字段。
- `notification-service` 现在拥有该 request 装配 seam。
- 但 `CP05-4` 仍未闭环，`Step 05` 仍未闭环，`91 / 95 / 97` 仍不能整体判定通过，`Wave B / 93` 仍阻塞。