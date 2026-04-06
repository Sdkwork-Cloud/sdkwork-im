# local-minimal Windows 启动日志接管行为回归实施计划

## 1. 当前阶段

- `local-minimal-node` 已进入“可运行 + 可运维”闭环强化阶段。
- 上一轮已修复 Windows `start-local.ps1` 未重定向 stdout/stderr 的脚本缺陷。
- 当前目标不是再改生产逻辑，而是把该修复提升为行为级回归测试，防止将来再次退化。

## 2. 本轮问题

### 2.1 中风险：现有回归只验证脚本文本，不验证实际日志接管行为

问题表现：
- 现有 `deployment_profile_test.rs` 只能检查脚本里是否包含 `-RedirectStandardOutput` / `-RedirectStandardError`
- 但这还不能证明脚本在真实启动失败场景下，确实把子进程 stdout 写进了约定日志文件

风险：
- 后续如果脚本参数、工作目录、可执行文件替身或启动方式变化，文本断言可能仍通过，但真实日志接管失效
- 运维层面会重新掉回“日志路径存在但内容为空”的假契约

## 3. 实施策略

1. 使用 Windows-only 行为级测试搭一个临时工作目录。
2. 复制 `start-local.ps1`、`install-local.ps1`、`init-config-local.ps1` 到临时目录。
3. 用假的 `cargo.cmd` 绕过编译。
4. 用 `whoami.exe` 伪装成 `local-minimal-node.exe`，制造“有 stdout、无 health、快速退出”的稳定场景。
5. 运行 `start-local.ps1`，预期脚本返回失败。
6. 验证 `.runtime/local-minimal/logs/local-minimal-node.out.log` 中真实包含 `whoami.exe` 的输出。

## 4. 验证命令

- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_start_local_ps1_captures_background_process_stdout_into_documented_log_file -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo test -p local-minimal-node --offline`

## 5. 下一步

1. 把同类行为级测试扩展到 `stderr` 捕获和 `restart-local.ps1` / `stop-local.ps1` 的失败诊断链路。
2. 继续收敛 Windows/Linux 启动脚本的运维一致性。
3. 回到剩余 backlog，进入下一轮高优先级功能/安全/运维缺口闭环。
