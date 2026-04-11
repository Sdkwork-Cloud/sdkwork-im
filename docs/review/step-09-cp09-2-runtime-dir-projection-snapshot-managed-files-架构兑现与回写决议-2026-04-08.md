# Step 09 / CP09-2 runtime-dir projection snapshot managed files 架构兑现与回写决议 - 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`

## 本轮已兑现能力
- `09-实施计划`
  - `Wave C / Step 09 / CP09-2` 已把 projection snapshot 文件正式纳入 runtime-dir 运维工具链的受管状态集合。
  - 这让 `inspection / backup catalog / restore preview / restore / repair` 与启动恢复消费同一组 projection snapshot 文件。
- `132`
  - file-backed `MetadataStore + TimelineProjectionStore` 不再只支持 snapshot 落盘与启动恢复，还开始承担运维侧 inspection / backup / restore 的统一受管合同。
- `138`
  - `Local Minimal` profile 的灾备恢复证据从“启动时能恢复 projection read-side”推进到“运维工具链也能管理 projection snapshot 文件差异与恢复”。

## 本轮未兑现能力
- `138`
  - `registered_devices / device_sync_feeds / device_sync_sequences` 仍未确认是否进入同一恢复合同。
- `140`
  - projection snapshot persist / restore 仍未形成 metrics / tracing / structured logging 闭环。
- `141`
  - archive / retention / lifecycle 治理未被本轮触达。

## 是否偏离架构
- 无偏离。
- 本轮实现继续沿用统一的 file-backed storage seam，没有为 runtime-dir 工具链另造一套旁路格式或恢复机制。

## 回写决议
- `docs/架构/09-实施计划.md`
  - 追加 `As-Built 87`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
  - 追加 `As-Built 5`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
  - 追加 `As-Built 2`

## 证据
- 代码：
  - `services/local-minimal-node/src/node/runtime_dir.rs`
- 测试：
  - `services/local-minimal-node/tests/runtime_dir_inspection_test.rs`
  - `services/local-minimal-node/tests/runtime_dir_backup_catalog_test.rs`
  - `services/local-minimal-node/tests/runtime_dir_repair_test.rs`
  - `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`
  - `services/local-minimal-node/tests/runtime_dir_restore_test.rs`
- 验证：
  - `cargo test -p local-minimal-node --offline --test runtime_dir_inspection_test --test runtime_dir_backup_catalog_test --test runtime_dir_repair_test --test runtime_dir_restore_test --test runtime_dir_restore_preview_test`
  - `cargo test -p local-minimal-node --offline`
  - `cargo fmt --all --check`

## 当前判断
- 这是 `CP09-2` 的第四个真实代码增量，不是 `Step 09` 的整步通过。
- `CP09-2` 已明显增强，但 `Step 09` 仍未闭环。
- `Wave C / 93` 继续阻塞于 `Step 09`。
