# Step 09 / CP09-2 runtime-dir projection snapshot recovery 架构兑现与回写决议 - 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`

## 本轮已兑现能力
- `09`
  - `Wave C / Step 09 / CP09-2` 已把 projection snapshot 恢复从 service 内部能力推进到 managed runtime-dir 启动恢复路径
  - `local-minimal-node` 在 file-backed runtime-dir 下已真实生成 projection snapshot 文件，并在重启时优先消费它们恢复 read-side
- `132`
  - file-backed metadata store 已不再只是 snapshot KV 落盘器，而开始承担 runtime-dir scope 发现与恢复入口职责
  - `ProjectionJournal + ProjectionSnapshotStores` 已证明统一 store seam 可以同时支撑写时快照与读时恢复
- `138`
  - 本地受管恢复路径已出现第一条“metadata snapshot -> projection query restore”的真实证据
  - 即使 `commit-journal.json` 被置空，runtime-dir 仍可恢复 `inbox / summary / timeline / read-cursor`

## 本轮未兑现能力
- `138`
  - `backup / restore / preview / repair` 工具链尚未把新 snapshot 文件纳入受管清单
- `140`
  - snapshot persist / restore 仍没有 metrics / tracing / structured logging
- `Step 09`
  - `conversation_runtime` 仍不能在 journal 丢失时恢复 domain owner 状态
  - `registered_devices / device_sync_feeds` 仍未进入同一条 runtime-dir 恢复合同

## 是否偏离架构
- 无偏离。
- 本轮实现继续遵守 `132 / 138` 的路线：
  - 先让恢复路径真实消费 metadata snapshot
  - 再逐步把备份、恢复、修复和观测补齐
  - 没有直接把 read-side 恢复重新绑回某个单体内存对象

## 回写决议
- `docs/架构/09-实施计划.md`
  - 追加 `As-Built 86`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
  - 追加 `As-Built 4`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
  - 追加 `As-Built 1`
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`
  - 本轮仅复核，不追加回写，等待 `CP09-3` 证据

## 证据
- 代码：
  - `adapters/local-disk/src/metadata.rs`
  - `services/local-minimal-node/src/node.rs`
  - `services/local-minimal-node/src/node/build.rs`
  - `services/local-minimal-node/src/node/projection.rs`
- 测试：
  - `services/local-minimal-node/tests/domain_recovery_persistence_test.rs`
- 验证：
  - `cargo test -p local-minimal-node --test domain_recovery_persistence_test test_default_local_minimal_profile_restores_projection_queries_from_runtime_dir_snapshots_when_commit_journal_is_missing --offline`
  - `cargo test -p local-minimal-node --offline`
  - `cargo test -p projection-service --offline`
  - `cargo test -p im-adapters-local-disk --offline`
  - `cargo fmt --all --check`

## 当前判断
- 这是 `CP09-2` 的第三个真实增量，不是 `Step 09` 的整步通过
- `CP09-2`：现在已经具备 managed runtime-dir 的第一条 snapshot recovery consumer path
- `Step 09`：未闭环
- `Wave C / 93`：继续阻塞于 `Step 09`
