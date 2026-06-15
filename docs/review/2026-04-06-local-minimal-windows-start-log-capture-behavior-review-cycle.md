# local-minimal Windows 启动日志接管行为回归 Review Cycle

## 1. Review 范围

- [deployment_profile_test.rs](<workspace-root>/sdkwork-im/services/local-minimal-node/tests/deployment_profile_test.rs)
- [start-local.ps1](<workspace-root>/sdkwork-im/bin/start-local.ps1)
- `/docs/review/` 与 `/docs/架构/` 中本轮行为回归标准文档

## 2. 问题列表

### 2.1 中风险：日志接管修复缺少行为级回归

问题表现：
- 之前只验证脚本中存在重定向参数
- 没有验证失败启动时，stdout 是否真的落到约定日志文件

影响：
- 真实行为和文本契约仍可能分叉
- 运维回归风险无法被自动测试及时拦住

## 3. 根因

- 之前的修复先收敛了脚本实现，但测试层还停留在“文本包含关系”
- 缺少一个稳定、无需真实服务的行为注入器来验证 Windows 后台启动日志链路

## 4. 本轮修复

- 新增 Windows-only 行为级测试：
  - `test_start_local_ps1_captures_background_process_stdout_into_documented_log_file`
- 测试方法：
  - 临时复制启动脚本
  - 用 `cargo.cmd` 假桩绕过编译
  - 用 `whoami.exe` 作为稳定 stdout 发生器
  - 验证 `local-minimal-node.out.log` 真实包含子进程输出

## 5. 当前结论

- 本轮已把“Windows 启动日志接管”从文本契约提升为行为契约
- 这轮没有扩大生产改动面，重点是把上一轮修复固化成长期回归能力

## 6. 下一步计划

1. 继续补 `stderr` 行为回归。
2. 继续检查 `restart-local.ps1`、`status-local.ps1`、`stop-local.ps1` 的失败诊断一致性。
3. 进入下一批高优先级 backlog 缺口闭环。
