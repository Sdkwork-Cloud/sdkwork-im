# 持续优化评审记录：PrincipalProfileProvider 消息 mutation actor 收口

- 日期：2026-04-08
- 轮次：持续优化模式，第 21 轮增量
- 当前 step / 波次：
  - Step 05
  - `CP05-2` 收口轮
- 本轮为什么做：
  - 上一轮已把 `PrincipalProfileProvider` 接入 bootstrap member
  - 当前剩余最高价值缺口集中在 `edit_message / recall_message`
  - 若 mutation actor 仍沿用裸 `AuthContext` sender snapshot，则 `sender / actor / principal-profile` 的 authority 边界仍不完整

## 本轮结论

- 已完成：
  - `edit_message` 已先经 `PrincipalProfileProvider` 富化 `editor`
  - `recall_message` 已先经 `PrincipalProfileProvider` 富化 `recalled_by`
  - `message.edited` 与 `message.recalled` 的 commit payload 已携带 provider metadata
  - `principal-profile-upstream-context / principal-profile-external-catalog` 两种形态都已补齐 mutation actor 自动化验证
- 当前判断：
  - `PrincipalProfileProvider` 已覆盖：
    - bootstrap member
    - add-member
    - post_message
    - publish_system_channel_message
    - edit_message
    - recall_message
  - 以当前仓库真实状态看，Step 05 中 `CP05-2` 的 `sender / auth-context / principal-profile` 主链路收口已可判定通过
- 未完成：
  - Step 05 整体是否闭环仍取决于更大范围的 `CP05-4 / 93` 总体验收，不因本轮自动等于 Step 05 全量通过
  - provider/plugin 下一阶段的真实 adapter 仍未开始：`rtc-volcengine / object-storage-s3 / iot-mqtt`

## 实际落地

- 入口修复：
  - `services/local-minimal-node/src/node/message.rs`
    - `EditMessageCommand::from_auth_context(...)` 之后，改为用 `PrincipalProfileProvider` 富化 `command.editor`
    - `RecallMessageCommand::from_auth_context(...)` 之后，改为用 `PrincipalProfileProvider` 富化 `command.recalled_by`
  - 保持：
    - 仍由 runtime command constructor 捕获 authority snapshot
    - 不新增 service-edge 私有 sender builder
    - 不把 provider 细节下沉到 conversation domain
- 测试补齐：
  - `services/local-minimal-node/tests/principal_profile_provider_mainline_test.rs`
    - 新增 `local` 场景：验证 `message.edited / message.recalled`
    - 新增 `external` 场景：验证 `message.edited / message.recalled`
    - 断言来源为 `commit-journal.json` 中的真实事件 payload，而不是仅看 HTTP 返回

## 改动文件

- `services/local-minimal-node/src/node/message.rs`
- `services/local-minimal-node/tests/principal_profile_provider_mainline_test.rs`

## 验证结果

- 先红后绿：
  - `cargo test -p local-minimal-node --offline --test principal_profile_provider_mainline_test message_edit_and_recall_actor_metadata -- --nocapture`
    - 红灯阶段：2 个测试失败，`editor / recalledBy.metadata.displayName` 为 `Null`
    - 绿灯阶段：2 个测试通过
- 通过：
  - `cargo test -p local-minimal-node --offline --test lib_structure_test -- --nocapture`
  - `cargo test -p local-minimal-node --offline --test principal_profile_provider_mainline_test -- --nocapture`
  - `cargo test -p local-minimal-node --offline test_local_minimal_profile_edits_and_recalls_message_with_sync_feed_projection -- --nocapture`

## 当前还差什么

- Step 05 仍需结合 `CP05-4 / 93` 的整体证据判断是否可以出 step
- provider/plugin 体系的下一条真实代码主线还未启动：
  - RTC provider adapter
  - Object Storage S3 adapter
  - IoT MQTT adapter

## 下一轮建议

1. 先复核 Step 05 当前总体验收状态，确认是否还存在 `CP05-4 / 93` 阻塞项。
2. 若 Step 05 无新的高优先级缺口，按既定架构顺序进入 `rtc-volcengine` 的最小 runtime adapter。
3. 继续保持每轮必须同步交付测试、`docs/review`、`docs/step`、`docs/架构` 回写。
