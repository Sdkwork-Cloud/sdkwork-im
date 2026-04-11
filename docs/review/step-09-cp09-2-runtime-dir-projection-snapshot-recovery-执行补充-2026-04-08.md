# Step 09 / CP09-2 runtime-dir projection snapshot recovery 执行补充 - 2026-04-08

## 当前上下文
- 当前波次：`Wave C`
- 当前 step：`Step 09`
- 当前子任务：`CP09-2`
- 前置状态：
  - `As-Built 84` 已让 `projection-service` 具备 `summary + timeline` snapshot/export/restore
  - `As-Built 85` 已把 `conversation catalog / members / read_cursors` 纳入同一条 snapshot rebuild 语义
  - 但 `local-minimal-node` 的 `managed runtime-dir` 启动恢复仍然只消费 `commit-journal replay`

## 本轮为什么继续做这个增量
- `docs/review/step-09-cp09-2-projection-snapshot-rebuild-执行卡-2026-04-08.md` 已明确下一阻塞点：
  - 让 `managed runtime-dir` 恢复链路真实消费 projection snapshot/rebuild
- 如果 runtime-dir 重启时仍然只能靠 `commit-journal` 回放，那么 `CP09-2` 仍然只是 `projection-service` 内部能力，而不是本地受管恢复链路的真实能力。

## 本轮实际完成

### 1. 让 managed runtime-dir 在会话运行时持续写出 projection snapshot 文件
- `services/local-minimal-node/src/node.rs`
  - `ProjectionJournal` 新增可选 `ProjectionSnapshotStores`
  - file-backed journal append 在 conversation scope 事件投影完成后，会把当前 conversation snapshot 持久化到：
    - `state/projection-metadata.json`
    - `state/projection-timeline.json`
  - 同时为每个 conversation scope 写入 `conversation-snapshot-checkpoint`

### 2. 让 startup recovery 能从 snapshot 自身发现可恢复 scope，而不是只依赖 journal
- `adapters/local-disk/src/metadata.rs`
  - `FileMetadataStore` 新增 `scopes_for_key(...)`
- `services/local-minimal-node/src/node.rs`
  - runtime-dir 启动恢复现在会：
    - 从 `commit-journal` 收集 conversation scope
    - 再从 `projection-metadata.json` 中按 checkpoint key 反查可恢复 scope
  - 这让 “journal 已空，但 snapshot 仍在” 的场景不再丢失恢复入口

### 3. 让 projection read query 在恢复后直接消费 projection-owned access seam
- `services/local-minimal-node/src/node/projection.rs`
  - `get_conversation_summary(...)`
  - `get_timeline(...)`
  - `get_read_cursor(...)`
  - 不再先走 `conversation_runtime.require_active_member_from_auth_context(...)`
  - 而是直接消费 `projection-service` 自己的 auth-context access seam
- 这让 runtime-dir 从 snapshot 恢复出的 read-side 状态可以真正被对外查询使用

### 4. 用真实红绿测试冻结“journal 丢失时仍可从 runtime-dir snapshot 恢复 read-side”语义
- `services/local-minimal-node/tests/domain_recovery_persistence_test.rs`
  - 新增：
    - `test_default_local_minimal_profile_restores_projection_queries_from_runtime_dir_snapshots_when_commit_journal_is_missing`
- 该测试覆盖：
  - 在受管 runtime-dir 下创建 conversation、投递 message、更新 read cursor
  - 验证 snapshot 文件真实落盘
  - 把 `commit-journal.json` 直接替换成空数组
  - 重启 `local-minimal-node`
  - 验证：
    - `/api/v1/inbox`
    - `/api/v1/conversations/{id}`
    - `/api/v1/conversations/{id}/messages`
    - `/api/v1/conversations/{id}/read-cursor`
    仍可从 snapshot 恢复

## 改动范围
- 代码：
  - `adapters/local-disk/src/metadata.rs`
  - `services/local-minimal-node/src/node.rs`
  - `services/local-minimal-node/src/node/build.rs`
  - `services/local-minimal-node/src/node/projection.rs`
- 测试：
  - `services/local-minimal-node/tests/domain_recovery_persistence_test.rs`
- 文档：
  - 本执行补充
  - 本轮质量审计与复盘
  - 本轮架构兑现与回写决议
  - `docs/架构/09-实施计划.md`
  - `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
  - `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`

## TDD 证据

### Red
- `cargo test -p local-minimal-node --test domain_recovery_persistence_test test_default_local_minimal_profile_restores_projection_queries_from_runtime_dir_snapshots_when_commit_journal_is_missing --offline`
  - 失败点与预期一致：
    - `managed runtime dir should persist projection metadata snapshots`
  - 说明 runtime-dir 当时还没有真实消费 projection snapshot 路径

### Green
- `cargo test -p local-minimal-node --test domain_recovery_persistence_test test_default_local_minimal_profile_restores_projection_queries_from_runtime_dir_snapshots_when_commit_journal_is_missing --offline`

## 回归验证
- `cargo test -p local-minimal-node --offline`
- `cargo test -p projection-service --offline`
- `cargo test -p im-adapters-local-disk --offline`
- `cargo fmt --all --check`

## 结论
- 这是 `Wave C / Step 09 / CP09-2` 的第三个真实代码增量。
- `managed runtime-dir` 启动恢复现在已不再完全依赖 `commit-journal replay`：
  - read-side projection query 已能真实消费 conversation snapshot restore
  - 即使 `commit-journal.json` 被清空，受管 runtime-dir 仍可恢复 `inbox / summary / timeline / read-cursor`
- 当前 step 还差什么：
  - runtime-dir `backup / restore / preview / repair` 仍未把新 snapshot 文件纳入同一套 managed 文件清单
  - `registered_devices / device_sync_feeds` 仍未确认是否进入同一条 snapshot 恢复合同
  - `CP09-3` 的 metrics / tracing / logging 仍未开始闭环
  - `CP09-4` 的 backup / restore / repair / archive 审计闭环仍未完成

## 下一轮继续做什么
1. 把 `projection-metadata.json / projection-timeline.json` 纳入 runtime-dir `inspection / backup catalog / preview restore / restore / repair` 的受管文件集合
2. 决定 `registered_devices / device_sync_feeds / device_sync_sequences` 是否也要进入 `CP09-2` 的 runtime-dir 恢复合同
3. 开始 `CP09-3` 的 plane-level metrics / tracing / logging
