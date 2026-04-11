# Step 09 / CP09-2 runtime-dir projection snapshot recovery 质量审计与复盘 - 2026-04-08

## 审计范围
- `adapters/local-disk/src/metadata.rs`
- `services/local-minimal-node/src/node.rs`
- `services/local-minimal-node/src/node/build.rs`
- `services/local-minimal-node/src/node/projection.rs`
- `services/local-minimal-node/tests/domain_recovery_persistence_test.rs`

## 审计结论
- 本轮未发现阻塞当前增量交付的剩余缺陷。
- 改动符合 `CP09-2` 的真实目标：
  - 让 managed runtime-dir 启动恢复消费 projection snapshot/rebuild
  - 而不是继续让 snapshot 只停留在 `projection-service` 内部
- 结构边界未回退：
  - `build.rs` 负责装配 snapshot stores 与 startup recovery 顺序
  - `node.rs` 负责 journal + snapshot stores 的组合语义
  - `projection.rs` 只是切回 projection-owned access seam

## 正向结果
- runtime-dir 现在会真实生成：
  - `projection-metadata.json`
  - `projection-timeline.json`
- startup recovery 不再只依赖 journal 来发现可恢复 scope。
- read-side 查询已经可以消费 runtime-dir snapshot 恢复结果，而不是恢复了也查不到。
- `FileMetadataStore` 的 scope 发现能力使 file-backed snapshot 不再是“只能盲写、不能枚举恢复目标”的死端口。

## 本轮发现并修正的问题
- 第一版 runtime-dir 接入只做到“append 时可写 snapshot”，但 startup recovery 仍然只从 `commit-journal` 发现 scope。
- 当 `commit-journal.json` 被清空时：
  - runtime-dir 虽然已经持有 snapshot 文件
  - 却完全不知道该恢复哪些 conversation
- 本轮通过 `scopes_for_key(...)` 补齐 file metadata 的 scope 枚举能力，解决了这个断口。

## 剩余风险
- 目前只打通了 managed runtime-dir 的 startup recovery，不等于 backup/restore 工具链已经管理新 snapshot 文件。
- `conversation_runtime` 本身在 journal 丢失时仍无法恢复 domain owner 状态，本轮恢复的是 projection read-side。
- sync-plane 的设备注册与 device-sync feed 仍未纳入同一条 runtime-dir snapshot 恢复合同。
- 仍无 snapshot persist/restore 的 metrics / tracing / structured logging。

## 验证证据
- `cargo test -p local-minimal-node --test domain_recovery_persistence_test test_default_local_minimal_profile_restores_projection_queries_from_runtime_dir_snapshots_when_commit_journal_is_missing --offline`
- `cargo test -p local-minimal-node --offline`
- `cargo test -p projection-service --offline`
- `cargo test -p im-adapters-local-disk --offline`
- `cargo fmt --all --check`

## 复盘结论
- 本轮最正确的决策，是先把 runtime-dir “启动恢复”这条主路径打通，而不是立刻扩散到 `preview / restore / repair` 全量工具面。
- 这样 `CP09-2` 已从：
  - “projection snapshot 只在 service 内部可恢复”
- 推进到：
  - “managed runtime-dir 已有第一条真实消费 snapshot restore 的恢复路径”
- 但 `Step 09` 仍未闭环，因为运维工具链和可观测链路还没接上这组新状态文件。
