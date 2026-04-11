# Step 09 / CP09-2 runtime-dir projection snapshot managed files 质量审计与复盘 - 2026-04-08

## 审计范围
- `services/local-minimal-node/src/node/runtime_dir.rs`
- `services/local-minimal-node/tests/runtime_dir_inspection_test.rs`
- `services/local-minimal-node/tests/runtime_dir_backup_catalog_test.rs`
- `services/local-minimal-node/tests/runtime_dir_repair_test.rs`
- `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`
- `services/local-minimal-node/tests/runtime_dir_restore_test.rs`

## 审计结论
- 本轮未发现阻塞交付的剩余缺陷。
- 改动保持在 runtime-dir 受管文件合同和对应测试中，没有把变更扩散到无关运行时路径。
- TDD 顺序成立：
  - 先通过 runtime-dir 测试把 `managed_file_count = 11` 的缺口打红
  - 再最小化修改 `runtime_dir.rs`
  - 最后完成定向测试、整包测试和格式验证

## 正向结果
- runtime-dir inspection 已把 projection snapshot 文件纳入正式健康检查。
- runtime-dir backup catalog 现在能正确区分：
  - `full_snapshot`
  - `partial_snapshot`
  - `empty_snapshot`
  且统计口径包含 projection snapshot 文件。
- restore preview 现在能在不改写 runtime-dir 的前提下，对 projection snapshot 文件给出：
  - `content_differs`
  - `target_missing`
- restore / repair 现在都会对 projection snapshot 文件执行真实操作，而不是留在“启动路径能恢复、运维工具链不管理”的断裂状态。

## 本轮修正的问题
- 红灯表明 runtime-dir 的快照统计口径仍停留在旧的 9 文件集合。
- 这会带来三个具体问题：
  - backup catalog 会低估 full snapshot 完整度
  - repair 不会补 projection snapshot 缺失文件
  - restore / preview 不会纳入 projection snapshot 差异
- 本轮通过统一扩展 `EXPECTED_RUNTIME_STATE_FILES` 和校验分支，修正了这一组断裂。

## 剩余风险
- 当前 inspection 只验证 projection snapshot 文件的 JSON 结构合法性，还没有建立跨文件语义一致性校验。
- `registered_devices / device_sync_feeds / device_sync_sequences` 仍未确认是否进入同一 runtime-dir snapshot 合同。
- `CP09-3 / CP09-4` 仍未闭环，因此 Step 09 不能提前判定完成。

## 验证证据
- `cargo test -p local-minimal-node --offline --test runtime_dir_inspection_test --test runtime_dir_backup_catalog_test --test runtime_dir_repair_test --test runtime_dir_restore_test --test runtime_dir_restore_preview_test`
- `cargo test -p local-minimal-node --offline`
- `cargo fmt --all --check`

## 复盘结论
- 本轮最优决策是先把 runtime-dir 受管文件合同补齐，再考虑把更多 read-side 状态推进到同一恢复语义。
- 这样可以避免过早转入 `CP09-3`，却把 `CP09-2` 留在“启动恢复成立、运维恢复不成立”的半闭环状态。
- `Wave C / Step 09` 仍处于进行中，但 `CP09-2` 的 runtime-dir 运维面已经明显增强。
