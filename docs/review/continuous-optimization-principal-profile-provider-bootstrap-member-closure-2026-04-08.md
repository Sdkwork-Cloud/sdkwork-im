# 持续优化评审记录：PrincipalProfileProvider bootstrap member 收口

- 日期：2026-04-08
- 轮次：持续优化模式，第 20 轮增量
- 当前 step / 波次：
  - Step 05
  - `CP05-2` 第二阶段补强
- 本轮为什么做：
  - 上一轮已把 `PrincipalProfileProvider` 接入消息 sender 与显式 `add-member`
  - 但 conversation create / agent dialog / system channel / handoff 的 bootstrap member 仍未统一走 provider
  - 这会导致 `local / external` 两种用户模块形态在“创建即入群”的主链路上出现 authority metadata 缺口

## 本轮结论

- 已完成：
  - `create_conversation` 的 owner member 已支持注入 provider 富化 attributes
  - `create_agent_dialog` 的 requester member 已支持注入 provider 富化 attributes
  - `create_system_channel` 的 subscriber member 已支持注入 provider 富化 attributes
  - `create_agent_handoff` 的 user target member 已支持注入 provider 富化 attributes
  - `local / external` 两种用户模块形态都已通过 bootstrap member 自动化验证
  - `local-minimal-node` 仍只通过 runtime 的 auth-context 入口进入 conversation runtime，只是入口升级为可承载 attributes
- 未完成：
  - `edit_message / recall_message` 的 actor metadata 仍沿用 runtime auth snapshot，尚未统一进入 `PrincipalProfileProvider`
  - provider-aware create entrypoint 当前主要由 `local-minimal-node` 消费，`conversation-runtime` 独立 HTTP surface 仍保持 provider-agnostic

## 实际落地

- 运行时扩展：
  - `services/conversation-runtime/src/runtime/creation.rs`
    - 新增 provider-aware auth-context create 入口：
      - `create_conversation_from_auth_context_with_creator_attributes(...)`
      - `create_agent_dialog_from_auth_context_with_requester_attributes(...)`
      - `create_system_channel_from_auth_context_with_subscriber_attributes(...)`
      - `create_agent_handoff_from_auth_context_with_target_attributes(...)`
    - 保留原有 auth-context create 入口与 `*with_*kind` 入口，避免破坏既有 runtime 结构契约
    - bootstrap member 构建改为按需使用 `build_conversation_member_with_attributes(...)`
- 本地节点接入：
  - `services/local-minimal-node/src/node/conversation.rs`
    - create conversation / agent dialog / system channel / handoff 在进入 runtime 前先解析 provider attributes
    - handoff target 若为 `user`，统一经 `PrincipalProfileProvider` 富化后再进入 runtime
- 测试与结构契约：
  - `services/local-minimal-node/tests/principal_profile_provider_mainline_test.rs`
    - 新增 bootstrap member 覆盖：
      - group owner
      - agent dialog requester
      - system channel subscriber
      - handoff user target
    - `local / external` 两种 provider 形态均已覆盖
  - `services/local-minimal-node/tests/lib_structure_test.rs`
    - 结构门禁更新为要求 `auth-context-with-attributes` runtime entrypoint
    - 保持“authority capture 仍归 runtime auth-context 边界所有”这一约束不变

## 改动文件

- `services/conversation-runtime/src/runtime.rs`
- `services/conversation-runtime/src/runtime/creation.rs`
- `services/local-minimal-node/src/node/conversation.rs`
- `services/local-minimal-node/tests/lib_structure_test.rs`
- `services/local-minimal-node/tests/principal_profile_provider_mainline_test.rs`

## 验证结果

- 通过：
  - `cargo fmt --all --check`
  - `cargo test -p conversation-runtime --offline -- --nocapture`
  - `cargo test -p local-minimal-node --offline --test lib_structure_test -- --nocapture`
  - `cargo test -p local-minimal-node --offline --test provider_plugin_docs_test -- --nocapture`
  - `cargo test -p local-minimal-node --offline --test principal_profile_provider_mainline_test -- --nocapture`
  - `cargo test -p local-minimal-node --offline test_local_minimal_profile_runs_end_to_end_flow -- --nocapture`
  - `cargo test -p local-minimal-node --offline test_local_minimal_profile_exposes_conversation_member_management -- --nocapture`

## 当前判断

- `PrincipalProfileProvider` 对 conversation/member 主链路的覆盖已经从：
  - “message sender + add-member”
- 推进到：
  - “message sender + bootstrap members + add-member”
- `CP05-2` 的成员侧收口已基本完成，但消息 mutation actor 侧仍有残留，因此本 step 还不能按最终闭环判定。

## 下一轮建议

1. 优先把 `edit_message / recall_message` 的 actor enrichment 统一接入 `PrincipalProfileProvider`，补完 Step 05 在消息 mutation 侧的 authority metadata。
2. 若消息 mutation actor 也收口完成，再复核 `CP05-2` 是否可以判定最终通过，并决定是否转入 Step 06 / RTC provider 实现。
3. 在进入 `rtc-volcengine / object-storage-s3 / iot-mqtt` 真实 adapter 之前，保持 `docs/review`、`docs/step`、`docs/架构` 同步回写。
