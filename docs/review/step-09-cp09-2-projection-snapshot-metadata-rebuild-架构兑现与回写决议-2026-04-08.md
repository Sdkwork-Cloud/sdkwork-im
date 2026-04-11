# Step 09 / CP09-2 projection snapshot metadata rebuild 架构兑现与回写决议 - 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`
- `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md`

## 本轮已兑现能力
- `09`
  - `Wave C / Step 09 / CP09-2` 已把 projection snapshot restore 从“只恢复 `summary + timeline`”推进到“可恢复完整 conversation read-side metadata”
  - `inbox` 虽然不是独立存储，但其依赖的：
    - `conversation catalog`
    - `members`
    - `read_cursors`
    已全部进入统一 snapshot/export/import/restore 路径
- `132`
  - 存储抽象的消费面进一步从“能导出部分视图”推进到“能恢复 read-side rebuild 所需 metadata”
  - `projection-service` 没有绕开统一 store 抽象去依赖某个具体 adapter，而是继续消费同一条 `MetadataStore + TimelineProjectionStore` seam

## 本轮未兑现能力
- `140`
  - snapshot persist / restore 仍没有 metrics / tracing / structured logging
- `141`
  - retention / archive / lifecycle / repair 治理仍未触达
- `Step 09`
  - `managed runtime-dir` 尚未真实消费 projection snapshot restore
  - sync-plane 的 `registered_devices / device_sync_feeds` 还未确认是否需要进入同一条恢复合同

## 是否偏离架构
- 无偏离。
- 本轮实现继续遵守 `132` 的路线：
  - 先扩展统一 snapshot 合同
  - 再让 projection read-side 真实消费它
  - 不把恢复逻辑绑死在本地文件布局或单一 runtime builder 上

## 回写决议
- `docs/架构/09-实施计划.md`
  - 追加 `As-Built 85`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
  - 追加 `As-Built 3`
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`
  - 本轮仅复核，不追加回写，等待 `CP09-3` 证据
- `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md`
  - 本轮仅复核，不追加回写，等待 `CP09-4` 证据

## 证据
- 代码：
  - `services/projection-service/src/model.rs`
  - `services/projection-service/src/snapshot.rs`
- 测试：
  - `services/projection-service/tests/projection_snapshot_test.rs`
- 验证：
  - `cargo test -p projection-service --test projection_snapshot_test --offline`
  - `cargo test -p projection-service --offline`
  - `cargo fmt --all --check`

## 当前判断
- 这是 `CP09-2` 的第二个真实增量，不是 `Step 09` 的整步通过
- `CP09-2`：继续推进中，离 runtime-dir 恢复消费还差一段
- `Step 09`：未闭环
- `Wave C / 93`：继续阻塞于 `Step 09`
