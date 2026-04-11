# Step 09 / CP09-2 projection snapshot metadata rebuild 质量审计与复盘 - 2026-04-08

## 审计范围
- `services/projection-service/src/model.rs`
- `services/projection-service/src/snapshot.rs`
- `services/projection-service/tests/projection_snapshot_test.rs`

## 审计结论
- 本轮未发现阻塞当前增量交付的剩余缺陷。
- 改动与 `CP09-2` 的真实目标一致：
  - 不是新增另一套 rebuild 抽象
  - 而是继续扩展现有 `projection-service` snapshot/export/restore 语义
- 现有结构边界未回退：
  - restore 语义仍集中在 `projection-service/src/snapshot.rs`
  - `lib.rs` 没有重新吸回 snapshot 编排逻辑

## 正向结果
- `member_snapshot(...)` 不再依赖进程内热状态，已可通过 snapshot restore 恢复。
- `read_cursor(...)` 在 restore 后可继续基于 summary 计算 `unread_count`。
- `inbox(...)` 恢复后不再因为 catalog 缺失退化成：
  - 空结果
  - 或 `conversation_type = unknown`
- `ConversationCatalogEntry` 已进入可序列化 metadata snapshot 语义。

## 本轮发现并修正的问题
- 上一轮 snapshot restore 虽然已经覆盖 `summary + timeline`，但对 read-side 查询仍不够：
  - `member_snapshot(...)` 直接返回空
  - `read_cursor(...)` 因找不到 member -> member_id 映射而失效
  - `inbox(...)` 因缺少 `conversations / members / read_cursors` 输入而无法正确恢复
- 本轮通过新增红测把这些缺口固定下来，并按最小范围补齐 metadata rebuild。

## 剩余风险
- 当前恢复范围仍集中在 projection read-side：
  - `managed runtime-dir` 还没有真实消费这条 snapshot restore 路径
- `registered_devices / device_sync_feeds / device_sync_sequences` 目前仍未纳入同一条 snapshot 恢复语义
- `metrics / tracing / logging` 仍无法观测 snapshot persist/restore 的耗时、失败率和 lag

## 验证证据
- `cargo test -p projection-service --test projection_snapshot_test --offline`
- `cargo test -p projection-service --offline`
- `cargo fmt --all --check`

## 复盘结论
- 本轮最正确的决策，是继续沿着 `CP09-2` 当前已经打开的 seam 往前走，把 read-side rebuild 的缺口补实，而不是跳到 `CP09-3` 或重新设计新的 storage layer。
- 这让 `Step 09` 从：
  - “conversation snapshot 只有 summary + timeline”
- 推进到：
  - “conversation read-side rebuild 已覆盖 inbox/member/read-cursor/catalog 所需 metadata”
- 但 `Step 09` 仍不能判定闭环，因为恢复链路仍未走进 `managed runtime-dir`，观测与 backup/restore 也仍未收口。
