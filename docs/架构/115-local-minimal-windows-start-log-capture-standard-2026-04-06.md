# 115. local-minimal Windows 启动日志接管标准

## 1. 目标

冻结 `local-minimal` 私有化/本地部署脚本在 Windows 平台上的最小日志接管标准，保证后台启动与 Linux 启动契约一致。

## 2. 标准

### 2.1 后台启动脚本必须真实接管标准输出与标准错误

当 `bin/start-local.ps1` 以后台模式启动 `local-minimal-node.exe` 时，必须将子进程输出写入：

- `.runtime/local-minimal/logs/local-minimal-node.out.log`
- `.runtime/local-minimal/logs/local-minimal-node.err.log`

PowerShell 实现要求：

- `Start-Process` 必须包含 `-RedirectStandardOutput`
- `Start-Process` 必须包含 `-RedirectStandardError`

### 2.2 暴露出来的日志路径必须是可兑现契约

若脚本或状态命令向操作者打印了日志文件路径，则这些路径必须是真实可用于排障的入口，不能只是占位文件或空声明。

### 2.3 跨平台行为必须一致

- Linux `start-local.sh` 已通过 `nohup ... >>stdout 2>>stderr` 接管输出。
- Windows `start-local.ps1` 必须达到相同语义：后台进程输出可追踪、可复盘、可定位。

## 3. 不做什么

- 本标准不要求服务在正常运行时必须持续输出业务日志。
- 本标准只要求一旦服务有 stdout/stderr 输出，部署脚本必须接住它。
- 本标准不涉及日志滚动、结构化日志或集中采集方案。

## 4. 验证要求

至少满足以下验证：

1. 部署契约测试验证 Windows 启动脚本包含 stdout/stderr 重定向。
2. 本地启动验证确认修改未破坏 pid 管理与健康检查。
3. 后续补充脚本行为级测试，验证失败启动时日志文件可捕获子进程输出。

## 5. 意义

这不是“脚本美化”，而是商业部署的基础运维能力：

- 失败时可诊断
- 跨平台契约一致
- 私有化交付可复盘
- 运维手册与真实行为一致
