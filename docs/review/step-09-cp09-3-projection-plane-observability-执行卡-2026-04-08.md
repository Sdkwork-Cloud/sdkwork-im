# Step 09 / CP09-3 projection plane observability 执行卡 - 2026-04-08

## 当前上下文
- 当前波次：`Wave C`
- 当前 step：`Step 09`
- 当前子任务：`CP09-3`
- 前置状态：
  - `Step 08` 已于 `2026-04-08` 闭环完成
  - `Step 09 / CP09-2` 已完成 `projection snapshot` 与 `device-sync snapshot` 的统一恢复合同
  - 但 `projection-service` 的 snapshot persist / restore 仍缺少真实的 `metrics / tracing / structured logging`
  - `ops-service` 的 `/backend/v3/api/ops/health` 与 `/backend/v3/api/ops/diagnostics` 也还没有 projection plane 观测面

## 本轮为什么做这个子任务
- `docs/step/09-存储投影与可观测治理.md` 把 `CP09-3` 定义为“metrics / tracing / logging 已按 plane 基本收口”。
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md` 明确要求：
  - plane 级能力必须补 `health / diagnostics`
  - 新增主链能力必须同时补 `metric / trace / error taxonomy`
- 当前最小、最真实的收口点不是先接外部观测框架，而是先让 `projection snapshot` 这条已存在的恢复路径具备：
  - 真实计数
  - 最近 trace
  - 最近 structured log
  - 真实暴露到 `ops` 读面

## 本轮实际完成

### 1. 为 `projection-service` 落地 projection plane observability owner
- `services/projection-service/src/observability.rs`
  - 新增：
    - `ProjectionOperationMetricView`
    - `ProjectionPlaneMetricsView`
    - `ProjectionTraceView`
    - `ProjectionLogView`
    - `ProjectionPlaneObservabilityView`
    - `ProjectionObservabilityState`
- `services/projection-service/src/lib.rs`
  - `TimelineProjectionService` 新增 `observability` 状态
  - 对外暴露 `projection_plane_observability()`

### 2. 把 snapshot persist / restore 变成真实可记录事件
- `services/projection-service/src/snapshot.rs`
  - `persist_conversation_snapshot(...)`
  - `restore_conversation_snapshot(...)`
  - `persist_device_sync_snapshot(...)`
  - `restore_device_sync_snapshot(...)`
  - 现在都会按真实执行结果记录：
    - `Ok(true)`：成功计数、成功 trace、成功 structured log
    - `Ok(false)`：不计入成功或失败
    - `Err(...)`：失败计数、失败 trace、失败 structured log、last failure

### 3. 为 `ops-service` 增加 projection plane health / diagnostics schema
- `services/ops-service/src/lib.rs`
  - `OpsHealthResponse` 新增 `projectionPlane`
  - `DiagnosticBundle` 新增 `projectionPlane`
  - `OpsRuntime` 新增 projection plane 状态与 `update_projection_plane(...)`
  - 默认 `build_default_app()` 会返回：
    - `projectionPlane.status = idle`
    - 所有 snapshot counter 为 `0`
    - trace / log 数组为空

### 4. 让 `local-minimal-node` 把真实 projection plane 状态映射到 ops 视图
- `services/local-minimal-node/src/node/platform.rs`
  - `refresh_node_operational_view(...)` 现在会读取：
    - `state.projection_service.projection_plane_observability()`
  - 并映射写入：
    - `state.ops_runtime.update_projection_plane(...)`
- 这让 `managed runtime-dir` 下的 snapshot persist / restore 观测面进入真实 `/backend/v3/api/ops/health` 与 `/backend/v3/api/ops/diagnostics`

### 5. 用真实红绿测试冻结最小观测合同
- `services/projection-service/tests/projection_snapshot_test.rs`
  - 新增：
    - `test_projection_service_records_snapshot_observability_metrics_traces_and_logs`
- `services/ops-service/tests/http_smoke_test.rs`
  - 默认 `ops health / diagnostics` 现在校验 `projectionPlane`
- `services/local-minimal-node/tests/domain_recovery_persistence_test.rs`
  - 新增：
    - `test_default_local_minimal_profile_surfaces_projection_plane_observability_over_ops_health_and_diagnostics`

## 改动范围
- 代码：
  - `services/projection-service/src/lib.rs`
  - `services/projection-service/src/observability.rs`
  - `services/projection-service/src/snapshot.rs`
  - `services/ops-service/src/lib.rs`
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
  - `cargo test -p projection-service --offline --test projection_snapshot_test test_projection_service_records_snapshot_observability_metrics_traces_and_logs`
  - `cargo test -p ops-service --offline --test http_smoke_test test_cluster_lag_health_runtime_dir_and_diagnostics_over_http`
  - `cargo test -p local-minimal-node --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_surfaces_projection_plane_observability_over_ops_health_and_diagnostics`
- 红测失败点与预期一致：
  - `TimelineProjectionService` 还没有 `projection_plane_observability()`
  - `ops health / diagnostics` 还没有 `projectionPlane`
  - `local-minimal-node` 还没有把 projection plane 映射到 ops runtime

### Green
- 以上三条定向测试现已全部通过

## 回归验证
- `cargo test -p projection-service --offline --test projection_snapshot_test test_projection_service_records_snapshot_observability_metrics_traces_and_logs`
- `cargo test -p ops-service --offline --test http_smoke_test test_cluster_lag_health_runtime_dir_and_diagnostics_over_http`
- `cargo test -p local-minimal-node --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_surfaces_projection_plane_observability_over_ops_health_and_diagnostics`
- `cargo test -p projection-service --offline`
- `cargo test -p ops-service --offline`
- `cargo test -p local-minimal-node --offline`
- `cargo fmt --all --check`

## 结论
- 这是 `Wave C / Step 09 / CP09-3` 的第一个真实代码增量。
- `projection snapshot persist / restore` 已不再是黑盒恢复路径，而是具备最小可用的：
  - plane metrics
  - recent traces
  - structured logs
  - ops health / diagnostics 暴露
- 但 `CP09-3` 仍不应整体判定通过，因为当前只覆盖了 `Projection Plane` 的 snapshot/recovery 观测面，还没有把：
  - 更广泛的 projection lag / replay duration / backlog 指标
  - 其他 plane 的统一 health/diagnostics
  - SLO / 告警 / error taxonomy 收口
  做成整步闭环

## 下一轮继续做什么
1. 继续在 `CP09-3` 内补 `projection lag / replay duration / backlog` 这类更接近 `140` 原文的 plane 指标。
2. 评估是否把 `runtime-dir inspection / restore / repair` 过程中的关键恢复节点也纳入同一份 diagnostics 证据。
3. 在 `CP09-3` 具备更完整的 projection plane 观测证据后，再进入 `CP09-4` 的 backup / restore / repair / archive 代码与脚本收口。
