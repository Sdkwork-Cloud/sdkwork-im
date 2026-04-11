# start-local.ps1 Health Timeout Test Stability Design

## Decision

- 对启动超时回滚这类异步用例，测试必须保留最小可调度窗口，不能把验证速度建立在极端压缩的进程启动时序上。

## Contract

- 回归测试可以加速等待窗口，但必须：
  - 给子进程留出最小初始化机会
  - 对关键 marker 使用短暂轮询而不是瞬时断言
  - 在不改生产逻辑的前提下验证回滚结果

## Boundary

- 该设计只约束测试时序，不改变 `start-local.ps1` 的 runtime contract。
