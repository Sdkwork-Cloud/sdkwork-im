# Step 05 CP05-4 local-minimal-node disconnect lifecycle owner 质量审计 - 2026-04-07

## 1. 审计范围

- `Wave B / Step 05 / CP05-4`
- 审计对象：
  - `services/local-minimal-node/src/node/device_registration.rs`
  - `services/local-minimal-node/src/node.rs`
  - `services/local-minimal-node/src/node/session.rs`
  - `services/local-minimal-node/tests/lib_structure_test.rs`

## 2. 审计结论

- `local-minimal-node` 的 disconnect 生命周期已与 `session-gateway` 对齐，下沉到 device lifecycle owner seam。
- 本地 profile 额外的 `platform::refresh_node_operational_view(...)` 也已经随 disconnect lifecycle 一并下沉，没有继续残留在 session entrypoint。
- 断开、重连、presence、cluster rebalance、runtime-dir 相关回归均通过，说明这次 seam 收敛未打破本地 profile 的复合运行路径。

## 3. 证据

- 结构红绿测试：
  - `target-cp054s-red-local-disconnect`
  - `target-cp054s-green-local-disconnect`
- 结构回归：
  - `target-cp054s-structure`
- 包级回归：
  - `target-cp054s-reg-local-minimal-node`
- 代码扫描：
  - `services/session-gateway/src/session.rs`
  - `services/local-minimal-node/src/node/session.rs`
  - raw disconnect lifecycle glue 已无命中

## 4. 剩余风险

- `CP05-4` 虽然已完成已知 owner seam 收口，但 `Step 05` 仍需整步闭环审计才能宣告通过。
- `91 / 95 / 97` 目前还没有针对“最新仓库状态”的 step-wide 结论。
- `Wave B / 93` 仍然不能执行。
