# Step 09 / CP09-2 device-sync projection snapshot recovery 质量审计与复盘 - 2026-04-08

## 审计范围
- `services/projection-service/src/model.rs`
- `services/projection-service/src/snapshot.rs`
- `services/local-minimal-node/src/node.rs`
- `services/local-minimal-node/src/node/device_registration.rs`
- `services/local-minimal-node/src/node/build.rs`
- `services/projection-service/tests/projection_snapshot_test.rs`
- `services/local-minimal-node/tests/domain_recovery_persistence_test.rs`

## 审计结论
- 本轮未发现阻塞交付的剩余缺陷。
- 改动保持在 projection snapshot recovery 合同与其 consumer path 内，没有把变更扩散到无关业务面。
- TDD 主线成立：
  - 先用服务级与 runtime-dir 恢复测试把缺口打红
  - 再补 device-sync snapshot persist/restore
  - 最后完成包级回归与格式验证

## 正向结果
- `projection-service` 现在可以在新实例中恢复：
  - `registered_devices(...)`
  - `device_sync_feed(...)`
  - `latest_device_sync_seq(...)`
- `local-minimal-node` 现在即使在 `commit-journal.json = []` 的情况下，也能从同一份 projection snapshot 恢复：
  - `/api/v1/sessions/resume`
  - `/api/v1/devices/{device}/sync-feed`
- device registration 路径已复用同一份 `ProjectionSnapshotStores`，没有引入第二套旁路落盘机制。
- device-sync scope catalog 让恢复逻辑继续停留在统一平台抽象上，而不是把 file-only 发现能力上推到所有 store 实现。

## 本轮修正的问题
- 红灯表明此前 snapshot recovery 只覆盖 conversation read-side，不覆盖 device-sync read-side。
- 这带来三个具体问题：
  - `registered_devices` 在新实例里丢失
  - `latest_device_sync_seq` 回到 `0`
  - `resume / sync-feed` 在 journal 丢失时无法恢复
- 本轮通过 device-sync catalog + metadata/timeline 同存储恢复，修正了这一组断裂。

## 额外质量修正
- 包级回归阶段暴露 `domain_recovery_persistence_test` 的 runtime-dir 命名存在并行碰撞风险。
- 本轮已把 `unique_runtime_dir()` 增强为：
  - 时间戳
  - 原子递增计数
- 这样测试隔离恢复稳定，不再出现多个测试共享同一 runtime-dir 并相互覆盖 `commit-journal` 的伪失败。

## 剩余风险
- 当前 device-sync snapshot 仍是 file-backed recoverability 语义，不等同于 `Step 09` 完整观测闭环。
- `CP09-3` 的 `metrics / tracing / structured logging` 尚未建立，恢复过程仍缺少运行时可观测证据。
- `conversation_runtime` 在 `journal` 丢失时的 domain owner 恢复仍不是本轮覆盖范围。

## 验证证据
- `cargo test -p projection-service --offline --test projection_snapshot_test test_projection_service_restores_device_sync_state_from_projection_snapshot`
- `cargo test -p local-minimal-node --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_restores_device_sync_resume_and_feed_from_runtime_dir_snapshots_when_commit_journal_is_missing`
- `cargo test -p projection-service --offline`
- `cargo test -p local-minimal-node --offline`
- `cargo fmt --all --check`

## 复盘结论
- 本轮最优决策是继续留在 `CP09-2`，把 device-sync 恢复合同补齐后再转 `CP09-3`。
- 这样可以避免把 `Step 09` 留在“conversation 可恢复、session/device-sync 不可恢复”的半闭环状态。
- 当前判断是：
  - `CP09-2` 已实质收口
  - `Step 09` 仍未闭环
  - 下一轮应转入 `CP09-3`
