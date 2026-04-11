# 持续优化评审记录：UserModuleProvider 主链路接入

- 日期：2026-04-08
- 轮次：持续优化模式，第 19 轮增量
- 对应目标：
  - 将 `UserModuleProvider` 从 provider 契约层推进到消息与成员主链路
  - 保持 `local / external` 两种用户模块形态可插拔
  - 不把 provider 细节下沉到 conversation domain

## 本轮结论

- 已完成：
  - `local-minimal-node` 已支持注入 `Arc<dyn UserModuleProvider>`
  - 默认提供本地实现 `LocalUserModuleProvider`
  - 消息发送入口已先经 `UserModuleProvider` 解析，再组装 `sender`
  - 成员新增入口已先经 `UserModuleProvider` 解析，再把 attributes 注入成员记录
  - 已新增可验证 `local / external` 两种 provider 形态的主链路测试
- 未完成：
  - 会话创建、agent dialog、agent handoff、system channel 的初始成员构建仍未统一经过 `UserModuleProvider`
  - `UserModuleProvider` 目前只完成主链路注入与默认本地实现，尚未对接真实外部目录系统

## 实际落地

- 运行时装配：
  - `services/local-minimal-node/src/node/build.rs`
    - 新增带 `user_module_provider` 的 app 构建路径
    - 默认装配 `LocalUserModuleProvider`
  - `services/local-minimal-node/src/node.rs`
    - `AppState` 新增 `user_module_provider`
    - `ApiError` 新增 provider 合同错误映射
- Provider 与主链路接入：
  - `services/local-minimal-node/src/node/user_module.rs`
    - 新增默认本地 provider
    - 新增 `sender` 解析与成员 principal/attributes 解析
  - `services/local-minimal-node/src/node/effects.rs`
    - `post_message` / `publish_system_channel_message` 不再直接复用裸 `AuthContext` sender
    - 改为先通过 `UserModuleProvider` 富化 `sender.metadata`
  - `services/local-minimal-node/src/node/membership.rs`
    - `add_member` 在 user principal 场景下先经 provider 解析
- conversation-runtime 最小扩展：
  - `services/conversation-runtime/src/runtime/membership.rs`
    - `add_member_from_auth_context` 新增 attributes 入参
    - 运行时内部使用 `build_conversation_member_with_attributes(...)`
  - `services/conversation-runtime/src/runtime/http.rs`
    - HTTP surface 默认仍走 auth-context 入口，只是补齐空 attributes

## 验证结果

- 通过：
  - `cargo fmt --all --check`
  - `cargo test -p conversation-runtime --offline -- --nocapture`
  - `cargo test -p local-minimal-node --offline --test lib_structure_test -- --nocapture`
  - `cargo test -p local-minimal-node --offline --test provider_plugin_docs_test -- --nocapture`
  - `cargo test -p local-minimal-node --offline --test user_module_provider_mainline_test -- --nocapture`
  - `cargo test -p local-minimal-node --offline test_local_minimal_profile_runs_end_to_end_flow -- --nocapture`
  - `cargo test -p local-minimal-node --offline test_local_minimal_profile_exposes_conversation_member_management -- --nocapture`

## 风险与余项

- 当前 `sender` 富化只覆盖消息发送与系统频道发布；消息编辑、撤回仍沿用 runtime 的 auth snapshot。
- 当前成员 attributes 富化只覆盖显式 add-member 路径；创建会话时的 owner/bootstrap members 仍未统一收口。
- 默认本地 provider 目前是轻量内存实现，满足主链路和插件注入验证，但不等于最终用户维护系统。

## 下一轮建议

1. 优先把 `UserModuleProvider` 继续推进到 conversation create / agent dialog / handoff / system channel 的 bootstrap member 构建。
2. 在 `projection-service` 或控制面增加 provider health / selected plugin 的可见性，避免运行时排障只能查 commit journal。
3. 完成 `rtc-volcengine`、`object-storage-s3`、`iot-mqtt` 的最小 runtime adapter，实现 provider 契约到真实运行链路的下一层闭环。
