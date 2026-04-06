# local-minimal Windows 启动日志接管实施计划

## 1. 当前阶段

- `local-minimal-node` 已具备可启动、可健康检查、可对外提供基础 IM API 的能力。
- `runtime-dir` 的 inspection / repair / backup / restore / preview 与 typed diff 波次已完成。
- 当前进入“可商用可运维闭环”加固阶段，重点从功能正确性延伸到脚本一致性、诊断可观测性和跨平台运维契约。

## 2. 本轮问题

### 2.1 中高风险：Windows 后台启动脚本未真正接管 stdout/stderr

问题表现：
- `bin/start-local.ps1` 会输出 `stdout log` / `stderr log` 路径。
- `bin/status-local.ps1` 也把这两个日志文件作为标准诊断入口暴露。
- 但后台启动时 `Start-Process` 没有配置 `-RedirectStandardOutput` / `-RedirectStandardError`。

风险：
- 进程启动后若快速退出，Windows 运维侧拿不到服务输出，排障链路断裂。
- 同一套安装/启动契约在 Linux 与 Windows 上行为不一致。
- 文档、状态脚本与实际脚本行为不一致，会误导私有化部署与一线运维。

## 3. 根因判断

- Linux `bin/start-local.sh` 使用 `nohup ... >>stdout 2>>stderr`，具备真实日志接管。
- Windows `bin/start-local.ps1` 只创建日志文件路径，但没有把子进程输出绑定到这些文件。
- 这是典型的 operator contract drift：脚本宣告了日志入口，却没有完成实现。

## 4. 实施步骤

1. 先把部署契约测试改成红灯，要求 Windows 启动脚本必须包含 stdout/stderr 重定向。
2. 最小化修改 `bin/start-local.ps1`，只补齐 `Start-Process` 的标准输出/错误输出重定向，不改变已有启动、健康检查和 pid 管理逻辑。
3. 补充本轮 review 文档与架构标准，冻结“跨平台启动日志接管一致性”。
4. 运行针对性测试与本地启动验证，确认修复没有破坏现有启动路径。

## 5. 验证命令

- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo test -p local-minimal-node --offline`
- `powershell -ExecutionPolicy Bypass -File bin/start-local.ps1`
- `powershell -ExecutionPolicy Bypass -File bin/status-local.ps1`

## 6. 下一步

1. 继续检查 `restart-local.ps1` / `stop-local.ps1` / `status-local.ps1` 的失败面诊断是否同样满足跨平台一致性。
2. 增补“启动失败时日志非空”的脚本级回归测试，把当前文本契约测试进一步推进到行为契约测试。
3. 继续沿 `/docs/review/` 中尚未关闭的剩余风险做下一轮闭环修复。
