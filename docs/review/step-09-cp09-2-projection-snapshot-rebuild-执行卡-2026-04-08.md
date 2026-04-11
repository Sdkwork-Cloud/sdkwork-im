# Step 09 / CP09-2 projection snapshot rebuild 执行卡 - 2026-04-08

## 当前上下文
- 当前波次：`Wave C`
- 当前 step：`Step 09`
- 当前子任务：`CP09-2`
- 前置状态：
  - `Step 08` 已于 `2026-04-08` 闭环完成
  - `Step 09 / CP09-1` 已补齐 `local-disk` 的 `MetadataStore` 与 `TimelineProjectionStore`
  - 但 `projection-service` 仍缺少显式 `snapshot / export / import / restore` 语义，导致 `CP09-2` 仍停留在“有 storage port、无可验证 rebuild 路径”的断口

## 本轮为什么做这个子任务
- `docs/review/step-09-cp09-1-local-disk-storage-port-执行卡-2026-04-08.md` 已明确下一轮优先级：
  1. 先把 `projection-service` 的 `snapshot / rebuild` 能力变成真实可验证路径
  2. 再让 `managed runtime-dir` 的恢复链路消费这条路径
- `docs/step/09-存储投影与可观测治理.md` 把 `CP09-2` 定义为“projection service 已能支持重建与恢复”。
- 如果继续只补 store 名称或空抽象，而不让 `projection-service` 真正消费这些端口，`Step 09` 会继续停在伪完成状态。

## 本轮实际完成

### 1. 把 storage port 从“只写”补到“可读回”
- `crates/craw-chat-contract-core/src/lib.rs`
  - `MetadataStore` 新增 `load_snapshot(...)`
- `crates/craw-chat-contract-message/src/lib.rs`
  - `TimelineProjectionStore` 新增 `load_timeline(...)`
- 对应 memory / file adapter 已同步实现：
  - `adapters/local-memory/src/lib.rs`
  - `adapters/local-disk/src/metadata.rs`
  - `adapters/local-disk/src/projection.rs`
- 这使 `CP09-1` 新补出的 storage port 不再只是“能写进去”，而是开始具备被 rebuild / restore 真正消费的基础语义。

### 2. 为 `projection-service` 落地显式 conversation snapshot 持久化 / 恢复
- `services/projection-service/src/snapshot.rs`
  - 新增 `persist_conversation_snapshot(...)`
  - 新增 `restore_conversation_snapshot(...)`
- `services/projection-service/src/model.rs`
  - 为 `TimelineViewEntry / SummarySenderView / ConversationSummaryView` 补齐 `Deserialize`
- `services/projection-service/src/projection.rs`
  - `ProjectionError` 新增：
    - `InvalidSnapshot(...)`
    - `StoreFailure(...)`

### 3. 把 tenant 作用域显式带进 snapshot store key
- snapshot 持久化与恢复统一使用 `scope_key(tenant_id, conversation_id)` 作为底层 store scope。
- 这避免了“同一个 `conversation_id` 在不同 tenant 间写进同一份 projection store 时互相覆盖”的恢复级漂移。

### 4. 用真实红绿测试冻结 restore 语义
- `services/projection-service/tests/projection_snapshot_test.rs`
  - 新增：
    - `test_projection_service_restores_tenant_scoped_conversation_snapshots_from_shared_stores`
- 该测试覆盖：
  - 同一 `conversation_id` 在不同 tenant 下共享同一套 storage port
  - 先导出 snapshot，再在新 `TimelineProjectionService` 中恢复
  - 恢复后 summary / timeline 仍保持 tenant 级隔离

## 改动范围
- 代码：
  - `crates/craw-chat-contract-core/src/lib.rs`
  - `crates/craw-chat-contract-message/src/lib.rs`
  - `adapters/local-memory/src/lib.rs`
  - `adapters/local-disk/src/metadata.rs`
  - `adapters/local-disk/src/projection.rs`
  - `services/projection-service/Cargo.toml`
  - `services/projection-service/src/lib.rs`
  - `services/projection-service/src/model.rs`
  - `services/projection-service/src/projection.rs`
  - `services/projection-service/src/snapshot.rs`
- 测试：
  - `services/projection-service/tests/projection_snapshot_test.rs`
  - `crates/im-platform-contracts/tests/contract_split_smoke_test.rs`
  - `crates/im-platform-contracts/tests/contracts_smoke_test.rs`
- 文档：
  - 本执行卡
  - 本轮质量审计与复盘
  - 本轮架构兑现与回写决议
  - `docs/架构/09-实施计划.md`
  - `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`

## TDD 证据

### Red
- `cargo test -p projection-service --test projection_snapshot_test --offline`
  - 失败点与预期一致：
    - `persist_conversation_snapshot` 不存在
    - `restore_conversation_snapshot` 不存在
  - 说明本轮命中的是真缺口，而不是测试误判

### Green
- `cargo test -p projection-service --test projection_snapshot_test --offline`

## 回归验证
- `cargo test -p projection-service --offline`
- `cargo test -p im-platform-contracts --offline`
- `cargo test -p im-adapters-local-memory --offline`
- `cargo test -p im-adapters-local-disk --offline`
- `cargo fmt --all --check`

## 结论
- 这是 `Wave C / Step 09 / CP09-2` 的第一个真实代码增量。
- `projection-service` 已经开始具备“通过统一 storage port 导出 / 恢复 conversation 级 snapshot”的真实路径。
- 但 `CP09-2` 仍不能整体判定通过，因为当前只覆盖了：
  - conversation summary
  - timeline projection
- 当前 step 还差什么：
  - inbox / member / read-cursor / conversation catalog 的 rebuild 语义
  - `managed runtime-dir` 对 projection snapshot restore 的真实消费
  - `CP09-3` 的 plane-level metrics / tracing / logging
  - `CP09-4` 的 backup / restore / repair / archive 审计闭环

## 下一轮继续做什么
1. 把 `projection-service` 的 snapshot restore 从“summary + timeline 点恢复”扩到 `inbox / member / read-cursor` 所需索引与视图
2. 让 `managed runtime-dir` 恢复链路能消费 projection snapshot/rebuild，而不是只依赖 commit journal replay
3. 在 `CP09-3 / CP09-4` 之前先把 `CP09-2` 的 rebuild coverage 做到可支撑 step 级 recovery 讨论
