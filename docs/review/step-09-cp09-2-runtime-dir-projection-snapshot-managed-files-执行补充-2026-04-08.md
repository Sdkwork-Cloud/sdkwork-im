# Step 09 / CP09-2 runtime-dir projection snapshot managed files 执行补充 - 2026-04-08

## 当前上下文
- 当前波次：`Wave C`
- 当前 step：`Step 09`
- 当前检查点：`CP09-2`
- 前置状态：
  - `As-Built 84` 已让 `projection-service` 具备 `summary + timeline` snapshot/export/restore
  - `As-Built 85` 已把 `conversation catalog / members / read_cursors` 纳入同一条 snapshot rebuild 语义
  - `As-Built 86` 已让 `local-minimal-node` 在启动恢复时消费 `projection-metadata.json / projection-timeline.json`
  - 但 `runtime-dir inspection / backup catalog / restore preview / restore / repair` 仍只管理旧的 9 个状态文件

## 本轮为什么继续做这个增量
- 如果 projection snapshot 文件不进入 runtime-dir 受管文件合同，`CP09-2` 的恢复语义就只在启动路径成立，不在运维工具链成立。
- `Step 09` 明确要求 backup / restore / repair 形成最小可运行闭环，因此本轮最优决策不是跳去 `CP09-3`，而是先补齐 runtime-dir managed file contract。

## 本轮实际完成

### 1. runtime-dir 受管状态文件集合扩展到 11 个文件
- `services/local-minimal-node/src/node/runtime_dir.rs`
  - `EXPECTED_RUNTIME_STATE_FILES` 从 `9` 扩展到 `11`
  - 新增：
    - `projection-metadata.json`
    - `projection-timeline.json`

### 2. inspection 现在会校验 projection snapshot 文件
- `services/local-minimal-node/src/node/runtime_dir.rs`
  - `validate_runtime_state_file(...)` 新增：
    - `validate_metadata_store_file(...)`
    - `validate_timeline_projection_store_file(...)`
- 这让 runtime-dir inspection 可以把 projection snapshot 文件准确区分为：
  - `ok`
  - `missing`
  - `corrupt`

### 3. backup catalog / restore preview / restore / repair 全部自动纳入新文件
- 同一份 `EXPECTED_RUNTIME_STATE_FILES` 被以下路径统一消费：
  - `describe_runtime_backup_snapshot(...)`
  - `snapshot_runtime_state_files(...)`
  - `repair_runtime_dir(...)`
  - `preview_restore_runtime_dir(...)`
  - `restore_runtime_dir_with_expected_preview_fingerprint(...)`
- 因此新文件现在会被：
  - repair 重建
  - backup catalog 统计
  - restore preview 预览差异
  - restore 实际恢复
  - pre-restore / repair backup 一并快照

### 4. 按 TDD 补齐跨工具链测试证据
- `services/local-minimal-node/tests/runtime_dir_inspection_test.rs`
  - inspection 报告现在必须包含 projection snapshot 文件，并把总健康文件数提升到 `11`
- `services/local-minimal-node/tests/runtime_dir_backup_catalog_test.rs`
  - full snapshot 的 `managed_file_count` 现在必须是 `11`
- `services/local-minimal-node/tests/runtime_dir_repair_test.rs`
  - repair 现在必须重建 projection snapshot 文件
- `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`
  - preview 现在必须显式报告 projection snapshot 的 `content_differs / target_missing`
- `services/local-minimal-node/tests/runtime_dir_restore_test.rs`
  - restore 现在必须恢复 projection snapshot 内容

## TDD 证据

### Red
- `cargo test -p local-minimal-node --offline --test runtime_dir_inspection_test --test runtime_dir_backup_catalog_test --test runtime_dir_repair_test --test runtime_dir_restore_test --test runtime_dir_restore_preview_test`
  - 首轮红灯命中：
    - `runtime_dir_backup_catalog_test`
    - 断言 `managed_file_count`
    - 实际值 `9`，期望值 `11`
  - 说明 runtime-dir 工具链当时确实还没有管理 projection snapshot 文件

### Green
- 同一组定向 runtime-dir 测试在扩展受管文件集合与校验分支后全部转绿

## 改动范围
- 生产代码：
  - `services/local-minimal-node/src/node/runtime_dir.rs`
- 测试：
  - `services/local-minimal-node/tests/runtime_dir_inspection_test.rs`
  - `services/local-minimal-node/tests/runtime_dir_backup_catalog_test.rs`
  - `services/local-minimal-node/tests/runtime_dir_repair_test.rs`
  - `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`
  - `services/local-minimal-node/tests/runtime_dir_restore_test.rs`
- 文档：
  - 本执行补充
  - 本轮质量审计与复盘
  - 本轮架构兑现与回写决议
  - `docs/架构/09-实施计划.md`
  - `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
  - `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`

## 回归验证
- `cargo test -p local-minimal-node --offline --test runtime_dir_inspection_test --test runtime_dir_backup_catalog_test --test runtime_dir_repair_test --test runtime_dir_restore_test --test runtime_dir_restore_preview_test`
- `cargo test -p local-minimal-node --offline`
- `cargo fmt --all --check`

## 结论
- 这是 `Wave C / Step 09 / CP09-2` 的第四个真实代码增量。
- `CP09-2` 已从“启动时可消费 projection snapshot”推进到“runtime-dir 运维工具链也受管 projection snapshot 文件”。
- 当前 step 还差什么：
  - `registered_devices / device_sync_feeds / device_sync_sequences` 是否进入同一恢复合同仍待决策
  - `CP09-3` 的 plane-level `metrics / tracing / logging` 仍未开始闭环
  - `CP09-4` 的 archive / retention / 更完整的 backup-restore 治理仍未闭环

## 下一轮继续做什么
1. 判断 `registered_devices / device_sync_feeds / device_sync_sequences` 是否应继续留在 `CP09-2` 一并收口。
2. 如果该合同已足够完整，则转入 `CP09-3` 的 metrics / tracing / logging 收口。
