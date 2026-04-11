# Step 09 / CP09-2 device-sync projection snapshot recovery 执行补充 - 2026-04-08

## 当前上下文
- 当前波次：`Wave C`
- 当前 step：`Step 09`
- 当前检查点：`CP09-2`
- 前置状态：
  - `As-Built 84` 已让 `projection-service` 具备 `summary + timeline` 的 snapshot/export/restore
  - `As-Built 85` 已把 `conversation catalog / members / read_cursors` 纳入同一条 rebuild 合同
  - `As-Built 86` 已让 `local-minimal-node` 启动恢复消费 `projection-metadata.json / projection-timeline.json`
  - `As-Built 87` 已把 projection snapshot 文件纳入 runtime-dir `inspection / backup / preview / restore / repair`
  - 但 `registered_devices / device_sync_feeds / device_sync_sequences` 仍在该恢复合同之外，导致 `resume / sync-feed` 在 `commit-journal` 丢失时没有同等级恢复证据

## 本轮为什么继续做这个增量
- 如果 `device-sync` 状态继续留在进程内内存里，`CP09-2` 的 snapshot recovery 只覆盖 conversation read-side，而不覆盖 session/device-sync read-side。
- `docs/架构/09-实施计划.md`、`132`、`138` 都明确把这一点列为 `CP09-2` 最后的待收口项，所以本轮最优决策不是提前跳到 `CP09-3`，而是先把 device-sync 状态接入同一条恢复合同。

## 本轮实际完成

### 1. `projection-service` 把 device-sync 状态接入同一份 snapshot 存储
- `services/projection-service/src/snapshot.rs`
  - 新增 `persist_device_sync_snapshot(...) / restore_device_sync_snapshot(...)`
  - `persist_conversation_snapshot(...)` 现在会连同 `registered_devices / device_sync_feeds / device_sync_sequences` 一并写入同一套：
    - `MetadataStore`
    - `TimelineProjectionStore`
  - `restore_conversation_snapshot(...)` 现在也会把同一份 device-sync 快照恢复回来

### 2. 用显式 catalog 解决“通用 MetadataStore 无法列 scope”问题
- `services/projection-service/src/snapshot.rs`
  - 新增统一 catalog scope：`projection-device-sync`
  - 通过 metadata snapshot 维护：
    - `registered-device-principals`
    - `device-sync-scopes`
- 这样 `MemoryMetadataStore` 和 `FileMetadataStore` 都能恢复 device-sync 快照，不需要把 `scopes_for_key(...)` 之类 file-only 能力上推到平台抽象层。

### 3. `local-minimal-node` 启动恢复与注册路径都接入同一快照合同
- `services/local-minimal-node/src/node.rs`
  - `ProjectionJournal::restore_projection_snapshots(...)` 现在会先恢复 device-sync snapshot，再做 conversation snapshot checkpoint 恢复
  - `ProjectionSnapshotStores` 新增 device-sync persist/restore helper
- `services/local-minimal-node/src/node/device_registration.rs`
  - `register_device(...)` 现在在 runtime-dir 模式下会把 device snapshot 写回同一套 projection snapshot store
- `services/local-minimal-node/src/node/build.rs`
  - device registration owner 现在拿到 `ProjectionSnapshotStores`，避免再造第二条落盘路径

### 4. 补齐 serialization 与回归测试
- `services/projection-service/src/model.rs`
  - `RegisteredDeviceView` 新增 `Deserialize`
- `services/projection-service/tests/projection_snapshot_test.rs`
  - 新增服务级快照恢复测试，证明新实例可恢复：
    - `registered_devices`
    - `device_sync_feed`
    - `latest_device_sync_seq`
- `services/local-minimal-node/tests/domain_recovery_persistence_test.rs`
  - 新增 runtime-dir 恢复测试，证明在 `commit-journal.json` 变成 `[]` 后仍可恢复：
    - `/api/v1/sessions/resume`
    - `/api/v1/devices/{device}/sync-feed`
  - 同时修正 `unique_runtime_dir()`，避免并行测试因时间戳碰撞共享 runtime-dir，污染 `commit-journal`

## TDD 证据

### Red
- `cargo test -p projection-service --offline --test projection_snapshot_test test_projection_service_restores_device_sync_state_from_projection_snapshot`
  - 首轮红灯：`registered_devices.len()` 实际 `0`，期望 `2`
- `cargo test -p local-minimal-node --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_restores_device_sync_resume_and_feed_from_runtime_dir_snapshots_when_commit_journal_is_missing`
  - 首轮红灯：`resumeRequired` 实际 `false`，期望 `true`
- 这证明缺口确实存在于 device-sync snapshot recovery 语义，而不是测试误判

### Green
- 同一组测试在 device-sync snapshot catalog + persist/restore + runtime-dir registration 落盘接入后全部转绿
- 随后的包级回归又暴露了并行测试 runtime-dir 冲突，修正 `unique_runtime_dir()` 后再次回归转绿

## 改动范围
- 生产代码：
  - `services/projection-service/src/model.rs`
  - `services/projection-service/src/snapshot.rs`
  - `services/local-minimal-node/src/node.rs`
  - `services/local-minimal-node/src/node/device_registration.rs`
  - `services/local-minimal-node/src/node/build.rs`
- 测试：
  - `services/projection-service/tests/projection_snapshot_test.rs`
  - `services/local-minimal-node/tests/domain_recovery_persistence_test.rs`
- 文档：
  - 本执行补充
  - 本轮质量审计与复盘
  - 本轮架构兑现与回写决议
  - `docs/架构/09-实施计划.md`
  - `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
  - `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`

## 回归验证
- `cargo test -p projection-service --offline --test projection_snapshot_test test_projection_service_restores_device_sync_state_from_projection_snapshot`
- `cargo test -p local-minimal-node --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_restores_device_sync_resume_and_feed_from_runtime_dir_snapshots_when_commit_journal_is_missing`
- `cargo test -p projection-service --offline`
- `cargo test -p local-minimal-node --offline`
- `cargo fmt --all --check`

## 结论
- 这是 `Wave C / Step 09 / CP09-2` 的第五个真实代码增量。
- `CP09-2` 不再只恢复 conversation projection，而是把 `registered_devices / device_sync_feeds / device_sync_sequences` 也并入同一条 snapshot recovery 合同。
- 当前 step 还差什么：
  - `CP09-3` 的 `metrics / tracing / structured logging` 仍未启动闭环
  - `CP09-4` 的 archive / retention / 更完整的 backup-restore 治理仍未闭环
  - `Step 09` 因此仍未完成

## 下一轮继续做什么
1. 把当前状态从 `CP09-2` 切到 `CP09-3`，优先补 projection snapshot persist/restore 的 `metrics / tracing / structured logging`。
2. 继续保留 `Wave C / 93` 阻塞状态，直到 `Step 09` 全步闭环。
