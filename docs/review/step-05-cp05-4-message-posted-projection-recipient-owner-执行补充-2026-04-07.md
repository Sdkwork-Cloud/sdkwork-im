# Step 05 / CP05-4 message-posted projection recipient owner 执行补充 - 2026-04-07

## 1. 当前上下文
- 波次：`Wave B`
- Step：`Step 05`
- 当前子项：`CP05-4`
- 本轮目标：把 message-posted notification 的 recipient 解析从 `local-minimal-node` side-effect 路径继续收口到 `notification-service` owner seam，并要求该 owner seam 通过共享的 `projection-service` auth-context seam 自行解析当前 active recipients。

## 2. 本轮实际落地

- `services/notification-service/Cargo.toml`
  - 新增 `projection-service` 依赖，用于消费 projection auth-context owner seam。
- `services/notification-service/src/lib.rs`
  - `RequestMessagePostedNotifications` 删除 `recipient_ids` 输入。
  - `NotificationRuntime` 新增共享 `projection_service` 依赖。
  - 新增 `with_journal_and_projection(...)`、`with_journal_and_store_and_projection(...)`。
  - `request_message_posted_notifications(...)` 改为在 owner 边界内部调用 `active_conversation_principal_ids_from_auth_context(...)` 解析 recipients，再进入既有 `request_notification_fanout(...)`。
- `services/local-minimal-node/src/node/build.rs`
  - 本地运行时与依赖注入路径改为给 `NotificationRuntime` 传入共享的 `projection_service`，避免 notification owner 与 projection owner 状态漂移。
- `services/local-minimal-node/src/node/effects.rs`
  - `fanout_message_notifications(...)` 不再向 `RequestMessagePostedNotifications` 透传 `recipient_ids`。
- `services/notification-service/tests/lib_structure_test.rs`
  - 锁定 `request_message_posted_notifications(...)` 必须命中 projection auth-context seam，且不能继续透传 `recipient_ids`。
- `services/notification-service/tests/notification_pipeline_test.rs`
  - 新增行为测试，验证 runtime 会基于 projection 中当前 active members 解析 recipients，并自动排除当前 actor 与已移除成员。
- `services/local-minimal-node/tests/lib_structure_test.rs`
  - 新增结构断言，锁定 `effects.rs` 不得继续线程化 `recipient_ids`。

## 3. 本轮验证

- Red
  - `cargo test -p notification-service --test lib_structure_test test_notification_runtime_exposes_message_posted_notification_owner_seam --offline`
  - `cargo test -p notification-service --test notification_pipeline_test test_request_message_posted_notifications_resolves_current_active_recipients_from_projection_auth_context --offline`
    - 当时失败点是真实缺口：`projection_service` 依赖不存在、`NotificationRuntime::with_journal_and_projection(...)` 不存在、`RequestMessagePostedNotifications` 仍强制要求 `recipient_ids`。
- Green / Regression
  - `cargo test -p notification-service --test lib_structure_test test_notification_runtime_exposes_message_posted_notification_owner_seam --offline --target-dir target-cp054k-green-notification-structure`
  - `cargo test -p notification-service --test notification_pipeline_test test_request_message_posted_notifications_resolves_current_active_recipients_from_projection_auth_context --offline --target-dir target-cp054k-green-notification-behavior`
  - `cargo test -p local-minimal-node --test lib_structure_test test_local_minimal_node_effects_do_not_thread_message_posted_recipient_ids --offline --target-dir target-cp054k-green-local-structure`
  - `cargo test -p notification-service --offline --target-dir target-cp054k-reg-notification-full`
  - `cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_fanouts_message_notifications_to_other_active_members_only --offline --target-dir target-cp054k-reg-message-notification`
  - `cargo test -p local-minimal-node --test lib_structure_test --offline --target-dir target-cp054k-reg-local-structure-full`
  - `rustfmt --edition 2024 --check services/notification-service/src/lib.rs services/notification-service/tests/lib_structure_test.rs services/notification-service/tests/notification_pipeline_test.rs services/local-minimal-node/src/node/build.rs services/local-minimal-node/src/node/effects.rs services/local-minimal-node/tests/lib_structure_test.rs`

## 4. 本轮结论

- `notification-service` 现在不仅拥有 message-posted notification 的默认字段装配，还开始拥有 recipient authority 的实际解析。
- `local-minimal-node` 的 message side-effect 进一步退化为只提供 `AuthContext + conversationId + message metadata`。
- 本轮是 `CP05-4` 的有效增量，但不构成 `CP05-4` 整体闭环。
- 当前仍未完成：
  - `CP05-4`
  - `Step 05`
  - `91 / 95 / 97` 针对 `Step 05` 的整体通过
  - `Wave B / 93`
- 基于当前代码搜索，剩余主 seam 已进一步收敛到 multi-device sync final closure；可见的下一处高概率 blocker 是 `services/session-gateway/src/lib.rs` 里的 `resume / get_presence_me / heartbeat / disconnect` 路径仍在本地读取 `registered_devices(...)` 与 `latest_device_sync_seq(...)`。
