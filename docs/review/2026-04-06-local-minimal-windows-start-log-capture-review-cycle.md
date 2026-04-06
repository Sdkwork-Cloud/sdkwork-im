# local-minimal Windows 启动日志接管 Review Cycle

## 1. Review 范围

- `bin/start-local.ps1`
- `services/local-minimal-node/tests/deployment_profile_test.rs`
- `/docs/架构/` 与 `/docs/review/` 中和本轮运维契约相关的文档沉淀

## 2. 问题列表

### 2.1 中高风险：Windows 后台启动时 stdout/stderr 日志路径是“空契约”

问题表现：
- 运维脚本向用户暴露了两个日志路径：
  - `.runtime/local-minimal/logs/local-minimal-node.out.log`
  - `.runtime/local-minimal/logs/local-minimal-node.err.log`
- 但 PowerShell 后台启动没有把服务输出真正写入这两个文件。

影响：
- 启动失败时只看到“进程提前退出”，看不到根因输出。
- 和 Linux `start-local.sh` 的行为不一致。
- 脚本文档与真实行为不一致，属于可运维性缺陷。

## 3. 根因

- `Start-Process` 缺少：
  - `-RedirectStandardOutput`
  - `-RedirectStandardError`

## 4. 修复策略

- 不修改 IM 服务内核。
- 不重写启动链路。
- 只补齐 Windows 后台启动的输出接管，保持 pid 文件、健康检查、环境变量注入和前台运行逻辑不变。

## 5. 实施记录

### 5.1 红灯

- 先修改 `deployment_profile_test.rs`，把 Windows 启动脚本契约从“禁止重定向”改为“必须重定向”。
- 红灯结果：`test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints` 失败。

### 5.2 绿灯

- 在 `bin/start-local.ps1` 的 `Start-Process` 增加：
  - `-RedirectStandardOutput $stdoutLog`
  - `-RedirectStandardError $stderrLog`

## 6. 当前结论

- 本轮已关闭“Windows 启动脚本声明日志路径但不接管输出”的缺陷。
- 这轮关闭的是运维契约缺口，不是新的业务功能。
- 当前剩余风险主要在“更强的脚本行为回归测试”和“其它脚本的一致性复核”，不是这次修复的阻塞项。

## 7. 下一步计划

1. 把文本契约测试推进为行为级脚本测试。
2. 扩展到 `restart/status/stop` 的 Windows 失败诊断一致性。
3. 回到更高优先级的剩余 backlog，继续做功能/安全/运维闭环。
