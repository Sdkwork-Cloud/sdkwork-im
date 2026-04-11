# Continuous Optimization: start-local.ps1 Health Timeout Test Stability

## Goal

- 消除 `start-local.ps1` 健康超时回滚测试的时序脆弱性，恢复整包回归稳定性。

## Scope

- 修改 `services/local-minimal-node/tests/deployment_profile_test.rs`。
- 不修改 `bin/start-local.ps1` 生产逻辑。

## Implementation

- 先用失败回归确认 marker 落盘断言在压缩窗口下不稳定。
- 增加短暂轮询 helper，并放宽测试加速窗口到 `5 x 100ms`。
- 同步对齐 Bash 同类测试的时序模型。
- 回跑重复单测、全量 `deployment_profile_test`、格式检查与包级回归。

## Expected State

- 健康超时测试仍然快速，但不再依赖过窄的进程调度窗口。
- 整包测试结果不再因该用例随机失败。

## Boundary

- 这是测试稳定性修复，不是产品行为变更。
