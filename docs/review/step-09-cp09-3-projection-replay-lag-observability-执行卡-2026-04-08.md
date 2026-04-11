# Step 09 / CP09-3 projection replay lag observability 执行卡 - 2026-04-08

## 当前上下文
- 当前波次：`Wave C`
- 当前 step：`Step 09`
- 当前子任务：`CP09-3`
- 前置状态：
  - `Step 09 / CP09-2` 已完成 `conversation snapshot` 与 `device-sync snapshot` 的统一恢复合同
  - `CP09-3` 的第一段增量已完成 `projection snapshot persist / restore` 的基本 `metrics / tracing / structured logging`
  - 但上一轮 review 明确保留了三项缺口：
    - `projection lag`
    - `replay duration`
    - `backlog size`

## 本轮为什么继续做这个子任务
- `docs/step/09-存储投影与可观测治理.md` 要求 `CP09-3` 不是“有一点 trace/log”就算完成，而是要让 plane 级观测能真实解释恢复与重放状态。
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md` 要求 `Projection Plane` 的恢复链必须具备：
  - lag 可观测
  - replay 可观测
  - backlog 可观测
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md` 也把 `projection lag / backlog / replay duration` 列为 Projection Plane 的核心指标。
- 因此本轮最优决策不是切到 `CP09-4`，而是继续在当前已存在的 `snapshot + journal replay + ops diagnostics` 主路径上，把缺失指标补成真实合同。

## 本轮实际完成

### 1. `projection-service` 新增 replay 指标 owner
- `services/projection-service/src/observability.rs`
  - 新增 `ProjectionReplayMetricsView`
  - `ProjectionPlaneObservabilityView` 新增 `replay`
  - `TimelineProjectionService` 新增 `record_projection_replay_metrics(...)`
- replay 指标当前明确包括：
  - `backlogSize`
  - `replayedEventCount`
  - `durationMs`

### 2. `local-minimal-node` 在真实 startup replay 路径计算 replay/backlog/lag
- `services/local-minimal-node/src/node/build.rs`
  - `replay_projection_journal(...)` 不再只做无返回值回放，而是返回 `ProjectionReplaySummary`
  - summary 来自真实：
    - `commit-journal`
    - restored snapshot checkpoint
    - startup replay 实际 apply 次数
  - 同时生成按 scope 的 `projection_replay` lag item：
    - `scopeId`
    - `currentOffset`
    - `committedOffset`
    - `lag`

### 3. `ops-service` 补齐 projection replay schema 与可变 lag 通道
- `services/ops-service/src/lib.rs`
  - `ProjectionPlaneHealthView` / `ProjectionPlaneDiagnosticsView` 新增 `replay`
  - `OpsRuntime` 的 lag 状态改为可更新
  - 新增 `update_projection_replay_lag(...)`
  - 默认 `/api/v1/ops/lag` 现在包含零值 `projection_replay` 项，而不是完全缺字段

### 4. `local-minimal-node` 把 replay 指标与 lag 一并映射到 ops 面
- `services/local-minimal-node/src/node/platform.rs`
  - projection plane diagnostics 现在会把 `projection-service` 的 `replay` 映射到 `ops-service`
- `services/local-minimal-node/src/node/build.rs`
  - startup replay 完成后会：
    - 写入 `projection_service.record_projection_replay_metrics(...)`
    - 写入 `ops_runtime.update_projection_replay_lag(...)`

### 5. 用真实 stale snapshot 场景锁定合同
- `services/projection-service/tests/projection_snapshot_test.rs`
  - 新增 `test_projection_service_records_projection_replay_metrics`
- `services/ops-service/tests/http_smoke_test.rs`
  - 默认 `ops health / lag / diagnostics` 现在校验：
    - `projectionPlane.replay.*`
    - 默认 `projection_replay` lag item
- `services/local-minimal-node/tests/domain_recovery_persistence_test.rs`
  - 扩展已有恢复观测测试，校验 snapshot-only recovery 下 replay/lag 为零
  - 新增 `test_default_local_minimal_profile_reports_projection_replay_backlog_and_lag_after_stale_snapshot_restart`
    - 先保留旧 snapshot 文件
    - 再追加新消息
    - 再把 snapshot 文件回退到旧版本
    - 重启后验证：
      - `projectionPlane.replay.backlogSize >= 1`
      - `projectionPlane.replay.replayedEventCount >= 1`
      - `/api/v1/ops/lag` 存在对应 conversation scope 的 `projection_replay` 项
      - timeline 被 replay 恢复为两条消息

## 改动范围
- 代码：
  - `services/projection-service/src/observability.rs`
  - `services/projection-service/src/lib.rs`
  - `services/ops-service/src/lib.rs`
  - `services/local-minimal-node/src/node.rs`
  - `services/local-minimal-node/src/node/build.rs`
  - `services/local-minimal-node/src/node/platform.rs`
- 测试：
  - `services/projection-service/tests/projection_snapshot_test.rs`
  - `services/ops-service/tests/http_smoke_test.rs`
  - `services/local-minimal-node/tests/domain_recovery_persistence_test.rs`
- 文档：
  - 本执行卡
  - 本轮质量审计与复盘
  - 本轮架构兑现与回写决议
  - `docs/架构/09-实施计划.md`
  - `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
  - `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
  - `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`

## TDD 证据

### Red
- 先写测试，再验证真实缺口：
  - `cargo test -p projection-service --offline --test projection_snapshot_test test_projection_service_records_projection_replay_metrics`
  - `cargo test -p ops-service --offline --test http_smoke_test test_cluster_lag_health_runtime_dir_and_diagnostics_over_http`
  - `cargo test -p local-minimal-node --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_reports_projection_replay_backlog_and_lag_after_stale_snapshot_restart`
- 红测失败点与预期一致：
  - `ProjectionPlaneObservabilityView` 还没有 `replay`
  - `TimelineProjectionService` 还没有 `record_projection_replay_metrics(...)`
  - `projectionPlane.replay.*` 在 ops 接口上还是 `null`
  - stale snapshot restart 场景还不能在 `ops` 面证明 replay lag/backlog

### Green
- 上述三条定向测试现已全部通过

## 回归验证
- `cargo test -p projection-service --offline --test projection_snapshot_test test_projection_service_records_projection_replay_metrics`
- `cargo test -p ops-service --offline --test http_smoke_test test_cluster_lag_health_runtime_dir_and_diagnostics_over_http`
- `cargo test -p local-minimal-node --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_surfaces_projection_plane_observability_over_ops_health_and_diagnostics`
- `cargo test -p local-minimal-node --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_reports_projection_replay_backlog_and_lag_after_stale_snapshot_restart`
- `cargo test -p projection-service --offline`
- `cargo test -p ops-service --offline`
- `cargo test -p local-minimal-node --offline`
- `cargo fmt --all --check`

## 结论
- 这是 `Wave C / Step 09 / CP09-3` 的第二个真实代码增量。
- `Projection Plane` 现在不只会报告 snapshot persist / restore 的计数、trace、log，还能报告：
  - replay backlog
  - replay event count
  - replay duration
  - per-scope projection replay lag
- 但 `CP09-3` 仍不应整体判定通过，因为当前仍缺：
  - 持续运行态的 live projection lag，而不仅是 startup replay 证据
  - `replay throughput / rebuild duration / inbox/timeline update delay`
  - 外部 telemetry sink 与更完整的 SLO / 告警

## 下一轮继续做什么
1. 继续留在 `CP09-3`，评估是否补 `ops/replay-status` 或更明确的 replay drill 视图。
2. 继续补 `Projection Plane` 的 live lag / replay throughput / rebuild duration 指标，避免只保留 startup replay 证据。
3. 待 `CP09-3` 证据再完整一段后，再判断是否进入 `CP09-4` 的 backup / restore / repair / archive 收口。
