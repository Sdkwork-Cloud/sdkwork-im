# Step 09 / CP09-3 projection replay lag observability 质量审计与复盘 - 2026-04-08

## 审计范围
- `services/projection-service/src/observability.rs`
- `services/projection-service/src/lib.rs`
- `services/ops-service/src/lib.rs`
- `services/local-minimal-node/src/node.rs`
- `services/local-minimal-node/src/node/build.rs`
- `services/local-minimal-node/src/node/platform.rs`
- `services/projection-service/tests/projection_snapshot_test.rs`
- `services/ops-service/tests/http_smoke_test.rs`
- `services/local-minimal-node/tests/domain_recovery_persistence_test.rs`

## 审计结论
- 本轮未发现阻塞当前增量交付的剩余缺陷。
- 改动符合 `CP09-3` 的真实目标：
  - replay/backlog/duration 指标继续由 `projection-service` 拥有
  - `ops-service` 继续只承担公开 schema 与 runtime 暴露
  - `local-minimal-node` 继续只承担 startup replay 计算与 owner-to-ops 映射
- 本轮没有为补指标而引入新的旁路状态机，也没有绕开现有 `snapshot + journal replay` 主路径。

## 正向结果
- `projectionPlane.replay` 现在在默认空闲态和真实恢复态都有稳定 schema，不再依赖调用方猜字段是否存在。
- startup replay 不再只是“恢复后查询能对”，而是能在 `ops` 面说明：
  - backlog 有多少
  - 实际 replay 了多少事件
  - replay 花了多久
- `/backend/v3/api/ops/lag` 现在不再只有静态 `commit_journal` 占位项，而是能给出按 scope 的 `projection_replay` lag 证据。
- stale snapshot restart 的回归测试证明：
  - projection snapshot 可以落后于 journal
  - 重启后 replay 会把缺失消息补回
  - lag/backlog 证据与恢复结果是一致的

## 本轮发现并修正的问题
- 上一轮虽然已经有 `projectionPlane` 读面，但还无法说明“恢复时到底补了多少、滞后了多少、耗时多久”。
- `ops-service` 之前的 lag 视图是静态结构，无法承载 projection replay 这类真实运行态证据。
- `local-minimal-node` 的 startup replay 之前只执行动作，不保留 replay summary，因此无法为 `ops` 面提供最小真实 replay 观测。

## 剩余风险
- 当前 `projection_replay` lag 更接近“最近一次 startup replay 的证据”，还不是持续运行态的 live lag。
- 当前按 scope 的 lag 只覆盖 conversation snapshot checkpoint 语义；device-sync replay 目前只计入 replay/backlog 计数，没有单独 lag item。
- 当前 `durationMs` 是 startup replay 的耗时快照，不等于完整 `rebuild duration` 或持续 `replay throughput`。
- 当前 traces / logs 仍是进程内 recent view，不是外部 telemetry sink。
- `CP09-4` 的 backup / restore / repair / archive 仍未闭环。

## 验证证据
- `cargo test -p projection-service --offline --test projection_snapshot_test test_projection_service_records_projection_replay_metrics`
- `cargo test -p ops-service --offline --test http_smoke_test test_cluster_lag_health_runtime_dir_and_diagnostics_over_http`
- `cargo test -p local-minimal-node --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_surfaces_projection_plane_observability_over_ops_health_and_diagnostics`
- `cargo test -p local-minimal-node --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_reports_projection_replay_backlog_and_lag_after_stale_snapshot_restart`
- `cargo test -p projection-service --offline`
- `cargo test -p ops-service --offline`
- `cargo test -p local-minimal-node --offline`
- `cargo fmt --all --check`

## 复盘结论
- 本轮最正确的决策，是继续深挖已经落地的 projection recovery 主路径，而不是转去补一个外层模拟指标。
- 这让 `CP09-3` 从“能看见 restore 成功过”继续推进到“能解释 replay gap 是多少、补了多少、花了多久”。
- 但这依然只是 `Projection Plane` 在 startup replay 维度上的一段落地，不应误判为整个 `Step 09` 或整个 observability 体系已经完成。
