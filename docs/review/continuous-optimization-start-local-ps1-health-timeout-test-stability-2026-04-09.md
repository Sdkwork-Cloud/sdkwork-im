# Continuous Optimization: start-local.ps1 Health Timeout Test Stability

## Context

- `test_start_local_ps1_stops_background_process_and_clears_pid_file_when_health_check_times_out` 在单跑与整包回归中表现不稳定。
- 失败点是 probe marker 未及时落盘，而不是启动回滚路径本身失效。

## Confirmed Bug

- 测试把 `start-local.ps1` 的健康等待窗口压缩到 `2 x 100ms`。
- 在调度抖动下，probe 进程可能尚未写出 marker 就已触发超时回滚，导致测试误判失败。

## Root Cause

- 测试加速过于激进，破坏了“已启动进程至少有机会执行初始化代码”的前提。
- 断言直接检查 marker 即时存在，没有为进程调度保留缓冲。

## Fix

- 在测试中新增 `wait_for_path`，允许短暂轮询 marker 落盘。
- 将 PowerShell 与 Bash 健康超时测试的加速窗口从 `2 x 100ms` 提高到 `5 x 100ms`。
- 保持产品脚本不变，只修正测试时序模型。

## Verification

Red:

```powershell
cargo test -p local-minimal-node --offline --test deployment_profile_test test_start_local_ps1_stops_background_process_and_clears_pid_file_when_health_check_times_out -- --exact --nocapture
```

Green:

```powershell
cargo test -p local-minimal-node --offline --test deployment_profile_test test_start_local_ps1_stops_background_process_and_clears_pid_file_when_health_check_times_out -- --exact --nocapture
cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture
cargo fmt --all --check
cargo test -p local-minimal-node --offline -- --nocapture
```

## Result

- 健康超时回滚测试在重复执行与整包回归下稳定通过。
- 本轮确认问题属于测试脆弱性，而非 `start-local.ps1` 回滚实现缺陷。

## Boundary

- 本轮没有改变生产脚本健康等待逻辑。
- Bash 原生实机验证仍受当前 Windows 会话无可用 Bash 约束。
