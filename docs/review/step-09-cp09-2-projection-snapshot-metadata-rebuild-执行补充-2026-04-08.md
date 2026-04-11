# Step 09 / CP09-2 projection snapshot metadata rebuild 执行补充 - 2026-04-08

## 当前上下文
- 当前波次：`Wave C`
- 当前 step：`Step 09`
- 当前子任务：`CP09-2`
- 前置状态：
  - `As-Built 84` 已把 `projection-service` 的 snapshot/export/restore 从零推进到 `summary + timeline`
  - 但恢复后的 `member snapshot / read cursor / inbox / conversation catalog` 仍然缺失
  - 这意味着 `CP09-2` 还不能支持真实的 read-side rebuild 讨论

## 本轮为什么继续做这个增量
- `docs/review/step-09-cp09-2-projection-snapshot-rebuild-执行卡-2026-04-08.md` 已明确：
  - `inbox / member / read-cursor / conversation catalog` 仍未进入统一 snapshot rebuild 语义
- 当前 `projection-service` 的 `inbox(...)` 并不是独立存储，而是依赖：
  - `summaries`
  - `members`
  - `read_cursors`
  - `conversations`
- 如果快照只恢复 `summary + timeline`，那么 restore 完成后：
  - `member_snapshot(...)` 为空
  - `read_cursor(...)` 为空
  - `inbox(...)` 不是空，就是退化成缺少 `conversation_type` 的伪结果

## 本轮实际完成

### 1. 把 conversation metadata rebuild 纳入同一条 snapshot 路径
- `services/projection-service/src/snapshot.rs`
  - `persist_conversation_snapshot(...)` 现在除了 `summary + timeline`，还会持久化：
    - `conversation catalog`
    - `members`
    - `read_cursors`
  - `restore_conversation_snapshot(...)` 现在会把这些 metadata 重新装回：
    - `self.conversations`
    - `self.members`
    - `self.read_cursors`
- 结果是：
  - `member_snapshot(...)` 可恢复
  - `read_cursor(...)` 可恢复
  - `inbox(...)` 所需的派生输入已可恢复

### 2. 让 conversation catalog 成为可序列化的 snapshot 输入
- `services/projection-service/src/model.rs`
  - `ConversationCatalogEntry` 补齐 `Serialize / Deserialize`
- 这让 `conversation.created` 形成的 catalog 信息可以进入 snapshot/export/import/restore 流程

### 3. 用红绿测试冻结 member/cursor/inbox rebuild 语义
- `services/projection-service/tests/projection_snapshot_test.rs`
  - 新增：
    - `test_projection_service_restores_member_cursor_and_inbox_views_from_snapshot_metadata`
- 该测试覆盖：
  - `conversation.created`
  - `conversation.member_joined`
  - `message.posted`
  - `conversation.read_cursor_updated`
  - 导出 snapshot 后，在新的 `TimelineProjectionService` 中恢复
  - 恢复后：
    - `member_snapshot(...)` 存在
    - `read_cursor(...)` 存在且 unread 计算正确
    - `inbox(...)` 能恢复 `conversation_type / last_summary / unread_count`

## 改动范围
- 代码：
  - `services/projection-service/src/model.rs`
  - `services/projection-service/src/snapshot.rs`
- 测试：
  - `services/projection-service/tests/projection_snapshot_test.rs`
- 文档：
  - 本执行补充
  - 本轮质量审计与复盘
  - 本轮架构兑现与回写决议
  - `docs/架构/09-实施计划.md`
  - `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`

## TDD 证据

### Red
- `cargo test -p projection-service --test projection_snapshot_test --offline`
  - 失败点与预期一致：
    - `test_projection_service_restores_member_cursor_and_inbox_views_from_snapshot_metadata`
    - 断言失败：`member should restore`
  - 说明本轮命中的是真缺口，而不是测试误写

### Green
- `cargo test -p projection-service --test projection_snapshot_test --offline`

## 回归验证
- `cargo test -p projection-service --offline`
- `cargo fmt --all --check`

## 结论
- 这是 `Wave C / Step 09 / CP09-2` 的第二个真实代码增量。
- `projection-service` 的 snapshot restore 已从“只恢复 `summary + timeline`”推进到：
  - `summary`
  - `timeline`
  - `conversation catalog`
  - `member snapshot`
  - `read cursor`
  - 以及由这些状态派生出的 `inbox`
- 当前 step 还差什么：
  - `managed runtime-dir` 对 projection snapshot restore 的真实消费
  - `CP09-3` 的 metrics / tracing / logging
  - `CP09-4` 的 backup / restore / repair / archive 审计闭环

## 下一轮继续做什么
1. 让 `local-minimal-node` 的 `managed runtime-dir` 恢复链路消费 projection snapshot/rebuild，而不是只靠 runtime state 文件与 journal replay
2. 评估 `registered_devices / device_sync_feeds` 是否需要进入 `CP09-2` 的恢复语义，避免恢复后 read-plane 与 sync-plane 脱节
3. 在 `CP09-3` 前补 plane-level snapshot persist/restore 观测点
