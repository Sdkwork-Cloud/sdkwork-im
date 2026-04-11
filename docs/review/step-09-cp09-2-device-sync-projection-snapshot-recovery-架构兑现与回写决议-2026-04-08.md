# Step 09 / CP09-2 device-sync projection snapshot recovery 架构兑现与回写决议 - 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`

## 本轮已兑现能力
- `09-实施计划`
  - `Wave C / Step 09 / CP09-2` 已把 `registered_devices / device_sync_feeds / device_sync_sequences` 纳入同一条 projection snapshot recovery 合同。
  - `CP09-2` 现在覆盖的不再只是 conversation read-side，也包括 session/device-sync read-side 的恢复入口。
- `132`
  - 统一 file-backed `MetadataStore + TimelineProjectionStore` 现在继续承担 device-sync projection recovery，不需要引入第二套 store 或旁路格式。
  - 通过 snapshot catalog，统一存储抽象既能写入也能恢复 device-sync scope。
- `138`
  - `Local Minimal` profile 现在已具备 journal 丢失后的 `resume / sync-feed` 恢复证据。
  - device registration 路径也会把快照写回同一份 runtime-dir projection store，使灾备恢复不依赖某一次后续 conversation 事件。

## 本轮未兑现能力
- `140`
  - projection snapshot persist/restore 的 `metrics / tracing / structured logging` 仍未闭环。
- `141`
  - archive / retention / lifecycle 治理未被本轮触达。
- `138`
  - `conversation_runtime` 在 `journal` 丢失时的 domain owner 恢复仍未建立。

## 是否偏离架构
- 无偏离。
- 本轮继续沿用统一 file-backed store seam，只是在同一份 metadata snapshot 中增加 device-sync catalog，而不是另造恢复发现机制。

## 回写决议
- `docs/架构/09-实施计划.md`
  - 追加 `As-Built 88`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
  - 追加 `As-Built 6`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
  - 追加 `As-Built 3`

## 证据
- 代码：
  - `services/projection-service/src/model.rs`
  - `services/projection-service/src/snapshot.rs`
  - `services/local-minimal-node/src/node.rs`
  - `services/local-minimal-node/src/node/device_registration.rs`
  - `services/local-minimal-node/src/node/build.rs`
- 测试：
  - `services/projection-service/tests/projection_snapshot_test.rs`
  - `services/local-minimal-node/tests/domain_recovery_persistence_test.rs`
- 验证：
  - `cargo test -p projection-service --offline --test projection_snapshot_test test_projection_service_restores_device_sync_state_from_projection_snapshot`
  - `cargo test -p local-minimal-node --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_restores_device_sync_resume_and_feed_from_runtime_dir_snapshots_when_commit_journal_is_missing`
  - `cargo test -p projection-service --offline`
  - `cargo test -p local-minimal-node --offline`
  - `cargo fmt --all --check`

## 当前判断
- 这是 `CP09-2` 的第五个真实代码增量，不是 `Step 09` 的整步通过。
- `CP09-2` 已完成收口，可以转入 `CP09-3`。
- `Step 09` 仍未闭环，`Wave C / 93` 继续阻塞于 `Step 09`。
