# Step 09 / CP09-3 projection plane observability 质量审计与复盘 - 2026-04-08

## 审计范围
- `services/projection-service/src/lib.rs`
- `services/projection-service/src/observability.rs`
- `services/projection-service/src/snapshot.rs`
- `services/ops-service/src/lib.rs`
- `services/local-minimal-node/src/node/platform.rs`
- `services/projection-service/tests/projection_snapshot_test.rs`
- `services/ops-service/tests/http_smoke_test.rs`
- `services/local-minimal-node/tests/domain_recovery_persistence_test.rs`

## 审计结论
- 本轮未发现阻塞当前增量交付的剩余缺陷。
- 改动符合 `CP09-3` 的真实目标：
  - 让 `projection snapshot persist / restore` 拥有最小可用的 plane-level observability
  - 让 `ops health / diagnostics` 对外暴露同一份真实状态
- 改动没有回退 `Step 02` 已建立的结构边界：
  - `projection-service` 的 owner 继续内聚在服务内部模块
  - `ops-service` 只维护公开 schema 与 runtime state
  - `local-minimal-node` 只承担映射与装配，不重新拥有 projection observability 状态

## 正向结果
- `projection-service` 现在可以稳定导出：
  - `status`
  - snapshot persist / restore counter
  - recent traces
  - recent structured logs
  - last failure
- `ops-service` 默认 runtime 对 projection plane 有明确空闲态，而不是 `null` 或缺字段。
- `local-minimal-node` 在真实 runtime-dir 恢复路径下，已经把：
  - conversation snapshot restore
  - device-sync snapshot restore
  的观测证据带到 `/backend/v3/api/ops/diagnostics`。

## 本轮发现并修正的问题
- `CP09-2` 已经落地了 snapshot/recovery 路径，但此前完全不可观测，无法通过 `ops` 面证明恢复真实发生。
- `ops-service` 之前只有通用 `items / lag / diagnostics`，缺少 projection plane 专属 schema。
- `local-minimal-node` 虽然已经在启动恢复时调用 snapshot restore，但没有把恢复证据写回 ops runtime。

## 剩余风险
- 当前 metrics 仍偏最小化，只覆盖 snapshot persist / restore 计数，尚未覆盖：
  - `projection lag`
  - `replay duration`
  - `backlog size`
- 当前 traces / logs 是内存内最近事件视图，还不是外部 telemetry sink。
- 当前 `CP09-3` 只补了 `Projection Plane` 的一条关键恢复路径，不等于所有 plane 都完成统一观测。
- `CP09-4` 的 backup / restore / repair / archive 仍未把 observability 证据与脚本资产合成完整收口。

## 验证证据
- `cargo test -p projection-service --offline --test projection_snapshot_test test_projection_service_records_snapshot_observability_metrics_traces_and_logs`
- `cargo test -p ops-service --offline --test http_smoke_test test_cluster_lag_health_runtime_dir_and_diagnostics_over_http`
- `cargo test -p local-minimal-node --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_surfaces_projection_plane_observability_over_ops_health_and_diagnostics`
- `cargo test -p projection-service --offline`
- `cargo test -p ops-service --offline`
- `cargo test -p local-minimal-node --offline`
- `cargo fmt --all --check`

## 复盘结论
- 本轮最正确的决策，是没有先引入更重的 metrics/tracing 基础设施，而是先让已经存在的 snapshot/recovery 主路径具备“最小但真实”的 plane 观测合同。
- 这让 `Step 09` 从“恢复已存在但不可证明”推进到“恢复已发生且能在 ops 读面给出证据”。
- 但 `CP09-3` 仍只是第一段落地，不应误判为整个 `Step 09` 已通过。
